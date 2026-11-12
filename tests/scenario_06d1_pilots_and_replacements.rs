//! Scenario 6d1: six days of raids on an airfield, with pilots and
//! replacements.
//!
//! Stage 6d1 of scenario 6 (decision 0038). The data is
//! `data/scenarios/06d1_pilots_and_replacements.toml`, run with seeds 1 to
//! 1000. Everything in it is a placeholder.
//!
//! # Hand calculation
//!
//! ## The setting
//!
//! It is scenario 6a (`tests/scenario_06a_base_raid.rs`) over six days.
//! There is no wear and no fatigue: no aircraft is ever damaged and every
//! group flies all it can. Airfield A, base 1, Japanese, is at (0,0).
//! Airfield B, base 2, Allied, is at (5,0), 200 nm away, with 1,000 t of
//! supply and 20 anti-aircraft shots.
//!
//! | Group | Side | At | Type | Aircraft | Pilots | Mission |
//! | ----- | ---- | -- | ---- | -------- | ------ | ------- |
//! | 1 | Japanese | A | dive bomber | 18 | 18 | base strike on airfield B |
//! | 2 | Japanese | A | fighter | 12 | 12 | combat air patrol, share 0.5 |
//! | 3 | Allied | B | fighter | 12 | 12 | combat air patrol, share 1 |
//! | 4 | Allied | B | dive bomber | 10 | 10 | rest |
//! | 5 | Allied | B | scout | 6 | 6 | search |
//!
//! A group's strength is what it starts with: 18, 12, 12, 10 and 6, in
//! aircraft and in pilots.
//!
//! | Replacements, a day | At | Type | Aircraft | Pilots |
//! | ------------------- | -- | ---- | -------- | ------ |
//! | Japanese | A | dive bomber | 2 | 1 |
//! | Allied | B | fighter | 2 | 0 |
//! | Allied | B | dive bomber | 1 | 0 |
//!
//! ## The rule
//!
//! Write R for a group's ready aircraft and P for its pilots.
//!
//! **What flies.** A group can fly the smaller of R and P. Everything in
//! scenario 6a's rule that says "its aircraft" means that number here: the
//! bombers launched, the fighters split into patrol and escorts, the
//! patrol, the scouts searching.
//!
//! **On the ground.** The aircraft on the ground at airfield B are the
//! ready aircraft not flying: all of group 4, and any aircraft of groups 3
//! and 5 beyond their pilots. Those destroyed are taken from group 3 first,
//! then group 4, then group 5.
//!
//! **Pilots lost.** A pilot is lost with each aircraft shot down away from
//! its group's own hex, and with no other loss:
//!
//! | Loss | Where | The pilot |
//! | ---- | ----- | --------- |
//! | A bomber of group 1 shot down by the patrol or the guns | Over airfield B | Lost |
//! | An escort of group 2 shot down by the patrol | Over airfield B | Lost |
//! | A fighter of group 3 shot down by the escorts | Over its own airfield | Saved |
//! | An aircraft destroyed on the ground | At its own airfield | Saved |
//!
//! So groups 3, 4 and 5 never lose a pilot: they have 12, 10 and 6
//! throughout. Group 2's patrol over airfield A meets nobody.
//!
//! **Replacements, at the start of each day.** Day 1 has nothing to
//! replace. From day 2:
//!
//! - Group 1 receives the smaller of 2 and (18 - R1) bombers, and the
//!   smaller of 1 and (18 - P1) pilots.
//! - Group 2 receives nothing: no fighters come to airfield A.
//! - Group 3 receives the smaller of 2 and (12 - R3) fighters and no pilot.
//! - Group 4 receives the smaller of 1 and (10 - R4) bombers and no pilot.
//! - Group 5 receives nothing, and needs nothing unless a scout is destroyed
//!   on the ground, which cannot happen while it has six pilots for six
//!   scouts: all six are in the air.
//!
//! What a day's replacements cannot place is lost.
//!
//! ## The first day and the second morning
//!
//! Day 1 is scenario 6a's first raid, roll for roll: 18 bombers, 6 escorts,
//! a patrol of 12, ten aircraft on the ground. Write b for the bombers shot
//! down that morning (by the patrol and the guns together), e for the
//! escorts shot down, and f for group 3's fighters shot down.
//!
//! | Group | Aircraft after day 1 | Pilots after day 1 | After the night's replacements | Flies on day 2 |
//! | ----- | -------------------- | ------------------ | ------------------------------ | -------------- |
//! | 1 | 18 - b | 18 - b | min(18, 20 - b) aircraft, min(18, 19 - b) pilots | 19 - b if b is 1 or more, else 18: its pilots |
//! | 2 | 12 - e | 12 - e | the same | 12 - e |
//! | 3 | 12 - f | 12 | min(12, 14 - f) aircraft | that many: it has the pilots |
//!
//! b is 2.8 on average, so on the second morning group 1 has about 17
//! aircraft and flies about 16: a bomber stands idle for want of a pilot.
//! It is not on the ground at airfield B, so nothing bombs it.
//!
//! ## Over the six days
//!
//! The test works every pulse of every run from the counts at its start:
//! what flies, the raid, the pilots lost, the replacements.
//!
//! For every group in every run:
//!
//! - Aircraft at the end are those at the start, less those shot down and
//!   destroyed on the ground, plus those received.
//! - Pilots at the end are those at the start, less those lost, plus those
//!   received.
//! - Neither is ever above the group's strength.
//!
//! Over seeds 1 to 1000:
//!
//! - Group 1's pilots are never more than its ready aircraft, at any time
//!   in any run: it loses one of each together and never gains more pilots
//!   than aircraft. So pilots, not aircraft, set what it flies.
//! - Group 3 has all 12 pilots in every run at the end, and group 2 has
//!   received nothing in any run.
//! - The Allied fighters hold up and the Japanese do not: summed over the
//!   runs, group 3's ready aircraft at the end are more than group 2's.
//!
//! ## Variants
//!
//! | Variant | What changes |
//! | ------- | ------------ |
//! | No group is given pilots | Pilots are not tracked: no pilot is lost or received and each group flies its ready aircraft. Aircraft replacements still arrive |
//! | No replacements | Nothing is received. With pilots tracked, each of the first three days is, event for event apart from the pilots lost, scenario 6a with the same seed |
//! | The Japanese receive 2 pilots a day | Group 1 flies the smaller of the two as before; its pilots keep pace with its aircraft |
//! | Group 3 starts with 8 pilots for its 12 fighters | It flies 8. The other 4 are on the ground at airfield B and are destroyed before group 4's |
//! | Group 1 starts with 18 aircraft and a strength of 24 | It can receive aircraft and pilots from day 2 even with no loss: 2 and 1 a day up to 24 |
//! | The Japanese replacements are fighters, not dive bombers | Group 2 receives them and group 1 nothing |
//!
//! ## What cannot happen
//!
//! - No group flies more aircraft than it has pilots.
//! - No pilot of group 3, 4 or 5 is lost.
//! - Nothing is received on day 1, in a daylight pulse, by a group of
//!   another type, or by a group at another base.
//! - No group has more aircraft or more pilots than its strength.
//! - No random number is drawn for pilots or replacements: the rolls of the
//!   first morning are those of scenario 6a with the same seed.
//!
//! # Properties
//!
//! - Aircraft and pilots are conserved, as "Over the six days" has it.
//! - More pilots never fly fewer aircraft: the smaller of R and P does not
//!   fall as P rises.
//! - More replacements never leave a group weaker: summed over the runs,
//!   group 1's ready aircraft at the end with 2 bombers a day are more than
//!   with none.
//! - Recording breakdowns changes neither the events nor the hashes.
//! - The same seed gives the same hashes twice.
//! - A scenario that gives no pilots and no replacements has the hashes it
//!   had before this stage: the golden files of scenarios 1 to 6c do not
//!   change.

// The test code is written by a developer from the hand calculation above.

mod common;

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use sim_core::events::{AirFirer, AirTarget, Cause, Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, EntityRef, EventId, OrderId};
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
/// The variants run fewer seeds, as scenarios 6a to 6c do.
const VARIANT_SEEDS: u64 = 300;
const DAYS: u32 = 6;
const START: [u32; 5] = [18, 12, 12, 10, 6];
/// The base each group is at, and the aircraft type it flies.
const GROUP_BASE: [BaseId; 5] = [AIRFIELD_A, AIRFIELD_A, AIRFIELD_B, AIRFIELD_B, AIRFIELD_B];
const GROUP_TYPE: [u32; 5] = [3, 4, 4, 3, 2];
const DIVE_BOMBER: u32 = 3;
const FIGHTER: u32 = 4;
/// The range of the scouts' arc.
const SEARCH_NM: i64 = 300;
const RAID_HEXES: i64 = 5;

fn scenario_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../data/scenarios/06d1_pilots_and_replacements.toml")
}

fn load() -> Scenario {
    common::load(&scenario_path()).expect("scenario 6d1 loads")
}

fn six_a() -> Scenario {
    common::load(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/06a_base_raid.toml"),
    )
    .expect("scenario 6a loads")
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

// ---------------------------------------------------------------------
// The rule, written from the hand calculation and nothing in sim-core.
// ---------------------------------------------------------------------

/// What a group can fly: the smaller of its ready aircraft R and its pilots
/// P, or R if its pilots are not tracked.
fn flies(ready: u32, pilots: Option<u32>) -> u32 {
    pilots.map_or(ready, |pilots| ready.min(pilots))
}

/// One stream of replacements: a base, a type, aircraft and pilots a day.
#[derive(Clone, Copy)]
struct Stream {
    base: BaseId,
    aircraft_type: u32,
    aircraft: u32,
    pilots: u32,
}

/// What differs between the scenario and its variants.
#[derive(Clone)]
struct Config {
    /// The pilots each group starts with; `None` where not tracked.
    pilots: [Option<u32>; 5],
    /// The strength of each group: the ceiling for replacements.
    strength: [u32; 5],
    /// The replacement streams, in order.
    streams: Vec<Stream>,
}

fn scenario_config() -> Config {
    Config {
        pilots: START.map(Some),
        strength: START,
        streams: vec![
            Stream {
                base: AIRFIELD_A,
                aircraft_type: DIVE_BOMBER,
                aircraft: 2,
                pilots: 1,
            },
            Stream {
                base: AIRFIELD_B,
                aircraft_type: FIGHTER,
                aircraft: 2,
                pilots: 0,
            },
            Stream {
                base: AIRFIELD_B,
                aircraft_type: DIVE_BOMBER,
                aircraft: 1,
                pilots: 0,
            },
        ],
    }
}

// ---------------------------------------------------------------------
// What one run did, read from its events and final state.
// ---------------------------------------------------------------------

struct Run {
    seed: u64,
    events: Vec<Event>,
    hashes: Vec<u64>,
    ready: [u32; 5],
    pilots: [Option<u32>; 5],
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

    /// The orders that say a group keeps a patrol: its mission order, and
    /// for group 2 its share.
    fn patrol_orders(&self, g: u32) -> Vec<Cause> {
        let wanted = if g == ESCORTS { 2 } else { 1 };
        self.orders_of(g)
            .into_iter()
            .take(wanted)
            .map(Cause::Order)
            .collect()
    }

    /// The order that set the group's mission: the first of its orders.
    fn mission_order(&self, g: u32) -> Option<OrderId> {
        self.orders_of(g).first().copied()
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
        pilots: GROUPS.map(|id| truth.air_groups[&group(id)].pilots),
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

fn stream_of(seed: u64, day: u32, pulse: Pulse, entity: EntityRef, other: EntityRef) -> RngStream {
    RngKey {
        seed,
        day,
        pulse: Some(pulse),
        subsystem: Subsystem::AirCombat,
        entity,
        other: Some(other),
        draw: Draw::Legacy,
    }
    .stream()
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

// ---------------------------------------------------------------------
// Working every pulse of a run from the counts at the start of it.
// ---------------------------------------------------------------------

/// What the run's state is, by the rule, as the pulses are worked.
struct State {
    ready: [u32; 5],
    pilots: [Option<u32>; 5],
    supply: i64,
}

impl State {
    /// What a group can fly now: min(R, P).
    fn able(&self, g: u32) -> u32 {
        flies(self.ready[of(g)], self.pilots[of(g)])
    }
}

/// One day of a run, by the rule.
#[derive(Clone, Default)]
struct Day {
    /// Ready aircraft and pilots at the start of the day, before the night's
    /// replacements, after them, and at the end of the day.
    ready_start: [u32; 5],
    pilots_start: [Option<u32>; 5],
    ready_night: [u32; 5],
    pilots_night: [Option<u32>; 5],
    ready_end: [u32; 5],
    pilots_end: [Option<u32>; 5],
    /// What each group received in the night.
    received_aircraft: [u32; 5],
    received_pilots: [u32; 5],
    /// What each group can fly in the morning and in the afternoon.
    able_morning: [u32; 5],
    able_afternoon: [u32; 5],
    /// The raid: bombers launched (0 if none), escorts sent, group 3's
    /// patrol, group 2's kept patrol.
    bombers: u32,
    escorts: u32,
    patrol: u32,
    kept: u32,
    /// Bombers that attacked.
    attacking: u32,
    /// Aircraft shot down, pilots lost and aircraft destroyed on the ground,
    /// by group, in the morning.
    shot_down: [u32; 5],
    pilots_lost: [u32; 5],
    on_the_ground: [u32; 5],
    /// The aircraft on the ground at airfield B at the start of the raid:
    /// ready and not flying (0 for a group elsewhere).
    ground_before: [u32; 5],
}

/// What working one run found.
#[derive(Default)]
struct Worked {
    days: Vec<Day>,
    ready: [u32; 5],
    pilots: [u32; 5],
    shot_down: [u32; 5],
    pilots_lost: [u32; 5],
    on_the_ground: [u32; 5],
    received_aircraft: [u32; 5],
    received_pilots: [u32; 5],
}

/// The kinds of the morning in the order the log writes them.
fn rank(kind: &EventKind) -> u32 {
    match kind {
        EventKind::SearchFlown { .. } => 0,
        EventKind::BaseStrikeLaunched { .. } => 1,
        EventKind::EscortsSent { .. } => 2,
        EventKind::AirFight { .. } => 3,
        EventKind::BaseAntiAircraftFire { .. } => 4,
        EventKind::AircraftLost { .. } | EventKind::PilotsLost { .. } => 5,
        EventKind::BaseAttacked { .. } => 6,
        EventKind::AircraftDestroyedOnGround { .. } => 7,
        EventKind::BaseSupplyDestroyed { .. } => 8,
        other => panic!("an event the day does not have: {other:?}"),
    }
}

/// Works every pulse of `run` by the rule of the hand calculation from the
/// counts at the start of it, asserts every event against it, and returns
/// what was worked. The counts are carried from pulse to pulse by the rule,
/// not read from the events.
fn work_the_run(run: &Run, config: &Config) -> Worked {
    let seed = run.seed;
    let mut state = State {
        ready: START,
        pilots: config.pilots,
        supply: 1000,
    };
    let mut worked = Worked::default();
    for day in 1..=DAYS {
        let mut record = Day {
            ready_start: state.ready,
            pilots_start: state.pilots,
            ..Day::default()
        };
        work_the_night(run, config, day, &mut state, &mut record);
        record.ready_night = state.ready;
        record.pilots_night = state.pilots;
        assert_within_strength(config, &state, seed, day);
        work_the_morning(run, day, &mut state, &mut record);
        assert_within_strength(config, &state, seed, day);
        work_the_afternoon(run, day, &mut state, &mut record);
        record.ready_end = state.ready;
        record.pilots_end = state.pilots;
        for g in GROUPS {
            worked.received_aircraft[of(g)] += record.received_aircraft[of(g)];
            worked.received_pilots[of(g)] += record.received_pilots[of(g)];
            worked.shot_down[of(g)] += record.shot_down[of(g)];
            worked.pilots_lost[of(g)] += record.pilots_lost[of(g)];
            worked.on_the_ground[of(g)] += record.on_the_ground[of(g)];
        }
        worked.days.push(record);
    }

    // The end of the run.
    assert_eq!(run.ready, state.ready, "seed {seed}: ready at the end");
    assert_eq!(run.pilots, state.pilots, "seed {seed}: pilots at the end");
    assert_eq!(
        run.supply_b,
        Fixed::from_int(state.supply),
        "seed {seed}: the supply at the end"
    );
    worked.ready = state.ready;
    worked.pilots = state.pilots.map(|pilots| pilots.unwrap_or(0));
    worked
}

/// No group has more aircraft or more pilots than its strength.
fn assert_within_strength(config: &Config, state: &State, seed: u64, day: u32) {
    for g in GROUPS {
        let i = of(g);
        assert!(
            state.ready[i] <= config.strength[i],
            "seed {seed}, day {day}, group {g}: aircraft {} over strength",
            state.ready[i]
        );
        if let Some(pilots) = state.pilots[i] {
            assert!(
                pilots <= config.strength[i],
                "seed {seed}, day {day}, group {g}: pilots {pilots} over strength"
            );
        }
    }
}

/// Night, at the start of the day: from day 2 each stream, in order, places
/// its aircraft and then its pilots with the groups at its base that fly its
/// type, the lowest first, each up to its strength.
fn work_the_night(run: &Run, config: &Config, day: u32, state: &mut State, record: &mut Day) {
    let seed = run.seed;
    let night = run.pulse(day, Pulse::Night);
    // (group, base, aircraft, pilots, ready after, pilots after)
    let mut expected: Vec<(u32, BaseId, u32, u32, u32, u32)> = Vec::new();
    if day >= 2 {
        // The streams are placed in the order the loader keeps them in
        // (decision 0043, 4c): side, base, type, then the figures. Airfield B
        // is the Allied one.
        let mut streams = config.streams.clone();
        streams.sort_by_key(|s| {
            (
                u8::from(s.base == AIRFIELD_A),
                s.base,
                s.aircraft_type,
                s.aircraft,
                s.pilots,
            )
        });
        for stream in &streams {
            let mut aircraft_left = stream.aircraft;
            let mut pilots_left = stream.pilots;
            for g in GROUPS {
                let i = of(g);
                if GROUP_BASE[i] != stream.base || GROUP_TYPE[i] != stream.aircraft_type {
                    continue;
                }
                let aircraft = aircraft_left.min(config.strength[i].saturating_sub(state.ready[i]));
                let pilots = match state.pilots[i] {
                    Some(have) => pilots_left.min(config.strength[i].saturating_sub(have)),
                    None => 0,
                };
                if aircraft == 0 && pilots == 0 {
                    continue;
                }
                aircraft_left -= aircraft;
                pilots_left -= pilots;
                state.ready[i] += aircraft;
                if let Some(have) = state.pilots[i].as_mut() {
                    *have += pilots;
                }
                record.received_aircraft[i] += aircraft;
                record.received_pilots[i] += pilots;
                expected.push((
                    g,
                    stream.base,
                    aircraft,
                    pilots,
                    state.ready[i],
                    state.pilots[i].unwrap_or(0),
                ));
            }
            // What could not be placed is lost: nothing is carried over.
        }
    }
    assert_eq!(
        night.len(),
        expected.len(),
        "seed {seed}, day {day}: the night has the replacements and nothing else"
    );
    for (event, (g, base, aircraft, pilots, ready, pilots_now)) in night.iter().zip(&expected) {
        assert_eq!(
            event.kind,
            EventKind::ReplacementsReceived {
                air_group: group(*g),
                base: *base,
                aircraft: *aircraft,
                pilots: *pilots,
                ready: *ready,
                pilots_now: *pilots_now,
            },
            "seed {seed}, day {day}"
        );
        assert_eq!(
            event.kind.to_string(),
            format!(
                "air_group:{g} received {aircraft} aircraft and {pilots} pilots at base:{}, {ready} ready and {pilots_now} pilots",
                base.raw()
            )
        );
        assert!(event.causes.is_empty(), "seed {seed}: nothing orders it");
    }
}

fn work_the_morning(run: &Run, day: u32, state: &mut State, record: &mut Day) {
    let seed = run.seed;
    let step = run.pulse(day, Pulse::Morning);
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
    let lost = kinds(|k| {
        matches!(
            k,
            EventKind::AircraftLost { .. } | EventKind::PilotsLost { .. }
        )
    });
    let attacked = kinds(|k| matches!(k, EventKind::BaseAttacked { .. }));
    let on_ground_events = kinds(|k| matches!(k, EventKind::AircraftDestroyedOnGround { .. }));
    let supply_events = kinds(|k| matches!(k, EventKind::BaseSupplyDestroyed { .. }));
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
            + supply_events.len(),
        "seed {seed}, day {day}"
    );

    // What each group can fly: min(R, P). Group 5 searches with all it can
    // fly; group 1 launches all it can fly, if that is at least one; group 2
    // keeps half of what it can fly, rounded down, and the rest escort the
    // raid if one launches; group 3 patrols with all it can fly.
    let able = GROUPS.map(|g| state.able(g));
    record.able_morning = able;
    for g in GROUPS {
        assert!(
            able[of(g)] <= state.ready[of(g)],
            "seed {seed}: no more than ready"
        );
        if let Some(pilots) = state.pilots[of(g)] {
            assert!(
                able[of(g)] <= pilots,
                "seed {seed}: no group flies more than it has pilots"
            );
        }
    }
    let s5 = able[of(SCOUTS)];
    let b = able[of(BOMBERS)];
    let big_f = able[of(ESCORTS)];
    let kept = big_f * 5 / 10;
    let p3 = able[of(DEFENDERS)];
    let raid = b >= 1;
    let e = if raid { big_f - kept } else { 0 };
    record.bombers = b;
    record.escorts = e;
    record.patrol = p3;
    record.kept = kept;

    // Group 5 searches.
    if s5 > 0 {
        assert_eq!(searches.len(), 1, "seed {seed}, day {day}");
        assert_eq!(
            searches[0].kind,
            EventKind::SearchFlown {
                air_group: group(SCOUTS),
                from: AirGroupLocation::Base(AIRFIELD_B),
                hex: Hex::new(RAID_HEXES as i32, 0),
                from_deg: 40,
                to_deg: 130,
                range_nm: Fixed::from_int(SEARCH_NM),
                aircraft: s5,
            },
            "seed {seed}, day {day}"
        );
    } else {
        assert!(searches.is_empty(), "seed {seed}, day {day}");
    }

    if raid {
        work_the_raid(
            run,
            day,
            state,
            record,
            [b, e, p3, s5],
            [&launches, &sent, &fights, &guns],
            [&lost, &attacked, &on_ground_events, &supply_events],
        );
    } else {
        assert!(
            launches.is_empty()
                && sent.is_empty()
                && fights.is_empty()
                && guns.is_empty()
                && lost.is_empty()
                && attacked.is_empty()
                && on_ground_events.is_empty()
                && supply_events.is_empty(),
            "seed {seed}, day {day}: no bombers, no raid, no fight"
        );
    }
}

fn work_the_afternoon(run: &Run, day: u32, state: &mut State, record: &mut Day) {
    let seed = run.seed;
    let step = run.pulse(day, Pulse::Afternoon);
    let able = GROUPS.map(|g| state.able(g));
    record.able_afternoon = able;
    let s5 = able[of(SCOUTS)];
    // No raid flies in the afternoon and nothing else is recorded: the scouts
    // search, and the patrols meet nobody.
    if s5 > 0 {
        assert_eq!(step.len(), 1, "seed {seed}, day {day}");
        assert_eq!(
            step[0].kind,
            EventKind::SearchFlown {
                air_group: group(SCOUTS),
                from: AirGroupLocation::Base(AIRFIELD_B),
                hex: Hex::new(RAID_HEXES as i32, 0),
                from_deg: 40,
                to_deg: 130,
                range_nm: Fixed::from_int(SEARCH_NM),
                aircraft: s5,
            },
            "seed {seed}, day {day}"
        );
    } else {
        assert!(step.is_empty(), "seed {seed}, day {day}");
    }
}

/// The morning's raid worked as in scenario 6a on the counts of `flying`
/// (bombers, escorts, group 3's patrol, the scouts searching). The aircraft
/// on the ground at airfield B are every ready aircraft there that is not
/// flying: the fighters of group 3 beyond what flies, all of group 4, and
/// the scouts that are not searching. The raid is alone, so it meets the
/// whole patrol. A bomber or an escort shot down takes its pilot; a fighter
/// of group 3 shot down by the escorts does not; nothing on the ground does.
#[allow(clippy::too_many_arguments)]
fn work_the_raid(
    run: &Run,
    day: u32,
    state: &mut State,
    record: &mut Day,
    flying: [u32; 4],
    first: [&[&Event]; 4],
    then: [&[&Event]; 4],
) {
    let seed = run.seed;
    let pulse = Pulse::Morning;
    let [b, e, p, scouts_flying] = flying;
    let [launches, sent, fights, guns] = first;
    let [lost, attacked, on_ground_events, supply_events] = then;
    assert_eq!(launches.len(), 1, "seed {seed}, day {day}");
    let launch = launches[0];
    assert_eq!(
        launch.kind,
        EventKind::BaseStrikeLaunched {
            air_group: group(BOMBERS),
            base: AIRFIELD_B,
            hex: Hex::new(RAID_HEXES as i32, 0),
            aircraft: b,
            distance_nm: Fixed::from_int(RAID_HEXES * 40),
        },
        "seed {seed}"
    );
    assert_eq!(
        launch.causes,
        vec![Cause::Order(run.mission_order(BOMBERS).expect("an order"))],
        "seed {seed}: the launch cites the mission order alone"
    );

    let mut patrol_pool: Vec<(u32, u32)> = [(DEFENDERS, p)]
        .into_iter()
        .filter(|(_, count)| *count > 0)
        .collect();
    // The ground: by group, the ready aircraft of every group at airfield B
    // that is not flying.
    let mut ground_pool: Vec<(u32, u32)> = Vec::new();
    for (g, flying) in [(DEFENDERS, p), (RESTING, 0), (SCOUTS, scouts_flying)] {
        let on_ground = state.ready[of(g)] - flying;
        record.ground_before[of(g)] = on_ground;
        if on_ground > 0 {
            ground_pool.push((g, on_ground));
        }
    }
    let on_ground: u32 = ground_pool.iter().map(|(_, count)| count).sum();

    if e > 0 {
        assert_eq!(sent.len(), 1, "seed {seed}, day {day}");
        assert_eq!(
            sent[0].kind,
            EventKind::EscortsSent {
                air_group: group(ESCORTS),
                with_air_group: group(BOMBERS),
                aircraft: e
            },
            "seed {seed}"
        );
        let mut causes = run.patrol_orders(ESCORTS);
        causes.push(Cause::Event(launch.id));
        assert_eq!(sent[0].causes, causes, "seed {seed}");
    } else {
        assert!(sent.is_empty(), "seed {seed}, day {day}");
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
        for (g, count) in taken {
            now[of(g)] -= count;
            expected_losses.push((g, count, now[of(g)], event.id));
        }
    }

    // The guns of airfield B, at the bombers left.
    if bomber_pool[0].1 > 0 {
        assert_eq!(guns.len(), 1, "seed {seed}, day {day}");
        let mut stream = stream_of(
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
        for (g, count) in take(hits, &mut bomber_pool) {
            now[of(g)] -= count;
            expected_losses.push((g, count, now[of(g)], guns[0].id));
        }
    } else {
        assert!(guns.is_empty(), "seed {seed}, day {day}");
    }

    // The aircraft lost in the air, one event for each group and roll, and
    // after each, if its pilots are tracked and it was shot down away from
    // home (a bomber or an escort), the pilots lost with it.
    let mut at = 0;
    let mut aircraft_events: Vec<EventId> = Vec::new();
    for (g, count, remaining, cause) in &expected_losses {
        let i = of(*g);
        assert!(at < lost.len(), "seed {seed}, day {day}: a loss is missing");
        let event = lost[at];
        at += 1;
        assert_eq!(
            event.kind,
            EventKind::AircraftLost {
                air_group: group(*g),
                aircraft: *count,
                remaining: *remaining
            },
            "seed {seed}"
        );
        assert_eq!(event.causes, vec![Cause::Event(*cause)], "seed {seed}");
        aircraft_events.push(event.id);
        record.shot_down[i] += count;
        if let Some(pilots) = state.pilots[i] {
            if *g == DEFENDERS {
                // Saved: shot down over their own airfield.
                continue;
            }
            assert!([BOMBERS, ESCORTS].contains(g));
            let taken = (*count).min(pilots);
            if taken == 0 {
                continue;
            }
            state.pilots[i] = Some(pilots - taken);
            record.pilots_lost[i] += taken;
            assert!(
                at < lost.len(),
                "seed {seed}, day {day}: a pilot loss is missing"
            );
            let pilot_event = lost[at];
            at += 1;
            assert_eq!(
                pilot_event.kind,
                EventKind::PilotsLost {
                    air_group: group(*g),
                    lost: taken,
                    left: pilots - taken
                },
                "seed {seed}, day {day}"
            );
            assert_eq!(
                pilot_event.causes,
                vec![Cause::Event(event.id)],
                "seed {seed}: caused by the loss of the aircraft"
            );
            assert_eq!(
                pilot_event.kind.to_string(),
                format!(
                    "air_group:{g} lost {taken} {}, {} left",
                    if taken == 1 { "pilot" } else { "pilots" },
                    pilots - taken
                )
            );
        }
    }
    assert_eq!(at, lost.len(), "seed {seed}, day {day}: no other loss");
    let bombers_left = bomber_pool[0].1;
    state.ready = now;

    // The bombers left each drop one bomb, whatever the type carries.
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
        let hits = hits_from(&mut stream, bombers_left, tenths(2));
        let (mut on_aircraft, mut on_supply) = (0, 0);
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
                .zip(&aircraft_events)
                .filter(|((g, ..), _)| *g == BOMBERS)
                .map(|(_, id)| Cause::Event(*id)),
        );
        assert_eq!(attacked[0].causes, causes, "seed {seed}");

        // Hits on aircraft destroy aircraft on the ground, the lowest group
        // first, and no more than were there at the start of the pulse.
        // No pilot goes with them.
        let taken = take(on_aircraft, &mut ground_pool);
        assert_eq!(
            on_ground_events.len(),
            taken.len(),
            "seed {seed}, day {day}"
        );
        for (event, (g, count)) in on_ground_events.iter().zip(&taken) {
            record.on_the_ground[of(*g)] += count;
            state.ready[of(*g)] -= count;
            assert_eq!(
                event.kind,
                EventKind::AircraftDestroyedOnGround {
                    air_group: group(*g),
                    base: AIRFIELD_B,
                    aircraft: *count,
                    remaining: state.ready[of(*g)],
                },
                "seed {seed}, day {day}"
            );
            assert_eq!(
                event.causes,
                vec![Cause::Event(attacked[0].id)],
                "seed {seed}"
            );
            let breakdown = event.breakdown.as_ref().expect("a breakdown");
            assert_eq!(breakdown.result, int(on_aircraft.min(on_ground)));
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
    record.attacking = bombers_left;
}

// ---------------------------------------------------------------------
// Variants: the scenario with one thing changed, written under the build
// directory. Nothing in the repository's data folder is touched.
// ---------------------------------------------------------------------

/// Loads the scenario on the frozen figures with `edit` applied to its text.
fn variant(_tag: &str, edit: &dyn Fn(String) -> String) -> Scenario {
    let path = scenario_path();
    let text = fs::read_to_string(&path).expect("read the scenario");
    common::from_text(&edit(text), &path).expect("the variant loads")
}

/// Replaces `original` by `changed`, which must appear exactly once.
fn once(text: String, original: &str, changed: &str) -> String {
    assert_eq!(
        text.matches(original).count(),
        1,
        "the text {original:?} appears exactly once"
    );
    text.replace(original, changed)
}

const STREAM_1: &str = "[[replacement]]\nside = \"japanese\"\nbase = 1\naircraft_type = 3\naircraft_per_day = 2\npilots_per_day = 1\n";
const STREAM_2: &str = "[[replacement]]\nside = \"allied\"\nbase = 2\naircraft_type = 4\naircraft_per_day = 2\npilots_per_day = 0\n";
const STREAM_3: &str = "[[replacement]]\nside = \"allied\"\nbase = 2\naircraft_type = 3\naircraft_per_day = 1\npilots_per_day = 0\n";

fn without_pilots() -> Scenario {
    variant("no_pilots", &|text| {
        let kept: Vec<&str> = text
            .lines()
            .filter(|line| !line.starts_with("pilots = "))
            .collect();
        kept.join("\n") + "\n"
    })
}

fn config_without_pilots() -> Config {
    Config {
        pilots: [None; 5],
        ..scenario_config()
    }
}

fn without_replacements() -> Scenario {
    variant("no_streams", &|text| {
        let text = once(text, STREAM_1, "");
        let text = once(text, STREAM_2, "");
        once(text, STREAM_3, "")
    })
}

fn config_without_replacements() -> Config {
    Config {
        streams: Vec::new(),
        ..scenario_config()
    }
}

fn japanese_receive_pilots(per_day: u32) -> Scenario {
    variant(&format!("pilots_{per_day}"), &|text| {
        once(
            text,
            "aircraft_per_day = 2\npilots_per_day = 1",
            &format!("aircraft_per_day = 2\npilots_per_day = {per_day}"),
        )
    })
}

fn config_japanese_pilots(per_day: u32) -> Config {
    let mut config = scenario_config();
    config.streams[0].pilots = per_day;
    config
}

fn japanese_receive_aircraft(per_day: u32) -> Scenario {
    variant(&format!("aircraft_{per_day}"), &|text| {
        once(
            text,
            "aircraft_per_day = 2\npilots_per_day = 1",
            &format!("aircraft_per_day = {per_day}\npilots_per_day = 1"),
        )
    })
}

fn config_japanese_aircraft(per_day: u32) -> Config {
    let mut config = scenario_config();
    config.streams[0].aircraft = per_day;
    config
}

const DEFENDERS_PILOTS: &str =
    "aircraft_type = 4\nready = 12\npilots = 12\n\n[[air_group]]\nid = 4";

fn defenders_with_pilots(pilots: u32) -> Scenario {
    variant(&format!("defenders_{pilots}"), &|text| {
        once(
            text,
            DEFENDERS_PILOTS,
            &format!("aircraft_type = 4\nready = 12\npilots = {pilots}\n\n[[air_group]]\nid = 4"),
        )
    })
}

fn bombers_with_pilots(pilots: u32) -> Scenario {
    variant(&format!("bombers_{pilots}"), &|text| {
        once(
            text,
            "ready = 18\npilots = 18",
            &format!("ready = 18\npilots = {pilots}"),
        )
    })
}

fn bombers_with_strength_24() -> Scenario {
    variant("strength_24", &|text| {
        once(
            text,
            "ready = 18\npilots = 18",
            "ready = 18\npilots = 18\nstrength = 24",
        )
    })
}

fn japanese_replacements_are_fighters() -> Scenario {
    variant("fighters", &|text| {
        once(
            text,
            "aircraft_type = 3\naircraft_per_day = 2",
            "aircraft_type = 4\naircraft_per_day = 2",
        )
    })
}

/// Every pulse of the runs of a variant worked by `config`.
fn work_all(runs: &[Run], config: &Config) -> Vec<Worked> {
    runs.iter().map(|run| work_the_run(run, config)).collect()
}

/// The aircraft and the pilots of each group are conserved in one run, from
/// its events alone.
fn assert_conserved(run: &Run, config: &Config) {
    for g in GROUPS {
        let i = of(g);
        let (mut shot, mut ground, mut got_aircraft) = (0, 0, 0);
        let (mut pilots_lost, mut got_pilots) = (0, 0);
        for event in &run.events {
            match event.kind {
                EventKind::AircraftLost {
                    air_group,
                    aircraft,
                    ..
                } if air_group == group(g) => shot += aircraft,
                EventKind::AircraftDestroyedOnGround {
                    air_group,
                    aircraft,
                    ..
                } if air_group == group(g) => ground += aircraft,
                EventKind::PilotsLost {
                    air_group, lost, ..
                } if air_group == group(g) => pilots_lost += lost,
                EventKind::ReplacementsReceived {
                    air_group,
                    aircraft,
                    pilots,
                    ..
                } if air_group == group(g) => {
                    got_aircraft += aircraft;
                    got_pilots += pilots;
                }
                _ => {}
            }
        }
        assert_eq!(
            START[i] + got_aircraft,
            run.ready[i] + shot + ground,
            "seed {}, group {g}: the aircraft are conserved",
            run.seed
        );
        if let Some(start) = config.pilots[i] {
            assert_eq!(
                start + got_pilots,
                run.pilots[i].expect("tracked") + pilots_lost,
                "seed {}, group {g}: the pilots are conserved",
                run.seed
            );
        } else {
            assert_eq!(run.pilots[i], None);
            assert_eq!((pilots_lost, got_pilots), (0, 0));
        }
        assert!(run.ready[i] <= config.strength[i], "seed {}", run.seed);
    }
}

// ---------------------------------------------------------------------
// The tests.
// ---------------------------------------------------------------------

#[test]
fn the_setting_is_as_the_hand_calculation_says() {
    let scenario = load();
    let truth = scenario.world.umpire();
    assert_eq!(truth.bases.len(), 2);
    assert_eq!(truth.bases[&AIRFIELD_A].side, Side::Japanese);
    assert_eq!(truth.bases[&AIRFIELD_A].hex, Hex::new(0, 0));
    assert_eq!(truth.bases[&AIRFIELD_B].side, Side::Allied);
    assert_eq!(truth.bases[&AIRFIELD_B].hex, Hex::new(5, 0));
    assert_eq!(truth.bases[&AIRFIELD_B].supply_t, Fixed::from_int(1000));
    assert_eq!(truth.bases[&AIRFIELD_B].anti_aircraft, 20);
    for g in GROUPS {
        let air_group = &truth.air_groups[&group(g)];
        let i = of(g);
        assert_eq!(air_group.ready, START[i], "group {g}");
        assert_eq!(air_group.pilots, Some(START[i]), "group {g}");
        // A group's strength is what it starts with.
        assert_eq!(air_group.strength, START[i], "group {g}");
        assert_eq!(air_group.damaged, 0);
        assert_eq!(air_group.aircraft_type.raw(), GROUP_TYPE[i]);
        assert_eq!(air_group.location, AirGroupLocation::Base(GROUP_BASE[i]));
        assert_eq!(
            air_group.side,
            if g <= 2 { Side::Japanese } else { Side::Allied }
        );
    }
    // The replacements, a day.
    let streams: Vec<(Side, u32, u32, u32, u32)> = truth
        .replacements
        .iter()
        .map(|s| {
            (
                s.side,
                s.base.raw(),
                s.aircraft_type.raw(),
                s.aircraft_per_day,
                s.pilots_per_day,
            )
        })
        .collect();
    assert_eq!(
        streams,
        vec![
            (Side::Allied, 2, 3, 1, 0),
            (Side::Allied, 2, 4, 2, 0),
            (Side::Japanese, 1, 3, 2, 1),
        ]
    );
    // No wear and no fatigue: no file for either.
    assert!(scenario.world.ref_data().air_upkeep.is_none());
    assert!(scenario.world.ref_data().fatigue.is_none());
    // Each side sees its own streams and groups' pilots and not the other's.
    let japanese = scenario.world.side_view(Side::Japanese);
    assert_eq!(japanese.replacements().count(), 1);
    assert_eq!(
        japanese.air_groups().map(|g| g.pilots).collect::<Vec<_>>(),
        vec![Some(18), Some(12)]
    );
    assert_eq!(
        scenario
            .world
            .side_view(Side::Allied)
            .replacements()
            .count(),
        2
    );
}

#[test]
fn the_rule_tables_are_as_the_hand_calculation_says() {
    // What flies: the smaller of R and P; R where not tracked.
    for ready in 0..=30 {
        for pilots in 0..=30 {
            assert_eq!(flies(ready, Some(pilots)), ready.min(pilots));
        }
        assert_eq!(flies(ready, None), ready);
    }
    // Pilots lost: groups 3, 4 and 5 never lose a pilot; groups 1 and 2 lose
    // one with each aircraft shot down, in every run (the loss table).
    for run in batch() {
        for event in &run.events {
            if let EventKind::PilotsLost { air_group, .. } = event.kind {
                assert!(
                    [BOMBERS, ESCORTS].contains(&air_group.raw()),
                    "seed {}: group {air_group} lost a pilot",
                    run.seed
                );
            }
        }
        assert_eq!(run.pilots[of(DEFENDERS)], Some(12), "seed {}", run.seed);
        assert_eq!(run.pilots[of(RESTING)], Some(10), "seed {}", run.seed);
        assert_eq!(run.pilots[of(SCOUTS)], Some(6), "seed {}", run.seed);
    }
    for worked in worked_batch() {
        assert_eq!(
            worked.pilots_lost[of(BOMBERS)],
            worked.shot_down[of(BOMBERS)]
        );
        assert_eq!(
            worked.pilots_lost[of(ESCORTS)],
            worked.shot_down[of(ESCORTS)]
        );
        for g in [DEFENDERS, RESTING, SCOUTS] {
            assert_eq!(worked.pilots_lost[of(g)], 0);
        }
    }
}

#[test]
fn every_pulse_of_every_run_is_worked_by_the_rule_from_the_counts_at_the_start_of_it() {
    let worked = worked_batch();
    assert_eq!(worked.len(), usize::try_from(SEEDS).expect("small"));
    for (run, worked) in batch().iter().zip(worked) {
        assert_eq!(worked.days.len(), 6);
        assert_eq!(run.hashes.len(), 6);
    }
    // Every event of every run is accounted for: the orders at the start,
    // and the pulses worked.
    for run in batch() {
        let accounted = run
            .events
            .iter()
            .filter(|e| e.pulse.is_none())
            .all(|e| matches!(e.kind, EventKind::OrderAccepted { .. }));
        assert!(accounted, "seed {}", run.seed);
    }
}

#[test]
fn the_first_day_and_the_second_morning_are_as_the_hand_calculation_says() {
    let (mut sum_b, mut sum_ready, mut sum_flies) = (0, 0, 0);
    for (run, worked) in batch().iter().zip(worked_batch()) {
        let seed = run.seed;
        let (day1, day2) = (&worked.days[0], &worked.days[1]);
        // Day 1 is scenario 6a's first raid: 18 bombers, 6 escorts, a patrol
        // of 12, ten aircraft on the ground.
        assert_eq!(
            (day1.bombers, day1.escorts, day1.patrol, day1.kept),
            (18, 6, 12, 6)
        );
        assert_eq!(day1.ground_before[of(RESTING)], 10, "seed {seed}");
        assert_eq!(day1.ground_before[of(DEFENDERS)], 0, "seed {seed}");
        // b, e and f of the run.
        let b = day1.shot_down[of(BOMBERS)];
        let e = day1.shot_down[of(ESCORTS)];
        let f = day1.shot_down[of(DEFENDERS)];
        // After day 1.
        assert_eq!(day1.ready_end[of(BOMBERS)], 18 - b, "seed {seed}");
        assert_eq!(day1.pilots_end[of(BOMBERS)], Some(18 - b), "seed {seed}");
        assert_eq!(day1.ready_end[of(ESCORTS)], 12 - e, "seed {seed}");
        assert_eq!(day1.pilots_end[of(ESCORTS)], Some(12 - e), "seed {seed}");
        assert_eq!(day1.ready_end[of(DEFENDERS)], 12 - f, "seed {seed}");
        assert_eq!(day1.pilots_end[of(DEFENDERS)], Some(12), "seed {seed}");
        // Day 1 delivered nothing, and nothing in the log says it did.
        assert_eq!(day1.received_aircraft, [0; 5]);
        assert_eq!(day1.received_pilots, [0; 5]);
        assert!(run.pulse(1, Pulse::Night).is_empty(), "seed {seed}");
        // After the night's replacements.
        assert_eq!(day2.ready_night[of(BOMBERS)], 18.min(20 - b), "seed {seed}");
        assert_eq!(
            day2.pilots_night[of(BOMBERS)],
            Some(18.min(19 - b)),
            "seed {seed}"
        );
        assert_eq!(day2.ready_night[of(ESCORTS)], 12 - e, "seed {seed}");
        assert_eq!(day2.pilots_night[of(ESCORTS)], Some(12 - e), "seed {seed}");
        assert_eq!(
            day2.ready_night[of(DEFENDERS)],
            12.min(14 - f),
            "seed {seed}"
        );
        assert_eq!(day2.pilots_night[of(DEFENDERS)], Some(12), "seed {seed}");
        // The same, read from the log of the second night.
        for event in run.pulse(2, Pulse::Night) {
            if let EventKind::ReplacementsReceived {
                air_group,
                ready,
                pilots_now,
                ..
            } = event.kind
            {
                match air_group.raw() {
                    BOMBERS => assert_eq!(
                        (ready, pilots_now),
                        (18.min(20 - b), 18.min(19 - b)),
                        "seed {seed}"
                    ),
                    DEFENDERS => assert_eq!((ready, pilots_now), (12.min(14 - f), 12)),
                    other => assert_eq!(other, RESTING, "seed {seed}"),
                }
            }
        }
        // What flies on day 2.
        let bombers_fly = if b >= 1 { 19 - b } else { 18 };
        assert_eq!(day2.able_morning[of(BOMBERS)], bombers_fly, "seed {seed}");
        assert_eq!(day2.bombers, bombers_fly, "seed {seed}");
        assert_eq!(day2.able_morning[of(ESCORTS)], 12 - e, "seed {seed}");
        assert_eq!(
            day2.able_morning[of(DEFENDERS)],
            12.min(14 - f),
            "seed {seed}"
        );
        assert_eq!(day2.patrol, 12.min(14 - f), "seed {seed}");
        sum_b += i64::from(b);
        sum_ready += i64::from(day2.ready_night[of(BOMBERS)]);
        sum_flies += i64::from(day2.able_morning[of(BOMBERS)]);
    }
    // "b is 2.8 on average, so on the second morning group 1 has about 17
    // aircraft and flies about 16."
    let runs = count(batch().len());
    eprintln!(
        "mean b x100 {}, group 1's aircraft x100 {}, flies x100 {}",
        sum_b * 100 / runs,
        sum_ready * 100 / runs,
        sum_flies * 100 / runs
    );
    assert!(
        (260..=300).contains(&(sum_b * 100 / runs)),
        "b is about 2.8"
    );
    assert!((1700..1750).contains(&(sum_ready * 100 / runs)), "about 17");
    assert!((1600..1650).contains(&(sum_flies * 100 / runs)), "about 16");
}

#[test]
fn over_the_six_days_aircraft_and_pilots_are_conserved_and_the_three_statements_hold() {
    let config = scenario_config();
    let (mut defenders_ready, mut escorts_ready) = (0, 0);
    for (run, worked) in batch().iter().zip(worked_batch()) {
        let seed = run.seed;
        assert_conserved(run, &config);
        // The same, from the rebuilt counts.
        for g in GROUPS {
            let i = of(g);
            assert_eq!(
                START[i] + worked.received_aircraft[i],
                worked.ready[i] + worked.shot_down[i] + worked.on_the_ground[i],
                "seed {seed}, group {g}"
            );
            assert_eq!(
                START[i] + worked.received_pilots[i],
                worked.pilots[i] + worked.pilots_lost[i],
                "seed {seed}, group {g}"
            );
            assert!(
                worked.ready[i] <= START[i],
                "seed {seed}: never above strength"
            );
            assert!(
                worked.pilots[i] <= START[i],
                "seed {seed}: never above strength"
            );
        }
        // The events say the same at every point: ready and pilots after a
        // replacement never pass the strength.
        for event in &run.events {
            if let EventKind::ReplacementsReceived {
                air_group,
                ready,
                pilots_now,
                ..
            } = event.kind
            {
                assert!(ready <= START[of(air_group.raw())], "seed {seed}");
                assert!(pilots_now <= START[of(air_group.raw())], "seed {seed}");
            }
        }
        // Group 1's pilots are never more than its ready aircraft, at any
        // time: after each night and at the end of each day.
        for day in &worked.days {
            for (ready, pilots) in [
                (day.ready_start, day.pilots_start),
                (day.ready_night, day.pilots_night),
                (day.ready_end, day.pilots_end),
            ] {
                assert!(
                    pilots[of(BOMBERS)].expect("tracked") <= ready[of(BOMBERS)],
                    "seed {seed}"
                );
            }
        }
        for event in &run.events {
            if let EventKind::ReplacementsReceived {
                air_group,
                ready,
                pilots_now,
                ..
            } = event.kind
            {
                if air_group == group(BOMBERS) {
                    assert!(pilots_now <= ready, "seed {seed}");
                }
            }
        }
        // Group 3 has all 12 pilots at the end, and group 2 received nothing.
        assert_eq!(run.pilots[of(DEFENDERS)], Some(12), "seed {seed}");
        assert_eq!(worked.received_aircraft[of(ESCORTS)], 0, "seed {seed}");
        assert_eq!(worked.received_pilots[of(ESCORTS)], 0, "seed {seed}");
        assert!(
            run.of(|k| matches!(
                k,
                EventKind::ReplacementsReceived { air_group, .. } if *air_group == group(ESCORTS)
            ))
            .is_empty(),
            "seed {seed}"
        );
        defenders_ready += i64::from(run.ready[of(DEFENDERS)]);
        escorts_ready += i64::from(run.ready[of(ESCORTS)]);
    }
    // The Allied fighters hold up and the Japanese do not.
    assert!(
        defenders_ready > escorts_ready,
        "group 3 ends with {defenders_ready} in all, group 2 with {escorts_ready}"
    );
}

#[test]
fn with_no_group_given_pilots_pilots_are_not_tracked_and_aircraft_replacements_still_arrive() {
    let variant = without_pilots();
    let runs = batch_of(&variant, VARIANT_SEEDS);
    let worked = work_all(&runs, &config_without_pilots());
    for (run, worked) in runs.iter().zip(&worked) {
        let seed = run.seed;
        assert_eq!(run.pilots, [None; 5], "seed {seed}");
        assert!(
            run.of(|k| matches!(k, EventKind::PilotsLost { .. }))
                .is_empty(),
            "seed {seed}"
        );
        for event in run.of(|k| matches!(k, EventKind::ReplacementsReceived { .. })) {
            if let EventKind::ReplacementsReceived {
                pilots, pilots_now, ..
            } = event.kind
            {
                assert_eq!((pilots, pilots_now), (0, 0), "seed {seed}");
            }
        }
        // Each group flies its ready aircraft.
        for day in &worked.days {
            assert_eq!(day.able_morning, day.ready_night, "seed {seed}");
        }
        assert_conserved(run, &config_without_pilots());
    }
    // Aircraft replacements still arrive.
    let received: u32 = worked
        .iter()
        .map(|w| w.received_aircraft[of(BOMBERS)])
        .sum();
    assert!(received > 0);
    // Pilots are in the state: the same seed ends in another state.
    let tracked = run_of(&load(), 5);
    assert_ne!(tracked.hashes, runs[4].hashes);
}

#[test]
fn with_no_replacements_nothing_is_received_and_the_first_three_days_are_scenario_6a() {
    let variant = without_replacements();
    assert!(variant.world.umpire().replacements.is_empty());
    let reference_scenario = six_a();
    let runs = batch_of(&variant, VARIANT_SEEDS);
    let worked = work_all(&runs, &config_without_replacements());
    for (run, worked) in runs.iter().zip(&worked) {
        let seed = run.seed;
        assert!(
            run.of(|k| matches!(k, EventKind::ReplacementsReceived { .. }))
                .is_empty(),
            "seed {seed}"
        );
        assert_eq!(worked.received_aircraft, [0; 5]);
        assert_eq!(worked.received_pilots, [0; 5]);
        assert_conserved(run, &config_without_replacements());
        // Event for event apart from the pilots lost, days 1 to 3 are
        // scenario 6a's with the same seed: same kinds, breakdowns and
        // causes, with the events numbered as 6a numbers them.
        let reference = run_of(&reference_scenario, seed);
        // The claims from the air are in neither list but are in both logs,
        // where they leave the same gaps in the numbers, so a number in 6a's
        // numbering is ours less the pilots lost before it.
        let mut mapped: BTreeMap<u32, u32> = BTreeMap::new();
        let mut skipped = 0;
        let mut ours: Vec<&Event> = Vec::new();
        for event in &run.events {
            if event.day > 3 {
                continue;
            }
            if matches!(event.kind, EventKind::PilotsLost { .. }) {
                skipped += 1;
                continue;
            }
            mapped.insert(event.id.raw(), event.id.raw() - skipped);
            ours.push(event);
        }
        assert_eq!(ours.len(), reference.events.len(), "seed {seed}");
        for (a, b) in ours.iter().zip(&reference.events) {
            assert_eq!(
                (a.day, a.pulse, &a.kind, a.breakdown.clone()),
                (b.day, b.pulse, &b.kind, b.breakdown.clone()),
                "seed {seed}"
            );
            let causes: Vec<Cause> = a
                .causes
                .iter()
                .map(|cause| match cause {
                    Cause::Event(id) => Cause::Event(EventId::new(mapped[&id.raw()])),
                    other => *other,
                })
                .collect();
            assert_eq!(causes, b.causes, "seed {seed}: {:?}", a.kind);
            assert_eq!(mapped[&a.id.raw()], b.id.raw(), "seed {seed}");
        }
    }
}

#[test]
fn if_the_japanese_receive_2_pilots_a_day_group_1s_pilots_keep_pace_with_its_aircraft() {
    let variant = japanese_receive_pilots(2);
    let runs = batch_of(&variant, VARIANT_SEEDS);
    let config = config_japanese_pilots(2);
    let worked = work_all(&runs, &config);
    for (run, worked) in runs.iter().zip(&worked) {
        assert_conserved(run, &config);
        // Aircraft and pilots go together: 2 and 2 a day, one lost with each.
        for day in &worked.days {
            for (ready, pilots) in [
                (day.ready_start, day.pilots_start),
                (day.ready_night, day.pilots_night),
                (day.ready_end, day.pilots_end),
            ] {
                assert_eq!(
                    pilots[of(BOMBERS)],
                    Some(ready[of(BOMBERS)]),
                    "seed {}",
                    run.seed
                );
            }
            // It flies the smaller of the two, as before: all its aircraft.
            assert_eq!(day.able_morning[of(BOMBERS)], day.ready_night[of(BOMBERS)]);
        }
    }
}

#[test]
fn if_group_3_starts_with_8_pilots_it_flies_8_and_the_other_4_are_on_the_ground() {
    let variant = defenders_with_pilots(8);
    let runs = batch_of(&variant, VARIANT_SEEDS);
    let mut config = scenario_config();
    config.pilots[of(DEFENDERS)] = Some(8);
    let worked = work_all(&runs, &config);
    let mut hit_group_4 = 0;
    for (run, worked) in runs.iter().zip(&worked) {
        let seed = run.seed;
        assert_conserved(run, &config);
        let day1 = &worked.days[0];
        // It flies 8 of its 12 ready, and the other 4 are on the ground.
        assert_eq!(day1.able_morning[of(DEFENDERS)], 8, "seed {seed}");
        assert_eq!(day1.patrol, 8, "seed {seed}");
        assert_eq!(day1.ground_before[of(DEFENDERS)], 4, "seed {seed}");
        // It receives no pilot (none are sent) and loses none: 8 throughout.
        assert_eq!(run.pilots[of(DEFENDERS)], Some(8), "seed {seed}");
        for day in &worked.days {
            assert!(day.able_morning[of(DEFENDERS)] <= 8, "seed {seed}");
            // The aircraft beyond its pilots are destroyed before group 4's:
            // group 4 loses one only when all of group 3's on the ground did.
            if day.on_the_ground[of(RESTING)] > 0 {
                hit_group_4 += 1;
                assert_eq!(
                    day.on_the_ground[of(DEFENDERS)],
                    day.ground_before[of(DEFENDERS)],
                    "seed {seed}"
                );
            }
        }
    }
    assert!(hit_group_4 > 0, "group 4 is hit in some run");
}

#[test]
fn if_group_1_has_a_strength_of_24_it_receives_2_aircraft_and_1_pilot_a_day_even_with_no_loss() {
    let variant = bombers_with_strength_24();
    let runs = batch_of(&variant, VARIANT_SEEDS);
    let mut config = scenario_config();
    config.strength[of(BOMBERS)] = 24;
    let worked = work_all(&runs, &config);
    let mut above_18 = 0;
    for (run, worked) in runs.iter().zip(&worked) {
        let seed = run.seed;
        assert_conserved(run, &config);
        // Day 1 delivers nothing; from day 2 group 1 is always short of 24:
        // it receives 2 and 1 every night.
        assert!(run.pulse(1, Pulse::Night).is_empty());
        for day in 2..=DAYS {
            let record = &worked.days[usize::try_from(day - 1).expect("small")];
            assert_eq!(
                record.received_aircraft[of(BOMBERS)],
                2,
                "seed {seed}, day {day}"
            );
            assert_eq!(
                record.received_pilots[of(BOMBERS)],
                1,
                "seed {seed}, day {day}"
            );
            above_18 += usize::from(record.ready_night[of(BOMBERS)] > 18);
        }
    }
    assert!(above_18 > 0, "the group climbs above what it started with");
}

#[test]
fn if_the_japanese_replacements_are_fighters_group_2_receives_them_and_group_1_nothing() {
    let variant = japanese_replacements_are_fighters();
    let runs = batch_of(&variant, VARIANT_SEEDS);
    let mut config = scenario_config();
    config.streams[0].aircraft_type = FIGHTER;
    let worked = work_all(&runs, &config);
    let mut group_2 = 0;
    for (run, worked) in runs.iter().zip(&worked) {
        assert_conserved(run, &config);
        assert_eq!(
            worked.received_aircraft[of(BOMBERS)],
            0,
            "seed {}",
            run.seed
        );
        assert_eq!(worked.received_pilots[of(BOMBERS)], 0, "seed {}", run.seed);
        group_2 += worked.received_aircraft[of(ESCORTS)];
        for event in run.of(|k| matches!(k, EventKind::ReplacementsReceived { .. })) {
            if let EventKind::ReplacementsReceived {
                air_group, base, ..
            } = event.kind
            {
                if base == AIRFIELD_A {
                    assert_eq!(air_group, group(ESCORTS));
                }
            }
        }
    }
    assert!(group_2 > 0);
}

#[test]
fn what_cannot_happen_does_not_in_any_run() {
    let config = scenario_config();
    for (run, worked) in batch().iter().zip(worked_batch()) {
        let seed = run.seed;
        for event in &run.events {
            match event.kind {
                // Nothing is received on day 1, in a daylight pulse, by a
                // group of another type, or by a group at another base.
                EventKind::ReplacementsReceived {
                    air_group, base, ..
                } => {
                    assert!(event.day >= 2, "seed {seed}: nothing on day 1");
                    assert_eq!(event.pulse, Some(Pulse::Night), "seed {seed}");
                    let i = of(air_group.raw());
                    assert_eq!(GROUP_BASE[i], base, "seed {seed}");
                    assert!(
                        config
                            .streams
                            .iter()
                            .any(|s| s.base == base && s.aircraft_type == GROUP_TYPE[i]),
                        "seed {seed}: no stream for group {air_group}"
                    );
                    assert!([BOMBERS, DEFENDERS, RESTING].contains(&air_group.raw()));
                }
                // No pilot of group 3, 4 or 5 is lost.
                EventKind::PilotsLost { air_group, .. } => {
                    assert!([BOMBERS, ESCORTS].contains(&air_group.raw()), "seed {seed}");
                }
                _ => {}
            }
        }
        // No group flies more aircraft than it has pilots, and none more
        // than strength: asserted pulse by pulse in the working; here the
        // records say so too.
        for day in &worked.days {
            for g in GROUPS {
                let i = of(g);
                assert!(day.able_morning[i] <= day.pilots_night[i].expect("tracked"));
                assert!(day.able_afternoon[i] <= day.pilots_end[i].expect("tracked"));
                assert!(day.ready_night[i] <= START[i] && day.ready_end[i] <= START[i]);
                assert!(day.pilots_night[i].expect("tracked") <= START[i]);
                assert!(day.pilots_end[i].expect("tracked") <= START[i]);
            }
            // Group 5 has six pilots for six scouts: all six are in the air
            // and none is ever destroyed on the ground.
            assert_eq!(day.able_morning[of(SCOUTS)], 6, "seed {seed}");
            assert_eq!(day.on_the_ground[of(SCOUTS)], 0, "seed {seed}");
        }
        assert_eq!(run.ready[of(SCOUTS)], 6, "seed {seed}");
    }
}

#[test]
fn no_random_number_is_drawn_for_pilots_or_replacements_the_first_morning_has_scenario_6as_rolls() {
    let reference_scenario = six_a();
    for seed in 1..=VARIANT_SEEDS {
        let run = run_of(&load(), seed);
        let reference = run_of(&reference_scenario, seed);
        let ours: Vec<&Event> = run
            .pulse(1, Pulse::Morning)
            .into_iter()
            .filter(|e| !matches!(e.kind, EventKind::PilotsLost { .. }))
            .collect();
        let theirs = reference.pulse(1, Pulse::Morning);
        assert_eq!(ours.len(), theirs.len(), "seed {seed}");
        for (a, b) in ours.iter().zip(&theirs) {
            assert_eq!(a.kind, b.kind, "seed {seed}");
        }
    }
}

#[test]
fn more_pilots_never_fly_fewer_aircraft() {
    // The smaller of R and P does not fall as P rises.
    for ready in 0..=30 {
        let mut last = 0;
        for pilots in 0..=40 {
            let now = flies(ready, Some(pilots));
            assert!(now >= last && now <= ready);
            last = now;
        }
    }
    // And in the engine: the bombers launched on the first morning.
    let mut last = 0;
    for pilots in 0..=20 {
        let run = run_of(&bombers_with_pilots(pilots), 1);
        let launched = run
            .pulse(1, Pulse::Morning)
            .into_iter()
            .find_map(|e| match e.kind {
                EventKind::BaseStrikeLaunched { aircraft, .. } => Some(aircraft),
                _ => None,
            })
            .unwrap_or(0);
        assert_eq!(launched, 18.min(pilots), "{pilots} pilots");
        assert!(launched >= last);
        last = launched;
    }
}

#[test]
fn more_replacements_never_leave_a_group_weaker() {
    let seeds = VARIANT_SEEDS;
    let ready_at_the_end = |scenario: &Scenario, config: &Config| -> i64 {
        let runs = batch_of(scenario, seeds);
        work_all(&runs, config);
        runs.iter().map(|r| i64::from(r.ready[of(BOMBERS)])).sum()
    };
    let none = ready_at_the_end(&japanese_receive_aircraft(0), &config_japanese_aircraft(0));
    let some = ready_at_the_end(&load(), &scenario_config());
    let more = ready_at_the_end(&japanese_receive_aircraft(4), &config_japanese_aircraft(4));
    assert!(some > none, "2 a day: {some}, none: {none}");
    assert!(more >= some, "4 a day: {more}, 2 a day: {some}");
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
        for (x, y) in a.iter().zip(&b) {
            assert_eq!(
                (x.id, x.day, x.pulse, &x.kind, &x.causes),
                (y.id, y.day, y.pulse, &y.kind, &y.causes)
            );
            assert!(y.breakdown.is_none());
        }
    }
}

#[test]
fn the_same_seed_gives_the_same_hashes_twice() {
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
        assert_eq!(hashes[0].len(), 6);
        assert_eq!(hashes[0], hashes[1], "seed {seed}");
        all_hashes.push(hashes[0].clone());
    }
    assert!(all_hashes.windows(2).any(|pair| pair[0] != pair[1]));
    // The batch holds the same hashes as a fresh run of the same seed.
    let mut again = base.with_seed(77).expect("a seed can be set");
    again.run_to_end().expect("the run resolves");
    assert_eq!(batch()[76].seed, 77);
    assert_eq!(batch()[76].hashes, again.world.day_hashes());
}

#[test]
fn a_scenario_that_gives_no_pilots_and_no_replacements_has_the_hashes_it_had_before_this_stage() {
    // The golden files of the command-line tests are those of the code before
    // pilots existed: every earlier scenario, run with its own seed, has the
    // day hashes they hold.
    let golden =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../crates/sim-cli/tests/run_baseline");
    let mut checked = 0;
    let mut names: Vec<String> = fs::read_dir(&golden)
        .expect("the golden folder")
        .filter_map(|entry| {
            let name = entry.expect("an entry").file_name().into_string().ok()?;
            let stem = name.strip_suffix(".txt")?;
            (!stem.ends_with(".events")
                && !stem.starts_with("06d1")
                // Scenario 6 itself has pilots, replacements and supply.
                && !stem.starts_with("06_")
                // So has the full slice of scenario 7 (decision 0043).
                && !stem.starts_with("07_")
                // And the soak, which carries it on for 180 days (decision 0044).
                && !stem.starts_with("08_")
                // Scenario 12a gives its bombing squadron pilots (decision 0059).
                && !stem.starts_with("12a_"))
            .then(|| stem.to_string())
        })
        .collect();
    names.sort();
    for name in names {
        let text = fs::read_to_string(golden.join(format!("{name}.txt"))).expect("a golden file");
        let wanted: Vec<u64> = text
            .lines()
            .filter(|line| line.starts_with("day "))
            .map(|line| {
                let hash = line.split_whitespace().nth(3).expect("a hash");
                u64::from_str_radix(hash, 16).expect("hex")
            })
            .collect();
        assert!(!wanted.is_empty(), "{name}");
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../data/scenarios/{name}.toml"));
        // The golden files are the run baselines: they follow the live
        // figures, so this run does too (docs/step2-spec.md, "Tests while
        // figures move").
        let mut scenario = common::load_live(&path).expect("the scenario loads");
        let truth = scenario.world.umpire();
        assert!(truth.replacements.is_empty(), "{name}");
        assert!(
            truth.air_groups.values().all(|g| g.pilots.is_none()),
            "{name}"
        );
        scenario.run_to_end().expect("runs");
        assert_eq!(scenario.world.day_hashes(), wanted, "{name}");
        checked += 1;
    }
    assert!(checked >= 15, "{checked} golden files checked");
}

#[test]
fn mean_figures_of_the_scenario_over_1000_seeds() {
    // Not an assertion about the rule: the figures a reader wants, printed
    // with --nocapture.
    let worked = worked_batch();
    let total = count(worked.len());
    let mean_x100 = |sum: i64| sum * 100 / total;
    for day in 0..6usize {
        let launched: i64 = worked.iter().map(|w| i64::from(w.days[day].bombers)).sum();
        let mut line = format!(
            "day {}: bombers launched x100 {}",
            day + 1,
            mean_x100(launched)
        );
        for g in [BOMBERS, ESCORTS, DEFENDERS] {
            let ready: i64 = worked
                .iter()
                .map(|w| i64::from(w.days[day].ready_end[of(g)]))
                .sum();
            let pilots: i64 = worked
                .iter()
                .map(|w| i64::from(w.days[day].pilots_end[of(g)].expect("tracked")))
                .sum();
            line.push_str(&format!(
                "; group {g} ready x100 {}, pilots x100 {}",
                mean_x100(ready),
                mean_x100(pilots)
            ));
        }
        eprintln!("{line}");
    }
    for g in GROUPS {
        let aircraft: i64 = worked
            .iter()
            .map(|w| i64::from(w.received_aircraft[of(g)]))
            .sum();
        let pilots: i64 = worked
            .iter()
            .map(|w| i64::from(w.received_pilots[of(g)]))
            .sum();
        eprintln!(
            "group {g} per run x100: received {} aircraft, {} pilots",
            mean_x100(aircraft),
            mean_x100(pilots)
        );
    }
}
