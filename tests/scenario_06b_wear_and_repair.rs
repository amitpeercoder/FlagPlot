//! Scenario 6b: five days of raids on an airfield, with wear and repair.
//!
//! Stage 6b of scenario 6 (decision 0035). The data is
//! `data/scenarios/06b_wear_and_repair.toml`, run with seeds 1 to 1000.
//! Everything in it is a placeholder.
//!
//! # Hand calculation
//!
//! ## The setting
//!
//! It is scenario 6a (`tests/scenario_06a_base_raid.rs`) over five days,
//! with the upkeep figures. Airfield A, base 1, Japanese, at (0,0), repairs
//! 2 aircraft a day. Airfield B, base 2, Allied, at (5,0), 200 nm away, has
//! 1,000 t of supply and 20 anti-aircraft shots and repairs 3 a day.
//!
//! | Group | Side | At | Aircraft | Mission |
//! | ----- | ---- | -- | -------- | ------- |
//! | 1 | Japanese | A | 18 bombers | base strike on airfield B |
//! | 2 | Japanese | A | 12 fighters | combat air patrol, share 0.5 |
//! | 3 | Allied | B | 12 fighters | combat air patrol, share 1 |
//! | 4 | Allied | B | 10 bombers | rest |
//! | 5 | Allied | B | 6 scouts | search, arc out to 300 nm |
//!
//! Every group starts with all its aircraft ready and none damaged.
//!
//! ## The wear chances
//!
//! The base chance is 0.05, and 0.02 more for each 100 nm of the sortie.
//!
//! | Sortie | Distance | Chance |
//! | ------ | -------- | ------ |
//! | A patrol | none | 0.05 |
//! | The raid on airfield B, bombers and escorts | 2 x 200 = 400 nm | 0.05 + 0.02 x 4 = 0.13 |
//! | A search out to 300 nm | 2 x 300 = 600 nm | 0.05 + 0.02 x 6 = 0.17 |
//!
//! Each aircraft that flew and came back is rolled on its own at the
//! chance. Each worn aircraft is rolled again at 0.2: written off if it
//! hits, damaged if not. So of n aircraft at chance p, n x p are worn on
//! average, with a standard deviation of the square root of n x p x
//! (1 - p), and 0.2 of the worn are written off on average.
//!
//! ## A day
//!
//! Write R for a group's ready aircraft and D for its damaged aircraft.
//!
//! **Night, at the start of the day: repair.** Airfield A repairs up to 2
//! damaged aircraft, group 1's first and then group 2's. Airfield B repairs
//! up to 3, group 3's first, then group 4's, then group 5's. A repaired
//! aircraft moves from damaged to ready. On day 1 nothing is damaged and
//! nothing is repaired.
//!
//! **Morning.** On the counts at the start of the pulse:
//!
//! - Group 5 searches with R5 aircraft.
//! - Group 1 launches R1 bombers at airfield B, if it has one. Group 2
//!   keeps R2 x 0.5, rounded down, over airfield A and sends the rest as
//!   escorts, if group 1 launches. Group 3's patrol is R3.
//! - The raid is worked as in scenario 6a, with one change: the aircraft on
//!   the ground at airfield B are group 4's ready aircraft and the damaged
//!   aircraft of groups 3, 4 and 5. Those destroyed are taken from group
//!   3's damaged first, then group 4's damaged, then group 4's ready, then
//!   group 5's damaged.
//! - Then the wear rolls, in group order:
//!
//! | Group | Aircraft that roll | Chance |
//! | ----- | ------------------ | ------ |
//! | 1 | The bombers that attacked | 0.13 |
//! | 2, first roll | The patrol it kept | 0.05 |
//! | 2, second roll | The escorts it sent, less those shot down | 0.13 |
//! | 3 | Its patrol, less those shot down | 0.05 |
//! | 5 | The R5 that searched | 0.17 |
//!
//! Group 4 rests and never rolls. A roll of no aircraft is not made and
//! records nothing.
//!
//! **Afternoon.** No raid flies. Group 5 searches again and rolls at 0.17.
//! Group 2 keeps R2 x 0.5, rounded down, on patrol, which rolls at 0.05; the
//! rest of group 2 does not fly. Group 3's patrol is all its ready aircraft
//! and rolls at 0.05. Group 1 does not fly.
//!
//! After each roll the group's ready aircraft fall by the worn, and its
//! damaged aircraft rise by the worn less the written off.
//!
//! ## The first morning
//!
//! Every group is whole: 18 bombers, 6 escorts and 6 kept, a patrol of 12,
//! 6 scouts. The fights are those of scenario 6a's first raid.
//!
//! | Roll | Aircraft | Chance | Expected worn |
//! | ---- | -------- | ------ | ------------- |
//! | Group 1 | 18 less its losses, 15.2 on average | 0.13 | 1.976 on average |
//! | Group 2's patrol | 6 | 0.05 | 0.3 |
//! | Group 2's escorts | 6 less its losses, 4.2 on average | 0.13 | 0.546 on average |
//! | Group 3 | 12 less its losses, 10.2 on average | 0.05 | 0.51 on average |
//! | Group 5 | 6 | 0.17 | 1.02 |
//!
//! Nothing is damaged at the start of the first morning, so the aircraft on
//! the ground at airfield B are group 4's ten, as in scenario 6a.
//!
//! ## Over the runs
//!
//! The test works every pulse of every run from the counts at the start of
//! the pulse: the repair, the raid, each wear roll's aircraft and chance.
//!
//! Over seeds 1 to 1000:
//!
//! - For each of the five kinds of roll in the table, taken over every
//!   pulse of every run, the worn aircraft in all are within four standard
//!   deviations of the sum of aircraft x chance, where the variance is the
//!   sum of aircraft x chance x (1 - chance).
//! - The aircraft written off in all are within four standard deviations
//!   of 0.2 of the worn.
//!
//! ## At the end of a run
//!
//! For every group, the ready and the damaged aircraft at the end, the
//! aircraft shot down, the aircraft destroyed on the ground and the
//! aircraft written off add up to the aircraft it started with. Repair
//! moves aircraft from damaged to ready and changes no total.
//!
//! Group 4 never flies: it has no damaged aircraft at any time, and loses
//! aircraft only on the ground.
//!
//! ## Variants
//!
//! | Variant | What changes |
//! | ------- | ------------ |
//! | The scenario names no upkeep file | No wear roll and no repair is recorded, and no aircraft is ever damaged. Each of the first three days is, event for event, scenario 6a with the same seed |
//! | Both airfields repair none | No repair is recorded. A group's damaged aircraft never fall except by being destroyed on the ground |
//! | The write-off share is 1 | Every worn aircraft is written off: no aircraft is ever damaged and nothing is repaired |
//! | The write-off share is 0 | No aircraft is written off |
//! | Both wear chances are 0 | No aircraft is worn. Rolls are still recorded, with a chance of 0 and none worn |
//! | Airfield B is at (2,0), 80 nm | The raid's sortie is 160 nm: bombers and escorts roll at 0.05 + 0.02 x 1.6 = 0.082. Patrols and the search roll as before |
//! | Airfield B repairs 10 a day | Every damaged aircraft at airfield B is ready again the next morning: none of its groups starts a day with a damaged aircraft |
//!
//! ## What cannot happen
//!
//! - No group rolls for wear in a night pulse, and nothing is repaired in a
//!   daylight pulse.
//! - Group 4 is never worn, written off or repaired.
//! - No aircraft worn in a pulse is destroyed on the ground in that pulse.
//! - A base repairs no more in a day than its figure, and never an aircraft
//!   of a group that is somewhere else.
//!
//! # Properties
//!
//! - Aircraft are conserved, as "At the end of a run" has it, for every
//!   group in every run and every variant.
//! - No count is ever negative, and a group never flies a damaged aircraft:
//!   every launch, escort, patrol and search is of ready aircraft only.
//! - A longer sortie never has a lower wear chance: 0.05 for none, 0.082
//!   for 160 nm, 0.13 for 400 nm, 0.17 for 600 nm.
//! - Repair leaves more aircraft ready: over the 1,000 seeds, the ready
//!   aircraft of group 3 at the end, in all, with airfield B repairing 3 a
//!   day are more than with none. (The first version also set 10 a day
//!   against 3. Group 3 seldom has more than 3 damaged, so the two differ
//!   by a handful of aircraft in a thousand runs, which chance could turn
//!   either way. The statement was dropped; no figure changed.)
//! - Recording breakdowns changes neither the events nor the hashes.
//! - The same seed gives the same hashes twice.
//! - A scenario without the upkeep file has the hashes it had before this
//!   stage: the golden files of scenarios 1 to 6a do not change.

// The test code is written by a developer from the hand calculation above.

mod common;

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use sim_core::events::{AirFirer, AirTarget, Cause, Event, EventKind, SortieKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, EntityRef, EventId, OrderId};
use sim_core::orders::AirMission;
use sim_core::rng::{Draw, RngKey, RngStream, Subsystem};
use sim_core::state::{AirGroupLocation, Side};
use sim_core::time::Pulse;
use sim_data::Scenario;

const BOMBERS: u32 = 1;
const ESCORTS: u32 = 2;
const DEFENDERS: u32 = 3;
const RESTING: u32 = 4;
const SCOUTS: u32 = 5;
const GROUPS: [u32; 5] = [BOMBERS, ESCORTS, DEFENDERS, RESTING, SCOUTS];
const AIRFIELD_A: BaseId = BaseId::new(1);
const AIRFIELD_B: BaseId = BaseId::new(2);
const SEEDS: u64 = 1000;
/// The seeds of a variant whose figures are asserted run by run.
const VARIANT_SEEDS: u64 = 300;
const DAYS: u32 = 5;
const START: [u32; 5] = [18, 12, 12, 10, 6];
/// The search arc's range: the sortie is twice that.
const SEARCH_NM: i64 = 300;

fn scenario_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/06b_wear_and_repair.toml")
}

fn load() -> Scenario {
    common::load(&scenario_path()).expect("scenario 6b loads")
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

/// The index of a group number in the arrays of five.
fn of(group: u32) -> usize {
    usize::try_from(group - 1).expect("a group number")
}

fn count(value: usize) -> i64 {
    i64::try_from(value).expect("small")
}

/// Whether k out of n is within four standard deviations of probability
/// `p` (in millionths), in exact integer arithmetic.
fn within_four_sigma(k: i64, n: i64, p: i64) -> bool {
    let (k, n, p) = (i128::from(k), i128::from(n), i128::from(p));
    let scale = 1_000_000;
    let off = k * scale - n * p;
    off * off <= 16 * n * p * (scale - p)
}

// ---------------------------------------------------------------------
// What one run did, read from its events and final state.
// ---------------------------------------------------------------------

struct Run {
    seed: u64,
    events: Vec<Event>,
    hashes: Vec<u64>,
    /// The aircraft ready and damaged at the end of groups 1 to 5.
    ready: [u32; 5],
    damaged: [u32; 5],
    /// Airfield B's supply at the end.
    supply_b: Fixed,
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

    /// The events of one pulse, in order.
    fn pulse(&self, day: u32, pulse: Pulse) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|event| event.day == day && event.pulse == Some(pulse))
            .collect()
    }
}

fn run_of(base: &Scenario, seed: u64) -> Run {
    let mut run = base.with_seed(seed).expect("a seed can be set");
    run.run_to_end().expect("the run resolves");
    digest(&run, seed)
}

fn digest(run: &Scenario, seed: u64) -> Run {
    let truth = run.world.umpire();
    // The claims from the air (decision 0056) are not part of what this
    // scenario works out: they come after each raid is resolved and are
    // asserted in scenario_11_claims.rs.
    let events: Vec<Event> = run
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
        .collect();
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
        ready: GROUPS.map(|id| truth.air_groups[&group(id)].ready),
        damaged: GROUPS.map(|id| truth.air_groups[&group(id)].damaged),
        supply_b: truth.bases[&AIRFIELD_B].supply_t,
        orders,
    }
}

fn batch_of(base: &Scenario, seeds: u64) -> Vec<Run> {
    (1..=seeds).map(|seed| run_of(base, seed)).collect()
}

fn batch() -> &'static Vec<Run> {
    static BATCH: OnceLock<Vec<Run>> = OnceLock::new();
    BATCH.get_or_init(|| batch_of(&load(), SEEDS))
}

/// Every pulse of every run of the scenario, worked and asserted, once.
fn worked_batch() -> &'static Vec<Worked> {
    static WORKED: OnceLock<Vec<Worked>> = OnceLock::new();
    WORKED.get_or_init(|| {
        batch()
            .iter()
            .map(|run| work_the_run(run, &scenario_config()))
            .collect()
    })
}

fn stream_of(
    seed: u64,
    day: u32,
    pulse: Pulse,
    subsystem: Subsystem,
    entity: EntityRef,
    other: Option<EntityRef>,
) -> RngStream {
    RngKey {
        seed,
        day,
        pulse: Some(pulse),
        subsystem,
        entity,
        other,
        draw: Draw::Legacy,
    }
    .stream()
}

fn air_stream(seed: u64, day: u32, pulse: Pulse, entity: EntityRef, other: EntityRef) -> RngStream {
    stream_of(seed, day, pulse, Subsystem::AirCombat, entity, Some(other))
}

/// The hits a roll of `shots` at `chance` gives from its stream: one draw
/// for each shot (decision 0033).
fn hits_from(stream: &mut RngStream, shots: u32, chance: Fixed) -> u32 {
    (0..shots).map(|_| u32::from(stream.chance(chance))).sum()
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

/// Takes `hits` from the ground at airfield B: (group, is it its damaged
/// aircraft, how many), the lowest group first, a group's damaged before its
/// ready aircraft (decision 11 of 0035).
fn take_ground(hits: u32, pool: &mut [(u32, bool, u32)]) -> Vec<(u32, bool, u32)> {
    let mut left = hits;
    let mut taken = Vec::new();
    for (group, damaged, count) in pool.iter_mut() {
        let now = left.min(*count);
        if now > 0 {
            *count -= now;
            left -= now;
            taken.push((*group, *damaged, now));
        }
    }
    taken
}

/// What the hand calculation says a sortie of `distance_nm` is worn at: the
/// base chance and the chance per 100 nm times the hundreds, 1 at most, in
/// whole millionths.
fn chance_of(config: &Config, distance_nm: i64) -> Fixed {
    Fixed::from_raw(
        (config.base_chance.raw() + config.per_100.raw() * distance_nm / 100).min(Fixed::SCALE),
    )
}

// ---------------------------------------------------------------------
// Working every pulse of a run from the counts at the start of it.
// ---------------------------------------------------------------------

/// What differs between the scenario and its variants.
#[derive(Clone)]
struct Config {
    /// What airfields A and B repair each day.
    repairs: [u32; 2],
    base_chance: Fixed,
    per_100: Fixed,
    write_off: Fixed,
    /// Airfield B's distance from airfield A, in hexes.
    raid_hexes: i64,
}

fn scenario_config() -> Config {
    Config {
        repairs: [2, 3],
        base_chance: hundredths(5),
        per_100: hundredths(2),
        write_off: tenths(2),
        raid_hexes: 5,
    }
}

impl Config {
    fn raid_nm(&self) -> i64 {
        self.raid_hexes * 40
    }
}

/// The five kinds of roll of the table of the first morning.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum RollKind {
    /// Group 1's bombers that attacked.
    Bombers,
    /// Group 2's patrol.
    EscortPatrol,
    /// Group 2's escorts.
    Escorts,
    /// Group 3's patrol.
    DefendersPatrol,
    /// Group 5's search.
    Scouts,
}

impl RollKind {
    const ALL: [RollKind; 5] = [
        RollKind::Bombers,
        RollKind::EscortPatrol,
        RollKind::Escorts,
        RollKind::DefendersPatrol,
        RollKind::Scouts,
    ];
}

/// One wear roll as the rule works it.
#[derive(Clone, Debug)]
struct Roll {
    day: u32,
    pulse: Pulse,
    kind: RollKind,
    aircraft: u32,
    chance: Fixed,
    worn: u32,
    written_off: u32,
}

/// What working one run found.
#[derive(Default)]
struct Worked {
    rolls: Vec<Roll>,
    /// The counts at the end, by the rule.
    ready: [u32; 5],
    damaged: [u32; 5],
    /// Shot down, destroyed on the ground (ready and damaged), written off
    /// and repaired, by group.
    shot_down: [u32; 5],
    on_the_ground: [u32; 5],
    written_off: [u32; 5],
    worn: [u32; 5],
    repaired: [u32; 5],
    /// The damaged aircraft of each group at the start of each morning, after
    /// the repair of the night.
    damaged_at_dawn: Vec<[u32; 5]>,
    /// For each day: the aircraft that attacked, the escorts left, group 3's
    /// aircraft left after the fights; and the ready aircraft of group 3 at
    /// the start of the morning.
    raids: Vec<RaidFigures>,
}

#[derive(Clone, Copy, Debug)]
struct RaidFigures {
    day: u32,
    bombers_attacking: u32,
    escorts_left: u32,
    defenders_left: u32,
    hits_on_aircraft: u32,
    destroyed_on_ground: u32,
}

/// What the run's state is, by the rule, as the pulses are worked.
struct State {
    ready: [u32; 5],
    damaged: [u32; 5],
    supply: i64,
    /// The last `AircraftWorn` that left aircraft damaged, by group.
    damaging_wear: [Option<EventId>; 5],
}

/// What the morning's raid left, for the wear rolls.
struct Raid {
    launch: EventId,
    sent: Option<EventId>,
    bombers_left: u32,
    escorts_left: u32,
}

/// A sortie as the wear step takes it: group, what kind of roll, how many,
/// how far (nm), and the causes the event must cite.
struct Flew {
    group: u32,
    sortie: SortieKind,
    roll: RollKind,
    aircraft: u32,
    distance_nm: i64,
    causes: Vec<Cause>,
}

/// The kinds of the morning in the order the log writes them.
fn rank(kind: &EventKind) -> u32 {
    match kind {
        EventKind::SearchFlown { .. } => 0,
        EventKind::BaseStrikeLaunched { .. } => 1,
        EventKind::EscortsSent { .. } => 2,
        EventKind::AirFight { .. } => 3,
        EventKind::BaseAntiAircraftFire { .. } => 4,
        EventKind::AircraftLost { .. } => 5,
        EventKind::BaseAttacked { .. } => 6,
        EventKind::AircraftDestroyedOnGround { .. }
        | EventKind::DamagedAircraftDestroyedOnGround { .. } => 7,
        EventKind::BaseSupplyDestroyed { .. } => 8,
        EventKind::AircraftWorn { .. } => 9,
        other => panic!("an event the day does not have: {other:?}"),
    }
}

/// Works every pulse of `run` by the rule of the hand calculation from the
/// counts at the start of it, asserts every event against it, and returns what
/// was worked. The counts are carried from pulse to pulse by the rule, not
/// read from the events.
fn work_the_run(run: &Run, config: &Config) -> Worked {
    let seed = run.seed;
    let mut state = State {
        ready: START,
        damaged: [0; 5],
        supply: 1000,
        damaging_wear: [None; 5],
    };
    let mut worked = Worked::default();
    for day in 1..=DAYS {
        // Night, at the start of the day: repair.
        let night = run.pulse(day, Pulse::Night);
        // (group, base, repaired, ready after, damaged after), airfield A's
        // groups first, then airfield B's, the lowest group first.
        let mut repairs: Vec<(u32, BaseId, u32, u32, u32)> = Vec::new();
        for (base, groups, limit) in [
            (AIRFIELD_A, [BOMBERS, ESCORTS].as_slice(), config.repairs[0]),
            (
                AIRFIELD_B,
                [DEFENDERS, RESTING, SCOUTS].as_slice(),
                config.repairs[1],
            ),
        ] {
            let mut left = limit;
            for g in groups {
                let n = left.min(state.damaged[of(*g)]);
                if n > 0 {
                    left -= n;
                    state.damaged[of(*g)] -= n;
                    state.ready[of(*g)] += n;
                    worked.repaired[of(*g)] += n;
                    repairs.push((*g, base, n, state.ready[of(*g)], state.damaged[of(*g)]));
                }
            }
        }
        assert_eq!(
            night.len(),
            repairs.len(),
            "seed {seed}, day {day}: the night has the repairs and nothing else"
        );
        for (event, (g, base, n, ready, damaged)) in night.iter().zip(&repairs) {
            assert_eq!(
                event.kind,
                EventKind::AircraftRepaired {
                    air_group: group(*g),
                    at: AirGroupLocation::Base(*base),
                    repaired: *n,
                    ready: *ready,
                    damaged: *damaged,
                },
                "seed {seed}, day {day}"
            );
            assert_eq!(
                event.kind.to_string(),
                format!(
                    "air_group:{g} had {n} aircraft repaired at base:{}, {ready} ready and {damaged} damaged",
                    base.raw()
                )
            );
            assert_eq!(
                event.causes,
                state.damaging_wear[of(*g)]
                    .into_iter()
                    .map(Cause::Event)
                    .collect::<Vec<_>>(),
                "seed {seed}: a repair cites the wear that last damaged the group"
            );
            assert!(event.breakdown.is_none());
        }
        worked.damaged_at_dawn.push(state.damaged);

        // Morning.
        work_the_morning(run, config, day, &mut state, &mut worked);
        // Afternoon.
        work_the_afternoon(run, config, day, &mut state, &mut worked);
    }

    // The end of the run.
    assert_eq!(run.ready, state.ready, "seed {seed}: ready at the end");
    assert_eq!(
        run.damaged, state.damaged,
        "seed {seed}: damaged at the end"
    );
    assert_eq!(
        run.supply_b,
        Fixed::from_int(state.supply),
        "seed {seed}: the supply at the end"
    );
    worked.ready = state.ready;
    worked.damaged = state.damaged;
    worked
}

fn work_the_morning(run: &Run, config: &Config, day: u32, state: &mut State, worked: &mut Worked) {
    let seed = run.seed;
    let pulse = Pulse::Morning;
    let step = run.pulse(day, pulse);
    // The log of the morning is in the order of the module comment.
    let ranks: Vec<u32> = step.iter().map(|event| rank(&event.kind)).collect();
    assert!(
        ranks.windows(2).all(|pair| pair[0] <= pair[1]),
        "seed {seed}, day {day}: the morning is in order, {ranks:?}"
    );
    let kinds = |keep: fn(&EventKind) -> bool| -> Vec<&Event> {
        step.iter().copied().filter(|e| keep(&e.kind)).collect()
    };
    let searches = kinds(|k| matches!(k, EventKind::SearchFlown { .. }));
    let launches = kinds(|k| matches!(k, EventKind::BaseStrikeLaunched { .. }));
    let sent = kinds(|k| matches!(k, EventKind::EscortsSent { .. }));
    let fights = kinds(|k| matches!(k, EventKind::AirFight { .. }));
    let guns = kinds(|k| matches!(k, EventKind::BaseAntiAircraftFire { .. }));
    let lost = kinds(|k| matches!(k, EventKind::AircraftLost { .. }));
    let attacked = kinds(|k| matches!(k, EventKind::BaseAttacked { .. }));
    let on_ground_events = kinds(|k| {
        matches!(
            k,
            EventKind::AircraftDestroyedOnGround { .. }
                | EventKind::DamagedAircraftDestroyedOnGround { .. }
        )
    });
    let supply_events = kinds(|k| matches!(k, EventKind::BaseSupplyDestroyed { .. }));
    let wear_events = kinds(|k| matches!(k, EventKind::AircraftWorn { .. }));
    assert_eq!(
        step.len(),
        searches.len()
            + launches.len()
            + sent.len()
            + fights.len()
            + guns.len()
            + lost.len()
            + attacked.len()
            + on_ground_events.len()
            + supply_events.len()
            + wear_events.len(),
        "seed {seed}, day {day}"
    );

    // Group 5 searches with what it has ready.
    let r5 = state.ready[of(SCOUTS)];
    let search_event = if r5 > 0 {
        assert_eq!(searches.len(), 1, "seed {seed}, day {day}");
        assert_eq!(
            searches[0].kind,
            EventKind::SearchFlown {
                air_group: group(SCOUTS),
                from: AirGroupLocation::Base(AIRFIELD_B),
                hex: Hex::new(config.raid_hexes as i32, 0),
                from_deg: 40,
                to_deg: 130,
                range_nm: Fixed::from_int(SEARCH_NM),
                aircraft: r5,
            },
            "seed {seed}, day {day}"
        );
        Some(searches[0].id)
    } else {
        assert!(searches.is_empty(), "seed {seed}, day {day}");
        None
    };

    // The raid, as in scenario 6a.
    let b = state.ready[of(BOMBERS)];
    let f = state.ready[of(ESCORTS)];
    let keep = f * 5 / 10;
    let mut flew: Vec<Flew> = Vec::new();
    let raid = if b == 0 {
        assert!(
            launches.is_empty() && sent.is_empty() && fights.is_empty() && guns.is_empty(),
            "seed {seed}, day {day}: no aircraft, no raid"
        );
        assert!(lost.is_empty() && attacked.is_empty() && on_ground_events.is_empty());
        assert!(supply_events.is_empty());
        worked.raids.push(RaidFigures {
            day,
            bombers_attacking: 0,
            escorts_left: 0,
            defenders_left: state.ready[of(DEFENDERS)],
            hits_on_aircraft: 0,
            destroyed_on_ground: 0,
        });
        None
    } else {
        Some(work_the_raid(
            run,
            config,
            day,
            state,
            worked,
            &step,
            &launches,
            &sent,
            &fights,
            &guns,
            &lost,
            &attacked,
            &on_ground_events,
            &supply_events,
        ))
    };
    if raid.is_none() {
        // No raid from airfield A: group 2's patrol is its share and the rest
        // do not fly.
        assert!(sent.is_empty());
    }

    // What flew, by group in order of group number, a patrol before the
    // escorts of the same group.
    if let Some(raid) = &raid {
        if raid.bombers_left > 0 {
            flew.push(Flew {
                group: BOMBERS,
                sortie: SortieKind::BaseStrike,
                roll: RollKind::Bombers,
                aircraft: raid.bombers_left,
                distance_nm: 2 * config.raid_nm(),
                causes: vec![Cause::Event(raid.launch)],
            });
        }
    }
    if keep > 0 {
        flew.push(Flew {
            group: ESCORTS,
            sortie: SortieKind::Patrol,
            roll: RollKind::EscortPatrol,
            aircraft: keep,
            distance_nm: 0,
            causes: run
                .orders_of(ESCORTS)
                .into_iter()
                .map(Cause::Order)
                .collect(),
        });
    }
    if let Some(raid) = &raid {
        if raid.escorts_left > 0 {
            flew.push(Flew {
                group: ESCORTS,
                sortie: SortieKind::Escort,
                roll: RollKind::Escorts,
                aircraft: raid.escorts_left,
                distance_nm: 2 * config.raid_nm(),
                causes: vec![Cause::Event(raid.sent.expect("escorts were sent"))],
            });
        }
    }
    if state.ready[of(DEFENDERS)] > 0 {
        flew.push(Flew {
            group: DEFENDERS,
            sortie: SortieKind::Patrol,
            roll: RollKind::DefendersPatrol,
            aircraft: state.ready[of(DEFENDERS)],
            distance_nm: 0,
            causes: run
                .orders_of(DEFENDERS)
                .into_iter()
                .map(Cause::Order)
                .collect(),
        });
    }
    if let Some(search) = search_event {
        flew.push(Flew {
            group: SCOUTS,
            sortie: SortieKind::Search,
            roll: RollKind::Scouts,
            aircraft: r5,
            distance_nm: 2 * SEARCH_NM,
            causes: vec![Cause::Event(search)],
        });
    }
    work_the_wear(run, config, day, pulse, &wear_events, flew, state, worked);
}

fn work_the_afternoon(
    run: &Run,
    config: &Config,
    day: u32,
    state: &mut State,
    worked: &mut Worked,
) {
    let seed = run.seed;
    let pulse = Pulse::Afternoon;
    let step = run.pulse(day, pulse);
    let mut searches = Vec::new();
    let mut wear_events = Vec::new();
    for event in &step {
        match event.kind {
            EventKind::SearchFlown { .. } => {
                assert!(
                    wear_events.is_empty(),
                    "seed {seed}: the search comes first"
                );
                searches.push(*event);
            }
            EventKind::AircraftWorn { .. } => wear_events.push(*event),
            ref other => {
                panic!("seed {seed}, day {day}: no raid flies in the afternoon: {other:?}")
            }
        }
    }
    let r5 = state.ready[of(SCOUTS)];
    let mut flew = Vec::new();
    // Group 2 keeps its share on patrol; the rest of it does not fly.
    let keep = state.ready[of(ESCORTS)] * 5 / 10;
    if keep > 0 {
        flew.push(Flew {
            group: ESCORTS,
            sortie: SortieKind::Patrol,
            roll: RollKind::EscortPatrol,
            aircraft: keep,
            distance_nm: 0,
            causes: run
                .orders_of(ESCORTS)
                .into_iter()
                .map(Cause::Order)
                .collect(),
        });
    }
    // Group 3's patrol is all its ready aircraft.
    if state.ready[of(DEFENDERS)] > 0 {
        flew.push(Flew {
            group: DEFENDERS,
            sortie: SortieKind::Patrol,
            roll: RollKind::DefendersPatrol,
            aircraft: state.ready[of(DEFENDERS)],
            distance_nm: 0,
            causes: run
                .orders_of(DEFENDERS)
                .into_iter()
                .map(Cause::Order)
                .collect(),
        });
    }
    if r5 > 0 {
        assert_eq!(searches.len(), 1, "seed {seed}, day {day}");
        assert_eq!(
            searches[0].kind,
            EventKind::SearchFlown {
                air_group: group(SCOUTS),
                from: AirGroupLocation::Base(AIRFIELD_B),
                hex: Hex::new(config.raid_hexes as i32, 0),
                from_deg: 40,
                to_deg: 130,
                range_nm: Fixed::from_int(SEARCH_NM),
                aircraft: r5,
            }
        );
        flew.push(Flew {
            group: SCOUTS,
            sortie: SortieKind::Search,
            roll: RollKind::Scouts,
            aircraft: r5,
            distance_nm: 2 * SEARCH_NM,
            causes: vec![Cause::Event(searches[0].id)],
        });
    } else {
        assert!(searches.is_empty(), "seed {seed}, day {day}");
    }
    work_the_wear(run, config, day, pulse, &wear_events, flew, state, worked);
}

/// The wear rolls of one pulse: each sortie of `flew`, in order, rolled from
/// the stream of its group (a group that rolls twice rolls its patrol first),
/// and asserted against the `AircraftWorn` events of the pulse.
#[allow(clippy::too_many_arguments)]
fn work_the_wear(
    run: &Run,
    config: &Config,
    day: u32,
    pulse: Pulse,
    events: &[&Event],
    flew: Vec<Flew>,
    state: &mut State,
    worked: &mut Worked,
) {
    let seed = run.seed;
    assert_eq!(
        events.len(),
        flew.len(),
        "seed {seed}, day {day}, {pulse:?}: one wear event for each sortie that flew"
    );
    let mut streams: BTreeMap<u32, RngStream> = BTreeMap::new();
    for (event, sortie) in events.iter().zip(&flew) {
        let g = sortie.group;
        let chance = chance_of(config, sortie.distance_nm);
        let aircraft = sortie.aircraft;
        let stream = streams.entry(g).or_insert_with(|| {
            stream_of(
                seed,
                day,
                pulse,
                Subsystem::Upkeep,
                EntityRef::AirGroup(group(g)),
                None,
            )
        });
        let expected = int(aircraft) * chance;
        let worn = hits_from(stream, aircraft, chance);
        let written_off = hits_from(stream, worn, config.write_off);
        state.ready[of(g)] -= worn;
        state.damaged[of(g)] += worn - written_off;
        worked.worn[of(g)] += worn;
        worked.written_off[of(g)] += written_off;
        assert_eq!(
            event.kind,
            EventKind::AircraftWorn {
                air_group: group(g),
                sortie: sortie.sortie,
                distance_nm: Fixed::from_int(sortie.distance_nm),
                aircraft,
                chance,
                expected,
                worn,
                written_off,
                ready: state.ready[of(g)],
                damaged: state.damaged[of(g)],
            },
            "seed {seed}, day {day}, {pulse:?}, group {g}"
        );
        assert_eq!(
            event.kind.to_string(),
            format!(
                "air_group:{g} {} sortie of {} nm: {aircraft} aircraft x {chance} = {expected} expected worn, {worn} worn, {written_off} written off, {} ready and {} damaged left",
                sortie.sortie.name(),
                sortie.distance_nm,
                state.ready[of(g)],
                state.damaged[of(g)]
            )
        );
        assert_eq!(event.causes, sortie.causes, "seed {seed}, group {g}");
        let breakdown = event.breakdown.as_ref().expect("a breakdown");
        assert_eq!(breakdown.result, expected);
        assert_eq!(
            breakdown
                .factors
                .iter()
                .map(|f| (f.name, f.value))
                .collect::<Vec<_>>(),
            vec![
                ("aircraft", int(aircraft)),
                ("wear_base_chance", config.base_chance),
                ("wear_chance_per_100_nm", config.per_100),
                ("distance_nm", Fixed::from_int(sortie.distance_nm)),
                ("chance", chance),
                ("write_off_share", config.write_off),
            ]
        );
        if worn > written_off {
            state.damaging_wear[of(g)] = Some(event.id);
        }
        worked.rolls.push(Roll {
            day,
            pulse,
            kind: sortie.roll,
            aircraft,
            chance,
            worn,
            written_off,
        });
    }
}

/// The morning's raid worked as in scenario 6a, with the aircraft on the
/// ground at airfield B being group 4's ready aircraft and the damaged
/// aircraft of groups 3, 4 and 5 (decision 11 of 0035). The raid is alone, so
/// it meets the whole patrol.
#[allow(clippy::too_many_arguments)]
fn work_the_raid(
    run: &Run,
    config: &Config,
    day: u32,
    state: &mut State,
    worked: &mut Worked,
    _step: &[&Event],
    launches: &[&Event],
    sent: &[&Event],
    fights: &[&Event],
    guns: &[&Event],
    lost: &[&Event],
    attacked: &[&Event],
    on_ground_events: &[&Event],
    supply_events: &[&Event],
) -> Raid {
    let seed = run.seed;
    let pulse = Pulse::Morning;
    let b = state.ready[of(BOMBERS)];
    assert_eq!(launches.len(), 1, "seed {seed}, day {day}");
    let launch = launches[0];
    assert_eq!(
        launch.kind,
        EventKind::BaseStrikeLaunched {
            air_group: group(BOMBERS),
            base: AIRFIELD_B,
            hex: Hex::new(config.raid_hexes as i32, 0),
            aircraft: b,
            distance_nm: Fixed::from_int(config.raid_nm()),
        },
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

    // F, E, P at the start of the pulse.
    let f = state.ready[of(ESCORTS)];
    let e = f - f * 5 / 10;
    let defenders = state.ready[of(DEFENDERS)];
    let mut patrol_pool: Vec<(u32, u32)> = [(DEFENDERS, defenders)]
        .into_iter()
        .filter(|(_, count)| *count > 0)
        .collect();
    let p = defenders;
    // The ground: by group, the damaged aircraft of every group at airfield B
    // and the ready aircraft of the group that rests.
    let mut ground_pool: Vec<(u32, bool, u32)> = Vec::new();
    for g in [DEFENDERS, RESTING, SCOUTS] {
        if state.damaged[of(g)] > 0 {
            ground_pool.push((g, true, state.damaged[of(g)]));
        }
        if g == RESTING && state.ready[of(g)] > 0 {
            ground_pool.push((g, false, state.ready[of(g)]));
        }
    }
    let on_ground: u32 = ground_pool.iter().map(|(_, _, count)| count).sum();

    let mut escorts_sent = None;
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
        let mut causes: Vec<Cause> = run
            .orders_of(ESCORTS)
            .into_iter()
            .map(Cause::Order)
            .collect();
        causes.push(Cause::Event(launch.id));
        assert_eq!(sent[0].causes, causes, "seed {seed}");
        escorts_sent = Some(sent[0].id);
    } else {
        assert!(sent.is_empty(), "seed {seed}");
    }

    // The fights, on the counts at the start.
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
    let mut escort_stream = air_stream(
        seed,
        day,
        pulse,
        EntityRef::AirGroup(group(ESCORTS)),
        EntityRef::Base(AIRFIELD_B),
    );
    let mut patrol_stream = first_patrol.map(|first| {
        air_stream(
            seed,
            day,
            pulse,
            EntityRef::AirGroup(group(first)),
            EntityRef::AirGroup(group(BOMBERS)),
        )
    });
    // (group, aircraft, remaining, the roll that cost it) in order.
    let mut expected_losses: Vec<(u32, u32, u32, EventId)> = Vec::new();
    let mut now = state.ready;
    let mut bomber_pool = [(BOMBERS, b)];
    let mut escort_pool = [(ESCORTS, e)];
    for ((firer, target, firing, at, shots), event) in planned.iter().zip(fights) {
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
        let mut causes = vec![Cause::Event(launch.id)];
        if *firing == AirFirer::Escorts || *at == AirTarget::Escorts {
            causes.push(Cause::Event(sent[0].id));
        }
        assert_eq!(event.causes, causes, "seed {seed}");
        let taken = match at {
            AirTarget::Patrol => take(hits, &mut patrol_pool),
            AirTarget::Escorts => take(hits, &mut escort_pool),
            AirTarget::Bombers => take(hits, &mut bomber_pool),
        };
        for (g, n) in taken {
            now[of(g)] -= n;
            expected_losses.push((g, n, now[of(g)], event.id));
        }
    }

    // The guns of airfield B, at the bombers left.
    if bomber_pool[0].1 > 0 {
        assert_eq!(guns.len(), 1, "seed {seed}, day {day}");
        let mut stream = air_stream(
            seed,
            day,
            pulse,
            EntityRef::Base(AIRFIELD_B),
            EntityRef::AirGroup(group(BOMBERS)),
        );
        let hits = hits_from(&mut stream, 20, hundredths(5));
        let expected = int(20) * hundredths(5);
        assert_eq!(
            guns[0].kind,
            EventKind::BaseAntiAircraftFire {
                base: AIRFIELD_B,
                air_group: group(BOMBERS),
                shots: int(20),
                chance: hundredths(5),
                expected,
                hits
            },
            "seed {seed}"
        );
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
        worked.shot_down[of(*g)] += n;
    }
    let bombers_left = bomber_pool[0].1;
    state.ready = now;

    // The bombers left each drop one bomb, whatever the type carries.
    let mut on_aircraft = 0;
    let mut destroyed_on_ground = 0;
    if bombers_left == 0 {
        assert!(
            attacked.is_empty() && on_ground_events.is_empty() && supply_events.is_empty(),
            "seed {seed}, day {day}: no bombers left, no attack"
        );
    } else {
        assert_eq!(attacked.len(), 1, "seed {seed}, day {day}");
        let mut stream = air_stream(
            seed,
            day,
            pulse,
            EntityRef::AirGroup(group(BOMBERS)),
            EntityRef::Base(AIRFIELD_B),
        );
        let hits = hits_from(&mut stream, bombers_left, tenths(2));
        let mut on_supply = 0;
        for _ in 0..hits {
            let draw = stream.below(Fixed::SCALE.unsigned_abs());
            if draw < 300_000 {
                on_aircraft += 1;
            } else if draw < 600_000 {
                on_supply += 1;
            }
        }
        let expected = int(bombers_left) * tenths(2);
        assert_eq!(
            attacked[0].kind,
            EventKind::BaseAttacked {
                air_group: group(BOMBERS),
                base: AIRFIELD_B,
                shots: int(bombers_left),
                chance: tenths(2),
                expected,
                hits,
                hits_on_aircraft: on_aircraft,
                hits_on_supply: on_supply,
            },
            "seed {seed}, day {day}"
        );
        let mut causes = vec![Cause::Event(launch.id)];
        causes.extend(
            expected_losses
                .iter()
                .zip(lost)
                .filter(|((g, ..), _)| *g == BOMBERS)
                .map(|(_, event)| Cause::Event(event.id)),
        );
        assert_eq!(attacked[0].causes, causes, "seed {seed}");

        // Hits on aircraft destroy aircraft on the ground, the lowest group
        // first, its damaged before its ready aircraft, and no more than were
        // there at the start of the pulse.
        let taken = take_ground(on_aircraft, &mut ground_pool);
        assert_eq!(
            on_ground_events.len(),
            taken.len(),
            "seed {seed}, day {day}"
        );
        for (event, (g, damaged, n)) in on_ground_events.iter().zip(&taken) {
            destroyed_on_ground += n;
            worked.on_the_ground[of(*g)] += n;
            if *damaged {
                state.damaged[of(*g)] -= n;
                assert_eq!(
                    event.kind,
                    EventKind::DamagedAircraftDestroyedOnGround {
                        air_group: group(*g),
                        base: AIRFIELD_B,
                        aircraft: *n,
                        remaining: state.damaged[of(*g)],
                    },
                    "seed {seed}, day {day}"
                );
                assert_eq!(
                    event.kind.to_string(),
                    format!(
                        "air_group:{g} lost {n} damaged aircraft on the ground at base:2, {} damaged left",
                        state.damaged[of(*g)]
                    )
                );
            } else {
                state.ready[of(*g)] -= n;
                assert_eq!(
                    event.kind,
                    EventKind::AircraftDestroyedOnGround {
                        air_group: group(*g),
                        base: AIRFIELD_B,
                        aircraft: *n,
                        remaining: state.ready[of(*g)],
                    },
                    "seed {seed}, day {day}"
                );
                assert_eq!(
                    event.kind.to_string(),
                    format!(
                        "air_group:{g} lost {n} aircraft on the ground at base:2, {} left",
                        state.ready[of(*g)]
                    )
                );
            }
            assert_eq!(
                event.causes,
                vec![Cause::Event(attacked[0].id)],
                "seed {seed}"
            );
            let breakdown = event.breakdown.as_ref().expect("a breakdown");
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
        let wanted = 50 * i64::from(on_supply);
        let destroyed = wanted.min(state.supply);
        if destroyed > 0 {
            assert_eq!(supply_events.len(), 1, "seed {seed}, day {day}");
            state.supply -= destroyed;
            assert_eq!(
                supply_events[0].kind,
                EventKind::BaseSupplyDestroyed {
                    base: AIRFIELD_B,
                    destroyed_t: Fixed::from_int(destroyed),
                    supply_t: Fixed::from_int(state.supply),
                },
                "seed {seed}"
            );
        } else {
            assert!(supply_events.is_empty(), "seed {seed}, day {day}");
        }
    }
    worked.raids.push(RaidFigures {
        day,
        bombers_attacking: bombers_left,
        escorts_left: escort_pool[0].1,
        defenders_left: state.ready[of(DEFENDERS)],
        hits_on_aircraft: on_aircraft,
        destroyed_on_ground,
    });
    Raid {
        launch: launch.id,
        sent: escorts_sent,
        bombers_left,
        escorts_left: escort_pool[0].1,
    }
}

// ---------------------------------------------------------------------
// Variants: the scenario with one thing changed, written under the build
// directory. Nothing in the repository's data folder is touched.
// ---------------------------------------------------------------------

/// Loads the scenario on the frozen figures with each exact, asserted-unique
/// text replacement made in its text and in a copy of the upkeep figures.
fn variant(tag: &str, scenario_edits: &[(&str, &str)], upkeep_edits: &[(&str, &str)]) -> Scenario {
    let path = scenario_path();
    let mut text = fs::read_to_string(&path).expect("read the scenario");
    for (original, changed) in scenario_edits {
        text = common::replace_once(&text, original, changed);
    }
    let options = common::altered_reference(tag, |dir| {
        common::edit_file_by(dir, "air_upkeep.toml", upkeep_edits);
    });
    common::from_text_with(&text, &path, &options).expect("the variant loads")
}

const UPKEEP_KEY: &str = "air_upkeep = \"../reference/air_upkeep.toml\"\n";
const REPAIRS_A: &str = "supply_t = 0\naircraft_repairs_per_day = 2";
const REPAIRS_B: &str = "anti_aircraft = 20\naircraft_repairs_per_day = 3";
const WRITE_OFF: &str = "write_off_share = \"0.2\"";
const BASE_CHANCE: &str = "wear_base_chance = \"0.05\"";
const PER_100: &str = "wear_chance_per_100_nm = \"0.02\"";

fn without_upkeep() -> Scenario {
    variant("no_upkeep", &[(UPKEEP_KEY, "")], &[])
}

fn repairs(a: u32, b: u32) -> Scenario {
    let mut edits = Vec::new();
    let (text_a, text_b);
    if a != 2 {
        text_a = format!("supply_t = 0\naircraft_repairs_per_day = {a}");
        edits.push((REPAIRS_A, text_a.as_str()));
    }
    if b != 3 {
        text_b = format!("anti_aircraft = 20\naircraft_repairs_per_day = {b}");
        edits.push((REPAIRS_B, text_b.as_str()));
    }
    variant(&format!("repairs_{a}_{b}"), &edits, &[])
}

fn write_off_share(share: &str) -> Scenario {
    variant(
        &format!("write_off_{share}"),
        &[],
        &[(WRITE_OFF, &format!("write_off_share = \"{share}\""))],
    )
}

fn no_wear_chances() -> Scenario {
    variant(
        "no_chances",
        &[],
        &[
            (BASE_CHANCE, "wear_base_chance = \"0\""),
            (PER_100, "wear_chance_per_100_nm = \"0\""),
        ],
    )
}

fn airfield_b_at_80_nm() -> Scenario {
    variant(
        "near",
        &[(
            "q = 5\nr = 0\nsupply_t = 1000",
            "q = 2\nr = 0\nsupply_t = 1000",
        )],
        &[],
    )
}

fn config_with(repairs: [u32; 2]) -> Config {
    Config {
        repairs,
        ..scenario_config()
    }
}

/// Every pulse of the runs of a variant worked by `config`.
fn work_all(runs: &[Run], config: &Config) -> Vec<Worked> {
    runs.iter().map(|run| work_the_run(run, config)).collect()
}

/// The aircraft of each group are conserved in one run, from its events alone:
/// ready and damaged at the end, shot down, destroyed on the ground and
/// written off add up to the aircraft it started with.
fn assert_conserved(run: &Run) {
    for g in GROUPS {
        let mut gone = 0;
        for event in &run.events {
            match event.kind {
                EventKind::AircraftLost {
                    air_group,
                    aircraft,
                    ..
                }
                | EventKind::AircraftDestroyedOnGround {
                    air_group,
                    aircraft,
                    ..
                }
                | EventKind::DamagedAircraftDestroyedOnGround {
                    air_group,
                    aircraft,
                    ..
                } if air_group == group(g) => gone += aircraft,
                EventKind::AircraftWorn {
                    air_group,
                    written_off,
                    ..
                } if air_group == group(g) => gone += written_off,
                _ => {}
            }
        }
        assert_eq!(
            run.ready[of(g)] + run.damaged[of(g)] + gone,
            START[of(g)],
            "seed {}, group {g}: the aircraft are conserved",
            run.seed
        );
    }
}

// ---------------------------------------------------------------------
// The setting and the wear chances.
// ---------------------------------------------------------------------

#[test]
fn the_setting_is_as_the_hand_calculation_says() {
    let scenario = load();
    assert_eq!(scenario.days, 5);
    let truth = scenario.world.umpire();
    let (a, b) = (&truth.bases[&AIRFIELD_A], &truth.bases[&AIRFIELD_B]);
    assert_eq!((a.side, a.hex), (Side::Japanese, Hex::new(0, 0)));
    assert_eq!((b.side, b.hex), (Side::Allied, Hex::new(5, 0)));
    assert_eq!(a.hex.distance(b.hex), 5);
    assert_eq!(b.supply_t, Fixed::from_int(1000));
    assert_eq!((a.anti_aircraft, b.anti_aircraft), (0, 20));
    assert_eq!(
        (a.aircraft_repairs_per_day, b.aircraft_repairs_per_day),
        (2, 3)
    );
    let at = AirGroupLocation::Base;
    for (id, side, base, aircraft_type, aircraft) in [
        (BOMBERS, Side::Japanese, AIRFIELD_A, 3, 18),
        (ESCORTS, Side::Japanese, AIRFIELD_A, 4, 12),
        (DEFENDERS, Side::Allied, AIRFIELD_B, 4, 12),
        (RESTING, Side::Allied, AIRFIELD_B, 3, 10),
        (SCOUTS, Side::Allied, AIRFIELD_B, 2, 6),
    ] {
        let g = &truth.air_groups[&group(id)];
        assert_eq!(
            (
                g.side,
                g.location,
                g.aircraft_type.raw(),
                g.ready,
                g.damaged
            ),
            (side, at(base), aircraft_type, aircraft, 0),
            "group {id}: every group starts with all its aircraft ready and none damaged"
        );
    }
    let figures = scenario
        .world
        .ref_data()
        .air_upkeep
        .expect("the upkeep figures are loaded");
    assert_eq!(figures.wear_base_chance, hundredths(5));
    assert_eq!(figures.wear_chance_per_100_nm, hundredths(2));
    assert_eq!(figures.write_off_share, tenths(2));
    assert_eq!(figures.carrier_repairs_per_day, 2);
    // The orders, as they stand after the run.
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

#[test]
fn the_wear_chances_are_those_of_the_table() {
    let scenario = load();
    let figures = scenario
        .world
        .ref_data()
        .air_upkeep
        .expect("the upkeep figures are loaded");
    let config = scenario_config();
    // A patrol 0.05, the raid 2 x 200 = 400 nm 0.13, a search out to 300 nm
    // 2 x 300 = 600 nm 0.17; the 160 nm of the near variant 0.082.
    for (distance, chance) in [
        (0, hundredths(5)),
        (160, Fixed::from_raw(82_000)),
        (400, hundredths(13)),
        (600, hundredths(17)),
    ] {
        assert_eq!(chance_of(&config, distance), chance, "{distance} nm");
        assert_eq!(
            figures.wear_chance(Fixed::from_int(distance)),
            chance,
            "{distance} nm"
        );
    }
    // It never passes 1.
    assert_eq!(figures.wear_chance(Fixed::from_int(100_000)), Fixed::ONE);
    // Every roll of every run is at one of the three chances, by its kind.
    for worked in worked_batch() {
        for roll in &worked.rolls {
            let expected = match roll.kind {
                RollKind::Bombers | RollKind::Escorts => hundredths(13),
                RollKind::EscortPatrol | RollKind::DefendersPatrol => hundredths(5),
                RollKind::Scouts => hundredths(17),
            };
            assert_eq!(roll.chance, expected);
        }
    }
}

// ---------------------------------------------------------------------
// The days.
// ---------------------------------------------------------------------

#[test]
fn every_pulse_of_every_run_is_worked_by_the_rule_from_the_counts_at_the_start_of_it() {
    // `worked_batch` asserts every repair, raid and wear event of every
    // pulse of every run, and the counts at the end of each run.
    let worked = worked_batch();
    assert_eq!(worked.len(), 1000);
    for (run, worked) in batch().iter().zip(worked) {
        assert_eq!(worked.damaged_at_dawn.len(), 5, "seed {}", run.seed);
        assert_eq!(worked.raids.len(), 5, "seed {}", run.seed);
        // The first raid always flies.
        assert!(worked.raids[0].bombers_attacking > 0, "seed {}", run.seed);
    }
}

#[test]
fn the_day_runs_repair_at_night_then_search_raid_and_wear_in_the_morning_then_search_and_wear() {
    for run in batch().iter().take(100) {
        for day in 1..=DAYS {
            for event in run.pulse(day, Pulse::Night) {
                assert!(
                    matches!(event.kind, EventKind::AircraftRepaired { .. }),
                    "seed {}: night has only repairs: {:?}",
                    run.seed,
                    event.kind
                );
            }
            // In a daylight pulse the wear events come last, and there is no
            // repair.
            for pulse in [Pulse::Morning, Pulse::Afternoon] {
                let step = run.pulse(day, pulse);
                let first_wear = step
                    .iter()
                    .position(|e| matches!(e.kind, EventKind::AircraftWorn { .. }))
                    .unwrap_or(step.len());
                for (index, event) in step.iter().enumerate() {
                    assert!(!matches!(event.kind, EventKind::AircraftRepaired { .. }));
                    assert_eq!(
                        matches!(event.kind, EventKind::AircraftWorn { .. }),
                        index >= first_wear,
                        "seed {}: wear comes after the air combat",
                        run.seed
                    );
                }
            }
            // The afternoon has no raid.
            assert!(!run
                .pulse(day, Pulse::Afternoon)
                .iter()
                .any(|e| matches!(e.kind, EventKind::BaseStrikeLaunched { .. })));
        }
        // On day 1 nothing is damaged and nothing is repaired.
        assert!(run.pulse(1, Pulse::Night).is_empty(), "seed {}", run.seed);
    }
}

#[test]
fn the_first_morning_has_the_rolls_of_the_table() {
    let worked = worked_batch();
    // Per kind: the sum over the runs of the aircraft that rolled.
    let mut aircraft = [0i64; 5];
    let mut expected_worn_x1000 = [0i64; 5];
    for worked in worked {
        // Nothing is damaged at the start of the first morning, so the
        // aircraft on the ground at airfield B are group 4's ten.
        assert_eq!(worked.damaged_at_dawn[0], [0; 5]);
        let first = worked.raids[0];
        assert_eq!(first.day, 1);
        // The ground holds group 4's ten and nothing else.
        assert_eq!(first.destroyed_on_ground, first.hits_on_aircraft.min(10));
        assert!(first.escorts_left <= 6 && first.defenders_left <= 12);
        for roll in worked
            .rolls
            .iter()
            .filter(|roll| roll.day == 1 && roll.pulse == Pulse::Morning)
        {
            let index = RollKind::ALL
                .iter()
                .position(|kind| *kind == roll.kind)
                .expect("a kind");
            aircraft[index] += i64::from(roll.aircraft);
            expected_worn_x1000[index] += i64::from(roll.aircraft) * roll.chance.raw() / 1000;
            match roll.kind {
                // Group 2 keeps 6 and group 5 sends 6 scouts, always.
                RollKind::EscortPatrol => assert_eq!(roll.aircraft, 6),
                RollKind::Scouts => assert_eq!(roll.aircraft, 6),
                RollKind::Bombers => assert!(roll.aircraft <= 18),
                RollKind::Escorts => assert!(roll.aircraft <= 6),
                RollKind::DefendersPatrol => assert!(roll.aircraft <= 12),
            }
        }
    }
    let runs = 1000;
    // Group 2's patrol: 6 x 0.05 = 0.3; group 5: 6 x 0.17 = 1.02.
    assert_eq!(aircraft[1], 6 * runs);
    assert_eq!(expected_worn_x1000[1], 300 * runs);
    assert_eq!(aircraft[4], 6 * runs);
    assert_eq!(expected_worn_x1000[4], 1020 * runs);
    // The others are 18, 6 and 12 less their losses: 15.2, 4.2 and 10.2 on
    // average, within four standard deviations of 1,000 runs.
    for (index, mean_x10, tolerance) in [(0, 152, 190), (2, 42, 142), (3, 102, 142)] {
        let off = aircraft[index] - mean_x10 * runs / 10;
        eprintln!(
            "first morning, kind {index}: {} aircraft, {off:+} from the mean",
            aircraft[index]
        );
        assert!(off.abs() <= tolerance, "kind {index}: {off:+}");
    }
    // The expected worn of the table on average: 1.976, 0.546 and 0.51.
    for (index, mean_x1000, tolerance) in [(0, 1976, 25), (2, 546, 19), (3, 510, 8)] {
        let off = expected_worn_x1000[index] / 1000 - mean_x1000 * runs / 1000;
        eprintln!("first morning, kind {index}: expected worn {off:+} from the mean");
        assert!(off.abs() <= tolerance, "kind {index}: {off:+}");
    }
}

#[test]
fn over_the_runs_the_worn_and_the_written_off_are_within_four_standard_deviations() {
    let mut worn = [0i64; 5];
    let mut mean_raw = [0i128; 5];
    let mut variance_raw = [0i128; 5];
    let (mut all_worn, mut all_written_off) = (0i64, 0i64);
    for worked in worked_batch() {
        for roll in &worked.rolls {
            let index = RollKind::ALL
                .iter()
                .position(|kind| *kind == roll.kind)
                .expect("a kind");
            let (a, p) = (i128::from(roll.aircraft), i128::from(roll.chance.raw()));
            worn[index] += i64::from(roll.worn);
            mean_raw[index] += a * p;
            variance_raw[index] += a * p * (1_000_000 - p);
            all_worn += i64::from(roll.worn);
            all_written_off += i64::from(roll.written_off);
        }
    }
    for (index, kind) in RollKind::ALL.iter().enumerate() {
        let off = i128::from(worn[index]) * 1_000_000 - mean_raw[index];
        eprintln!(
            "{kind:?}: {} worn, {} expected",
            worn[index],
            mean_raw[index] / 1_000_000
        );
        assert!(
            off * off <= 16 * variance_raw[index],
            "{kind:?}: {} worn against {} expected",
            worn[index],
            mean_raw[index] / 1_000_000
        );
    }
    eprintln!("written off {all_written_off} of {all_worn} worn");
    assert!(within_four_sigma(all_written_off, all_worn, 200_000));
}

#[test]
fn at_the_end_of_a_run_the_aircraft_add_up_and_group_4_never_flies() {
    let (mut ground_4, mut runs_with_damage) = (0, 0);
    for (run, worked) in batch().iter().zip(worked_batch()) {
        assert_conserved(run);
        // By the rule too.
        for g in GROUPS {
            assert_eq!(
                worked.ready[of(g)]
                    + worked.damaged[of(g)]
                    + worked.shot_down[of(g)]
                    + worked.on_the_ground[of(g)]
                    + worked.written_off[of(g)],
                START[of(g)],
                "seed {}, group {g}",
                run.seed
            );
        }
        // Repair moves aircraft from damaged to ready and changes no total:
        // worn less written off less repaired less the damaged destroyed on
        // the ground is what is damaged at the end.
        for g in GROUPS {
            assert!(
                worked.worn[of(g)] - worked.written_off[of(g)]
                    >= worked.repaired[of(g)] + worked.damaged[of(g)],
                "seed {}, group {g}",
                run.seed
            );
        }
        // Group 4 never flies: nothing is damaged, and it loses aircraft only
        // on the ground.
        assert_eq!(worked.damaged[of(RESTING)], 0);
        assert!(worked.damaged_at_dawn.iter().all(|d| d[of(RESTING)] == 0));
        assert_eq!(worked.shot_down[of(RESTING)], 0);
        assert_eq!(worked.written_off[of(RESTING)], 0);
        assert_eq!(
            run.ready[of(RESTING)],
            10 - worked.on_the_ground[of(RESTING)],
            "seed {}",
            run.seed
        );
        ground_4 += worked.on_the_ground[of(RESTING)];
        runs_with_damage += usize::from(run.damaged.iter().any(|d| *d > 0));
    }
    eprintln!("group 4 lost {ground_4} on the ground in all; {runs_with_damage} runs end with damaged aircraft");
    assert!(ground_4 > 0);
}

// ---------------------------------------------------------------------
// Variants.
// ---------------------------------------------------------------------

#[test]
fn a_scenario_with_no_upkeep_file_has_no_wear_no_repair_and_is_scenario_6a_for_three_days() {
    let variant = without_upkeep();
    assert!(variant.world.ref_data().air_upkeep.is_none());
    let six_a = common::load(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/06a_base_raid.toml"),
    )
    .expect("scenario 6a loads");
    for seed in 1..=VARIANT_SEEDS {
        let run = run_of(&variant, seed);
        assert!(
            run.of(|k| matches!(
                k,
                EventKind::AircraftWorn { .. }
                    | EventKind::AircraftRepaired { .. }
                    | EventKind::DamagedAircraftDestroyedOnGround { .. }
            ))
            .is_empty(),
            "seed {seed}"
        );
        assert_eq!(run.damaged, [0; 5]);
        let first_three: Vec<&Event> = run.events.iter().filter(|e| e.day <= 3).collect();
        let reference = run_of(&six_a, seed);
        assert_eq!(first_three.len(), reference.events.len(), "seed {seed}");
        for (a, b) in first_three.iter().zip(&reference.events) {
            assert_eq!(*a, b, "seed {seed}");
        }
        // The hashes differ on purpose: the airfields' repair figures are in
        // the state, and scenario 6a's airfields have none.
        assert_ne!(run.hashes[..3], reference.hashes[..], "seed {seed}");
    }
}

#[test]
fn with_both_airfields_repairing_none_nothing_is_repaired_and_damaged_aircraft_only_fall_on_the_ground(
) {
    let runs = batch_of(&repairs(0, 0), VARIANT_SEEDS);
    let config = config_with([0, 0]);
    let worked = work_all(&runs, &config);
    let mut damaged_at_end = 0;
    for (run, worked) in runs.iter().zip(&worked) {
        assert!(
            run.of(|k| matches!(k, EventKind::AircraftRepaired { .. }))
                .is_empty(),
            "seed {}",
            run.seed
        );
        assert_conserved(run);
        assert_eq!(worked.repaired, [0; 5]);
        // A group's damaged aircraft rise by wear and fall by being destroyed
        // on the ground, and by nothing else.
        for g in GROUPS {
            let mut damaged = 0u32;
            for event in &run.events {
                match event.kind {
                    EventKind::AircraftWorn {
                        air_group,
                        worn,
                        written_off,
                        damaged: after,
                        ..
                    } if air_group == group(g) => {
                        damaged += worn - written_off;
                        assert_eq!(after, damaged);
                    }
                    EventKind::DamagedAircraftDestroyedOnGround {
                        air_group,
                        aircraft,
                        remaining,
                        ..
                    } if air_group == group(g) => {
                        damaged -= aircraft;
                        assert_eq!(remaining, damaged);
                    }
                    _ => {}
                }
            }
            assert_eq!(damaged, run.damaged[of(g)], "seed {}, group {g}", run.seed);
        }
        damaged_at_end += run.damaged.iter().sum::<u32>();
    }
    eprintln!("no repairs: {damaged_at_end} damaged at the end of {VARIANT_SEEDS} runs");
    assert!(damaged_at_end > 0);
}

#[test]
fn with_a_write_off_share_of_1_every_worn_aircraft_is_lost_and_none_is_damaged_or_repaired() {
    let runs = batch_of(&write_off_share("1"), VARIANT_SEEDS);
    let mut config = scenario_config();
    config.write_off = Fixed::ONE;
    let worked = work_all(&runs, &config);
    let mut worn = 0;
    for (run, worked) in runs.iter().zip(&worked) {
        assert_eq!(run.damaged, [0; 5], "seed {}", run.seed);
        assert!(worked.damaged_at_dawn.iter().all(|d| *d == [0; 5]));
        assert!(run
            .of(|k| matches!(k, EventKind::AircraftRepaired { .. }))
            .is_empty());
        for event in run.of(|k| matches!(k, EventKind::AircraftWorn { .. })) {
            if let EventKind::AircraftWorn {
                worn: w,
                written_off,
                damaged,
                ..
            } = event.kind
            {
                assert_eq!((w, damaged), (written_off, 0));
                worn += w;
            }
        }
        assert_conserved(run);
    }
    assert!(worn > 0);
}

#[test]
fn with_a_write_off_share_of_0_no_aircraft_is_written_off() {
    let runs = batch_of(&write_off_share("0"), VARIANT_SEEDS);
    let mut config = scenario_config();
    config.write_off = Fixed::ZERO;
    let worked = work_all(&runs, &config);
    let mut worn = 0;
    for (run, worked) in runs.iter().zip(&worked) {
        assert_eq!(worked.written_off, [0; 5]);
        for event in run.of(|k| matches!(k, EventKind::AircraftWorn { .. })) {
            if let EventKind::AircraftWorn {
                worn: w,
                written_off,
                ..
            } = event.kind
            {
                assert_eq!(written_off, 0);
                worn += w;
            }
        }
        assert_conserved(run);
    }
    assert!(worn > 0);
}

#[test]
fn with_both_wear_chances_0_rolls_are_still_recorded_with_a_chance_of_0_and_none_worn() {
    let runs = batch_of(&no_wear_chances(), VARIANT_SEEDS);
    let mut config = scenario_config();
    config.base_chance = Fixed::ZERO;
    config.per_100 = Fixed::ZERO;
    let worked = work_all(&runs, &config);
    let mut recorded = 0;
    for (run, worked) in runs.iter().zip(&worked) {
        for event in run.of(|k| matches!(k, EventKind::AircraftWorn { .. })) {
            if let EventKind::AircraftWorn {
                chance,
                expected,
                worn,
                written_off,
                ..
            } = event.kind
            {
                assert_eq!(
                    (chance, expected, worn, written_off),
                    (Fixed::ZERO, Fixed::ZERO, 0, 0)
                );
                recorded += 1;
            }
        }
        assert_eq!(run.damaged, [0; 5]);
        assert_eq!(worked.worn, [0; 5]);
        assert!(run
            .of(|k| matches!(k, EventKind::AircraftRepaired { .. }))
            .is_empty());
        assert_conserved(run);
    }
    assert!(recorded > VARIANT_SEEDS);
}

#[test]
fn with_airfield_b_at_80_nm_the_raid_rolls_at_0_082_and_patrols_and_the_search_as_before() {
    let runs = batch_of(&airfield_b_at_80_nm(), VARIANT_SEEDS);
    let mut config = scenario_config();
    config.raid_hexes = 2;
    let worked = work_all(&runs, &config);
    let mut seen = [0; 5];
    for (run, worked) in runs.iter().zip(&worked) {
        for roll in &worked.rolls {
            let index = RollKind::ALL
                .iter()
                .position(|kind| *kind == roll.kind)
                .expect("a kind");
            seen[index] += 1;
            let expected = match roll.kind {
                // 160 nm: 0.05 + 0.02 x 1.6.
                RollKind::Bombers | RollKind::Escorts => Fixed::from_raw(82_000),
                RollKind::EscortPatrol | RollKind::DefendersPatrol => hundredths(5),
                RollKind::Scouts => hundredths(17),
            };
            assert_eq!(roll.chance, expected, "seed {}", run.seed);
        }
        assert_conserved(run);
    }
    assert!(seen.iter().all(|n| *n > 0), "{seen:?}");
}

#[test]
fn with_airfield_b_repairing_10_a_day_no_group_there_starts_a_day_with_damaged_aircraft() {
    let runs = batch_of(&repairs(2, 10), VARIANT_SEEDS);
    let worked = work_all(&runs, &config_with([2, 10]));
    let mut repaired = 0;
    for (run, worked) in runs.iter().zip(&worked) {
        for dawn in &worked.damaged_at_dawn {
            assert_eq!(dawn[2..], [0, 0, 0], "seed {}", run.seed);
        }
        repaired += worked.repaired[2..].iter().sum::<u32>();
        assert_conserved(run);
    }
    assert!(repaired > 0);
}

// ---------------------------------------------------------------------
// What cannot happen.
// ---------------------------------------------------------------------

#[test]
fn no_group_rolls_at_night_nothing_is_repaired_by_day_and_group_4_never_wears_or_repairs() {
    for run in batch() {
        let mut repaired_in_a_day: BTreeMap<(u32, u32), u32> = BTreeMap::new();
        for (index, event) in run.events.iter().enumerate() {
            match event.kind {
                EventKind::AircraftWorn { air_group, .. } => {
                    assert!(
                        matches!(event.pulse, Some(Pulse::Morning | Pulse::Afternoon)),
                        "seed {}: a roll at night",
                        run.seed
                    );
                    assert_ne!(air_group, group(RESTING));
                }
                EventKind::AircraftRepaired {
                    air_group,
                    at,
                    repaired,
                    ..
                } => {
                    assert_eq!(event.pulse, Some(Pulse::Night), "seed {}", run.seed);
                    assert_ne!(air_group, group(RESTING));
                    // Never an aircraft of a group that is somewhere else.
                    let base = if air_group.raw() <= 2 {
                        AIRFIELD_A
                    } else {
                        AIRFIELD_B
                    };
                    assert_eq!(at, AirGroupLocation::Base(base));
                    *repaired_in_a_day
                        .entry((event.day, base.raw()))
                        .or_default() += repaired;
                }
                // Aircraft worn in a pulse are not on the ground in that
                // pulse: every ground loss comes before the pulse's wear.
                EventKind::AircraftDestroyedOnGround { .. }
                | EventKind::DamagedAircraftDestroyedOnGround { .. } => {
                    assert!(
                        run.events[index..]
                            .iter()
                            .take_while(|e| e.day == event.day && e.pulse == event.pulse)
                            .count()
                            > 0
                    );
                    assert!(
                        !run.events[..index].iter().any(|e| e.day == event.day
                            && e.pulse == event.pulse
                            && matches!(e.kind, EventKind::AircraftWorn { .. })),
                        "seed {}: no wear before the ground losses of the pulse",
                        run.seed
                    );
                }
                _ => {}
            }
        }
        // A base repairs no more in a day than its figure.
        for ((day, base), repaired) in repaired_in_a_day {
            let limit = if base == AIRFIELD_A.raw() { 2 } else { 3 };
            assert!(repaired <= limit, "seed {}, day {day}", run.seed);
        }
        // Group 4 has no damaged aircraft to lose on the ground.
        assert!(
            run.of(|k| matches!(k, EventKind::DamagedAircraftDestroyedOnGround { air_group, .. } if *air_group == group(RESTING)))
                .is_empty()
        );
    }
}

// ---------------------------------------------------------------------
// Properties.
// ---------------------------------------------------------------------

#[test]
fn aircraft_are_conserved_and_no_group_flies_a_damaged_aircraft_in_every_run() {
    // Conserved for every group of every run; the rule's own counts (which
    // fly only ready aircraft) agree with every launch, escort, patrol and
    // search in the log, as `worked_batch` asserted.
    for run in batch() {
        assert_conserved(run);
    }
    for worked in worked_batch() {
        for roll in &worked.rolls {
            assert!(roll.aircraft > 0);
        }
    }
}

#[test]
fn a_longer_sortie_never_has_a_lower_wear_chance() {
    let scenario = load();
    let figures = scenario
        .world
        .ref_data()
        .air_upkeep
        .expect("the upkeep figures are loaded");
    let mut last = Fixed::ZERO;
    for nm in 0..=1200 {
        let chance = figures.wear_chance(Fixed::from_int(nm));
        assert!(chance >= last, "{nm} nm");
        last = chance;
    }
    let at = |nm: i64| figures.wear_chance(Fixed::from_int(nm));
    assert!(at(0) < at(160) && at(160) < at(400) && at(400) < at(600));
}

#[test]
fn repair_leaves_more_aircraft_ready() {
    let ready_of_group_3 =
        |runs: &[Run]| -> u32 { runs.iter().map(|run| run.ready[of(DEFENDERS)]).sum() };
    let none = batch_of(&repairs(2, 0), SEEDS);
    // The model holds for it.
    work_all(&none, &config_with([2, 0]));
    let (three, none) = (ready_of_group_3(batch()), ready_of_group_3(&none));
    eprintln!("group 3 ready at the end over 1,000 runs: B repairs 3: {three}, none: {none}");
    assert!(three > none, "{three} {none}");
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
        assert!(breakdowns > 20);
    }
}

#[test]
fn the_same_seed_gives_the_same_hashes_twice_and_wear_changes_the_state() {
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
        assert_eq!(hashes[0].len(), 5);
        assert_eq!(hashes[0], hashes[1], "seed {seed}");
        all_hashes.push(hashes[0].clone());
    }
    assert!(all_hashes.windows(2).any(|pair| pair[0] != pair[1]));
    // The batch holds the same hashes as a fresh run of the same seed.
    let mut again = base.with_seed(77).expect("a seed can be set");
    again.run_to_end().expect("the run resolves");
    assert_eq!(batch()[76].seed, 77);
    assert_eq!(batch()[76].hashes, again.world.day_hashes());
    // Damaged aircraft are in the hash: the same seed without the upkeep file
    // ends in other states.
    let plain = run_of(&without_upkeep(), 77);
    assert_ne!(plain.hashes, again.world.day_hashes());
}

#[test]
fn mean_figures_of_the_scenario_over_1000_seeds() {
    // Not an assertion about the rule: the figures a reader wants, printed
    // with --nocapture.
    let runs = batch();
    let worked = worked_batch();
    let n = count(runs.len());
    let mean = |total: i64| total * 100 / n;
    for g in GROUPS {
        let worn: i64 = worked.iter().map(|w| i64::from(w.worn[of(g)])).sum();
        let off: i64 = worked.iter().map(|w| i64::from(w.written_off[of(g)])).sum();
        let damaged: i64 = runs.iter().map(|r| i64::from(r.damaged[of(g)])).sum();
        let ready: i64 = runs.iter().map(|r| i64::from(r.ready[of(g)])).sum();
        let repaired: i64 = worked.iter().map(|w| i64::from(w.repaired[of(g)])).sum();
        eprintln!(
            "group {g} per run x100: worn {}, written off {}, repaired {}, damaged at the end {}, ready at the end {}",
            mean(worn),
            mean(off),
            mean(repaired),
            mean(damaged),
            mean(ready)
        );
    }
    // Scenario 6a's three days, for comparison.
    let six_a = common::load(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/06a_base_raid.toml"),
    )
    .expect("scenario 6a loads");
    let reference = batch_of(&six_a, SEEDS);
    for g in GROUPS {
        let ready: i64 = reference.iter().map(|r| i64::from(r.ready[of(g)])).sum();
        eprintln!(
            "6a, group {g} ready at the end after 3 days x100: {}",
            mean(ready)
        );
    }
    assert!(!worked.is_empty());
}
