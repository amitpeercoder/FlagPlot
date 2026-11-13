//! Scenario 6c: eight days of raids on an airfield, with fatigue and rest.
//!
//! Stage 6c of scenario 6 (decision 0036). The data is
//! `data/scenarios/06c_fatigue_and_rest.toml`, run with seeds 1 to 1000.
//! Everything in it is a placeholder.
//!
//! # Hand calculation
//!
//! ## The setting
//!
//! It is scenario 6a (`tests/scenario_06a_base_raid.rs`) over eight days,
//! with the fatigue figures and no wear: no aircraft is ever damaged.
//! Airfield A, base 1, Japanese, is at (0,0). Airfield B, base 2, Allied,
//! is at (5,0), 200 nm away, with 1,000 t of supply and 20 anti-aircraft
//! shots.
//!
//! | Group | Side | At | Aircraft | Mission | Rests above |
//! | ----- | ---- | -- | -------- | ------- | ----------- |
//! | 1 | Japanese | A | 18 bombers | base strike on airfield B | 30 |
//! | 2 | Japanese | A | 12 fighters | combat air patrol, share 0.5 | 40 |
//! | 3 | Allied | B | 12 fighters | combat air patrol, share 1 | never |
//! | 4 | Allied | B | 10 bombers | rest | never |
//! | 5 | Allied | B | 6 scouts | search, arc out to 300 nm | 50 |
//!
//! Every group starts with a fatigue of 0.
//!
//! ## The figures
//!
//! A group that flew in a daylight pulse gains 3, and 2 more for each 100
//! nm of the sortie. A group that did not fly loses 5, and not below 0.
//!
//! | Sortie | Distance | Gain |
//! | ------ | -------- | ---- |
//! | A patrol | none | 3 |
//! | The raid on airfield B, bombers or escorts | 400 nm | 3 + 2 x 4 = 11 |
//! | A search out to 300 nm | 600 nm | 3 + 2 x 6 = 15 |
//!
//! A fighter group that kept a patrol and sent escorts gains 11, once.
//!
//! A group with R ready aircraft and fatigue f flies R x (100 - f) / 100,
//! rounded down. Write that as fly(R, f).
//!
//! ## A day
//!
//! **Night, at the start of the day.** Each group with a threshold is stood
//! down if its fatigue is above the threshold, and returned to flying if it
//! was stood down and its fatigue is not above it. Nothing else happens at
//! night.
//!
//! **Morning.** On the counts and fatigue at the start of the pulse:
//!
//! - Group 5, if not stood down, searches with fly(R5, f5) aircraft.
//! - Group 1, if not stood down, launches fly(R1, f1) bombers at airfield B,
//!   if that is at least one.
//! - Group 2, if not stood down, can fly F = fly(R2, f2). It keeps F x 0.5,
//!   rounded down, on patrol over airfield A. The rest escort the raid, if
//!   one launches; if none does they do not fly.
//! - Group 3's patrol is fly(R3, f3).
//! - The raid is worked as in scenario 6a from those numbers. The aircraft
//!   on the ground at airfield B are every ready aircraft there that is not
//!   flying: all of group 4, the R3 - fly(R3, f3) fighters group 3 left
//!   behind, and the scouts group 5 left behind, which is all of them when
//!   it is stood down. Those destroyed are taken from group 3 first, then
//!   group 4, then group 5.
//! - Then fatigue. A group that flew at least one aircraft in the pulse
//!   gains for its sortie; a group that flew none loses 5.
//!
//! **Afternoon.** No raid flies. Group 5, if not stood down, searches with
//! fly(R5, f5). Group 2, if not stood down, keeps fly(R2, f2) x 0.5,
//! rounded down, on patrol and the rest do not fly. Group 3's patrol is
//! fly(R3, f3). Group 1 does not fly. Then fatigue, as in the morning.
//!
//! (Added with decision 0063, and accepted by the architect on Nov 12,
//! 2026: the decision lets a base strike launch in the first
//! daylight pulse of the day in which it can, once a day. The paragraph above
//! holds for every day on which the bombers launch in the morning, which is
//! every day of the tables below. In a run in which they can fly no aircraft
//! in the morning, for their fatigue, and one in the afternoon, once the
//! morning's rest has lowered it, they launch the raid in the afternoon, and
//! it is worked as the morning's is. No number of the tables moves.)
//!
//! Group 4 never flies and its fatigue is 0 throughout: it has nothing to
//! lose.
//!
//! ## Fatigue day by day
//!
//! Fatigue does not depend on any roll, only on whether a group flew. A
//! group that is not stood down flies as long as fly(R, f) is at least 1.
//! For group 2 it is what it flies in the pulse that must be at least 1: its
//! patrol and its escorts on a morning with a raid, its patrol alone
//! otherwise. The tables below hold for every run in which that is so in
//! every pulse; the test works the rule for the others. (The first version
//! said "its patrol or its escorts" for group 2. With one aircraft able to
//! fly the patrol is none and the one escort flies only with a raid, so in
//! the afternoon the group does not fly and rests. No figure changed.)
//!
//! Group 1, the bombers, threshold 30. A day flown is +11 in the morning and
//! -5 in the afternoon. A day stood down is -5 and -5.
//!
//! | Day | At the start | The staff | After the morning | After the afternoon |
//! | --- | ------------ | --------- | ----------------- | ------------------- |
//! | 1 | 0 | flies | 11 | 6 |
//! | 2 | 6 | flies | 17 | 12 |
//! | 3 | 12 | flies | 23 | 18 |
//! | 4 | 18 | flies | 29 | 24 |
//! | 5 | 24 | flies | 35 | 30 |
//! | 6 | 30, not above 30 | flies | 41 | 36 |
//! | 7 | 36 | stood down | 31 | 26 |
//! | 8 | 26 | returned, flies | 37 | 32 |
//!
//! So a raid is launched on every morning but the seventh, and the bombers
//! it launches are fly(R1, f): on day 1 all 18, and on day 2, at a fatigue
//! of 6, R1 x 0.94 rounded down.
//!
//! Group 2, the escort group, threshold 40. A morning with a raid is +11, a
//! morning without one +3, an afternoon +3.
//!
//! | Day | At the start | The staff | After the morning | After the afternoon |
//! | --- | ------------ | --------- | ----------------- | ------------------- |
//! | 1 | 0 | flies | 11 | 14 |
//! | 2 | 14 | flies | 25 | 28 |
//! | 3 | 28 | flies | 39 | 42 |
//! | 4 | 42 | stood down | 37 | 32 |
//! | 5 | 32 | returned, flies | 43 | 46 |
//! | 6 | 46 | stood down | 41 | 36 |
//! | 7 | 36 | returned, flies; no raid | 39 | 42 |
//! | 8 | 42 | stood down | 37 | 32 |
//!
//! So the raids of days 4, 6 and 8 have no escort, and the whole of group
//! 3's patrol fires at the bombers.
//!
//! Group 3, the defending fighters, no threshold: +3 in each daylight
//! pulse, so 6 a day. Its fatigue is 6 x (d - 1) at the start of day d, 3
//! more after the morning, and 48 at the end of day 8. Its patrol on the
//! morning of day d is R3 x (100 - 6 x (d - 1)) / 100, rounded down: all 12
//! on day 1, and at most 12 x 0.58 = 6 on day 8.
//!
//! Group 5, the scouts, threshold 50. Nobody shoots at it, so it has 6
//! aircraft unless one is destroyed on the ground. A day flown is +15 and
//! +15.
//!
//! | Day | At the start | The staff | Searches, morning | After the morning | Searches, afternoon | After the afternoon |
//! | --- | ------------ | --------- | ----------------- | ----------------- | ------------------- | ------------------- |
//! | 1 | 0 | flies | 6 | 15 | 6 x 0.85 = 5.1: 5 | 30 |
//! | 2 | 30 | flies | 6 x 0.7 = 4.2: 4 | 45 | 6 x 0.55 = 3.3: 3 | 60 |
//! | 3 | 60 | stood down | none | 55 | none | 50 |
//! | 4 | 50, not above 50 | returned, flies | 6 x 0.5 = 3 | 65 | 6 x 0.35 = 2.1: 2 | 80 |
//! | 5 | 80 | stood down | none | 75 | none | 70 |
//! | 6 | 70 | stood down | none | 65 | none | 60 |
//! | 7 | 60 | stood down | none | 55 | none | 50 |
//! | 8 | 50 | returned, flies | 3 | 65 | 2 | 80 |
//!
//! The searches in the table are for a group still at 6. The scouts it
//! leaves behind are on the ground, all six on the mornings it is stood
//! down, and a hit on aircraft can destroy one. The fatigue in the table
//! holds while the group can still fly one aircraft when it is not stood
//! down: at a fatigue of 65 that takes 3 ready aircraft. With fewer it does
//! not fly, and rests. The test works the rule from what the group has.
//!
//! ## The first two mornings
//!
//! Day 1 is scenario 6a's first raid: every fatigue is 0, so 18 bombers, 6
//! escorts and a patrol of 12 fly, and group 4's ten are the aircraft on
//! the ground.
//!
//! On day 2 the fatigues are 6, 14, 6 and 30. With R1, R2 and R3 ready:
//!
//! - fly(R1, 6) bombers launch.
//! - Group 2 can fly fly(R2, 14); it keeps half of that, rounded down, and
//!   the rest escort.
//! - The patrol is fly(R3, 6), and R3 less that are on the ground.
//! - Four scouts search and two are on the ground.
//!
//! ## Variants
//!
//! | Variant | What changes |
//! | ------- | ------------ |
//! | The scenario names no fatigue file | No fatigue is gained, nobody is stood down, and every group flies all its ready aircraft. Each of the first three days is, event for event, scenario 6a with the same seed. The rest orders are accepted and do nothing |
//! | No group has a threshold | Nobody is stood down. Group 5's fatigue is 30, 60 and 90 at the end of days 1, 2 and 3, with 6, 5, 4, 3, 2 and 1 scouts searching in those six pulses. At 90 fly(6, 90) is none, so it does not fly and rests: 85, where 6 x 0.15 still rounds down to none, then 80. At 80 one scout flies and it gains 15: 95. Then none, 90, and so on. A tired group with no threshold rests itself, badly |
//! | Group 1's threshold is 0 | It flies on day 1 and ends it at 6. It is stood down on day 2 and rests to 1 and then 0, which is not below 0. At 0 it is not above its threshold, so it flies on day 3. So it flies on days 1, 3, 5 and 7: four raids in eight days |
//! | Airfield B is at (2,0), 80 nm | The raid's sortie is 160 nm: bombers and escorts gain 3 + 2 x 1.6 = 6.2 |
//! | Group 3 starts at a fatigue of 100 | It flies nothing on the first morning: no patrol, all 12 on the ground, and the escorts fire at nothing. It rests to 95, and flies nothing that afternoon: 12 x 0.05 rounds down to none |
//!
//! ## What cannot happen
//!
//! - No fatigue is below 0 or above 100.
//! - No group flies more aircraft than it has ready, and none flies while
//!   stood down.
//! - No fatigue changes at night, and nobody is stood down or returned in a
//!   daylight pulse.
//! - A group with no threshold is never stood down.
//! - No random number is drawn for fatigue: with the fatigue file, the
//!   rolls of the first morning are those of scenario 6a with the same
//!   seed.
//!
//! # Properties
//!
//! - A higher fatigue never flies more aircraft: fly(R, f) does not rise
//!   with f, for every R from 0 to 30.
//! - A longer sortie never gains less fatigue: 3, 6.2, 11, 15.
//! - A lower threshold never has a group fly on more days: over the eight
//!   days group 1 flies on 4 days with a threshold of 0, on 7 with 30 and
//!   on 8 with none, in every run in which it still has an aircraft to fly.
//! - Every group's aircraft at the end are those at the start less its
//!   recorded losses.
//! - Recording breakdowns changes neither the events nor the hashes.
//! - The same seed gives the same hashes twice.
//! - A scenario without the fatigue file has the hashes it had before this
//!   stage: the golden files of scenarios 1 to 6b do not change.

// The test code is written by a developer from the hand calculation above.

mod common;

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use sim_core::events::{AirFirer, AirTarget, Cause, Event, EventKind, FatigueChange, SortieKind};
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
/// The variants run fewer seeds, as scenarios 6a and 6b do.
const VARIANT_SEEDS: u64 = 300;
const DAYS: u32 = 8;
const START: [u32; 5] = [18, 12, 12, 10, 6];
/// The range of the scouts' arc.
const SEARCH_NM: i64 = 300;

fn scenario_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/06c_fatigue_and_rest.toml")
}

fn load() -> Scenario {
    common::load(&scenario_path()).expect("scenario 6c loads")
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

/// A fatigue of whole units, from a number of the tables.
fn n(value: i64) -> Fixed {
    Fixed::from_int(value)
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

/// fly(R, f): R x (100 - f) / 100, rounded down, in exact integer
/// arithmetic on millionths.
fn fly(ready: u32, fatigue: Fixed) -> u32 {
    let hundred = 100 * i128::from(Fixed::SCALE);
    let flown = i128::from(ready) * (hundred - i128::from(fatigue.raw())) / hundred;
    u32::try_from(flown).expect("a count of aircraft")
}

/// The fatigue figures of the hand calculation: 3 for a sortie, 2 for each
/// 100 nm, 5 rest.
#[derive(Clone, Copy)]
struct Figures {
    per_sortie: Fixed,
    per_100: Fixed,
    rest: Fixed,
}

impl Figures {
    /// What a sortie of `distance_nm` gains: the figure for a sortie plus
    /// the figure per 100 nm times the distance over 100.
    fn gain(&self, distance_nm: i64) -> Fixed {
        Fixed::from_raw(self.per_sortie.raw() + self.per_100.raw() * distance_nm / 100)
    }
}

/// What differs between the scenario and its variants.
#[derive(Clone)]
struct Config {
    /// `None`: the scenario names no fatigue file.
    figures: Option<Figures>,
    /// The fatigue above which each group is stood down.
    thresholds: [Option<Fixed>; 5],
    start_fatigue: [Fixed; 5],
    /// Airfield B's distance from airfield A, in hexes.
    raid_hexes: i64,
}

fn scenario_config() -> Config {
    Config {
        figures: Some(Figures {
            per_sortie: n(3),
            per_100: n(2),
            rest: n(5),
        }),
        thresholds: [Some(n(30)), Some(n(40)), None, None, Some(n(50))],
        start_fatigue: [Fixed::ZERO; 5],
        raid_hexes: 5,
    }
}

impl Config {
    fn raid_nm(&self) -> i64 {
        self.raid_hexes * 40
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
    fatigue: [Fixed; 5],
    stood_down: [bool; 5],
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
    /// for group 2 its share. They come before the threshold in the file.
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

    /// The group's rest threshold order: the last of its orders, in the
    /// scenario's file.
    fn threshold_order(&self, g: u32) -> OrderId {
        *self
            .orders_of(g)
            .last()
            .expect("a group with a threshold has orders")
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
        fatigue: GROUPS.map(|id| truth.air_groups[&group(id)].fatigue),
        stood_down: GROUPS.map(|id| truth.air_groups[&group(id)].stood_down),
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

// ---------------------------------------------------------------------
// Working every pulse of a run from the counts at the start of it.
// ---------------------------------------------------------------------

/// What the run's state is, by the rule, as the pulses are worked.
struct State {
    ready: [u32; 5],
    fatigue: [Fixed; 5],
    stood: [bool; 5],
    supply: i64,
    /// The latest `AirGroupStoodDown` of each group.
    stood_event: [Option<EventId>; 5],
}

impl State {
    /// What a group can fly now: all its ready aircraft without the figures,
    /// none when stood down, else fly(R, f).
    fn able(&self, config: &Config, g: u32) -> u32 {
        let i = of(g);
        if config.figures.is_none() {
            self.ready[i]
        } else if self.stood[i] {
            0
        } else {
            fly(self.ready[i], self.fatigue[i])
        }
    }
}

/// One day of a run, by the rule.
#[derive(Clone, Default)]
struct Day {
    /// After the night: each group's fatigue, whether it is stood down, and
    /// whether it was returned that night.
    start_fatigue: [Fixed; 5],
    stood: [bool; 5],
    returned: [bool; 5],
    /// What each group can fly in the morning and in the afternoon, by
    /// fly(R, f) (all its ready aircraft without the figures).
    able_morning: [u32; 5],
    able_afternoon: [u32; 5],
    /// The raid: bombers launched (0 if none), escorts sent, group 3's
    /// patrol, group 2's kept patrol in the morning.
    bombers: u32,
    escorts: u32,
    patrol: u32,
    kept: u32,
    /// Scouts searching in each pulse.
    search_morning: u32,
    search_afternoon: u32,
    /// Fatigue after each pulse.
    after_morning: [Fixed; 5],
    after_afternoon: [Fixed; 5],
    /// Bombers that attacked, aircraft destroyed on the ground.
    attacking: u32,
    destroyed_on_ground: u32,
    /// The ready aircraft of each group at the start of the morning.
    ready_morning: [u32; 5],
    /// Group 5's ready aircraft at the start of the afternoon.
    scouts_afternoon: u32,
}

/// What working one run found.
#[derive(Default)]
struct Worked {
    days: Vec<Day>,
    ready: [u32; 5],
    shot_down: [u32; 5],
    on_the_ground: [u32; 5],
    supply_destroyed: i64,
}

impl Worked {
    /// "A group that is not stood down flies as long as fly(R, f) is at
    /// least 1": the conditions of the hand calculation's day-by-day tables.
    fn bombers_fly(&self) -> bool {
        self.days
            .iter()
            .all(|day| day.stood[of(BOMBERS)] || day.able_morning[of(BOMBERS)] >= 1)
    }

    /// For group 2: when it is not stood down it flies an aircraft in each
    /// pulse. The hand calculation says "as long as its patrol or its escorts
    /// are" at least one; in the afternoon no raid flies, so only the patrol
    /// is, and with fly(R, f) = 1 the one aircraft is an escort that does not
    /// fly. So the condition is that what it flies is at least 1.
    fn escorts_fly(&self) -> bool {
        self.days.iter().all(|day| {
            day.stood[of(ESCORTS)]
                || (day.kept + day.escorts >= 1 && day.able_afternoon[of(ESCORTS)] / 2 >= 1)
        })
    }

    fn defenders_fly(&self) -> bool {
        self.days.iter().all(|day| {
            day.able_morning[of(DEFENDERS)] >= 1 && day.able_afternoon[of(DEFENDERS)] >= 1
        })
    }

    fn scouts_fly(&self) -> bool {
        self.days.iter().all(|day| {
            day.stood[of(SCOUTS)]
                || (day.able_morning[of(SCOUTS)] >= 1 && day.able_afternoon[of(SCOUTS)] >= 1)
        })
    }

    /// The days on which a raid was launched.
    fn raid_days(&self) -> usize {
        self.days.iter().filter(|day| day.bombers > 0).count()
    }
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
        EventKind::AircraftDestroyedOnGround { .. } => 7,
        EventKind::BaseSupplyDestroyed { .. } => 8,
        EventKind::AirGroupFatigue { .. } => 9,
        other => panic!("an event the day does not have: {other:?}"),
    }
}

/// A sortie as the fatigue step takes it: what kind, how far (nm), and the
/// causes that say it flew.
struct Flew {
    sortie: SortieKind,
    distance_nm: i64,
    causes: Vec<Cause>,
}

/// Works every pulse of `run` by the rule of the hand calculation from the
/// counts and fatigue at the start of it, asserts every event against it,
/// and returns what was worked. The counts are carried from pulse to pulse by
/// the rule, not read from the events.
fn work_the_run(run: &Run, config: &Config) -> Worked {
    let seed = run.seed;
    let mut state = State {
        ready: START,
        fatigue: config.start_fatigue,
        stood: [false; 5],
        supply: 1000,
        stood_event: [None; 5],
    };
    let mut worked = Worked::default();
    for day in 1..=DAYS {
        let mut record = Day::default();
        work_the_night(run, config, day, &mut state, &mut record);
        record.start_fatigue = state.fatigue;
        record.stood = state.stood;
        for pulse in [Pulse::Morning, Pulse::Afternoon] {
            work_the_pulse(
                run,
                config,
                day,
                pulse,
                &mut state,
                &mut worked,
                &mut record,
            );
        }
        worked.days.push(record);
    }

    // The end of the run.
    assert_eq!(run.ready, state.ready, "seed {seed}: ready at the end");
    assert_eq!(
        run.fatigue, state.fatigue,
        "seed {seed}: fatigue at the end"
    );
    assert_eq!(
        run.stood_down, state.stood,
        "seed {seed}: who is stood down at the end"
    );
    assert_eq!(
        run.supply_b,
        Fixed::from_int(state.supply),
        "seed {seed}: the supply at the end"
    );
    worked.ready = state.ready;
    worked.supply_destroyed = 1000 - state.supply;
    worked
}

/// Night, at the start of the day: the staff stands down each group with a
/// threshold whose fatigue is above it, and returns each stood-down group
/// whose fatigue is not.
fn work_the_night(run: &Run, config: &Config, day: u32, state: &mut State, record: &mut Day) {
    let seed = run.seed;
    let night = run.pulse(day, Pulse::Night);
    // (group, stood down, fatigue, threshold)
    let mut expected: Vec<(u32, bool, Fixed, Fixed)> = Vec::new();
    if config.figures.is_some() {
        for g in GROUPS {
            let Some(threshold) = config.thresholds[of(g)] else {
                continue;
            };
            let above = state.fatigue[of(g)] > threshold;
            if above != state.stood[of(g)] {
                state.stood[of(g)] = above;
                expected.push((g, above, state.fatigue[of(g)], threshold));
                record.returned[of(g)] = !above;
            }
        }
    }
    assert_eq!(
        night.len(),
        expected.len(),
        "seed {seed}, day {day}: the night has the stand-downs and returns and nothing else"
    );
    for (event, (g, above, fatigue, threshold)) in night.iter().zip(&expected) {
        let (kind, text) = if *above {
            (
                EventKind::AirGroupStoodDown {
                    air_group: group(*g),
                    fatigue: *fatigue,
                    threshold: *threshold,
                },
                format!("air_group:{g} stood down: fatigue {fatigue} is above its threshold of {threshold}"),
            )
        } else {
            (
                EventKind::AirGroupReturned {
                    air_group: group(*g),
                    fatigue: *fatigue,
                    threshold: *threshold,
                },
                format!("air_group:{g} returned to flying: fatigue {fatigue} is not above its threshold of {threshold}"),
            )
        };
        assert_eq!(event.kind, kind, "seed {seed}, day {day}");
        assert_eq!(event.kind.to_string(), text);
        assert_eq!(
            event.causes,
            vec![Cause::Order(run.threshold_order(*g))],
            "seed {seed}: caused by the threshold order"
        );
        let breakdown = event.breakdown.as_ref().expect("a breakdown");
        assert_eq!(breakdown.result, *fatigue);
        assert_eq!(
            breakdown
                .factors
                .iter()
                .map(|f| (f.name, f.value))
                .collect::<Vec<_>>(),
            vec![("fatigue", *fatigue), ("rest_threshold", *threshold)]
        );
        if *above {
            state.stood_event[of(*g)] = Some(event.id);
        }
    }
}

/// One daylight pulse by the rule. The bombers launch in the first pulse of
/// the day in which they can fly an aircraft, once a day (decision 7 of 0063):
/// the morning if they can fly in it, else the afternoon if they can then (a
/// group that could fly none in the morning, for its fatigue, may fly one when
/// the morning's rest has lowered it). The two pulses are worked alike.
fn work_the_pulse(
    run: &Run,
    config: &Config,
    day: u32,
    pulse: Pulse,
    state: &mut State,
    worked: &mut Worked,
    record: &mut Day,
) {
    let seed = run.seed;
    let morning = pulse == Pulse::Morning;
    let step = run.pulse(day, pulse);
    // The log of the morning is in the order of the module comment, and the
    // fatigue of each group comes last.
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
    let on_ground_events = kinds(|k| matches!(k, EventKind::AircraftDestroyedOnGround { .. }));
    let supply_events = kinds(|k| matches!(k, EventKind::BaseSupplyDestroyed { .. }));
    let fatigue_events = kinds(|k| matches!(k, EventKind::AirGroupFatigue { .. }));
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
            + fatigue_events.len(),
        "seed {seed}, day {day}"
    );

    if morning {
        record.ready_morning = state.ready;
    } else {
        record.scouts_afternoon = state.ready[of(SCOUTS)];
    }
    let able = GROUPS.map(|g| state.able(config, g));
    if morning {
        record.able_morning = able;
    } else {
        record.able_afternoon = able;
    }
    // What each group flies, by the rule: group 5 searches with all it can
    // fly; group 1 launches all it can fly, if that is at least one; group 2
    // keeps half of what it can fly, rounded down, and the rest escort the
    // raid if one launches; group 3 patrols with all it can fly.
    let s5 = able[of(SCOUTS)];
    let b = able[of(BOMBERS)];
    let big_f = able[of(ESCORTS)];
    let kept = big_f * 5 / 10;
    let p3 = able[of(DEFENDERS)];
    // A raid launches if the bombers can fly and have not launched today.
    let raid = b >= 1 && record.bombers == 0;
    let e = if raid { big_f - kept } else { 0 };
    for g in GROUPS {
        assert!(
            able[of(g)] <= state.ready[of(g)],
            "seed {seed}: no more than ready"
        );
        if state.stood[of(g)] && config.figures.is_some() {
            assert_eq!(
                able[of(g)],
                0,
                "seed {seed}: a stood-down group flies nothing"
            );
        }
    }
    if morning {
        record.bombers = b;
        record.escorts = e;
        record.patrol = p3;
        record.kept = kept;
        record.search_morning = s5;
    } else {
        record.search_afternoon = s5;
        if raid {
            record.bombers = b;
            record.escorts = e;
        }
    }

    // Group 5 searches.
    let search_event = if s5 > 0 {
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
                aircraft: s5,
            },
            "seed {seed}, day {day}"
        );
        Some(searches[0].id)
    } else {
        assert!(searches.is_empty(), "seed {seed}, day {day}");
        None
    };

    // The raid, as in scenario 6a, on those numbers.
    let mut launch_event = None;
    let mut sent_event = None;
    if raid {
        let figures = RaidInputs {
            bombers: b,
            escorts: e,
            patrol: p3,
            scouts_flying: s5,
        };
        let raid = work_the_raid(
            run,
            config,
            day,
            pulse,
            state,
            worked,
            record,
            figures,
            &launches,
            &sent,
            &fights,
            &guns,
            &lost,
            &attacked,
            &on_ground_events,
            &supply_events,
        );
        launch_event = Some(raid.0);
        sent_event = raid.1;
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

    // What flew in the pulse, by group.
    let mut flew: BTreeMap<u32, Vec<Flew>> = BTreeMap::new();
    if let Some(launch) = launch_event {
        flew.entry(BOMBERS).or_default().push(Flew {
            sortie: SortieKind::BaseStrike,
            distance_nm: 2 * config.raid_nm(),
            causes: vec![Cause::Event(launch)],
        });
    }
    if kept > 0 {
        flew.entry(ESCORTS).or_default().push(Flew {
            sortie: SortieKind::Patrol,
            distance_nm: 0,
            causes: run.patrol_orders(ESCORTS),
        });
    }
    if let Some(sent) = sent_event {
        flew.entry(ESCORTS).or_default().push(Flew {
            sortie: SortieKind::Escort,
            distance_nm: 2 * config.raid_nm(),
            causes: vec![Cause::Event(sent)],
        });
    }
    if p3 > 0 {
        flew.entry(DEFENDERS).or_default().push(Flew {
            sortie: SortieKind::Patrol,
            distance_nm: 0,
            causes: run.patrol_orders(DEFENDERS),
        });
    }
    if let Some(search) = search_event {
        flew.entry(SCOUTS).or_default().push(Flew {
            sortie: SortieKind::Search,
            distance_nm: 2 * SEARCH_NM,
            causes: vec![Cause::Event(search)],
        });
    }
    work_the_fatigue(run, config, day, pulse, &fatigue_events, &flew, state);
    if morning {
        record.after_morning = state.fatigue;
    } else {
        record.after_afternoon = state.fatigue;
    }
}

/// The fatigue of one daylight pulse, after the air combat: each group, in
/// order of group number, that flew an aircraft gains for its longest
/// sortie, once, 100 at most; any other loses the rest figure, 0 at least.
/// An event for each group whose fatigue changed, asserted against `events`.
fn work_the_fatigue(
    run: &Run,
    config: &Config,
    day: u32,
    pulse: Pulse,
    events: &[&Event],
    flew: &BTreeMap<u32, Vec<Flew>>,
    state: &mut State,
) {
    let seed = run.seed;
    let Some(figures) = config.figures else {
        assert!(
            events.is_empty(),
            "seed {seed}, day {day}, {pulse:?}: without the figures fatigue never changes"
        );
        return;
    };
    let mut expected: Vec<Expected> = Vec::new();
    for g in GROUPS {
        let before = state.fatigue[of(g)];
        let (change, after, causes, factors) = match flew.get(&g) {
            Some(sorties) => {
                // The longest sortie; the first of equal ones.
                let mut longest = &sorties[0];
                for sortie in sorties {
                    if sortie.distance_nm > longest.distance_nm {
                        longest = sortie;
                    }
                }
                let gain = figures.gain(longest.distance_nm);
                let after = (before + gain).min(n(100));
                let mut causes: Vec<Cause> = Vec::new();
                for sortie in sorties {
                    for cause in &sortie.causes {
                        if !causes.contains(cause) {
                            causes.push(*cause);
                        }
                    }
                }
                (
                    FatigueChange::Gained {
                        sortie: longest.sortie,
                        distance_nm: Fixed::from_int(longest.distance_nm),
                    },
                    after,
                    causes,
                    vec![
                        ("fatigue_before", before),
                        ("air_fatigue_per_sortie", figures.per_sortie),
                        ("air_fatigue_per_100_nm", figures.per_100),
                        ("distance_nm", Fixed::from_int(longest.distance_nm)),
                        ("gain", gain),
                    ],
                )
            }
            None => {
                let after = (before - figures.rest).max(Fixed::ZERO);
                let causes: Vec<Cause> = if state.stood[of(g)] {
                    state.stood_event[of(g)]
                        .map(Cause::Event)
                        .into_iter()
                        .collect()
                } else {
                    run.mission_order(g).map(Cause::Order).into_iter().collect()
                };
                (
                    FatigueChange::Rested,
                    after,
                    causes,
                    vec![
                        ("fatigue_before", before),
                        ("air_rest_per_pulse", figures.rest),
                    ],
                )
            }
        };
        if after != before {
            let amount = if after > before {
                after - before
            } else {
                before - after
            };
            expected.push((g, change, amount, after, causes, factors));
            state.fatigue[of(g)] = after;
        }
    }
    assert_eq!(
        events.len(),
        expected.len(),
        "seed {seed}, day {day}, {pulse:?}: one event for each group whose fatigue changed"
    );
    for (event, (g, change, amount, after, causes, factors)) in events.iter().zip(&expected) {
        assert_eq!(
            event.kind,
            EventKind::AirGroupFatigue {
                air_group: group(*g),
                change: *change,
                amount: *amount,
                fatigue: *after,
            },
            "seed {seed}, day {day}, {pulse:?}, group {g}"
        );
        let text = match change {
            FatigueChange::Gained {
                sortie,
                distance_nm,
            } => format!(
                "air_group:{g} gained {amount} fatigue from its {} sortie of {distance_nm} nm, now {after}",
                sortie.name()
            ),
            FatigueChange::Rested => format!("air_group:{g} rested {amount} fatigue, now {after}"),
        };
        assert_eq!(event.kind.to_string(), text);
        assert_eq!(&event.causes, causes, "seed {seed}, day {day}, group {g}");
        let breakdown = event.breakdown.as_ref().expect("a breakdown");
        assert_eq!(breakdown.result, *after);
        assert_eq!(
            breakdown
                .factors
                .iter()
                .map(|f| (f.name, f.value))
                .collect::<Vec<_>>(),
            *factors
        );
        assert!(
            *after >= Fixed::ZERO && *after <= n(100),
            "seed {seed}: fatigue stays within 0 to 100"
        );
    }
}

/// One expected fatigue event: group, change, amount, fatigue after, causes
/// and the factors of the breakdown.
type Expected = (
    u32,
    FatigueChange,
    Fixed,
    Fixed,
    Vec<Cause>,
    Vec<(&'static str, Fixed)>,
);

/// What the raid is worked from.
#[derive(Clone, Copy)]
struct RaidInputs {
    bombers: u32,
    escorts: u32,
    /// Group 3's patrol.
    patrol: u32,
    /// The scouts that are searching, so in the air.
    scouts_flying: u32,
}

/// A pulse's raid worked as in scenario 6a on `inputs`. The aircraft on
/// the ground at airfield B are every ready aircraft there that is not
/// flying (decision 11 of 0036): the fighters of group 3 that are not on
/// patrol, all of group 4, and the scouts that are not searching. The raid is
/// alone, so it meets the whole patrol. Returns the launch event and the
/// event that sent the escorts, if any.
#[allow(clippy::too_many_arguments)]
fn work_the_raid(
    run: &Run,
    config: &Config,
    day: u32,
    pulse: Pulse,
    state: &mut State,
    worked: &mut Worked,
    record: &mut Day,
    inputs: RaidInputs,
    launches: &[&Event],
    sent: &[&Event],
    fights: &[&Event],
    guns: &[&Event],
    lost: &[&Event],
    attacked: &[&Event],
    on_ground_events: &[&Event],
    supply_events: &[&Event],
) -> (EventId, Option<EventId>) {
    let seed = run.seed;
    let RaidInputs {
        bombers: b,
        escorts: e,
        patrol: p,
        scouts_flying,
    } = inputs;
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
        if on_ground > 0 {
            ground_pool.push((g, on_ground));
        }
    }
    let on_ground: u32 = ground_pool.iter().map(|(_, count)| count).sum();

    let mut escorts_sent = None;
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
        escorts_sent = Some(sent[0].id);
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
        for (g, count) in taken {
            now[of(g)] -= count;
            expected_losses.push((g, count, now[of(g)], event.id));
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
                hits,
                after_attack: false,
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

    // The aircraft lost in the air, one event for each group and roll.
    assert_eq!(lost.len(), expected_losses.len(), "seed {seed}, day {day}");
    for (event, (g, count, remaining, cause)) in lost.iter().zip(&expected_losses) {
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
        worked.shot_down[of(*g)] += count;
    }
    let bombers_left = bomber_pool[0].1;
    state.ready = now;

    // The bombers left each drop one bomb, whatever the type carries.
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
                .zip(lost)
                .filter(|((g, ..), _)| *g == BOMBERS)
                .map(|(_, event)| Cause::Event(event.id)),
        );
        assert_eq!(attacked[0].causes, causes, "seed {seed}");

        // Hits on aircraft destroy aircraft on the ground, the lowest group
        // first, and no more than were there at the start of the pulse.
        let taken = take(on_aircraft, &mut ground_pool);
        assert_eq!(
            on_ground_events.len(),
            taken.len(),
            "seed {seed}, day {day}"
        );
        for (event, (g, count)) in on_ground_events.iter().zip(&taken) {
            destroyed_on_ground += count;
            worked.on_the_ground[of(*g)] += count;
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
                event.kind.to_string(),
                format!(
                    "air_group:{g} lost {count} aircraft on the ground at base:2, {} left",
                    state.ready[of(*g)]
                )
            );
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
    record.attacking = bombers_left;
    record.destroyed_on_ground = destroyed_on_ground;
    (launch.id, escorts_sent)
}

// ---------------------------------------------------------------------
// Variants: the scenario with one thing changed, written under the build
// directory. Nothing in the repository's data folder is touched.
// ---------------------------------------------------------------------

/// Loads the scenario on the frozen figures with each exact, asserted-unique
/// text replacement made in its text and in a copy of the fatigue figures.
fn variant(tag: &str, scenario_edits: &[(&str, &str)], fatigue_edits: &[(&str, &str)]) -> Scenario {
    let path = scenario_path();
    let mut text = fs::read_to_string(&path).expect("read the scenario");
    for (original, changed) in scenario_edits {
        text = common::replace_once(&text, original, changed);
    }
    let options = common::altered_reference(tag, |dir| {
        common::edit_file_by(dir, "fatigue.toml", fatigue_edits);
    });
    common::from_text_with(&text, &path, &options).expect("the variant loads")
}

const FATIGUE_KEY: &str = "fatigue = \"../reference/fatigue.toml\"\n";
const THRESHOLD_1: &str = "[[order]]\nday = 1\nside = \"japanese\"\nair_group = 1\nkind = \"set_rest_threshold\"\nfatigue = 30";
const THRESHOLD_2: &str = "[[order]]\nday = 1\nside = \"japanese\"\nair_group = 2\nkind = \"set_rest_threshold\"\nfatigue = 40";
const THRESHOLD_5: &str = "[[order]]\nday = 1\nside = \"allied\"\nair_group = 5\nkind = \"set_rest_threshold\"\nfatigue = 50";
const THRESHOLD_1_VALUE: &str = "kind = \"set_rest_threshold\"\nfatigue = 30";

fn without_fatigue() -> Scenario {
    variant("no_file", &[(FATIGUE_KEY, "")], &[])
}

fn no_thresholds() -> Scenario {
    variant(
        "no_thresholds",
        &[(THRESHOLD_1, ""), (THRESHOLD_2, ""), (THRESHOLD_5, "")],
        &[],
    )
}

fn bombers_threshold(level: i64) -> Scenario {
    variant(
        &format!("threshold_{level}"),
        &[(
            THRESHOLD_1_VALUE,
            &format!("kind = \"set_rest_threshold\"\nfatigue = {level}"),
        )],
        &[],
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

fn defenders_start_at(fatigue: i64) -> Scenario {
    variant(
        &format!("defenders_{fatigue}"),
        &[(
            "name = \"Placeholder defending fighters\"",
            &format!("name = \"Placeholder defending fighters\"\nfatigue = {fatigue}"),
        )],
        &[],
    )
}

fn scouts_start_at(fatigue: i64) -> Scenario {
    variant(
        &format!("scouts_{fatigue}"),
        &[(
            "name = \"Placeholder scouts\"",
            &format!("name = \"Placeholder scouts\"\nfatigue = {fatigue}"),
        )],
        &[],
    )
}

fn config_without_figures() -> Config {
    Config {
        figures: None,
        ..scenario_config()
    }
}

fn config_without_thresholds() -> Config {
    Config {
        thresholds: [None; 5],
        ..scenario_config()
    }
}

/// Every pulse of the runs of a variant worked by `config`.
fn work_all(runs: &[Run], config: &Config) -> Vec<Worked> {
    runs.iter().map(|run| work_the_run(run, config)).collect()
}

/// The aircraft of each group are conserved in one run, from its events
/// alone: ready at the end, shot down and destroyed on the ground add up to
/// the aircraft it started with.
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
                } if air_group == group(g) => gone += aircraft,
                _ => {}
            }
        }
        assert_eq!(
            run.ready[of(g)] + gone,
            START[of(g)],
            "seed {}, group {g}: the aircraft are conserved",
            run.seed
        );
    }
}

/// Group 1's table, group 2's, group 3's, and group 5's of the hand
/// calculation: for each day (start of the day, stood down after the night,
/// returned that night, after the morning, after the afternoon).
const GROUP_1_TABLE: [(i64, bool, bool, i64, i64); 8] = [
    (0, false, false, 11, 6),
    (6, false, false, 17, 12),
    (12, false, false, 23, 18),
    (18, false, false, 29, 24),
    (24, false, false, 35, 30),
    (30, false, false, 41, 36),
    (36, true, false, 31, 26),
    (26, false, true, 37, 32),
];
const GROUP_2_TABLE: [(i64, bool, bool, i64, i64); 8] = [
    (0, false, false, 11, 14),
    (14, false, false, 25, 28),
    (28, false, false, 39, 42),
    (42, true, false, 37, 32),
    (32, false, true, 43, 46),
    (46, true, false, 41, 36),
    (36, false, true, 39, 42),
    (42, true, false, 37, 32),
];
/// Group 5: start, stood down, returned, searches in the morning, after the
/// morning, searches in the afternoon, after the afternoon.
const GROUP_5_TABLE: [(i64, bool, bool, u32, i64, u32, i64); 8] = [
    (0, false, false, 6, 15, 5, 30),
    (30, false, false, 4, 45, 3, 60),
    (60, true, false, 0, 55, 0, 50),
    (50, false, true, 3, 65, 2, 80),
    (80, true, false, 0, 75, 0, 70),
    (70, true, false, 0, 65, 0, 60),
    (60, true, false, 0, 55, 0, 50),
    (50, false, true, 3, 65, 2, 80),
];

fn assert_group_1_table(worked: &Worked, seed: u64) {
    for (index, (start, stood, returned, morning, afternoon)) in GROUP_1_TABLE.iter().enumerate() {
        let day = &worked.days[index];
        let i = of(BOMBERS);
        assert_eq!(
            day.start_fatigue[i],
            n(*start),
            "seed {seed}, day {}",
            index + 1
        );
        assert_eq!((day.stood[i], day.returned[i]), (*stood, *returned));
        assert_eq!(
            day.after_morning[i],
            n(*morning),
            "seed {seed}, day {}",
            index + 1
        );
        assert_eq!(
            day.after_afternoon[i],
            n(*afternoon),
            "seed {seed}, day {}",
            index + 1
        );
        // A raid on every morning but the seventh, of fly(R, f) bombers.
        assert_eq!(day.bombers > 0, !stood, "seed {seed}, day {}", index + 1);
        assert_eq!(
            day.bombers,
            fly(day.ready_morning[i], n(*start)) * u32::from(!stood),
            "seed {seed}"
        );
    }
}

fn assert_group_2_table(worked: &Worked, seed: u64) {
    for (index, (start, stood, returned, morning, afternoon)) in GROUP_2_TABLE.iter().enumerate() {
        let day = &worked.days[index];
        let i = of(ESCORTS);
        assert_eq!(
            day.start_fatigue[i],
            n(*start),
            "seed {seed}, day {}",
            index + 1
        );
        assert_eq!((day.stood[i], day.returned[i]), (*stood, *returned));
        assert_eq!(
            day.after_morning[i],
            n(*morning),
            "seed {seed}, day {}",
            index + 1
        );
        assert_eq!(
            day.after_afternoon[i],
            n(*afternoon),
            "seed {seed}, day {}",
            index + 1
        );
    }
}

fn assert_group_3_table(worked: &Worked, seed: u64) {
    for (index, day) in worked.days.iter().enumerate() {
        let i = of(DEFENDERS);
        let d = i64::try_from(index).expect("small");
        assert_eq!(day.start_fatigue[i], n(6 * d), "seed {seed}, day {}", d + 1);
        assert_eq!(day.after_morning[i], n(6 * d + 3));
        assert_eq!(day.after_afternoon[i], n(6 * d + 6));
        assert!(!day.stood[i] && !day.returned[i]);
        // The patrol is R x (100 - 6 x (d - 1)) / 100, rounded down.
        assert_eq!(day.patrol, fly(day.ready_morning[i], n(6 * d)));
        assert!(day.patrol <= 12 * u32::try_from(100 - 6 * d).expect("positive") / 100);
    }
    assert_eq!(worked.days[7].after_afternoon[of(DEFENDERS)], n(48));
}

fn assert_group_5_table(worked: &Worked, seed: u64) {
    for (index, (start, stood, returned, am, after_am, pm, after_pm)) in
        GROUP_5_TABLE.iter().enumerate()
    {
        let day = &worked.days[index];
        let i = of(SCOUTS);
        assert_eq!(
            day.start_fatigue[i],
            n(*start),
            "seed {seed}, day {}",
            index + 1
        );
        assert_eq!((day.stood[i], day.returned[i]), (*stood, *returned));
        assert_eq!(
            day.after_morning[i],
            n(*after_am),
            "seed {seed}, day {}",
            index + 1
        );
        assert_eq!(
            day.after_afternoon[i],
            n(*after_pm),
            "seed {seed}, day {}",
            index + 1
        );
        // The searches in the table are for a group still at 6.
        if day.ready_morning[i] == 6 {
            assert_eq!(day.search_morning, *am, "seed {seed}, day {}", index + 1);
        }
        if day.scouts_afternoon == 6 {
            assert_eq!(day.search_afternoon, *pm, "seed {seed}, day {}", index + 1);
        }
    }
}

// ---------------------------------------------------------------------
// The setting and the figures.
// ---------------------------------------------------------------------

#[test]
fn the_setting_is_as_the_hand_calculation_says() {
    let scenario = load();
    assert_eq!(scenario.days, 8);
    let truth = scenario.world.umpire();
    let (a, b) = (&truth.bases[&AIRFIELD_A], &truth.bases[&AIRFIELD_B]);
    assert_eq!((a.side, a.hex), (Side::Japanese, Hex::new(0, 0)));
    assert_eq!((b.side, b.hex), (Side::Allied, Hex::new(5, 0)));
    assert_eq!(a.hex.distance(b.hex), 5);
    assert_eq!(b.supply_t, Fixed::from_int(1000));
    assert_eq!((a.anti_aircraft, b.anti_aircraft), (0, 20));
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
            "group {id}"
        );
        assert_eq!(
            g.fatigue,
            Fixed::ZERO,
            "every group starts with a fatigue of 0"
        );
        assert!(!g.stood_down);
    }
    // There is no wear: the upkeep file is not named.
    assert!(scenario.world.ref_data().air_upkeep.is_none());
    let figures = scenario
        .world
        .ref_data()
        .fatigue
        .expect("the fatigue figures are loaded");
    assert_eq!(figures.air_fatigue_per_sortie, n(3));
    assert_eq!(figures.air_fatigue_per_100_nm, n(2));
    assert_eq!(figures.air_rest_per_pulse, n(5));
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
    for (id, threshold) in [
        (BOMBERS, Some(n(30))),
        (ESCORTS, Some(n(40))),
        (DEFENDERS, None),
        (RESTING, None),
        (SCOUTS, Some(n(50))),
    ] {
        assert_eq!(
            truth.air_groups[&group(id)].rest_above,
            threshold,
            "group {id}"
        );
    }
}

#[test]
fn the_figures_table_and_fly_are_as_the_hand_calculation_says() {
    let scenario = load();
    let figures = scenario
        .world
        .ref_data()
        .fatigue
        .expect("the fatigue figures are loaded");
    let mine = scenario_config().figures.expect("figures");
    // A patrol 3; the raid, 2 x 200 = 400 nm, 11; a search out to 300 nm,
    // 600 nm, 15; the 160 nm of the near variant, 6.2.
    for (distance, gain) in [
        (0, n(3)),
        (160, "6.2".parse::<Fixed>().expect("fixed")),
        (400, n(11)),
        (600, n(15)),
    ] {
        assert_eq!(mine.gain(distance), gain, "{distance} nm");
        assert_eq!(
            figures.gain(Fixed::from_int(distance)),
            gain,
            "{distance} nm"
        );
    }
    // fly(R, f): R x (100 - f) / 100, rounded down.
    assert_eq!(fly(18, n(0)), 18);
    assert_eq!(fly(18, n(6)), 16);
    assert_eq!(fly(6, n(15)), 5);
    assert_eq!(fly(6, n(30)), 4);
    assert_eq!(fly(6, n(45)), 3);
    assert_eq!(fly(6, n(50)), 3);
    assert_eq!(fly(6, n(65)), 2);
    assert_eq!(fly(6, n(1)), 5, "6 scouts at fatigue 1 fly 5");
    assert_eq!(fly(12, n(42)), 6, "at most 12 x 0.58 = 6 on day 8");
    assert_eq!(fly(6, n(90)), 0);
    assert_eq!(fly(6, n(100)), 0);
}

// ---------------------------------------------------------------------
// The days.
// ---------------------------------------------------------------------

#[test]
fn every_pulse_of_every_run_is_worked_by_the_rule_from_the_counts_and_fatigue_at_the_start_of_it() {
    // `worked_batch` asserts every stand-down, return, search, raid and
    // fatigue event of every pulse of every run, and the state at the end of
    // each run.
    let worked = worked_batch();
    assert_eq!(worked.len(), 1000);
    for (run, worked) in batch().iter().zip(worked) {
        assert_eq!(worked.days.len(), 8, "seed {}", run.seed);
        // The first raid always flies.
        assert!(worked.days[0].bombers > 0, "seed {}", run.seed);
    }
}

#[test]
fn the_day_runs_stand_downs_at_night_then_search_raid_and_fatigue_in_the_morning_then_search_and_fatigue(
) {
    for run in batch() {
        for day in 1..=DAYS {
            for event in run.pulse(day, Pulse::Night) {
                assert!(
                    matches!(
                        event.kind,
                        EventKind::AirGroupStoodDown { .. } | EventKind::AirGroupReturned { .. }
                    ),
                    "seed {}: night has only stand-downs and returns: {:?}",
                    run.seed,
                    event.kind
                );
            }
            // In a daylight pulse the fatigue events come last, and nobody
            // is stood down or returned.
            for pulse in [Pulse::Morning, Pulse::Afternoon] {
                let step = run.pulse(day, pulse);
                let first = step
                    .iter()
                    .position(|e| matches!(e.kind, EventKind::AirGroupFatigue { .. }))
                    .unwrap_or(step.len());
                for (index, event) in step.iter().enumerate() {
                    assert!(!matches!(
                        event.kind,
                        EventKind::AirGroupStoodDown { .. } | EventKind::AirGroupReturned { .. }
                    ));
                    assert_eq!(
                        matches!(event.kind, EventKind::AirGroupFatigue { .. }),
                        index >= first,
                        "seed {}: fatigue comes after the air combat",
                        run.seed
                    );
                }
            }
            // The raid launches once a day (decision 7 of 0063): the
            // afternoon has one only on a day whose morning had none.
            let raids_in = |pulse| {
                run.pulse(day, pulse)
                    .iter()
                    .filter(|e| matches!(e.kind, EventKind::BaseStrikeLaunched { .. }))
                    .count()
            };
            assert!(
                raids_in(Pulse::Morning) + raids_in(Pulse::Afternoon) <= 1,
                "seed {}, day {day}: one raid a day at most",
                run.seed
            );
        }
        // On day 1 nobody is stood down.
        assert!(run.pulse(1, Pulse::Night).is_empty(), "seed {}", run.seed);
    }
}

#[test]
fn the_tables_of_the_four_groups_hold_in_the_runs_that_meet_their_conditions() {
    let worked = worked_batch();
    // Group 1: not stood down, it flies at least one aircraft each day.
    // Group 2's table presumes group 1's raids, so it needs both.
    // Group 3: it patrols at least one aircraft in every pulse. Group 5: not
    // stood down, it searches with at least one in both pulses.
    let (mut ok_1, mut ok_2, mut ok_3, mut ok_5) = (0, 0, 0, 0);
    for (run, worked) in batch().iter().zip(worked) {
        let seed = run.seed;
        if worked.bombers_fly() {
            ok_1 += 1;
            assert_group_1_table(worked, seed);
            if worked.escorts_fly() {
                ok_2 += 1;
                assert_group_2_table(worked, seed);
                // The raids of days 4, 6 and 8 have no escort, and the
                // whole of group 3's patrol fires at the bombers.
                for day in [4usize, 6, 8] {
                    let record = &worked.days[day - 1];
                    assert!(
                        record.bombers > 0 && record.escorts == 0,
                        "seed {seed}, day {day}"
                    );
                    let events = run.pulse(u32::try_from(day).expect("small"), Pulse::Morning);
                    assert!(
                        !events
                            .iter()
                            .any(|e| matches!(e.kind, EventKind::EscortsSent { .. })),
                        "seed {seed}, day {day}: no escorts"
                    );
                    let fights: Vec<_> = events
                        .iter()
                        .filter_map(|e| match e.kind {
                            EventKind::AirFight {
                                firing, at, shots, ..
                            } => Some((firing, at, shots)),
                            _ => None,
                        })
                        .collect();
                    if record.patrol > 0 {
                        assert_eq!(
                            fights,
                            vec![(AirFirer::Patrol, AirTarget::Bombers, int(record.patrol))],
                            "seed {seed}, day {day}: all the patrol fires at the bombers"
                        );
                    } else {
                        assert!(fights.is_empty());
                    }
                }
                // The raids of days 1, 2, 3 and 5 are escorted, and no raid
                // flies on day 7.
                for day in [1usize, 2, 3, 5] {
                    assert!(worked.days[day - 1].escorts > 0, "seed {seed}, day {day}");
                }
                assert_eq!(worked.days[6].bombers, 0, "seed {seed}: no raid on day 7");
                assert!(worked.days[6].kept > 0);
            }
        }
        if worked.defenders_fly() {
            ok_3 += 1;
            assert_group_3_table(worked, seed);
        }
        if worked.scouts_fly() {
            ok_5 += 1;
            assert_group_5_table(worked, seed);
        }
        // Group 4 never flies and its fatigue is 0 throughout.
        for day in &worked.days {
            assert_eq!(day.start_fatigue[of(RESTING)], Fixed::ZERO);
            assert_eq!(day.after_morning[of(RESTING)], Fixed::ZERO);
            assert_eq!(day.after_afternoon[of(RESTING)], Fixed::ZERO);
            assert_eq!(
                day.able_morning[of(RESTING)],
                day.ready_morning[of(RESTING)]
            );
        }
    }
    eprintln!(
        "runs meeting each table's condition, of 1,000: group 1 {ok_1}, group 2 {ok_2}, group 3 {ok_3}, group 5 {ok_5}"
    );
    // Most runs meet each condition, so each table is a statement about
    // most of the runs.
    for (name, ok) in [("1", ok_1), ("2", ok_2), ("3", ok_3), ("5", ok_5)] {
        assert!(ok > 500, "group {name}: only {ok} runs meet the condition");
    }
}

#[test]
fn the_first_two_mornings_are_as_the_hand_calculation_says() {
    let worked = worked_batch();
    for (run, worked) in batch().iter().zip(worked) {
        let seed = run.seed;
        // Day 1: every fatigue is 0, so 18 bombers, 6 escorts and a patrol of
        // 12 fly, and group 4's ten and nothing else are on the ground.
        let first = &worked.days[0];
        assert_eq!(first.start_fatigue, [Fixed::ZERO; 5]);
        assert_eq!(
            (
                first.bombers,
                first.escorts,
                first.patrol,
                first.kept,
                first.search_morning
            ),
            (18, 6, 12, 6, 6),
            "seed {seed}"
        );
        assert_eq!(first.destroyed_on_ground, {
            // Hits on aircraft that fall on group 4's ten.
            let attacked = run.pulse(1, Pulse::Morning);
            attacked
                .iter()
                .filter_map(|e| match e.kind {
                    EventKind::BaseAttacked {
                        hits_on_aircraft, ..
                    } => Some(hits_on_aircraft.min(10)),
                    _ => None,
                })
                .sum::<u32>()
        });
        // Day 2: the fatigues are 6, 14, 6, 0 and 30.
        let second = &worked.days[1];
        assert_eq!(
            second.start_fatigue,
            [n(6), n(14), n(6), Fixed::ZERO, n(30)],
            "seed {seed}"
        );
        let [r1, r2, r3, _, r5] = second.ready_morning;
        assert_eq!(second.bombers, fly(r1, n(6)), "seed {seed}");
        let can = fly(r2, n(14));
        assert_eq!((second.kept, second.escorts), (can / 2, can - can / 2));
        assert_eq!(second.patrol, fly(r3, n(6)), "seed {seed}");
        // R3 less that are on the ground: the ground at airfield B holds the
        // fighters left behind, and group 4's ten.
        // Four scouts search and two are on the ground.
        assert_eq!((r5, second.search_morning), (6, 4), "seed {seed}");
        let events = run.pulse(2, Pulse::Morning);
        let search = events
            .iter()
            .find_map(|e| match e.kind {
                EventKind::SearchFlown { aircraft, .. } => Some(aircraft),
                _ => None,
            })
            .expect("a search");
        assert_eq!(search, 4);
        let launched = events
            .iter()
            .find_map(|e| match e.kind {
                EventKind::BaseStrikeLaunched { aircraft, .. } => Some(aircraft),
                _ => None,
            })
            .expect("a raid");
        assert_eq!(launched, second.bombers);
    }
}

// ---------------------------------------------------------------------
// The variants.
// ---------------------------------------------------------------------

#[test]
fn a_scenario_with_no_fatigue_file_has_no_fatigue_no_stand_down_and_is_scenario_6a_for_three_days()
{
    let variant = without_fatigue();
    assert!(variant.world.ref_data().fatigue.is_none());
    let reference_scenario = six_a();
    let runs = batch_of(&variant, VARIANT_SEEDS);
    work_all(&runs, &config_without_figures());
    for run in &runs {
        let seed = run.seed;
        assert!(
            run.of(|k| matches!(
                k,
                EventKind::AirGroupFatigue { .. }
                    | EventKind::AirGroupStoodDown { .. }
                    | EventKind::AirGroupReturned { .. }
            ))
            .is_empty(),
            "seed {seed}"
        );
        assert_eq!(run.fatigue, [Fixed::ZERO; 5]);
        assert_eq!(run.stood_down, [false; 5]);
        // The rest orders are accepted (and do nothing): three more
        // acceptances than scenario 6a's six, all in the first day.
        let accepted = run.of(|k| matches!(k, EventKind::OrderAccepted { .. }));
        assert_eq!(accepted.len(), 9, "seed {seed}");
        let reference = run_of(&reference_scenario, seed);
        // Every event of the first three days is scenario 6a's: the same
        // kinds and, once the nine orders' numbers and the events they
        // number are mapped to 6a's six, the same causes.
        let order_map = |order: OrderId| -> OrderId {
            OrderId::for_umpire(
                match order.umpire_raw(common::umpire_key()) {
                    1..=3 => order.umpire_raw(common::umpire_key()),
                    5 => 4,
                    7 => 5,
                    8 => 6,
                    other => panic!("an order of 6a: {other}"),
                },
                common::umpire_key(),
            )
        };
        let map_cause = |cause: &Cause| match cause {
            Cause::Order(order) => Cause::Order(order_map(*order)),
            Cause::Event(event) => Cause::Event(EventId::for_umpire(
                event.umpire_raw(common::umpire_key()) - 3,
                common::umpire_key(),
            )),
        };
        let first_three: Vec<&Event> = run
            .events
            .iter()
            .filter(|e| e.day <= 3 && !matches!(e.kind, EventKind::OrderAccepted { .. }))
            .collect();
        let reference_events: Vec<&Event> = reference
            .events
            .iter()
            .filter(|e| !matches!(e.kind, EventKind::OrderAccepted { .. }))
            .collect();
        assert_eq!(first_three.len(), reference_events.len(), "seed {seed}");
        for (a, b) in first_three.iter().zip(&reference_events) {
            assert_eq!(
                (a.day, a.pulse, &a.kind, a.breakdown.clone()),
                (b.day, b.pulse, &b.kind, b.breakdown.clone()),
                "seed {seed}"
            );
            assert_eq!(
                a.causes.iter().map(map_cause).collect::<Vec<_>>(),
                b.causes,
                "seed {seed}: {:?}",
                a.kind
            );
            assert_eq!(
                a.id.umpire_raw(common::umpire_key()) - 3,
                b.id.umpire_raw(common::umpire_key()),
                "seed {seed}"
            );
        }
        // The hashes differ on purpose: the thresholds are in the state.
        assert_ne!(run.hashes[..3], reference.hashes[..], "seed {seed}");
    }
}

#[test]
fn with_no_threshold_nobody_is_stood_down_and_group_5_tires_and_rests_itself_badly() {
    let runs = batch_of(&no_thresholds(), VARIANT_SEEDS);
    let worked = work_all(&runs, &config_without_thresholds());
    // Group 5: 6, 5, 4, 3, 2, 1 scouts searching in the first six pulses and
    // 30, 60 and 90 at the end of days 1, 2 and 3; then none and rest: 85,
    // 80; one scout flies at 80 and it gains 15: 95; then none, 90, and so
    // on. For a group that still has its six.
    let searching: [u32; 16] = [6, 5, 4, 3, 2, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0];
    let after: [i64; 16] = [
        15, 30, 45, 60, 75, 90, 85, 80, 95, 90, 85, 80, 95, 90, 85, 80,
    ];
    let mut whole = 0;
    for (run, worked) in runs.iter().zip(&worked) {
        assert!(run
            .of(|k| matches!(
                k,
                EventKind::AirGroupStoodDown { .. } | EventKind::AirGroupReturned { .. }
            ))
            .is_empty());
        assert!(worked.days.iter().all(|day| day.stood == [false; 5]));
        // Every pulse had its six, if none was destroyed on the ground.
        if worked.on_the_ground[of(SCOUTS)] == 0 {
            whole += 1;
            for (index, day) in worked.days.iter().enumerate() {
                assert_eq!(
                    day.search_morning,
                    searching[2 * index],
                    "seed {}",
                    run.seed
                );
                assert_eq!(day.search_afternoon, searching[2 * index + 1]);
                assert_eq!(day.after_morning[of(SCOUTS)], n(after[2 * index]));
                assert_eq!(day.after_afternoon[of(SCOUTS)], n(after[2 * index + 1]));
            }
            // Group 5's fatigue at the end of days 1, 2 and 3.
            assert_eq!(worked.days[0].after_afternoon[of(SCOUTS)], n(30));
            assert_eq!(worked.days[1].after_afternoon[of(SCOUTS)], n(60));
            assert_eq!(worked.days[2].after_afternoon[of(SCOUTS)], n(90));
        }
    }
    eprintln!("no thresholds: {whole} of {VARIANT_SEEDS} runs keep all six scouts");
    assert!(whole > usize::try_from(VARIANT_SEEDS).expect("small") / 2);
}

#[test]
fn with_a_threshold_of_0_for_group_1_it_flies_on_days_1_3_5_and_7() {
    let runs = batch_of(&bombers_threshold(0), VARIANT_SEEDS);
    let config = Config {
        thresholds: [Some(n(0)), Some(n(40)), None, None, Some(n(50))],
        ..scenario_config()
    };
    let worked = work_all(&runs, &config);
    let mut ok = 0;
    for (run, worked) in runs.iter().zip(&worked) {
        if !worked.bombers_fly() {
            continue;
        }
        ok += 1;
        // It flies on day 1 and ends it at 6; it is stood down on day 2 and
        // rests to 1 and then 0, which is not below 0; at 0 it is not above
        // its threshold, so it flies on day 3.
        for (index, day) in worked.days.iter().enumerate() {
            let flies = index % 2 == 0;
            let i = of(BOMBERS);
            assert_eq!(day.stood[i], !flies, "seed {}, day {}", run.seed, index + 1);
            assert_eq!(day.bombers > 0, flies);
            assert_eq!(day.start_fatigue[i], if flies { n(0) } else { n(6) });
            if flies {
                assert_eq!(
                    (day.after_morning[i], day.after_afternoon[i]),
                    (n(11), n(6))
                );
            } else {
                assert_eq!((day.after_morning[i], day.after_afternoon[i]), (n(1), n(0)));
            }
        }
        assert_eq!(worked.raid_days(), 4, "four raids in eight days");
    }
    eprintln!("threshold 0: {ok} of {VARIANT_SEEDS} runs keep bombers to fly");
    assert!(ok > 0);
}

#[test]
fn a_lower_threshold_never_has_the_bombers_fly_on_more_days() {
    // Over the eight days group 1 flies on 4 days with a threshold of 0, on
    // 7 with 30 and on 8 with none, in every run in which it still has an
    // aircraft to fly.
    let mut found = [0usize; 3];
    for (slot, (scenario, config, days)) in [
        (
            bombers_threshold(0),
            Config {
                thresholds: [Some(n(0)), Some(n(40)), None, None, Some(n(50))],
                ..scenario_config()
            },
            4usize,
        ),
        (load(), scenario_config(), 7),
        (
            variant("no_bomber_threshold", &[(THRESHOLD_1, "")], &[]),
            Config {
                thresholds: [None, Some(n(40)), None, None, Some(n(50))],
                ..scenario_config()
            },
            8,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let runs = batch_of(&scenario, VARIANT_SEEDS);
        for (run, worked) in runs.iter().zip(work_all(&runs, &config)) {
            let flies = worked.days.iter().all(|day| {
                if config.thresholds[of(BOMBERS)].is_none() {
                    day.able_morning[of(BOMBERS)] >= 1
                } else {
                    day.stood[of(BOMBERS)] || day.able_morning[of(BOMBERS)] >= 1
                }
            });
            if flies {
                found[slot] += 1;
                assert_eq!(worked.raid_days(), days, "seed {}", run.seed);
            }
        }
    }
    eprintln!("runs in which group 1 keeps an aircraft to fly, thresholds 0, 30, none: {found:?}");
    assert!(found.iter().all(|found| *found > 0));
}

#[test]
fn with_airfield_b_at_80_nm_the_raid_gains_6_2() {
    let runs = batch_of(&airfield_b_at_80_nm(), VARIANT_SEEDS);
    let config = Config {
        raid_hexes: 2,
        ..scenario_config()
    };
    work_all(&runs, &config);
    let sixty_two = Fixed::from_raw(6_200_000);
    for run in &runs {
        // Bombers and escorts gain 3 + 2 x 1.6 = 6.2 on day 1.
        let day_1 = run.pulse(1, Pulse::Morning);
        let gains: Vec<(u32, FatigueChange, Fixed)> = day_1
            .iter()
            .filter_map(|e| match e.kind {
                EventKind::AirGroupFatigue {
                    air_group,
                    change,
                    amount,
                    ..
                } if air_group.raw() <= 2 => Some((air_group.raw(), change, amount)),
                _ => None,
            })
            .collect();
        assert_eq!(
            gains,
            vec![
                (
                    1,
                    FatigueChange::Gained {
                        sortie: SortieKind::BaseStrike,
                        distance_nm: n(160)
                    },
                    sixty_two
                ),
                (
                    2,
                    FatigueChange::Gained {
                        sortie: SortieKind::Escort,
                        distance_nm: n(160)
                    },
                    sixty_two
                ),
            ],
            "seed {}",
            run.seed
        );
    }
}

#[test]
fn with_group_3_starting_at_100_it_flies_nothing_on_day_1_and_rests_to_95_and_90() {
    let runs = batch_of(&defenders_start_at(100), VARIANT_SEEDS);
    let config = Config {
        start_fatigue: [Fixed::ZERO, Fixed::ZERO, n(100), Fixed::ZERO, Fixed::ZERO],
        ..scenario_config()
    };
    let worked = work_all(&runs, &config);
    for (run, worked) in runs.iter().zip(&worked) {
        let first = &worked.days[0];
        // No patrol; all 12 on the ground; the escorts fire at nothing.
        assert_eq!(first.patrol, 0, "seed {}", run.seed);
        assert_eq!(first.escorts, 6);
        let morning = run.pulse(1, Pulse::Morning);
        assert!(
            !morning
                .iter()
                .any(|e| matches!(e.kind, EventKind::AirFight { .. })),
            "seed {}: nobody to fight",
            run.seed
        );
        // It rests to 95, and flies nothing that afternoon: 12 x 0.05 rounds
        // down to none; 90.
        assert_eq!(first.after_morning[of(DEFENDERS)], n(95));
        assert_eq!(first.after_afternoon[of(DEFENDERS)], n(90));
        // The ground holds group 3's 12 and group 4's 10: every hit on
        // aircraft destroys one while there is one, group 3's first.
        let hits: u32 = morning
            .iter()
            .filter_map(|e| match e.kind {
                EventKind::BaseAttacked {
                    hits_on_aircraft, ..
                } => Some(hits_on_aircraft),
                _ => None,
            })
            .sum();
        assert_eq!(first.destroyed_on_ground, hits.min(22), "seed {}", run.seed);
        let lost_by_defenders: u32 = morning
            .iter()
            .filter_map(|e| match e.kind {
                EventKind::AircraftDestroyedOnGround {
                    air_group,
                    aircraft,
                    ..
                } if air_group == group(DEFENDERS) => Some(aircraft),
                _ => None,
            })
            .sum();
        assert_eq!(lost_by_defenders, hits.min(12));
    }
}

#[test]
fn the_aircraft_a_scout_group_flies_never_rise_with_its_fatigue_in_the_engine_either() {
    // Group 5 starts at each of several fatigues; its first search sends
    // fly(6, f), asserted against the log, and a stood-down group does not
    // fly.
    let mut last = u32::MAX;
    for fatigue in [0i64, 1, 5, 10, 17, 25, 33, 50, 66, 75, 84, 99, 100] {
        let run = run_of(&scouts_start_at(fatigue), 1);
        // The scouts are stood down at the first night if they are above
        // their threshold of 50.
        let searched = run
            .pulse(1, Pulse::Morning)
            .iter()
            .find_map(|e| match e.kind {
                EventKind::SearchFlown { aircraft, .. } => Some(aircraft),
                _ => None,
            })
            .unwrap_or(0);
        let expected = if fatigue > 50 { 0 } else { fly(6, n(fatigue)) };
        assert_eq!(searched, expected, "fatigue {fatigue}");
        assert!(searched <= last);
        last = searched;
    }
}

// ---------------------------------------------------------------------
// What cannot happen, and the properties.
// ---------------------------------------------------------------------

#[test]
fn what_cannot_happen_does_not_in_any_run() {
    for run in batch() {
        let seed = run.seed;
        for event in &run.events {
            match event.kind {
                // No fatigue is below 0 or above 100, and fatigue changes
                // only in a daylight pulse.
                EventKind::AirGroupFatigue {
                    air_group,
                    amount,
                    fatigue,
                    ..
                } => {
                    assert!(fatigue >= Fixed::ZERO && fatigue <= n(100), "seed {seed}");
                    assert!(amount > Fixed::ZERO, "an event only for a change");
                    assert!(
                        matches!(event.pulse, Some(Pulse::Morning | Pulse::Afternoon)),
                        "seed {seed}: no fatigue changes at night"
                    );
                    // Group 4 never flies and has nothing to lose.
                    assert_ne!(air_group, group(RESTING), "seed {seed}");
                }
                // Nobody is stood down or returned in a daylight pulse, and a
                // group with no threshold is never stood down.
                EventKind::AirGroupStoodDown { air_group, .. }
                | EventKind::AirGroupReturned { air_group, .. } => {
                    assert_eq!(event.pulse, Some(Pulse::Night), "seed {seed}");
                    assert!(
                        [BOMBERS, ESCORTS, SCOUTS].contains(&air_group.raw()),
                        "seed {seed}: group {air_group} has no threshold"
                    );
                }
                _ => {}
            }
        }
        // No group flies more aircraft than it has ready, and none flies
        // while stood down: every event of the log is the rule's, asserted
        // in `worked_batch`; and the end states are within range.
        for g in GROUPS {
            assert!(run.fatigue[of(g)] >= Fixed::ZERO && run.fatigue[of(g)] <= n(100));
        }
        assert_conserved(run);
    }
    for worked in worked_batch() {
        for day in &worked.days {
            for g in GROUPS {
                assert!(day.able_morning[of(g)] <= day.ready_morning[of(g)]);
            }
            for g in [BOMBERS, ESCORTS, SCOUTS] {
                if day.stood[of(g)] {
                    assert_eq!(day.able_morning[of(g)], 0);
                    assert_eq!(day.able_afternoon[of(g)], 0);
                }
            }
        }
    }
}

#[test]
fn no_random_number_is_drawn_for_fatigue_the_first_morning_has_scenario_6as_rolls() {
    let reference_scenario = six_a();
    for seed in 1..=VARIANT_SEEDS {
        let run = run_of(&load(), seed);
        let reference = run_of(&reference_scenario, seed);
        // The events of the first morning other than the fatigue events are
        // those of scenario 6a with the same seed.
        let ours: Vec<&Event> = run
            .pulse(1, Pulse::Morning)
            .into_iter()
            .filter(|e| !matches!(e.kind, EventKind::AirGroupFatigue { .. }))
            .collect();
        let theirs = reference.pulse(1, Pulse::Morning);
        assert_eq!(ours.len(), theirs.len(), "seed {seed}");
        for (a, b) in ours.iter().zip(&theirs) {
            assert_eq!(a.kind, b.kind, "seed {seed}");
        }
    }
}

#[test]
fn a_higher_fatigue_never_flies_more_aircraft() {
    for ready in 0..=30 {
        let mut last = u32::MAX;
        for fatigue in 0..=100 {
            let flies = fly(ready, n(fatigue));
            assert!(flies <= last && flies <= ready, "{ready} at {fatigue}");
            last = flies;
        }
        // And with decimals between them.
        let mut last = u32::MAX;
        for raw in (0..=100_000_000).step_by(250_000) {
            let flies = fly(ready, Fixed::from_raw(raw));
            assert!(flies <= last);
            last = flies;
        }
    }
}

#[test]
fn a_longer_sortie_never_gains_less_fatigue() {
    let figures = scenario_config().figures.expect("figures");
    let engine = load()
        .world
        .ref_data()
        .fatigue
        .expect("the fatigue figures are loaded");
    let mut last = Fixed::ZERO;
    for nm in 0..=1200 {
        let gain = figures.gain(nm);
        assert!(gain >= last, "{nm} nm");
        assert_eq!(engine.gain(Fixed::from_int(nm)), gain, "{nm} nm");
        last = gain;
    }
    let order: Vec<Fixed> = [0, 160, 400, 600].map(|nm| figures.gain(nm)).to_vec();
    assert_eq!(order, vec![n(3), Fixed::from_raw(6_200_000), n(11), n(15)]);
    assert!(order.windows(2).all(|pair| pair[0] < pair[1]));
    // In the log: every gain of every run is that of its sortie's distance.
    for run in batch().iter().take(100) {
        for event in &run.events {
            if let EventKind::AirGroupFatigue {
                change:
                    FatigueChange::Gained {
                        sortie,
                        distance_nm,
                    },
                fatigue,
                amount,
                ..
            } = event.kind
            {
                let expected = match sortie {
                    SortieKind::Patrol => n(3),
                    SortieKind::BaseStrike | SortieKind::Escort => n(11),
                    SortieKind::Search => n(15),
                    SortieKind::Strike => panic!("no strike at a contact"),
                };
                assert_eq!(
                    distance_nm,
                    Fixed::from_int(match sortie {
                        SortieKind::Patrol => 0,
                        SortieKind::Search => 600,
                        _ => 400,
                    })
                );
                // A gain is the figure unless the cap at 100 cut it.
                assert!(amount == expected || fatigue == n(100));
            }
        }
    }
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
        assert!(breakdowns > 40);
    }
}

#[test]
fn the_same_seed_gives_the_same_hashes_twice_and_fatigue_changes_the_state() {
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
        assert_eq!(hashes[0].len(), 8);
        assert_eq!(hashes[0], hashes[1], "seed {seed}");
        all_hashes.push(hashes[0].clone());
    }
    assert!(all_hashes.windows(2).any(|pair| pair[0] != pair[1]));
    // The batch holds the same hashes as a fresh run of the same seed.
    let mut again = base.with_seed(77).expect("a seed can be set");
    again.run_to_end().expect("the run resolves");
    assert_eq!(batch()[76].seed, 77);
    assert_eq!(batch()[76].hashes, again.world.day_hashes());
    // Fatigue is in the hash: the same seed without the figures ends in other
    // states.
    let plain = run_of(&without_fatigue(), 77);
    assert_ne!(plain.hashes, again.world.day_hashes());
}

#[test]
fn a_scenario_without_the_fatigue_file_has_the_hashes_it_had_before_this_stage() {
    // The golden files of the command-line tests were written before fatigue
    // existed and are unchanged: each scenario with no fatigue file, run with
    // its own seed, has the day hashes they hold.
    let golden =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../crates/sim-cli/tests/run_baseline");
    for name in [
        "01_transit",
        "03_search",
        "05_eastern_solomons",
        "06a_base_raid",
        "06b_wear_and_repair",
    ] {
        let text = fs::read_to_string(golden.join(format!("{name}.txt")))
            .unwrap_or_else(|_| panic!("a golden file for {name}"));
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
        assert!(scenario.world.ref_data().fatigue.is_none(), "{name}");
        scenario.run_to_end().expect("runs");
        assert_eq!(scenario.world.day_hashes(), wanted, "{name}");
    }
}

#[test]
fn mean_figures_of_the_scenario_over_1000_seeds() {
    // Not an assertion about the rule: the figures a reader wants, printed
    // with --nocapture.
    let runs = batch();
    let worked = worked_batch();
    let total = count(runs.len());
    let mean_x100 = |sum: i64| sum * 100 / total;
    for day in 0..8usize {
        let launched: i64 = worked.iter().map(|w| i64::from(w.days[day].bombers)).sum();
        let raids: i64 = worked
            .iter()
            .filter(|w| w.days[day].bombers > 0)
            .count()
            .try_into()
            .expect("small");
        let attacking: i64 = worked
            .iter()
            .map(|w| i64::from(w.days[day].attacking))
            .sum();
        eprintln!(
            "day {}: bombers launched per run x100 {}, per raid x100 {}, raids {raids}, attacking per run x100 {}",
            day + 1,
            mean_x100(launched),
            launched * 100 / raids.max(1),
            mean_x100(attacking)
        );
    }
    for g in GROUPS {
        let ready: i64 = runs.iter().map(|r| i64::from(r.ready[of(g)])).sum();
        let shot: i64 = worked.iter().map(|w| i64::from(w.shot_down[of(g)])).sum();
        let ground: i64 = worked
            .iter()
            .map(|w| i64::from(w.on_the_ground[of(g)]))
            .sum();
        let fatigue: i64 = runs.iter().map(|r| r.fatigue[of(g)].raw() / 10_000).sum();
        eprintln!(
            "group {g} per run x100: ready at the end {}, shot down {}, destroyed on the ground {}, fatigue at the end {}",
            mean_x100(ready),
            mean_x100(shot),
            mean_x100(ground),
            mean_x100(fatigue) / 100
        );
    }
    let ground: i64 = worked
        .iter()
        .map(|w| w.on_the_ground.iter().map(|c| i64::from(*c)).sum::<i64>())
        .sum();
    let supply: i64 = worked.iter().map(|w| w.supply_destroyed).sum();
    eprintln!(
        "per run x100: aircraft destroyed on the ground {}, supply destroyed (t) {}",
        mean_x100(ground),
        mean_x100(supply)
    );
    assert!(!worked.is_empty());
}
