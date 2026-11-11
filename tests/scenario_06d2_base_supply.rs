//! Scenario 6d2: six days of raids from an airfield that runs out of supply.
//!
//! Stage 6d2 of scenario 6 (decision 0039). The data is
//! `data/scenarios/06d2_base_supply.toml`, run with seeds 1 to 1000.
//! Everything in it is a placeholder.
//!
//! # Hand calculation
//!
//! ## The setting
//!
//! It is scenario 6a (`tests/scenario_06a_base_raid.rs`) over six days,
//! with the supply figures. There is no wear, no fatigue, no pilots and no
//! replacements, and no ship brings supply. Airfield A, base 1, Japanese,
//! at (0,0), has 150 t. Airfield B, base 2, Allied, at (5,0), 200 nm away,
//! has 1,000 t and 20 anti-aircraft shots.
//!
//! | Group | Side | At | Aircraft | Mission |
//! | ----- | ---- | -- | -------- | ------- |
//! | 1 | Japanese | A | 18 bombers | base strike on airfield B |
//! | 2 | Japanese | A | 12 fighters | combat air patrol, share 0.5 |
//! | 3 | Allied | B | 12 fighters | combat air patrol, share 1 |
//! | 4 | Allied | B | 10 bombers | rest |
//! | 5 | Allied | B | 6 scouts | search, arc out to 300 nm |
//!
//! ## The figures
//!
//! An aircraft's sortie costs its base 0.5 t, and 0.25 t more for each 100
//! nm. An aircraft at a base costs it 0.2 t a day.
//!
//! | Sortie | Distance | Cost for each aircraft sent |
//! | ------ | -------- | --------------------------- |
//! | A patrol | none | 0.5 t |
//! | The raid on airfield B, bombers or escorts | 400 nm | 0.5 + 0.25 x 4 = 1.5 t |
//! | A search out to 300 nm | 600 nm | 0.5 + 0.25 x 6 = 2 t |
//!
//! ## A day
//!
//! Write R for a group's ready aircraft and S for a base's supply.
//!
//! **Night, at the start of the day.** Each base pays 0.2 t for each
//! aircraft at it: airfield A for R1 + R2, airfield B for R3 + R4 + R5.
//!
//! **Morning and afternoon.** A base with S = 0 at the start of the pulse
//! flies nothing in it, and its ready aircraft are all on the ground.
//! Otherwise its groups fly as in scenario 6a: in the morning group 1
//! launches R1 bombers, group 2 keeps R2 x 0.5, rounded down, on patrol and
//! sends the rest as escorts, group 3's patrol is R3 and group 5 searches
//! with R5; in the afternoon there is no raid, group 2 keeps its patrol
//! only, group 3 patrols and group 5 searches.
//!
//! After the air combat of the pulse each base pays for the sorties flown
//! from it, on the aircraft sent:
//!
//! - Airfield A, morning: 1.5 t for each bomber launched and each escort
//!   sent, and 0.5 t for each fighter kept on patrol. Afternoon: 0.5 t for
//!   each fighter on patrol.
//! - Airfield B, each daylight pulse: 2 t for each scout and 0.5 t for each
//!   fighter on patrol.
//!
//! A payment the stock cannot cover takes what is left, and the stock is
//! then 0. Bombs that land on airfield B's supply destroy 50 t each in the
//! air combat step, before the base pays for the pulse.
//!
//! ## The first day
//!
//! | When | Airfield A | Airfield B |
//! | ---- | ---------- | ---------- |
//! | Night | 30 aircraft x 0.2 = 6 t: 144 t left | 28 x 0.2 = 5.6 t: 994.4 t left |
//! | Morning | 18 x 1.5 + 6 x 1.5 + 6 x 0.5 = 27 + 9 + 3 = 39 t: 105 t left | 6 x 2 + 12 x 0.5 = 18 t, after 50 t for each bomb on supply |
//! | Afternoon | 0.5 t for each of the (12 - e) x 0.5, rounded down, fighters on patrol, where e is the escorts shot down that morning | 6 x 2 = 12 t, and 0.5 t for each of the 12 - f fighters left, where f is those shot down that morning |
//!
//! The first morning's fights are scenario 6a's first raid, roll for roll.
//!
//! ## When airfield A runs dry
//!
//! With no losses at all a day costs airfield A 6 + 39 + 3 = 48 t, and any
//! loss makes it cheaper. So:
//!
//! - After three days it has at least 150 - 3 x 48 = 6 t: the raids of days
//!   1, 2 and 3 fly in every run, and airfield A is never out of supply
//!   before the night of day 4.
//! - Nothing brings it supply, so once its stock is 0 it stays 0. From the
//!   next daylight pulse groups 1 and 2 fly nothing for the rest of the
//!   run: no raid, no escort, no patrol over airfield A. They lose no more
//!   aircraft, and airfield B is not bombed again.
//! - Airfield B goes on flying and paying.
//!
//! The test works every pulse of every run from the stocks and counts at
//! its start, and finds the pulse in which airfield A's stock reaches 0.
//!
//! ## At the end of a run
//!
//! - Airfield A's supply is 150 t less what it paid by the day and for
//!   sorties.
//! - Airfield B's is 1,000 t less what it paid and less 50 t for each bomb
//!   that landed on supply.
//! - Neither is below 0.
//!
//! ## Variants
//!
//! | Variant | What changes |
//! | ------- | ------------ |
//! | The scenario names no supply file | Nothing is spent. Each of the first three days is, event for event, scenario 6a with the same seed |
//! | Airfield A has 1,000 t | It is never out of supply: a raid flies on each of the six mornings on which group 1 has a bomber |
//! | Airfield A has no supply | It flies nothing at any time and pays nothing. Airfield B is never attacked, and pays for its search and patrol |
//! | Airfield A has 45 t | Night: 6 t, 39 t left. The first morning costs exactly 39 t: 0 left. It flies nothing from that afternoon on. One raid |
//! | Airfield A has 20 t | Night: 6 t, 14 t left. The first raid flies in full and costs 39 t; the base pays the 14 t it has. One raid |
//! | Airfield B has 10 t | Night: 5.6 t, 4.4 t left. It flies the first morning in full and pays what is left. From that afternoon it flies nothing: no search and no patrol. The raid of day 2 meets no fighters, the guns still fire 20 shots, and every ready aircraft of groups 3, 4 and 5 is on the ground, destroyed in that order |
//! | Airfield B is at (2,0), 80 nm | A bomber or an escort costs 0.5 + 0.25 x 1.6 = 0.9 t |
//! | With the wear figures, and airfield B repairing 3 a day | Each repair costs 1 t, paid in the night before the daily cost. A base with less than 1 t repairs nothing |
//!
//! ## What cannot happen
//!
//! - No stock is below 0.
//! - A base with no supply at the start of a daylight pulse sends no
//!   aircraft in it.
//! - No base pays for a sortie in a night pulse, or by the day in a daylight
//!   pulse.
//! - No random number is drawn for supply: the rolls of the first morning
//!   are those of scenario 6a with the same seed.
//!
//! # Properties
//!
//! - Supply is conserved, as "At the end of a run" has it, for both bases
//!   in every run and every variant.
//! - More supply never means fewer raids: for each seed, airfield A flies
//!   no more raids with no supply than with 150 t, and no more with 150 t
//!   than with 1,000 t.
//! - A longer sortie never costs less: 0.5, 0.9, 1.5 and 2 t.
//! - Every group's aircraft at the end are those at the start less its
//!   recorded losses.
//! - Recording breakdowns changes neither the events nor the hashes.
//! - The same seed gives the same hashes twice.
//! - A scenario without the supply file has the hashes it had before this
//!   stage: the golden files of scenarios 1 to 6d1 do not change.

// The test code is written by a developer from the hand calculation above.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;

use sim_core::events::{AirFirer, AirTarget, Cause, Event, EventKind, SupplyUse};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, EntityRef, EventId, OrderId};
use sim_core::rng::{RngKey, RngStream, Subsystem};
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
/// The variants run fewer seeds, as scenarios 6a to 6d1 do.
const VARIANT_SEEDS: u64 = 300;
const DAYS: u32 = 6;
const START: [u32; 5] = [18, 12, 12, 10, 6];
/// The base each group is at.
const GROUP_BASE: [BaseId; 5] = [AIRFIELD_A, AIRFIELD_A, AIRFIELD_B, AIRFIELD_B, AIRFIELD_B];
/// The range of the scouts' arc.
const SEARCH_NM: i64 = 300;
/// The length of one hex in nautical miles.
const HEX_NM: i64 = 40;

fn scenario_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/06d2_base_supply.toml")
}

fn load() -> Scenario {
    Scenario::load(&scenario_path()).expect("scenario 6d2 loads")
}

fn six_a() -> Scenario {
    Scenario::load(
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

/// The index of a group's base in the arrays of two: 0 is airfield A, 1 is B.
fn at(group: u32) -> usize {
    usize::from(GROUP_BASE[of(group)] == AIRFIELD_B)
}

fn count(value: usize) -> i64 {
    i64::try_from(value).expect("small")
}

// Supply is worked in hundredths of a ton, as whole numbers.

/// Hundredths of a ton as the simulation's quantity.
fn fx(h: i64) -> Fixed {
    Fixed::from_ratio(h, 100)
}

/// Hundredths of a ton as the event log writes tons.
fn t(h: i64) -> String {
    assert!(h >= 0);
    if h % 100 == 0 {
        format!("{}", h / 100)
    } else if h % 10 == 0 {
        format!("{}.{}", h / 100, (h % 100) / 10)
    } else {
        format!("{}.{:02}", h / 100, h % 100)
    }
}

/// The figures and the places a run is worked with.
#[derive(Clone, Copy)]
struct Cfg {
    /// Whether the scenario names the supply figures.
    supplied: bool,
    /// What each airfield starts with, in hundredths of a ton.
    start: [i64; 2],
    /// Airfield B's q: its distance from airfield A in hexes.
    raid_hexes: i64,
    /// Supply for one aircraft's sortie, for each 100 nm more, and for a day,
    /// in hundredths of a ton.
    sortie: i64,
    per_100: i64,
    daily: i64,
}

fn scenario_cfg() -> Cfg {
    Cfg {
        supplied: true,
        start: [15_000, 100_000],
        raid_hexes: 5,
        sortie: 50,
        per_100: 25,
        daily: 20,
    }
}

impl Cfg {
    /// What one aircraft of the raid costs, bomber or escort: it goes out and
    /// back, twice the distance.
    fn raid_cost(&self) -> i64 {
        self.sortie + self.per_100 * (self.raid_hexes * HEX_NM * 2) / 100
    }

    /// What one scout costs: out to the range of its arc and back.
    fn search_cost(&self) -> i64 {
        self.sortie + self.per_100 * (SEARCH_NM * 2) / 100
    }

    /// What one fighter on patrol costs: no distance.
    fn patrol_cost(&self) -> i64 {
        self.sortie
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
    damaged: [u32; 5],
    /// The supply of airfield A and airfield B at the end.
    supply: [Fixed; 2],
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

    /// The raids launched.
    fn raids(&self) -> usize {
        self.of(|k| matches!(k, EventKind::BaseStrikeLaunched { .. }))
            .len()
    }

    /// The `BaseSupplySpent` events as (day, pulse, base, reason, aircraft,
    /// asked, paid, left).
    #[allow(clippy::type_complexity)]
    fn spent(&self) -> Vec<(u32, Pulse, u32, SupplyUse, u32, Fixed, Fixed, Fixed)> {
        self.events
            .iter()
            .filter_map(|event| match event.kind {
                EventKind::BaseSupplySpent {
                    base,
                    reason,
                    aircraft,
                    asked_t,
                    paid_t,
                    left_t,
                } => Some((
                    event.day,
                    event.pulse.expect("a pulse"),
                    base.raw(),
                    reason,
                    aircraft,
                    asked_t,
                    paid_t,
                    left_t,
                )),
                _ => None,
            })
            .collect()
    }

    /// The tons a base paid in all, and for one reason.
    fn paid(&self, base: BaseId, reason: Option<SupplyUse>) -> Fixed {
        self.spent()
            .iter()
            .filter(|row| row.2 == base.raw() && reason.is_none_or(|r| r == row.3))
            .fold(Fixed::ZERO, |sum, row| sum + row.6)
    }

    /// The tons a raid destroyed at a base.
    fn destroyed(&self, base: BaseId) -> Fixed {
        self.events
            .iter()
            .filter_map(|event| match event.kind {
                EventKind::BaseSupplyDestroyed {
                    base: b,
                    destroyed_t,
                    ..
                } if b == base => Some(destroyed_t),
                _ => None,
            })
            .fold(Fixed::ZERO, |sum, t| sum + t)
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
        ready: GROUPS.map(|id| truth.air_groups[&group(id)].ready),
        damaged: GROUPS.map(|id| truth.air_groups[&group(id)].damaged),
        supply: [
            truth.bases[&AIRFIELD_A].supply_t,
            truth.bases[&AIRFIELD_B].supply_t,
        ],
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
            .map(|run| work_the_run(run, &scenario_cfg()))
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
// Working every pulse of a run from the stocks and counts at the start of it.
// ---------------------------------------------------------------------

/// What the run's state is, by the rule, as the pulses are worked.
struct State {
    ready: [u32; 5],
    /// The supply of airfield A and airfield B, in hundredths of a ton.
    stock: [i64; 2],
}

/// One day of a run, by the rule.
#[derive(Clone, Default)]
struct Day {
    /// Whether airfield A and airfield B had no supply at the start of the
    /// morning and of the afternoon.
    grounded_morning: [bool; 2],
    grounded_afternoon: [bool; 2],
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
    /// Aircraft shot down and destroyed on the ground, by group, in the
    /// morning.
    shot_down: [u32; 5],
    on_the_ground: [u32; 5],
    /// The aircraft on the ground at airfield B at the start of the raid:
    /// ready and not flying (0 for a group elsewhere).
    ground_before: [u32; 5],
    /// Hits on airfield B's supply in the morning.
    hits_on_supply: u32,
    /// The stocks after the night, after the morning and after the
    /// afternoon.
    stock_night: [i64; 2],
    stock_morning: [i64; 2],
    stock_end: [i64; 2],
    ready_end: [u32; 5],
}

/// What working one run found.
#[derive(Default)]
struct Worked {
    days: Vec<Day>,
    ready: [u32; 5],
    stock: [i64; 2],
    shot_down: [u32; 5],
    on_the_ground: [u32; 5],
    /// What each base paid, in hundredths of a ton, for sorties, by the day
    /// and for repairs.
    paid: [[i64; 3]; 2],
    /// The raids launched.
    raids: u32,
    /// The pulse in which airfield A's stock fell to 0, if it did.
    dry: Option<(u32, Pulse)>,
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
        EventKind::BaseSupplySpent { .. } => 9,
        other => panic!("an event the day does not have: {other:?}"),
    }
}

const REASONS: [SupplyUse; 3] = [SupplyUse::Sorties, SupplyUse::Upkeep, SupplyUse::Repairs];

fn reason_index(reason: SupplyUse) -> usize {
    REASONS.iter().position(|r| *r == reason).expect("a reason")
}

/// A payment the rule expects: base, reason, aircraft, asked, paid, left.
type Payment = (BaseId, SupplyUse, u32, i64, i64, i64);

/// Takes what the stock covers off it and returns the payment, by decision 6
/// of 0039.
fn pay(stock: &mut i64, base: BaseId, reason: SupplyUse, aircraft: u32, asked: i64) -> Payment {
    let paid = asked.min(*stock);
    *stock -= paid;
    (base, reason, aircraft, asked, paid, *stock)
}

/// Asserts that `event` is `payment`, in its fields and in the words the log
/// writes it in, and that its breakdown says how it was reached.
fn assert_payment(event: &Event, payment: &Payment, seed: u64, day: u32) {
    let (base, reason, aircraft, asked, paid, left) = *payment;
    assert_eq!(
        event.kind,
        EventKind::BaseSupplySpent {
            base,
            reason,
            aircraft,
            asked_t: fx(asked),
            paid_t: fx(paid),
            left_t: fx(left),
        },
        "seed {seed}, day {day}"
    );
    let what = match reason {
        SupplyUse::Sorties => format!("{aircraft} aircraft sent on sorties"),
        SupplyUse::Upkeep => format!("{aircraft} aircraft at the base for the day"),
        SupplyUse::Repairs => format!("{aircraft} repairs"),
    };
    let mut text = if paid < asked {
        format!(
            "{base} spent {} t of supply on {what}: {} t was asked and the stock could not cover it, {} t left",
            t(paid),
            t(asked),
            t(left)
        )
    } else {
        format!(
            "{base} spent {} t of supply on {what}, {} t left",
            t(paid),
            t(left)
        )
    };
    if left == 0 {
        text.push_str(&format!(": {base} is out of supply"));
    }
    assert_eq!(event.kind.to_string(), text, "seed {seed}, day {day}");
    let breakdown = event.breakdown.as_ref().expect("a breakdown");
    assert_eq!(breakdown.result, fx(asked), "seed {seed}, day {day}");
    if reason == SupplyUse::Sorties {
        // The parts of the sum, for each kind of sortie, add up to it.
        let parts = breakdown
            .factors
            .iter()
            .filter(|f| f.name.ends_with("_t"))
            .fold(Fixed::ZERO, |sum, f| sum + f.value);
        assert_eq!(parts, fx(asked), "seed {seed}, day {day}: the parts");
    }
}

/// Works every pulse of `run` by the rule of the hand calculation from the
/// stocks and counts at the start of it, asserts every event against it, and
/// returns what was worked. The counts are carried from pulse to pulse by the
/// rule, not read from the events.
fn work_the_run(run: &Run, cfg: &Cfg) -> Worked {
    let seed = run.seed;
    let mut state = State {
        ready: START,
        stock: cfg.start,
    };
    let mut worked = Worked::default();
    for day in 1..=DAYS {
        let mut record = Day::default();
        work_the_night(run, cfg, day, &mut state, &mut record, &mut worked);
        record.stock_night = state.stock;
        work_the_morning(run, cfg, day, &mut state, &mut record, &mut worked);
        record.stock_morning = state.stock;
        work_the_afternoon(run, cfg, day, &mut state, &mut record, &mut worked);
        record.stock_end = state.stock;
        record.ready_end = state.ready;
        for g in GROUPS {
            worked.shot_down[of(g)] += record.shot_down[of(g)];
            worked.on_the_ground[of(g)] += record.on_the_ground[of(g)];
        }
        worked.days.push(record);
    }

    // The end of the run.
    assert_eq!(run.ready, state.ready, "seed {seed}: ready at the end");
    assert_eq!(run.damaged, [0; 5], "seed {seed}: nothing is damaged");
    for i in 0..2 {
        assert_eq!(
            run.supply[i],
            fx(state.stock[i]),
            "seed {seed}: the supply of base {} at the end",
            i + 1
        );
        assert!(state.stock[i] >= 0);
    }
    worked.ready = state.ready;
    worked.stock = state.stock;
    worked
}

/// Notes the pulse in which airfield A's stock first fell to 0.
fn note_dry(worked: &mut Worked, cfg: &Cfg, state: &State, day: u32, pulse: Pulse) {
    if cfg.start[0] > 0 && state.stock[0] == 0 && worked.dry.is_none() {
        worked.dry = Some((day, pulse));
    }
}

/// Night, at the start of the day: each base pays the daily figure for each
/// aircraft at it, as much as its stock covers, and a base with no stock
/// pays nothing and records nothing.
fn work_the_night(
    run: &Run,
    cfg: &Cfg,
    day: u32,
    state: &mut State,
    _record: &mut Day,
    worked: &mut Worked,
) {
    let seed = run.seed;
    let night = run.pulse(day, Pulse::Night);
    let mut expected: Vec<Payment> = Vec::new();
    if cfg.supplied {
        for (i, base) in [AIRFIELD_A, AIRFIELD_B].into_iter().enumerate() {
            let aircraft: u32 = GROUPS
                .iter()
                .filter(|g| GROUP_BASE[of(**g)] == base)
                .map(|g| state.ready[of(*g)])
                .sum();
            let asked = cfg.daily * i64::from(aircraft);
            if asked == 0 || state.stock[i] == 0 {
                continue;
            }
            let payment = pay(
                &mut state.stock[i],
                base,
                SupplyUse::Upkeep,
                aircraft,
                asked,
            );
            worked.paid[i][reason_index(SupplyUse::Upkeep)] += payment.4;
            expected.push(payment);
        }
    }
    note_dry(worked, cfg, state, day, Pulse::Night);
    assert_eq!(
        night.len(),
        expected.len(),
        "seed {seed}, day {day}: the night has the daily cost and nothing else"
    );
    for (event, payment) in night.iter().zip(&expected) {
        assert_payment(event, payment, seed, day);
        assert!(event.causes.is_empty(), "seed {seed}: nothing orders it");
    }
}

fn work_the_morning(
    run: &Run,
    cfg: &Cfg,
    day: u32,
    state: &mut State,
    record: &mut Day,
    worked: &mut Worked,
) {
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
    let lost = kinds(|k| matches!(k, EventKind::AircraftLost { .. }));
    let attacked = kinds(|k| matches!(k, EventKind::BaseAttacked { .. }));
    let on_ground_events = kinds(|k| matches!(k, EventKind::AircraftDestroyedOnGround { .. }));
    let supply_events = kinds(|k| matches!(k, EventKind::BaseSupplyDestroyed { .. }));
    let spent_events = kinds(|k| matches!(k, EventKind::BaseSupplySpent { .. }));
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
            + spent_events.len(),
        "seed {seed}, day {day}"
    );

    // A base with no supply at the start of the pulse flies nothing in it.
    let grounded = [
        cfg.supplied && state.stock[0] == 0,
        cfg.supplied && state.stock[1] == 0,
    ];
    record.grounded_morning = grounded;
    // What each group can fly: all its ready aircraft, or none at a grounded
    // base. Group 5 searches with all it can fly; group 1 launches all it can
    // fly, if that is at least one; group 2 keeps half of what it can fly,
    // rounded down, and the rest escort the raid if one launches; group 3
    // patrols with all it can fly.
    let able = GROUPS.map(|g| {
        if grounded[at(g)] {
            0
        } else {
            state.ready[of(g)]
        }
    });
    record.able_morning = able;
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
    let search_event = if s5 > 0 {
        assert_eq!(searches.len(), 1, "seed {seed}, day {day}");
        assert_eq!(
            searches[0].kind,
            EventKind::SearchFlown {
                air_group: group(SCOUTS),
                from: AirGroupLocation::Base(AIRFIELD_B),
                hex: Hex::new(cfg.raid_hexes as i32, 0),
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

    let mut launch_event = None;
    let mut sent_event = None;
    if raid {
        let (launch, sent_id) = work_the_raid(
            run,
            cfg,
            day,
            state,
            record,
            [b, e, p3, s5],
            [&launches, &sent, &fights, &guns],
            [&lost, &attacked, &on_ground_events, &supply_events],
        );
        launch_event = Some(launch);
        sent_event = sent_id;
        worked.raids += 1;
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

    // After the air combat each base pays for the sorties flown from it, on
    // the aircraft sent: a bomber and an escort the raid's figure, a fighter
    // on patrol the sortie figure, a scout the search figure. A raid's hits
    // on supply have come off the stock first.
    let mut expected: Vec<(Payment, Vec<Cause>)> = Vec::new();
    if cfg.supplied {
        if !grounded[0] {
            let aircraft = b + e + kept;
            let asked = i64::from(b + e) * cfg.raid_cost() + i64::from(kept) * cfg.patrol_cost();
            if asked > 0 {
                let mut causes: Vec<Cause> = Vec::new();
                causes.extend(launch_event.map(Cause::Event));
                if kept > 0 {
                    causes.extend(run.patrol_orders(ESCORTS));
                }
                causes.extend(sent_event.map(Cause::Event));
                let payment = pay(
                    &mut state.stock[0],
                    AIRFIELD_A,
                    SupplyUse::Sorties,
                    aircraft,
                    asked,
                );
                expected.push((payment, causes));
            }
        }
        if !grounded[1] {
            let aircraft = s5 + p3;
            let asked = i64::from(s5) * cfg.search_cost() + i64::from(p3) * cfg.patrol_cost();
            if asked > 0 {
                let mut causes: Vec<Cause> = Vec::new();
                if p3 > 0 {
                    causes.extend(run.patrol_orders(DEFENDERS));
                }
                causes.extend(search_event.map(Cause::Event));
                let payment = pay(
                    &mut state.stock[1],
                    AIRFIELD_B,
                    SupplyUse::Sorties,
                    aircraft,
                    asked,
                );
                expected.push((payment, causes));
            }
        }
    }
    note_dry(worked, cfg, state, day, Pulse::Morning);
    assert_eq!(
        spent_events.len(),
        expected.len(),
        "seed {seed}, day {day}: one sum for each base that flew"
    );
    for (event, (payment, causes)) in spent_events.iter().zip(&expected) {
        assert_payment(event, payment, seed, day);
        assert_eq!(&event.causes, causes, "seed {seed}, day {day}");
        worked.paid[usize::from(payment.0 == AIRFIELD_B)][0] += payment.4;
    }
}

fn work_the_afternoon(
    run: &Run,
    cfg: &Cfg,
    day: u32,
    state: &mut State,
    record: &mut Day,
    worked: &mut Worked,
) {
    let seed = run.seed;
    let step = run.pulse(day, Pulse::Afternoon);
    let grounded = [
        cfg.supplied && state.stock[0] == 0,
        cfg.supplied && state.stock[1] == 0,
    ];
    record.grounded_afternoon = grounded;
    let able = GROUPS.map(|g| {
        if grounded[at(g)] {
            0
        } else {
            state.ready[of(g)]
        }
    });
    record.able_afternoon = able;
    let s5 = able[of(SCOUTS)];
    let p3 = able[of(DEFENDERS)];
    let kept = able[of(ESCORTS)] * 5 / 10;
    // No raid flies in the afternoon: the scouts search, and the patrols
    // meet nobody.
    let mut expected_events = 0;
    let mut search_event = None;
    if s5 > 0 {
        let event = step.first().expect("the search");
        assert_eq!(
            event.kind,
            EventKind::SearchFlown {
                air_group: group(SCOUTS),
                from: AirGroupLocation::Base(AIRFIELD_B),
                hex: Hex::new(cfg.raid_hexes as i32, 0),
                from_deg: 40,
                to_deg: 130,
                range_nm: Fixed::from_int(SEARCH_NM),
                aircraft: s5,
            },
            "seed {seed}, day {day}"
        );
        search_event = Some(event.id);
        expected_events += 1;
    }
    let mut expected: Vec<(Payment, Vec<Cause>)> = Vec::new();
    if cfg.supplied {
        if !grounded[0] {
            let asked = i64::from(kept) * cfg.patrol_cost();
            if asked > 0 {
                let causes = run.patrol_orders(ESCORTS);
                let payment = pay(
                    &mut state.stock[0],
                    AIRFIELD_A,
                    SupplyUse::Sorties,
                    kept,
                    asked,
                );
                expected.push((payment, causes));
            }
        }
        if !grounded[1] {
            let asked = i64::from(s5) * cfg.search_cost() + i64::from(p3) * cfg.patrol_cost();
            if asked > 0 {
                let mut causes: Vec<Cause> = Vec::new();
                if p3 > 0 {
                    causes.extend(run.patrol_orders(DEFENDERS));
                }
                causes.extend(search_event.map(Cause::Event));
                let payment = pay(
                    &mut state.stock[1],
                    AIRFIELD_B,
                    SupplyUse::Sorties,
                    s5 + p3,
                    asked,
                );
                expected.push((payment, causes));
            }
        }
    }
    note_dry(worked, cfg, state, day, Pulse::Afternoon);
    assert_eq!(
        step.len(),
        expected_events + expected.len(),
        "seed {seed}, day {day}: the search and the sums, nothing else"
    );
    for (event, (payment, causes)) in step[expected_events..].iter().zip(&expected) {
        assert_payment(event, payment, seed, day);
        assert_eq!(&event.causes, causes, "seed {seed}, day {day}");
        worked.paid[usize::from(payment.0 == AIRFIELD_B)][0] += payment.4;
    }
}

/// The morning's raid worked as in scenario 6a on the counts of `flying`
/// (bombers, escorts, group 3's patrol, the scouts searching). The aircraft
/// on the ground at airfield B are every ready aircraft there that is not
/// flying: the fighters of group 3 beyond what flies, all of group 4, and
/// the scouts that are not searching; all of them if airfield B has no
/// supply. The raid is alone, so it meets the whole patrol. Returns the
/// launch event and the escorts' event, if any.
#[allow(clippy::too_many_arguments)]
fn work_the_raid(
    run: &Run,
    cfg: &Cfg,
    day: u32,
    state: &mut State,
    record: &mut Day,
    flying: [u32; 4],
    first: [&[&Event]; 4],
    then: [&[&Event]; 4],
) -> (EventId, Option<EventId>) {
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
            hex: Hex::new(cfg.raid_hexes as i32, 0),
            aircraft: b,
            distance_nm: Fixed::from_int(cfg.raid_hexes * HEX_NM),
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

    let mut sent_id = None;
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
        sent_id = Some(sent[0].id);
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

    // The guns of airfield B, at the bombers left: they fire whether or not
    // airfield B has supply.
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

    // The aircraft lost in the air, one event for each group and roll.
    assert_eq!(
        lost.len(),
        expected_losses.len(),
        "seed {seed}, day {day}: the losses"
    );
    let mut aircraft_events: Vec<EventId> = Vec::new();
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
        aircraft_events.push(event.id);
        record.shot_down[of(*g)] += count;
    }
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
        record.hits_on_supply = on_supply;
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
        let wanted = 5000 * i64::from(on_supply);
        let destroyed = wanted.min(state.stock[1]);
        if destroyed > 0 {
            assert_eq!(supply_events.len(), 1, "seed {seed}, day {day}");
            state.stock[1] -= destroyed;
            assert_eq!(
                supply_events[0].kind,
                EventKind::BaseSupplyDestroyed {
                    base: AIRFIELD_B,
                    destroyed_t: fx(destroyed),
                    supply_t: fx(state.stock[1]),
                },
                "seed {seed}"
            );
        } else {
            assert!(supply_events.is_empty(), "seed {seed}, day {day}");
        }
    }
    record.attacking = bombers_left;
    (launch.id, sent_id)
}

// ---------------------------------------------------------------------
// Reading the stocks and aircraft back from the events alone.
// ---------------------------------------------------------------------

/// The stocks and the aircraft of each group, followed through a run's
/// events, with no rule in it.
#[derive(Clone)]
struct Track {
    ready: [u32; 5],
    damaged: [u32; 5],
    stock: [Fixed; 2],
}

impl Track {
    fn new(start: [i64; 2]) -> Track {
        Track {
            ready: START,
            damaged: [0; 5],
            stock: [fx(start[0]), fx(start[1])],
        }
    }

    fn apply(&mut self, event: &Event) {
        match event.kind {
            EventKind::BaseSupplySpent {
                base,
                paid_t,
                left_t,
                ..
            } => {
                let i = usize::from(base == AIRFIELD_B);
                assert_eq!(
                    self.stock[i] - paid_t,
                    left_t,
                    "what is left is stock less paid"
                );
                self.stock[i] = left_t;
            }
            EventKind::BaseSupplyDestroyed { base, supply_t, .. } => {
                self.stock[usize::from(base == AIRFIELD_B)] = supply_t;
            }
            EventKind::AircraftLost {
                air_group,
                remaining,
                ..
            }
            | EventKind::AircraftDestroyedOnGround {
                air_group,
                remaining,
                ..
            } => self.ready[of(air_group.raw())] = remaining,
            EventKind::DamagedAircraftDestroyedOnGround {
                air_group,
                remaining,
                ..
            } => self.damaged[of(air_group.raw())] = remaining,
            EventKind::AircraftWorn {
                air_group,
                ready,
                damaged,
                ..
            }
            | EventKind::AircraftRepaired {
                air_group,
                ready,
                damaged,
                ..
            } => {
                self.ready[of(air_group.raw())] = ready;
                self.damaged[of(air_group.raw())] = damaged;
            }
            _ => {}
        }
    }

    /// The aircraft at a base, ready and damaged.
    fn aircraft_at(&self, base: BaseId) -> u32 {
        GROUPS
            .iter()
            .filter(|g| GROUP_BASE[of(**g)] == base)
            .map(|g| self.ready[of(*g)] + self.damaged[of(*g)])
            .sum()
    }
}

fn pulse_order(pulse: Pulse) -> u32 {
    match pulse {
        Pulse::Night => 0,
        Pulse::Morning => 1,
        Pulse::Afternoon => 2,
    }
}

/// The aircraft and the supply are conserved in one run, from its events
/// alone: each base's supply is what it started with less what it paid and
/// less what a raid destroyed, and each group's aircraft are what it started
/// with less what was recorded lost.
fn assert_conserved(run: &Run, start: [i64; 2]) {
    let seed = run.seed;
    for (i, base) in [AIRFIELD_A, AIRFIELD_B].into_iter().enumerate() {
        let left = fx(start[i]) - run.paid(base, None) - run.destroyed(base);
        assert_eq!(run.supply[i], left, "seed {seed}: base {} supply", i + 1);
        assert!(run.supply[i] >= Fixed::ZERO, "seed {seed}");
    }
    for g in GROUPS {
        let i = of(g);
        let (mut shot, mut ground, mut written_off) = (0, 0, 0);
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
                }
                | EventKind::DamagedAircraftDestroyedOnGround {
                    air_group,
                    aircraft,
                    ..
                } if air_group == group(g) => ground += aircraft,
                EventKind::AircraftWorn {
                    air_group,
                    written_off: w,
                    ..
                } if air_group == group(g) => written_off += w,
                _ => {}
            }
        }
        assert_eq!(
            START[i],
            run.ready[i] + run.damaged[i] + shot + ground + written_off,
            "seed {seed}, group {g}: the aircraft at the end are those at the start less the losses"
        );
    }
}

// ---------------------------------------------------------------------
// Variants: the scenario with one thing changed, written under the build
// directory. Nothing in the repository's data folder is touched.
// ---------------------------------------------------------------------

/// Copies the scenario and its data into a folder under the build directory,
/// applies `edit` to the copied scenario's text and `edit_supply` to the
/// supply figures, and loads it.
fn variant_with(
    tag: &str,
    edit: &dyn Fn(String) -> String,
    edit_supply: &dyn Fn(String) -> String,
) -> Scenario {
    let data = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data");
    // Tests run in parallel: each variant has a folder of its own.
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "base_supply_{tag}_{}_{}",
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
        "reference/air_upkeep.toml",
    ] {
        fs::copy(data.join(file), root.join(file)).expect("copy a data file");
    }
    let supply = fs::read_to_string(data.join("reference/air_supply.toml")).expect("read supply");
    fs::write(root.join("reference/air_supply.toml"), edit_supply(supply))
        .expect("write the supply figures");
    let scenario_file = root.join("scenarios/06d2_base_supply.toml");
    let text = fs::read_to_string(data.join("scenarios/06d2_base_supply.toml"))
        .expect("read the scenario");
    fs::write(&scenario_file, edit(text)).expect("write the variant");
    Scenario::load(&scenario_file).expect("the variant loads")
}

fn variant(tag: &str, edit: &dyn Fn(String) -> String) -> Scenario {
    variant_with(tag, edit, &|text| text)
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

const SUPPLY_LINE: &str = "air_supply = \"../reference/air_supply.toml\"\n";
const UPKEEP_LINE: &str = "air_upkeep = \"../reference/air_upkeep.toml\"\n";

fn without_the_supply_file() -> Scenario {
    variant("no_file", &|text| once(text, SUPPLY_LINE, ""))
}

fn airfield_a_has(tons: i64) -> Scenario {
    variant(&format!("a_{tons}"), &|text| {
        once(text, "supply_t = 150", &format!("supply_t = {tons}"))
    })
}

fn airfield_b_has(tons: i64) -> Scenario {
    variant(&format!("b_{tons}"), &|text| {
        once(text, "supply_t = 1000", &format!("supply_t = {tons}"))
    })
}

fn airfield_b_at_2_0() -> Scenario {
    variant("b_80_nm", &|text| once(text, "q = 5", "q = 2"))
}

/// The scenario with the wear figures and airfield B repairing 3 a day.
fn with_wear_and_repairs(
    tag: &str,
    edit_scenario: &dyn Fn(String) -> String,
    edit_supply: &dyn Fn(String) -> String,
) -> Scenario {
    variant_with(
        tag,
        &|text| {
            let text = once(text, SUPPLY_LINE, &format!("{SUPPLY_LINE}{UPKEEP_LINE}"));
            let text = once(
                text,
                "anti_aircraft = 20\n",
                "anti_aircraft = 20\naircraft_repairs_per_day = 3\n",
            );
            edit_scenario(text)
        },
        edit_supply,
    )
}

fn work_all(runs: &[Run], cfg: &Cfg) -> Vec<Worked> {
    runs.iter().map(|run| work_the_run(run, cfg)).collect()
}

/// A run's events for one group of the Japanese or Allied side, as the
/// group that did them: launches, escorts sent, fights and losses.
fn actor(kind: &EventKind) -> Option<u32> {
    match kind {
        EventKind::BaseStrikeLaunched { air_group, .. }
        | EventKind::EscortsSent { air_group, .. }
        | EventKind::SearchFlown { air_group, .. }
        | EventKind::BaseAttacked { air_group, .. }
        | EventKind::AircraftLost { air_group, .. } => Some(air_group.raw()),
        EventKind::AirFight { firer, .. } => Some(firer.raw()),
        _ => None,
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
    assert_eq!(truth.bases[&AIRFIELD_A].supply_t, Fixed::from_int(150));
    assert_eq!(truth.bases[&AIRFIELD_B].side, Side::Allied);
    assert_eq!(truth.bases[&AIRFIELD_B].hex, Hex::new(5, 0));
    assert_eq!(truth.bases[&AIRFIELD_B].supply_t, Fixed::from_int(1000));
    assert_eq!(truth.bases[&AIRFIELD_B].anti_aircraft, 20);
    for g in GROUPS {
        let air_group = &truth.air_groups[&group(g)];
        let i = of(g);
        assert_eq!(air_group.ready, START[i], "group {g}");
        assert_eq!(air_group.damaged, 0);
        assert_eq!(air_group.pilots, None, "no pilots are tracked");
        assert_eq!(air_group.location, AirGroupLocation::Base(GROUP_BASE[i]));
        assert_eq!(
            air_group.side,
            if g <= 2 { Side::Japanese } else { Side::Allied }
        );
    }
    assert!(truth.replacements.is_empty(), "no replacements");
    // No wear and no fatigue, and the supply figures are loaded.
    let data = scenario.world.ref_data();
    assert!(data.air_upkeep.is_none());
    assert!(data.fatigue.is_none());
    assert!(data.air_supply.is_some());
    assert_eq!(truth.bases[&AIRFIELD_A].aircraft_repairs_per_day, 0);
    assert_eq!(truth.bases[&AIRFIELD_B].aircraft_repairs_per_day, 0);
}

#[test]
fn the_figures_table_is_as_the_hand_calculation_says() {
    let scenario = load();
    let figures = scenario.world.ref_data().air_supply.expect("figures");
    assert_eq!(figures.sortie_supply_t, fx(50));
    assert_eq!(figures.sortie_supply_t_per_100_nm, fx(25));
    assert_eq!(figures.daily_supply_t_per_aircraft, fx(20));
    // A patrol 0.5 t, the raid 0.5 + 0.25 x 4 = 1.5 t, a search out to 300
    // nm 0.5 + 0.25 x 6 = 2 t.
    assert_eq!(figures.sortie_cost(Fixed::ZERO), fx(50));
    assert_eq!(figures.sortie_cost(Fixed::from_int(400)), fx(150));
    assert_eq!(figures.sortie_cost(Fixed::from_int(600)), fx(200));
    let cfg = scenario_cfg();
    assert_eq!(
        (cfg.patrol_cost(), cfg.raid_cost(), cfg.search_cost()),
        (50, 150, 200)
    );
    // Airfield B at 80 nm: 0.5 + 0.25 x 1.6 = 0.9 t.
    assert_eq!(figures.sortie_cost(Fixed::from_int(160)), fx(90));
}

#[test]
fn every_pulse_of_every_run_is_worked_by_the_rule_from_the_stocks_and_counts_at_the_start_of_it() {
    let worked = worked_batch();
    assert_eq!(worked.len(), 1000);
    for (run, worked) in batch().iter().zip(worked) {
        assert_eq!(worked.days.len(), DAYS as usize, "seed {}", run.seed);
        assert_conserved(run, scenario_cfg().start);
    }
}

#[test]
fn the_first_day_is_as_the_hand_calculation_says_in_terms_of_that_runs_e_f_and_bombs_on_supply() {
    for run in batch() {
        let seed = run.seed;
        // The first morning: all 18 bombers, 6 escorts and 6 fighters kept on
        // patrol fly, and nothing is lost before the raid is worked.
        // Hits on airfield B's supply that morning.
        let k = run
            .of(|k| matches!(k, EventKind::BaseAttacked { .. }))
            .iter()
            .find(|event| event.day == 1)
            .map_or(0, |event| match event.kind {
                EventKind::BaseAttacked { hits_on_supply, .. } => i64::from(hits_on_supply),
                _ => unreachable!(),
            });
        // Only the first day's losses count.
        let on_day_1 = |g: u32| -> u32 {
            run.of(|k| matches!(k, EventKind::AircraftLost { air_group, .. } if *air_group == group(g)))
                .iter()
                .filter(|event| event.day == 1)
                .map(|event| match event.kind {
                    EventKind::AircraftLost { aircraft, .. } => aircraft,
                    _ => unreachable!(),
                })
                .sum()
        };
        let (e, f) = (on_day_1(ESCORTS), on_day_1(DEFENDERS));

        let spent: Vec<_> = run.spent().into_iter().filter(|row| row.0 == 1).collect();
        // Night: 30 aircraft x 0.2 = 6 t, and 28 x 0.2 = 5.6 t.
        assert_eq!(
            spent[0],
            (
                1,
                Pulse::Night,
                1,
                SupplyUse::Upkeep,
                30,
                fx(600),
                fx(600),
                fx(14_400)
            ),
            "seed {seed}"
        );
        assert_eq!(
            spent[1],
            (
                1,
                Pulse::Night,
                2,
                SupplyUse::Upkeep,
                28,
                fx(560),
                fx(560),
                fx(99_440)
            ),
            "seed {seed}"
        );
        // Morning: airfield A pays 18 x 1.5 + 6 x 1.5 + 6 x 0.5 = 39 t, and
        // airfield B 6 x 2 + 12 x 0.5 = 18 t after 50 t for each bomb on
        // supply.
        assert_eq!(
            spent[2],
            (
                1,
                Pulse::Morning,
                1,
                SupplyUse::Sorties,
                30,
                fx(3900),
                fx(3900),
                fx(10_500)
            ),
            "seed {seed}"
        );
        let b_morning = 99_440 - 5000 * k - 1800;
        assert_eq!(
            spent[3],
            (
                1,
                Pulse::Morning,
                2,
                SupplyUse::Sorties,
                18,
                fx(1800),
                fx(1800),
                fx(b_morning)
            ),
            "seed {seed}"
        );
        // Afternoon: airfield A 0.5 t for each of the (12 - e) x 0.5 fighters
        // on patrol, rounded down; airfield B 6 x 2 = 12 t and 0.5 t for each
        // of the 12 - f fighters left.
        let kept = i64::from((12 - e) / 2);
        let a_asked = 50 * kept;
        assert_eq!(
            spent[4],
            (
                1,
                Pulse::Afternoon,
                1,
                SupplyUse::Sorties,
                u32::try_from(kept).expect("small"),
                fx(a_asked),
                fx(a_asked),
                fx(10_500 - a_asked)
            ),
            "seed {seed}"
        );
        let b_asked = 1200 + 50 * i64::from(12 - f);
        assert_eq!(
            spent[5],
            (
                1,
                Pulse::Afternoon,
                2,
                SupplyUse::Sorties,
                6 + (12 - f),
                fx(b_asked),
                fx(b_asked),
                fx(b_morning - b_asked)
            ),
            "seed {seed}"
        );
        assert_eq!(spent.len(), 6, "seed {seed}: six payments on the first day");
    }
}

#[test]
fn the_first_morning_has_scenario_6as_rolls_and_no_random_number_is_drawn_for_supply() {
    let reference_scenario = six_a();
    for seed in 1..=VARIANT_SEEDS {
        let run = run_of(&load(), seed);
        let reference = run_of(&reference_scenario, seed);
        let ours: Vec<&Event> = run
            .pulse(1, Pulse::Morning)
            .into_iter()
            .filter(|e| !matches!(e.kind, EventKind::BaseSupplySpent { .. }))
            .collect();
        let theirs = reference.pulse(1, Pulse::Morning);
        assert_eq!(ours.len(), theirs.len(), "seed {seed}");
        for (a, b) in ours.iter().zip(&theirs) {
            match (&a.kind, &b.kind) {
                // Airfield B has paid its night's cost: what is left differs.
                (
                    EventKind::BaseSupplyDestroyed { destroyed_t: x, .. },
                    EventKind::BaseSupplyDestroyed { destroyed_t: y, .. },
                ) => assert_eq!(x, y, "seed {seed}"),
                (x, y) => assert_eq!(x, y, "seed {seed}"),
            }
        }
    }
}

#[test]
fn when_airfield_a_runs_dry_each_bullet_holds_in_every_run() {
    let mut distribution: BTreeMap<String, u32> = BTreeMap::new();
    let mut raidless_early = 0;
    for (run, worked) in batch().iter().zip(worked_batch()) {
        let seed = run.seed;
        // After three days it has at least 150 - 3 x 48 = 6 t, and it is
        // never out of supply before the night of day 4.
        assert!(worked.days[2].stock_end[0] >= 600, "seed {seed}");
        for day in 0..3 {
            let d = &worked.days[day];
            assert!(
                !d.grounded_morning[0] && !d.grounded_afternoon[0],
                "seed {seed}, day {}: airfield A flies",
                day + 1
            );
            // The raid of the day flies when group 1 has a bomber.
            let launched = !run
                .pulse(day as u32 + 1, Pulse::Morning)
                .iter()
                .all(|e| !matches!(e.kind, EventKind::BaseStrikeLaunched { .. }));
            assert_eq!(launched, d.able_morning[of(BOMBERS)] >= 1, "seed {seed}");
            if d.able_morning[of(BOMBERS)] == 0 {
                raidless_early += 1;
            }
        }
        // A day costs at most 48 t with no loss at all, and any loss makes it
        // cheaper.
        for day in 0..3 {
            let start = if day == 0 {
                15_000
            } else {
                worked.days[day - 1].stock_end[0]
            };
            assert!(start - worked.days[day].stock_end[0] <= 4800, "seed {seed}");
        }
        // Nothing brings it supply: once it is 0 it stays 0, and from the next
        // daylight pulse groups 1 and 2 fly nothing for the rest of the run.
        let mut dry_at: Option<(u32, Pulse)> = None;
        let mut track = Track::new(scenario_cfg().start);
        for event in run.events.iter().filter(|e| e.pulse.is_some()) {
            let now = (event.day, event.pulse.expect("a pulse"));
            if let Some(at) = dry_at {
                assert_eq!(track.stock[0], Fixed::ZERO, "seed {seed}: it stays 0");
                let later = (now.0, pulse_order(now.1)) > (at.0, pulse_order(at.1));
                if later {
                    if let Some(g) = actor(&event.kind) {
                        assert!(
                            ![BOMBERS, ESCORTS].contains(&g),
                            "seed {seed}: group {g} acts after airfield A is dry: {:?}",
                            event.kind
                        );
                    }
                    // Airfield B is not bombed again, and no aircraft are lost.
                    assert!(
                        !matches!(
                            event.kind,
                            EventKind::BaseAttacked { .. }
                                | EventKind::AircraftLost { .. }
                                | EventKind::AircraftDestroyedOnGround { .. }
                                | EventKind::EscortsSent { .. }
                                | EventKind::BaseStrikeLaunched { .. }
                        ),
                        "seed {seed}: {:?}",
                        event.kind
                    );
                    // Airfield B goes on flying and paying.
                }
            }
            track.apply(event);
            if dry_at.is_none() && track.stock[0] == Fixed::ZERO {
                dry_at = Some((event.day, event.pulse.expect("a pulse")));
            }
        }
        assert_eq!(dry_at, worked.dry, "seed {seed}: the pulse it is dry in");
        let key = match worked.dry {
            Some((day, pulse)) => format!("day {day} {pulse:?}"),
            None => "never".to_string(),
        };
        *distribution.entry(key).or_insert(0) += 1;
        // Airfield B flies and pays in every daylight pulse: its search and its
        // patrol are paid for whenever it has aircraft to fly.
        for day in 1..=DAYS {
            for pulse in [Pulse::Morning, Pulse::Afternoon] {
                let pays = run
                    .spent()
                    .iter()
                    .filter(|row| row.0 == day && row.1 == pulse && row.2 == 2)
                    .count();
                assert_eq!(pays, 1, "seed {seed}, day {day}, {pulse:?}");
            }
        }
        assert!(
            worked.stock[1] > 0,
            "seed {seed}: airfield B never runs dry"
        );
    }
    eprintln!("the pulse in which airfield A's stock reaches 0: {distribution:?}");
    eprintln!("runs with no bomber on one of the first three days: {raidless_early}");
    // The first possible pulse is the morning of day 4 or the night before it.
    assert!(distribution.keys().all(|key| key == "never"
        || key.starts_with("day 4")
        || key.starts_with("day 5")
        || key.starts_with("day 6")));
}

#[test]
fn at_the_end_of_a_run_supply_is_what_was_started_with_less_what_was_paid_and_destroyed() {
    for run in batch() {
        let seed = run.seed;
        // Airfield A's supply is 150 t less what it paid by the day and for
        // sorties, and airfield B's is 1,000 t less what it paid and less 50 t
        // for each bomb that landed on supply; neither is below 0.
        let paid_a = run.paid(AIRFIELD_A, Some(SupplyUse::Upkeep))
            + run.paid(AIRFIELD_A, Some(SupplyUse::Sorties));
        assert_eq!(run.supply[0], Fixed::from_int(150) - paid_a, "seed {seed}");
        assert_eq!(run.paid(AIRFIELD_A, Some(SupplyUse::Repairs)), Fixed::ZERO);
        let paid_b = run.paid(AIRFIELD_B, None);
        let bombs_on_supply: i64 = run
            .of(|k| matches!(k, EventKind::BaseAttacked { .. }))
            .iter()
            .map(|event| match event.kind {
                EventKind::BaseAttacked { hits_on_supply, .. } => i64::from(hits_on_supply),
                _ => unreachable!(),
            })
            .sum();
        // Airfield B never has less than 50 t for a bomb, so each one
        // destroys its whole 50 t.
        assert_eq!(
            run.supply[1],
            Fixed::from_int(1000) - paid_b - Fixed::from_int(50 * bombs_on_supply),
            "seed {seed}"
        );
        assert!(run.supply[0] >= Fixed::ZERO && run.supply[1] >= Fixed::ZERO);
    }
}

// ---------------------------------------------------------------------
// The variants.
// ---------------------------------------------------------------------

#[test]
fn with_no_supply_file_nothing_is_spent_and_the_first_three_days_are_scenario_6a() {
    let no_file = without_the_supply_file();
    assert!(no_file.world.ref_data().air_supply.is_none());
    let reference_scenario = six_a();
    let cfg = Cfg {
        supplied: false,
        ..scenario_cfg()
    };
    let runs = batch_of(&no_file, VARIANT_SEEDS);
    work_all(&runs, &cfg);
    for run in &runs {
        let seed = run.seed;
        assert!(run.spent().is_empty(), "seed {seed}");
        assert_conserved(run, cfg.start);
        assert_eq!(run.paid(AIRFIELD_A, None), Fixed::ZERO);
        // Event for event, days 1 to 3 are scenario 6a's with the same seed:
        // same kinds, breakdowns and causes, numbered the same.
        let reference = run_of(&reference_scenario, seed);
        let ours: Vec<&Event> = run.events.iter().filter(|e| e.day <= 3).collect();
        assert_eq!(ours.len(), reference.events.len(), "seed {seed}");
        for (a, b) in ours.iter().zip(&reference.events) {
            assert_eq!(
                (
                    a.id,
                    a.day,
                    a.pulse,
                    &a.kind,
                    a.breakdown.clone(),
                    &a.causes
                ),
                (
                    b.id,
                    b.day,
                    b.pulse,
                    &b.kind,
                    b.breakdown.clone(),
                    &b.causes
                ),
                "seed {seed}"
            );
        }
    }
    // A base with no stock is not grounded without the figures: airfield A at
    // 0 t flies as in 6a.
    let at_zero = variant("no_file_a_zero", &|text| {
        let text = once(text, SUPPLY_LINE, "");
        once(text, "supply_t = 150", "supply_t = 0")
    });
    let run = run_of(&at_zero, 1);
    assert!(run.raids() >= 3);
    assert!(run.spent().is_empty());
}

#[test]
fn if_airfield_a_has_1000_t_it_is_never_out_of_supply_and_a_raid_flies_each_morning_with_a_bomber()
{
    let variant = airfield_a_has(1000);
    let cfg = Cfg {
        start: [100_000, 100_000],
        ..scenario_cfg()
    };
    let runs = batch_of(&variant, VARIANT_SEEDS);
    let worked = work_all(&runs, &cfg);
    for (run, worked) in runs.iter().zip(&worked) {
        let seed = run.seed;
        assert_conserved(run, cfg.start);
        assert!(worked.dry.is_none(), "seed {seed}");
        let mornings_with_a_bomber = worked
            .days
            .iter()
            .filter(|d| d.able_morning[of(BOMBERS)] >= 1)
            .count();
        assert_eq!(run.raids(), mornings_with_a_bomber, "seed {seed}");
        assert!(worked
            .days
            .iter()
            .all(|d| !d.grounded_morning[0] && !d.grounded_afternoon[0]));
        assert!(run.supply[0] > Fixed::ZERO, "seed {seed}");
    }
}

#[test]
fn if_airfield_a_has_no_supply_it_flies_nothing_pays_nothing_and_airfield_b_is_never_attacked() {
    let variant = airfield_a_has(0);
    let cfg = Cfg {
        start: [0, 100_000],
        ..scenario_cfg()
    };
    let runs = batch_of(&variant, VARIANT_SEEDS);
    let worked = work_all(&runs, &cfg);
    for (run, worked) in runs.iter().zip(&worked) {
        let seed = run.seed;
        assert_conserved(run, cfg.start);
        assert_eq!(run.raids(), 0, "seed {seed}");
        assert!(run
            .events
            .iter()
            .all(|e| !matches!(actor(&e.kind), Some(1) | Some(2))));
        assert!(run
            .events
            .iter()
            .all(|e| !matches!(e.kind, EventKind::BaseAttacked { .. })));
        assert!(
            run.spent().iter().all(|row| row.2 == 2),
            "seed {seed}: only airfield B pays"
        );
        assert_eq!(run.supply[0], Fixed::ZERO);
        assert!(worked.dry.is_none(), "it started with none");
        // Airfield B pays for its search and patrol in each daylight pulse: 6
        // scouts x 2 t and 12 fighters x 0.5 t, and 28 aircraft x 0.2 t a
        // night. It loses no aircraft.
        assert_eq!(run.ready, [18, 12, 12, 10, 6], "seed {seed}");
        let sorties: Vec<_> = run
            .spent()
            .into_iter()
            .filter(|row| row.3 == SupplyUse::Sorties)
            .collect();
        assert_eq!(sorties.len(), 12, "seed {seed}");
        assert!(sorties
            .iter()
            .all(|row| (row.4, row.5, row.6) == (18, fx(1800), fx(1800))));
        assert_eq!(
            run.supply[1],
            Fixed::from_int(1000) - fx(560 * 6) - fx(1800 * 12),
            "seed {seed}"
        );
        assert_eq!(
            run.events
                .iter()
                .filter(|e| matches!(e.kind, EventKind::SearchFlown { .. }))
                .count(),
            12
        );
    }
}

#[test]
fn if_airfield_a_has_45_t_the_first_morning_costs_all_of_it_and_it_flies_nothing_from_that_afternoon(
) {
    let variant = airfield_a_has(45);
    let cfg = Cfg {
        start: [4500, 100_000],
        ..scenario_cfg()
    };
    let runs = batch_of(&variant, VARIANT_SEEDS);
    let worked = work_all(&runs, &cfg);
    for (run, worked) in runs.iter().zip(&worked) {
        let seed = run.seed;
        assert_conserved(run, cfg.start);
        let a: Vec<_> = run.spent().into_iter().filter(|row| row.2 == 1).collect();
        // Night: 6 t, 39 t left. The first morning costs exactly 39 t: 0 left.
        assert_eq!(
            a,
            vec![
                (
                    1,
                    Pulse::Night,
                    1,
                    SupplyUse::Upkeep,
                    30,
                    fx(600),
                    fx(600),
                    fx(3900)
                ),
                (
                    1,
                    Pulse::Morning,
                    1,
                    SupplyUse::Sorties,
                    30,
                    fx(3900),
                    fx(3900),
                    Fixed::ZERO
                ),
            ],
            "seed {seed}"
        );
        let event = run
            .events
            .iter()
            .find(|e| matches!(e.kind, EventKind::BaseSupplySpent { base, reason: SupplyUse::Sorties, .. } if base == AIRFIELD_A))
            .expect("a payment");
        assert!(event
            .kind
            .to_string()
            .ends_with("0 t left: base:1 is out of supply"));
        assert!(!event.kind.to_string().contains("could not cover"));
        assert_eq!(worked.dry, Some((1, Pulse::Morning)), "seed {seed}");
        // One raid, and nothing from groups 1 and 2 from the afternoon on.
        assert_eq!(run.raids(), 1, "seed {seed}");
        assert!(worked.days[0].grounded_afternoon[0]);
        for day in 1..=DAYS {
            for pulse in [Pulse::Morning, Pulse::Afternoon] {
                if (day, pulse) == (1, Pulse::Morning) {
                    continue;
                }
                assert!(
                    run.pulse(day, pulse)
                        .iter()
                        .all(|e| !matches!(actor(&e.kind), Some(1) | Some(2))),
                    "seed {seed}"
                );
            }
        }
    }
}

#[test]
fn if_airfield_a_has_20_t_the_first_raid_flies_in_full_and_the_base_pays_the_14_t_it_has() {
    let variant = airfield_a_has(20);
    let cfg = Cfg {
        start: [2000, 100_000],
        ..scenario_cfg()
    };
    let runs = batch_of(&variant, VARIANT_SEEDS);
    let worked = work_all(&runs, &cfg);
    for (run, worked) in runs.iter().zip(&worked) {
        let seed = run.seed;
        assert_conserved(run, cfg.start);
        let a: Vec<_> = run.spent().into_iter().filter(|row| row.2 == 1).collect();
        assert_eq!(
            a,
            vec![
                (
                    1,
                    Pulse::Night,
                    1,
                    SupplyUse::Upkeep,
                    30,
                    fx(600),
                    fx(600),
                    fx(1400)
                ),
                (
                    1,
                    Pulse::Morning,
                    1,
                    SupplyUse::Sorties,
                    30,
                    fx(3900),
                    fx(1400),
                    Fixed::ZERO
                ),
            ],
            "seed {seed}"
        );
        let event = run
            .events
            .iter()
            .find(|e| matches!(e.kind, EventKind::BaseSupplySpent { base, reason: SupplyUse::Sorties, .. } if base == AIRFIELD_A))
            .expect("a payment");
        assert_eq!(
            event.kind.to_string(),
            "base:1 spent 14 t of supply on 30 aircraft sent on sorties: 39 t was asked and the stock could not cover it, 0 t left: base:1 is out of supply"
        );
        // The raid flew in full: 18 bombers and 6 escorts.
        let d = &worked.days[0];
        assert_eq!((d.bombers, d.escorts, d.kept), (18, 6, 6));
        assert_eq!(run.raids(), 1, "seed {seed}");
        assert_eq!(worked.dry, Some((1, Pulse::Morning)));
    }
}

#[test]
fn if_airfield_b_has_10_t_it_flies_the_first_morning_in_full_and_nothing_after_it() {
    let variant = airfield_b_has(10);
    let cfg = Cfg {
        start: [15_000, 1000],
        ..scenario_cfg()
    };
    let runs = batch_of(&variant, VARIANT_SEEDS);
    let worked = work_all(&runs, &cfg);
    let mut raids_on_day_2 = 0;
    for (run, worked) in runs.iter().zip(&worked) {
        let seed = run.seed;
        assert_conserved(run, cfg.start);
        // Night: 5.6 t, 4.4 t left.
        let night = &run.spent()[1];
        assert_eq!(
            *night,
            (
                1,
                Pulse::Night,
                2,
                SupplyUse::Upkeep,
                28,
                fx(560),
                fx(560),
                fx(440)
            ),
            "seed {seed}"
        );
        // It flies the first morning in full: 6 scouts search and 12 fighters
        // patrol, and it pays what is left after any hit on its supply.
        let d1 = &worked.days[0];
        assert_eq!(d1.able_morning[of(SCOUTS)], 6);
        assert_eq!(d1.patrol, 12);
        let morning = run
            .spent()
            .into_iter()
            .find(|row| (row.0, row.1, row.2) == (1, Pulse::Morning, 2))
            .expect("a payment");
        assert_eq!((morning.4, morning.5), (18, fx(1800)), "seed {seed}");
        assert!(
            morning.6 <= fx(440) && morning.7 == Fixed::ZERO,
            "seed {seed}"
        );
        // From that afternoon on it flies nothing: no search, no patrol.
        assert!(d1.grounded_afternoon[1]);
        assert_eq!(worked.stock[1], 0);
        for day in 1..=DAYS {
            for pulse in [Pulse::Morning, Pulse::Afternoon] {
                if (day, pulse) == (1, Pulse::Morning) {
                    continue;
                }
                let step = run.pulse(day, pulse);
                assert!(
                    step.iter()
                        .all(|e| !matches!(e.kind, EventKind::SearchFlown { .. })),
                    "seed {seed}, day {day}: no search"
                );
                assert!(
                    step.iter().all(|e| !matches!(e.kind, EventKind::AirFight { firer, .. } if firer.raw() == DEFENDERS)),
                    "seed {seed}, day {day}: no fighter fires"
                );
            }
        }
        // The raid of day 2 meets no fighters, the guns still fire 20 shots,
        // and every ready aircraft of groups 3, 4 and 5 is on the ground,
        // destroyed in that order.
        let d2 = &worked.days[1];
        if d2.bombers >= 1 {
            raids_on_day_2 += 1;
            let morning = run.pulse(2, Pulse::Morning);
            assert!(morning
                .iter()
                .all(|e| !matches!(e.kind, EventKind::AirFight { .. })));
            let guns = morning
                .iter()
                .filter(|e| matches!(e.kind, EventKind::BaseAntiAircraftFire { shots, .. } if shots == Fixed::from_int(20)))
                .count();
            assert_eq!(guns, 1, "seed {seed}");
            let groups: Vec<u32> = morning
                .iter()
                .filter_map(|e| match e.kind {
                    EventKind::AircraftDestroyedOnGround { air_group, .. } => Some(air_group.raw()),
                    _ => None,
                })
                .collect();
            let mut sorted = groups.clone();
            sorted.sort_unstable();
            assert_eq!(groups, sorted, "seed {seed}: groups 3, 4, 5 in order");
            // Everything that was ready at B is on the ground.
            assert_eq!(
                d2.ground_before[of(DEFENDERS)],
                worked.days[0].ready_end[of(DEFENDERS)]
            );
            assert_eq!(
                d2.ground_before[of(RESTING)],
                worked.days[0].ready_end[of(RESTING)]
            );
            assert_eq!(
                d2.ground_before[of(SCOUTS)],
                worked.days[0].ready_end[of(SCOUTS)]
            );
        }
    }
    assert!(raids_on_day_2 > 250, "{raids_on_day_2} raids on day 2");
}

#[test]
fn if_airfield_b_is_at_2_0_a_bomber_or_an_escort_costs_0_9_t() {
    let variant = airfield_b_at_2_0();
    let cfg = Cfg {
        raid_hexes: 2,
        ..scenario_cfg()
    };
    assert_eq!(cfg.raid_cost(), 90);
    let runs = batch_of(&variant, VARIANT_SEEDS);
    let worked = work_all(&runs, &cfg);
    for (run, worked) in runs.iter().zip(&worked) {
        let seed = run.seed;
        assert_conserved(run, cfg.start);
        // The first morning: 24 x 0.9 + 6 x 0.5 = 24.6 t.
        let morning = run
            .spent()
            .into_iter()
            .find(|row| (row.0, row.1, row.2) == (1, Pulse::Morning, 1))
            .expect("a payment");
        assert_eq!(
            (morning.4, morning.5, morning.6),
            (30, fx(2460), fx(2460)),
            "seed {seed}"
        );
        assert!(worked.raids >= 1);
    }
}

#[test]
fn with_the_wear_figures_and_airfield_b_repairing_3_a_day_a_repair_costs_1_t_and_comes_before_the_daily_cost(
) {
    let variant = with_wear_and_repairs("repair_1t", &|text| text, &|text| text);
    let data = variant.world.ref_data();
    assert!(data.air_upkeep.is_some() && data.air_supply.is_some());
    assert_eq!(
        variant.world.umpire().bases[&AIRFIELD_B].aircraft_repairs_per_day,
        3
    );
    let runs = batch_of(&variant, VARIANT_SEEDS);
    let mut repairs_paid = 0;
    for run in &runs {
        repairs_paid += check_repairs(run, fx(100), [15_000, 100_000]);
    }
    assert!(repairs_paid > 100, "{repairs_paid} repairs paid for");
}

/// With the wear figures and 3 repairs a day at airfield B (decision 3 of
/// 0039), asserts night by night, from the events alone, that: a base repairs
/// the damaged aircraft of its groups up to its 3 a day and only while its
/// stock covers a repair of `repair` hundredths of a ton; that it pays for
/// each, in one `Repairs` sum after its repair events and citing them; that
/// the daily cost follows, on the aircraft at the base, ready and damaged;
/// and that no repair is made or paid for anywhere else. Returns the repairs
/// paid for.
fn check_repairs(run: &Run, repair: Fixed, start: [i64; 2]) -> u32 {
    let seed = run.seed;
    let mut track = Track::new(start);
    let mut paid_for = 0;
    for day in 1..=DAYS {
        let night: Vec<&Event> = run.pulse(day, Pulse::Night);
        // What the night starts with.
        let before = track.clone();
        let repaired: Vec<&Event> = night
            .iter()
            .copied()
            .filter(|e| matches!(e.kind, EventKind::AircraftRepaired { .. }))
            .collect();
        let total: u32 = repaired
            .iter()
            .map(|e| match e.kind {
                EventKind::AircraftRepaired { repaired, at, .. } => {
                    assert_eq!(at, AirGroupLocation::Base(AIRFIELD_B), "seed {seed}");
                    repaired
                }
                _ => unreachable!(),
            })
            .sum();
        // Repaired: at most 3, at most the damaged aircraft at B, and at most
        // what the stock covers, and all of the least of them.
        let damaged_at_b: u32 = GROUPS
            .iter()
            .filter(|g| GROUP_BASE[of(**g)] == AIRFIELD_B)
            .map(|g| before.damaged[of(*g)])
            .sum();
        let affordable = (before.stock[1].mul_div(Fixed::ONE, repair)).trunc();
        let affordable = u32::try_from(affordable).expect("small");
        assert_eq!(
            total,
            3.min(damaged_at_b).min(affordable),
            "seed {seed}, day {day}: repairs"
        );
        // The events in order: repairs, their sum, then the daily cost.
        let spent: Vec<&Event> = night
            .iter()
            .copied()
            .filter(|e| matches!(e.kind, EventKind::BaseSupplySpent { .. }))
            .collect();
        let position = |event: &Event| {
            night
                .iter()
                .position(|e| e.id == event.id)
                .expect("in the night")
        };
        let mut expected_stock = before.stock;
        let mut index = 0;
        if total > 0 {
            let sum = spent[index];
            index += 1;
            let asked = repair * Fixed::from_int(i64::from(total));
            assert_eq!(
                sum.kind,
                EventKind::BaseSupplySpent {
                    base: AIRFIELD_B,
                    reason: SupplyUse::Repairs,
                    aircraft: total,
                    asked_t: asked,
                    paid_t: asked,
                    left_t: before.stock[1] - asked,
                },
                "seed {seed}, day {day}"
            );
            let causes: Vec<Cause> = repaired.iter().map(|e| Cause::Event(e.id)).collect();
            assert_eq!(sum.causes, causes, "seed {seed}: it cites the repairs");
            assert!(
                repaired.iter().all(|e| position(e) < position(sum)),
                "seed {seed}: repairs come before their sum"
            );
            expected_stock[1] = before.stock[1] - asked;
            paid_for += total;
        }
        // The daily cost: each base with a stock and aircraft, on the
        // aircraft there after the repairs.
        let mut after = before.clone();
        for event in &night {
            after.apply(event);
        }
        for (i, base) in [AIRFIELD_A, AIRFIELD_B].into_iter().enumerate() {
            let aircraft = before.aircraft_at(base);
            let asked = fx(20) * Fixed::from_int(i64::from(aircraft));
            if expected_stock[i] == Fixed::ZERO || aircraft == 0 {
                continue;
            }
            let event = spent[index];
            index += 1;
            let paid = asked.min(expected_stock[i]);
            assert_eq!(
                event.kind,
                EventKind::BaseSupplySpent {
                    base,
                    reason: SupplyUse::Upkeep,
                    aircraft,
                    asked_t: asked,
                    paid_t: paid,
                    left_t: expected_stock[i] - paid,
                },
                "seed {seed}, day {day}"
            );
            assert!(
                repaired.iter().all(|e| position(e) < position(event)),
                "seed {seed}: the daily cost comes after the repairs"
            );
        }
        assert_eq!(
            index,
            spent.len(),
            "seed {seed}, day {day}: nothing else spent"
        );
        // Advance the track over the whole day.
        for event in run.events.iter().filter(|e| e.day == day) {
            track.apply(event);
        }
    }
    paid_for
}

#[test]
fn with_a_repair_dearer_than_the_stock_can_cover_a_base_repairs_nothing_even_with_damaged_aircraft()
{
    // The scenario cannot leave a base with less than 1 t and damaged aircraft
    // (a night costs it more than that), so the figure is scaled: a repair
    // costs 400 t, airfield B has 1,000 t, and the daily cost and the sorties
    // cost nothing. It can afford 2 repairs and then none, whatever is damaged.
    // The sim-core unit tests have the same case with 1 t and a stock of 0.5 t.
    let variant = with_wear_and_repairs("repair_400t", &|text| text, &|text| {
        let text = once(text, "sortie_supply_t = \"0.5\"", "sortie_supply_t = \"0\"");
        let text = once(
            text,
            "sortie_supply_t_per_100_nm = \"0.25\"",
            "sortie_supply_t_per_100_nm = \"0\"",
        );
        let text = once(
            text,
            "daily_supply_t_per_aircraft = \"0.2\"",
            "daily_supply_t_per_aircraft = \"0\"",
        );
        once(text, "repair_supply_t = \"1\"", "repair_supply_t = \"400\"")
    });
    let runs = batch_of(&variant, VARIANT_SEEDS);
    let mut repairs_paid = 0;
    let mut starved_nights = 0;
    for run in &runs {
        let seed = run.seed;
        // With no daily cost and no sortie cost, only repairs are paid.
        for row in run.spent() {
            assert_eq!(row.3, SupplyUse::Repairs, "seed {seed}");
        }
        let mut track = Track::new([15_000, 100_000]);
        let repair = Fixed::from_int(400);
        for day in 1..=DAYS {
            let night = run.pulse(day, Pulse::Night);
            let damaged_at_b: u32 = GROUPS
                .iter()
                .filter(|g| GROUP_BASE[of(**g)] == AIRFIELD_B)
                .map(|g| track.damaged[of(*g)])
                .sum();
            let affordable =
                u32::try_from(track.stock[1].mul_div(Fixed::ONE, repair).trunc()).expect("small");
            let done: u32 = night
                .iter()
                .filter_map(|e| match e.kind {
                    EventKind::AircraftRepaired { repaired, .. } => Some(repaired),
                    _ => None,
                })
                .sum();
            assert_eq!(
                done,
                3.min(damaged_at_b).min(affordable),
                "seed {seed}, day {day}"
            );
            if damaged_at_b > 0 && affordable == 0 {
                // Damaged aircraft, and a stock below the price of a repair:
                // nothing is repaired and nothing is recorded.
                starved_nights += 1;
                assert!(night.is_empty(), "seed {seed}, day {day}");
            }
            repairs_paid += done;
            for event in run.events.iter().filter(|e| e.day == day) {
                track.apply(event);
            }
        }
        // Never below 0, and never more repairs than 1,000 / 400 allows.
        assert!(run.supply[1] >= Fixed::ZERO);
        let total: u32 = run
            .of(|k| matches!(k, EventKind::AircraftRepaired { .. }))
            .iter()
            .map(|e| match e.kind {
                EventKind::AircraftRepaired { repaired, .. } => repaired,
                _ => unreachable!(),
            })
            .sum();
        assert!(total <= 2, "seed {seed}: {total} repairs");
    }
    assert!(repairs_paid > 0, "some repairs were made");
    assert!(
        starved_nights > 0,
        "some night had damaged aircraft and too little supply"
    );
}

// ---------------------------------------------------------------------
// What cannot happen, and the properties.
// ---------------------------------------------------------------------

#[test]
fn what_cannot_happen_does_not_in_any_run() {
    for run in batch() {
        let seed = run.seed;
        let mut track = Track::new(scenario_cfg().start);
        // The stocks at the start of the pulse being read.
        let mut pulse_start = track.stock;
        let mut current: Option<(u32, Pulse)> = None;
        for event in run.events.iter().filter(|e| e.pulse.is_some()) {
            let now = (event.day, event.pulse.expect("a pulse"));
            if current != Some(now) {
                current = Some(now);
                pulse_start = track.stock;
            }
            let daylight = now.1 != Pulse::Night;
            match event.kind {
                // No stock is below 0.
                EventKind::BaseSupplySpent {
                    reason,
                    paid_t,
                    asked_t,
                    left_t,
                    ..
                } => {
                    assert!(
                        left_t >= Fixed::ZERO && paid_t >= Fixed::ZERO,
                        "seed {seed}"
                    );
                    assert!(paid_t <= asked_t, "seed {seed}");
                    // No base pays for a sortie in a night pulse, or by the
                    // day in a daylight pulse.
                    assert_eq!(reason == SupplyUse::Sorties, daylight, "seed {seed}");
                    assert_eq!(reason == SupplyUse::Upkeep, !daylight, "seed {seed}");
                    assert_ne!(reason, SupplyUse::Repairs, "no repairs without wear");
                    // A base pays for sorties only if it flew: with a stock at
                    // the start of the pulse.
                    let base = match event.kind {
                        EventKind::BaseSupplySpent { base, .. } => base,
                        _ => unreachable!(),
                    };
                    if reason == SupplyUse::Sorties {
                        assert!(
                            pulse_start[usize::from(base == AIRFIELD_B)] > Fixed::ZERO,
                            "seed {seed}: {base} paid for sorties with no supply at the start"
                        );
                    }
                }
                // A base with no supply at the start of a daylight pulse sends
                // no aircraft in it.
                EventKind::SearchFlown { air_group, .. }
                | EventKind::BaseStrikeLaunched { air_group, .. }
                | EventKind::EscortsSent { air_group, .. } => {
                    assert!(daylight, "seed {seed}");
                    let i = at(air_group.raw());
                    assert!(
                        pulse_start[i] > Fixed::ZERO,
                        "seed {seed}: {air_group} sends aircraft from a base with no supply"
                    );
                }
                // A patrol over a base with no supply is not there.
                EventKind::AirFight {
                    firer,
                    firing: AirFirer::Patrol,
                    ..
                } => {
                    assert!(
                        pulse_start[at(firer.raw())] > Fixed::ZERO,
                        "seed {seed}: {firer} patrols a base with no supply"
                    );
                }
                _ => {}
            }
            track.apply(event);
            assert!(track.stock[0] >= Fixed::ZERO && track.stock[1] >= Fixed::ZERO);
        }
    }
}

#[test]
fn more_supply_never_means_fewer_raids() {
    let none = batch_of(&airfield_a_has(0), SEEDS);
    let some = batch();
    let plenty = batch_of(&airfield_a_has(1000), SEEDS);
    let (mut none_total, mut some_total, mut plenty_total) = (0, 0, 0);
    for ((none, some), plenty) in none.iter().zip(some).zip(&plenty) {
        assert_eq!(none.seed, some.seed);
        assert!(none.raids() <= some.raids(), "seed {}", some.seed);
        assert!(some.raids() <= plenty.raids(), "seed {}", some.seed);
        none_total += none.raids();
        some_total += some.raids();
        plenty_total += plenty.raids();
    }
    eprintln!(
        "raids over 1000 runs: no supply {none_total}, 150 t {some_total}, 1000 t {plenty_total}"
    );
    assert_eq!(none_total, 0);
    assert!(some_total > 0 && plenty_total > some_total);
}

#[test]
fn a_longer_sortie_never_costs_less() {
    let scenario = load();
    let figures = scenario.world.ref_data().air_supply.expect("figures");
    let mut last = Fixed::ZERO;
    for nm in 0..=1200 {
        let cost = figures.sortie_cost(Fixed::from_int(nm));
        assert!(cost >= last, "{nm} nm");
        last = cost;
    }
    let costs: Vec<Fixed> = [0, 160, 400, 600]
        .iter()
        .map(|nm| figures.sortie_cost(Fixed::from_int(*nm)))
        .collect();
    assert_eq!(costs, vec![fx(50), fx(90), fx(150), fx(200)]);
}

#[test]
fn the_aircraft_at_the_end_are_those_at_the_start_less_the_recorded_losses() {
    // Asserted for every run by `assert_conserved`; here the records agree.
    for (run, worked) in batch().iter().zip(worked_batch()) {
        for g in GROUPS {
            let i = of(g);
            assert_eq!(
                START[i],
                worked.ready[i] + worked.shot_down[i] + worked.on_the_ground[i],
                "seed {}, group {g}",
                run.seed
            );
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
fn a_scenario_that_names_no_supply_file_has_the_hashes_it_had_before_this_stage() {
    // The golden files of the command-line tests are those of the code before
    // base supply existed: every earlier scenario, run with its own seed, has
    // the day hashes they hold.
    let golden =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../crates/sim-cli/tests/run_baseline");
    let mut checked = 0;
    let mut names: Vec<String> = fs::read_dir(&golden)
        .expect("the golden folder")
        .filter_map(|entry| {
            let name = entry.expect("an entry").file_name().into_string().ok()?;
            let stem = name.strip_suffix(".txt")?;
            (!stem.ends_with(".events")
                && !stem.starts_with("06d2")
                // Scenario 6 itself has pilots, replacements and supply.
                && !stem.starts_with("06_"))
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
        let mut scenario = Scenario::load(&path).expect("the scenario loads");
        assert!(scenario.world.ref_data().air_supply.is_none(), "{name}");
        scenario.run_to_end().expect("runs");
        assert_eq!(scenario.world.day_hashes(), wanted, "{name}");
        checked += 1;
    }
    assert!(checked >= 16, "{checked} golden files checked");
}

#[test]
fn mean_figures_of_the_scenario_over_1000_seeds() {
    // Not an assertion about the rule: the figures a reader wants, printed
    // with --nocapture.
    let worked = worked_batch();
    let total = count(worked.len());
    let mean_x100 = |sum: i64| sum * 100 / total;
    let raids: i64 = worked.iter().map(|w| i64::from(w.raids)).sum();
    eprintln!("raids per run x100: {}", mean_x100(raids));
    for (i, name) in ["A", "B"].into_iter().enumerate() {
        let left: i64 = worked.iter().map(|w| w.stock[i]).sum();
        eprintln!(
            "airfield {name}: supply left at the end, hundredths of a ton x100: {}",
            mean_x100(left)
        );
        for (r, reason) in REASONS.iter().enumerate() {
            let paid: i64 = worked.iter().map(|w| w.paid[i][r]).sum();
            eprintln!(
                "airfield {name}: paid for {} per run, hundredths of a ton x100: {}",
                reason.name(),
                mean_x100(paid)
            );
        }
    }
    let destroyed: i64 = batch()
        .iter()
        .map(|run| run.destroyed(AIRFIELD_B).trunc())
        .sum();
    eprintln!(
        "airfield B: tons destroyed per run x100: {}",
        mean_x100(destroyed * 100)
    );
    for g in GROUPS {
        let ready: i64 = worked.iter().map(|w| i64::from(w.ready[of(g)])).sum();
        eprintln!("group {g}: aircraft at the end x100: {}", mean_x100(ready));
    }
    for day in 0..6usize {
        let a: i64 = worked.iter().map(|w| w.days[day].stock_end[0]).sum();
        eprintln!(
            "day {}: airfield A stock at the end, hundredths x100: {}",
            day + 1,
            mean_x100(a)
        );
    }
}
