//! Scenario 27: five forces at one port. One prepares its landing for ten
//! days, one does not prepare at all, one prepares for one beach and lands
//! at another, one changes its landing place, and one takes on a second
//! unit.
//!
//! Proves decision 0095 (preparing a landing) against a hand calculation.
//! The data is placeholder data: see
//! `data/scenarios/27_preparing_a_landing.toml`. The frozen step 1 figures
//! switch this rule off (`unprepared_landing_share` 1,
//! `unprepared_landing_fatigue` 0), so this test gives the four figures the
//! values below, in the way the experience test gives a strength above 0.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Nothing here is decided by
//! dice. The tests assert these numbers exactly. If the code disagrees, the
//! code or the rule is wrong: do not change a number here to make a test
//! pass. A mismatch goes back to the architect.
//!
//! ## Figures
//!
//! `preparation_per_day` 5, `unprepared_landing_share` 0.5,
//! `preparation_kept_on_change` 0.5, `unprepared_landing_fatigue` 40. From
//! the earlier stages: `handling_t_per_man` 0.25, `daily_supply_t_per_man`
//! 0.005, `unfed_loss_share_per_day` 0.05, `rest_per_day` 5. A port of size
//! 0 works cargo by lighter at 20 t an hour.
//!
//! ## Inputs
//!
//! A placeholder map with three coast hexes: A at (0,0) with port size 5, B
//! at (4,0) with port size 0 and C at (0,6) with port size 0. The game is 12
//! days. Day d begins at hour 24 x (d - 1): day 11 at hour 240 and day 12 at
//! hour 264. The night pulse is 12 hours and each daylight pulse 6.
//!
//! All is Allied. Base 1 at A holds 1000 t of supply. Base 2 at B holds 100
//! t. There is no base at C.
//!
//! Classes: the placeholder troop transport (600 men, 10 kn at cruise, 0.1 t
//! of fuel a mile, 1000 t of fuel) and the placeholder fast transport (200
//! men, 16 kn at cruise, 0.125 t a mile, 500 t of fuel).
//!
//! | Force | Ship                | Unit aboard at the start       |
//! | ----- | ------------------- | ------------------------------ |
//! | 1     | 1, troop transport, with 60 t of supply | Unit 1, 600 men |
//! | 2     | 2, troop transport  | Unit 2, 600 men                |
//! | 3     | 3, fast transport   | Unit 3, 200 men                |
//! | 4     | 4, troop transport  | Unit 4, 600 men                |
//! | 5     | 5, troop transport  | Unit 5, 300 men                |
//!
//! Unit 6, 300 men, is ashore at A. Every force starts at base 1, full of
//! fuel. Every unit starts with fatigue 0.
//!
//! Orders:
//!
//! - Day 1: forces 1, 3, 4 and 5 are each ordered to prepare a landing at B,
//!   with the unit aboard as the roster. Force 2 is given no such order.
//! - Day 3: force 1 is given the same order again.
//! - Day 6: force 4 is ordered to prepare a landing at C, same roster. Force
//!   5 is ordered to prepare a landing at B with units 5 and 6 as its
//!   roster.
//! - Day 11: forces 1 and 2 are sent to B and force 3 to C, each at cruise,
//!   each with an unload order.
//!
//! Loading and rehearsal grow in the ground combat phase of each day, for a
//! force stopped at a base of its side then. A force that sailed that day is
//! at sea and gains nothing.
//!
//! ## What is prepared, day by day
//!
//! | After the gain of day | Force 1 and unit 1 | Force 3 and unit 3 | Force 4 and unit 4 | Force 5 | Unit 5 | Unit 6 |
//! | --------------------- | ------------------ | ------------------ | ------------------ | ------- | ------ | ------ |
//! | 5                     | 25                 | 25                 | 25, for B          | 25      | 25     | 0      |
//! | 6                     | 30                 | 30                 | 5, for C           | 17.5    | 30     | 5      |
//! | 10                    | 50                 | 50                 | 25                 | 37.5    | 50     | 25     |
//! | 12                    | 50, until it lands | 50, until it lands | 35                 | 47.5    | 60     | 35     |
//!
//! - Force 1's second order, on day 3, is the same order: nothing changes.
//! - Force 4, day 6: a new place loses everything. Its loading and unit 4's
//!   rehearsal go from 25 to 0 at intake and gain 5 that day. Unit 4's
//!   rehearsal is for C from then on.
//! - Force 5, day 6: a unit added keeps half the loading, 25 x 0.5 = 12.5,
//!   and it gains 5 that day. Unit 5 keeps its 25. Unit 6 starts at 0. Unit
//!   6 is ashore at A, in the force's hex, and gains with the rest.
//! - Force 2 and unit 2 have 0 throughout.
//!
//! ## The landings
//!
//! Forces 1 and 2: A to B is 4 hexes, 160 nm, at 10 kn: 16 hours. They
//! arrive at hour 256, 2 hours before the end of the morning pulse of day
//! 11. Each ship burns 16 t of fuel and has 984 t.
//!
//! Force 1 has loading 50 for B: its rate is 20 x (0.5 + 0.5 x 50/100) = 15
//! t an hour. Force 2 has none: 20 x 0.5 = 10 t an hour.
//!
//! | Pulse             | Hours | Force 1 at 15 t an hour            | Force 2 at 10 t an hour  |
//! | ----------------- | ----- | ---------------------------------- | ------------------------ |
//! | Day 11, morning   | 2     | 30 t: 120 men                      | 20 t: 80 men             |
//! | Day 11, afternoon | 6     | 90 t: 360 men                      | 60 t: 240 men            |
//! | Day 12, night     | 12    | 120 men take 30 t; then the 60 t of supply | 280 men take 70 t |
//!
//! - Unit 1's first men land with a rehearsal of 50 for B: it gains 40 x
//!   50/100 = 20 fatigue, and its rehearsal is 0.
//! - Unit 2's first men land with none: it gains 40.
//! - Force 1's supply is landed at 15 t an hour after its last man is
//!   ashore: the loading holds for the whole unloading. It is complete 90 /
//!   15 = 6 hours into the night pulse of day 12, at hour 270. Its loading
//!   is then 0 and its order is done.
//! - Force 2 is complete 70 / 10 = 7 hours into that pulse, at hour 271.
//!
//! Force 3: A to C is 6 hexes, 240 nm, at 16 kn: 15 hours. It arrives at
//! hour 255, 3 hours before the end of the morning pulse of day 11. Ship 3
//! burns 30 t and has 470 t. Its loading of 50 is for B and it lands at C:
//! it works at 10 t an hour.
//!
//! - Day 11, morning: 3 hours, 30 t, 120 men. The first man forms base 3 at
//!   C. Unit 3's rehearsal of 50 is for B and counts as none at C: it gains
//!   40 fatigue, and its rehearsal is 0.
//! - Day 11, afternoon: the last 80 men take 20 t. Complete 2 hours into the
//!   pulse, at hour 260.
//! - Force 3 has not landed at B: its loading is still 50 and its order
//!   stands.
//!
//! ## Ashore
//!
//! Fatigue. A unit ashore that neither attacks nor marches rests 5 a day in
//! the ground combat phase.
//!
//! | Unit | On landing, day 11 | After day 11 | After day 12 |
//! | ---- | ------------------ | ------------ | ------------ |
//! | 1    | 20                 | 15           | 10           |
//! | 2    | 40                 | 35           | 30           |
//! | 3    | 40                 | 35           | 30           |
//!
//! Supply. Base 1 feeds unit 6 every night: 300 x 0.005 = 1.5 t, twelve
//! times, and holds 982 t at the end.
//!
//! Base 2, night of day 12, charged before that pulse's loading: 480 men of
//! unit 1 and 320 of unit 2 are ashore, 800 x 0.005 = 4 t. It holds 96 t,
//! and then 156 t when force 1's 60 t are landed.
//!
//! Base 3 at C has no stock. On the night of day 12 unit 3's 200 men are
//! unfed: 200 x 0.05 = 10 are lost and 190 are left.
//!
//! At the end: unit 1 has 600 men at B, unit 2 has 600 at B, unit 3 has 190
//! at C. Units 4 and 5 are aboard at A, and unit 6 is ashore there.
//!
//! # Properties
//!
//! - More days of preparation never land a force's men later.
//! - Loading and rehearsal are never above 100 nor below 0, and rehearsal is
//!   never copied: the rehearsal of all units is no more after a force
//!   splits or joins than before.
//! - With the frozen values of the two switches, every scenario with land
//!   units gives what it gave before, apart from the hash version.
//! - The enemy's view is the same in two worlds that differ only in a
//!   prepare order.

mod common;

use std::path::PathBuf;

use sim_core::events::{
    Cause, Event, EventKind, LandFatigueChange, LoadingLoss, RehearsalLoss, RosterLeft,
};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, EntityRef, EventId, LandUnitId, OrderId, ShipId, TaskForceId};
use sim_core::orders::RejectReason;
use sim_core::side_events::SideEventKind;
use sim_core::state::{Side, TruthState};
use sim_core::time::Pulse;
use sim_data::{LoadOptions, Scenario};

// ------------------------------------------------------------------ Helpers

/// A is at (0,0), B at (4,0) and C at (0,6). Why C is on an axis:
/// `data/map/placeholder_preparing.toml`.
const A: Hex = Hex::new(0, 0);
const B: Hex = Hex::new(4, 0);
const C: Hex = Hex::new(0, 6);

fn path() -> PathBuf {
    common::data_dir().join("scenarios/27_preparing_a_landing.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

/// The frozen figures of step 1 with the two switches of the rule turned on:
/// `unprepared_landing_share` 0.5 and `unprepared_landing_fatigue` 40. The
/// frozen `preparation_per_day` 5 and `preparation_kept_on_change` 0.5 are
/// the calculation's values already.
fn working() -> LoadOptions {
    common::altered_reference("preparing", |dir| {
        common::edit_file_by(
            dir,
            "ground.toml",
            &[
                (
                    "unprepared_landing_share = 1",
                    "unprepared_landing_share = \"0.5\"",
                ),
                (
                    "unprepared_landing_fatigue = 0",
                    "unprepared_landing_fatigue = 40",
                ),
            ],
        );
    })
}

fn from(source: &str) -> Scenario {
    common::from_text_with(source, &path(), &working()).expect("the variant loads")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

fn unit(raw: u32) -> LandUnitId {
    LandUnitId::new(raw)
}

fn force(raw: u32) -> TaskForceId {
    TaskForceId::new(raw)
}

fn base(raw: u32) -> BaseId {
    BaseId::new(raw)
}

/// What the Japanese side may see, as text: its own numbers, so equal in two
/// worlds if what it sees is the same.
fn digest(scenario: &Scenario, side: Side) -> String {
    let view = scenario.world.side_view(side);
    let mut out = String::new();
    for u in view.land_units() {
        out += &format!("{u:?}\n");
    }
    for b in view.bases() {
        out += &format!("{b:?}\n");
    }
    for e in view.ground_estimates() {
        out += &format!("{:?} {} {}\n", e.hex, e.men, e.day);
    }
    for b in view.enemy_bases() {
        out += &format!("{b:?}\n");
    }
    for e in view.events() {
        out += &format!(
            "{} {} {:?} {:?} {:?} {:?} {} {:?}\n",
            e.number,
            e.day,
            e.pulse,
            e.kind,
            e.order_number,
            e.causes,
            e.hidden_causes,
            e.breakdown
        );
    }
    out
}

/// A run of the scenario: the truth at the start (day 0) and at the end of
/// each day, the scenario at the end, and its log.
struct Run {
    scenario: Scenario,
    days: Vec<TruthState>,
    events: Vec<Event>,
}

impl Run {
    fn of(mut scenario: Scenario) -> Run {
        let mut days = vec![scenario.world.umpire().clone()];
        while scenario.next_day().expect("a day resolves").is_some() {
            days.push(scenario.world.umpire().clone());
        }
        let events = scenario.world.events().iter().cloned().collect();
        Run {
            scenario,
            days,
            events,
        }
    }

    fn text(source: &str) -> Run {
        Run::of(from(source))
    }

    fn len(&self) -> u32 {
        u32::try_from(self.days.len() - 1).expect("a few days")
    }

    /// The truth at the end of `day`; day 0 is the start.
    fn at(&self, day: u32) -> &TruthState {
        &self.days[day as usize]
    }

    /// The loading of a force at the end of `day`, 0 for a force with no
    /// preparation.
    fn loading(&self, day: u32, raw: u32) -> Fixed {
        self.at(day).task_forces[&force(raw)]
            .preparation
            .as_ref()
            .map_or(Fixed::ZERO, |p| p.loading)
    }

    fn rehearsal(&self, day: u32, raw: u32) -> Fixed {
        self.at(day).land_units[&unit(raw)].rehearsal
    }

    fn rehearsal_hex(&self, day: u32, raw: u32) -> Option<Hex> {
        self.at(day).land_units[&unit(raw)].rehearsal_hex
    }

    fn fatigue(&self, day: u32, raw: u32) -> Fixed {
        self.at(day).land_units[&unit(raw)].fatigue
    }

    fn supply(&self, day: u32, raw: u32) -> Fixed {
        self.at(day).bases[&base(raw)].supply_t
    }

    fn roster(&self, day: u32, raw: u32) -> Vec<u32> {
        self.at(day).task_forces[&force(raw)]
            .preparation
            .as_ref()
            .map_or_else(Vec::new, |p| p.roster.iter().map(|u| u.raw()).collect())
    }

    fn on(&self, day: u32, pick: impl Fn(&EventKind) -> bool) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|e| e.day == day && pick(&e.kind))
            .collect()
    }

    fn one(&self, day: u32, pick: impl Fn(&EventKind) -> bool) -> &Event {
        let found = self.on(day, pick);
        assert_eq!(
            found.len(),
            1,
            "day {day}: exactly one such event: {found:?}"
        );
        found[0]
    }

    fn all(&self, pick: impl Fn(&EventKind) -> bool) -> Vec<&Event> {
        self.events.iter().filter(|e| pick(&e.kind)).collect()
    }

    /// The order accepted on `day` for `target`: the one.
    fn accepted(&self, day: u32, target: EntityRef) -> OrderId {
        let e = self.one(
            day,
            |k| matches!(k, EventKind::OrderAccepted { target: t, .. } if *t == target),
        );
        match e.kind {
            EventKind::OrderAccepted { order, .. } => order,
            _ => unreachable!(),
        }
    }

    /// The refusals of `day`: target and reason.
    fn refusals(&self, day: u32) -> Vec<(EntityRef, RejectReason)> {
        self.on(day, |k| matches!(k, EventKind::OrderRejected { .. }))
            .iter()
            .map(|e| match &e.kind {
                EventKind::OrderRejected { target, reason, .. } => (*target, reason.clone()),
                _ => unreachable!(),
            })
            .collect()
    }

    /// The men landed in each pulse of each day: (day, pulse, unit, men), in
    /// log order.
    fn landings(&self) -> Vec<(u32, Pulse, u32, u32)> {
        self.all(|k| matches!(k, EventKind::MenLanded { .. }))
            .iter()
            .map(|e| match e.kind {
                EventKind::MenLanded { unit, men, .. } => {
                    (e.day, e.pulse.expect("in a pulse"), unit.raw(), men)
                }
                _ => unreachable!(),
            })
            .collect()
    }

    /// The hour (since the scenario began) a force's unloading was complete.
    fn completed_at(&self, raw: u32) -> Option<Fixed> {
        self.all(|k| matches!(k, EventKind::UnloadingComplete { task_force, .. } if task_force.raw() == raw))
            .first()
            .map(|e| match e.kind {
                EventKind::UnloadingComplete { elapsed_hours, .. } => elapsed_hours,
                _ => unreachable!(),
            })
    }

    fn arrived_at(&self, raw: u32) -> Fixed {
        let e = self.one(
            11,
            |k| matches!(k, EventKind::TaskForceArrived { task_force, .. } if task_force.raw() == raw),
        );
        match e.kind {
            EventKind::TaskForceArrived { elapsed_hours, .. } => elapsed_hours,
            _ => unreachable!(),
        }
    }
}

/// A factor of an event's breakdown.
fn factor(event: &Event, name: &str) -> Fixed {
    event
        .breakdown
        .as_ref()
        .expect("a breakdown")
        .factors
        .iter()
        .find(|f| f.name == name)
        .unwrap_or_else(|| panic!("a factor {name} in {event:?}"))
        .value
}

fn has_factor(event: &Event, name: &str) -> bool {
    event
        .breakdown
        .as_ref()
        .is_some_and(|b| b.factors.iter().any(|f| f.name == name))
}

fn caused_by_order(event: &Event, order: OrderId) -> bool {
    event.causes.contains(&Cause::Order(order))
}

fn caused_by_event(event: &Event, cause: EventId) -> bool {
    event.causes.contains(&Cause::Event(cause))
}

/// The kinds of the nine events of decisions 0095 and 0097 that no scenario
/// before 27 records (but `FightDayCounted` and `ExperienceRaised`, which a
/// fight records on any figures).
fn is_preparation_event(kind: &EventKind) -> bool {
    matches!(
        kind,
        EventKind::LoadingGained { .. }
            | EventKind::LoadingLost { .. }
            | EventKind::RehearsalGained { .. }
            | EventKind::RehearsalLost { .. }
            | EventKind::LeftRoster { .. }
            | EventKind::PreparationShared { .. }
    )
}

/// The text of a prepare order.
fn prepare(day: u32, task_force: u32, hex: Hex, roster: &str) -> String {
    format!(
        "[[order]]\nday = {day}\nside = \"allied\"\ntask_force = {task_force}\nkind = \"prepare_landing\"\nhex = [{}, {}]\nroster = {roster}\n",
        hex.q, hex.r
    )
}

fn without(source: &str, block: &str) -> String {
    common::replace_once(source, block, "")
}

fn with(source: &str, block: &str) -> String {
    format!("{source}\n{block}")
}

fn with_order(source: &str, day: u32, side: &str, target: &str, kind: &str, keys: &str) -> String {
    with(
        source,
        &format!(
            "[[order]]\nday = {day}\nside = \"{side}\"\n{target}\nkind = \"{kind}\"\n{keys}\n"
        ),
    )
}

/// The scenario with force 1's two orders to prepare taken out.
fn without_force_1_orders(source: &str) -> String {
    without(
        &without(source, &prepare(1, 1, B, "[1]")),
        &prepare(3, 1, B, "[1]"),
    )
}

// ------------------------------------------------------- The hand calculation

#[test]
fn the_figures_are_the_calculations() {
    let scenario = from(&text());
    let g = scenario.world.ref_data().ground.expect("ground figures");
    assert_eq!(g.preparation_per_day, n(5));
    assert_eq!(g.unprepared_landing_share, fx("0.5"));
    assert_eq!(g.preparation_kept_on_change, fx("0.5"));
    assert_eq!(g.unprepared_landing_fatigue, n(40));
    assert_eq!(g.handling_t_per_man, fx("0.25"));
    assert_eq!(g.daily_supply_t_per_man, fx("0.005"));
    assert_eq!(g.unfed_loss_share_per_day, fx("0.05"));
    assert_eq!(g.rest_per_day, n(5));
    // The inputs: the ships, the units and the bases.
    let truth = scenario.world.umpire();
    for (unit_id, men) in [(1, 600), (2, 600), (3, 200), (4, 600), (5, 300)] {
        assert_eq!(truth.men_aboard(unit(unit_id)), men);
    }
    assert_eq!(truth.land_units[&unit(6)].men_ashore, 300);
    assert_eq!(truth.land_units[&unit(6)].hex, Some(A));
    assert!(truth.land_units.values().all(|u| u.fatigue.is_zero()));
    assert_eq!(truth.ships[&ShipId::new(1)].supply_t, n(60));
    assert_eq!(truth.bases[&base(1)].supply_t, n(1000));
    assert_eq!(truth.bases[&base(2)].supply_t, n(100));
    assert!(!truth.bases.contains_key(&base(3)));
}

#[test]
fn loading_and_rehearsal_follow_the_calculation_day_by_day() {
    let run = Run::text(&text());
    assert_eq!(run.len(), 12);
    // The table of the calculation, days 5, 6, 10 and 12, and every day
    // between, which add 5 to each count kept: force, unit, then per day 1 to
    // 12 the loading (`None` for no preparation) and the rehearsal with its
    // hex.
    let ramp = |from: i64, days: usize| -> Vec<Fixed> {
        (0..days).map(|d| n(from + 5 * d as i64)).collect()
    };
    // Force 1 and unit 1: 5 a day to 50 on day 10. Force 1 sails on day 11
    // and gains nothing; its loading of 50 is spent on day 12. Unit 1's
    // rehearsal is spent by its landing on day 11.
    let mut loading_1 = ramp(5, 10);
    loading_1.extend([n(50), n(0)]);
    let mut rehearsal_1 = ramp(5, 10);
    rehearsal_1.extend([n(0), n(0)]);
    // Force 3 and unit 3: as force 1, but its loading of 50 stays.
    let mut loading_3 = ramp(5, 10);
    loading_3.extend([n(50), n(50)]);
    // Force 4: 25 for B on day 5, 5 for C on day 6, 25 on day 10, 35 on day
    // 12. Unit 4 as it.
    let mut loading_4 = ramp(5, 5);
    loading_4.extend(ramp(5, 7));
    // Force 5: 25 on day 5, 17.5 on day 6, 37.5 on day 10, 47.5 on day 12.
    let mut loading_5 = ramp(5, 5);
    loading_5.extend((0..7).map(|d| fx("17.5") + n(5 * d)));
    // Unit 5: 25, 30, 50, 60. Unit 6: 0 until day 5, then 5 on day 6, 25 on
    // day 10, 35 on day 12.
    let mut rehearsal_5 = ramp(5, 5);
    rehearsal_5.extend(ramp(30, 7));
    let mut rehearsal_6 = vec![n(0); 5];
    rehearsal_6.extend(ramp(5, 7));
    for day in 1..=12u32 {
        let i = day as usize - 1;
        assert_eq!(run.loading(day, 1), loading_1[i], "force 1 day {day}");
        assert_eq!(run.rehearsal(day, 1), rehearsal_1[i], "unit 1 day {day}");
        assert_eq!(run.loading(day, 3), loading_3[i], "force 3 day {day}");
        assert_eq!(
            run.rehearsal(day, 3),
            rehearsal_1[i],
            "unit 3 day {day}: as unit 1"
        );
        assert_eq!(run.loading(day, 4), loading_4[i], "force 4 day {day}");
        assert_eq!(run.rehearsal(day, 4), loading_4[i], "unit 4 day {day}");
        assert_eq!(run.loading(day, 5), loading_5[i], "force 5 day {day}");
        assert_eq!(run.rehearsal(day, 5), rehearsal_5[i], "unit 5 day {day}");
        assert_eq!(run.rehearsal(day, 6), rehearsal_6[i], "unit 6 day {day}");
        // Force 2 and unit 2 have 0 throughout.
        assert!(run.at(day).task_forces[&force(2)].preparation.is_none());
        assert_eq!(run.rehearsal(day, 2), n(0), "unit 2 day {day}");
    }
    // The calculation's table, written out.
    assert_eq!(
        [5, 6, 10, 12].map(|d| (
            run.loading(d, 1),
            run.loading(d, 3),
            run.loading(d, 4),
            run.loading(d, 5)
        )),
        [
            (n(25), n(25), n(25), n(25)),
            (n(30), n(30), n(5), fx("17.5")),
            (n(50), n(50), n(25), fx("37.5")),
            (n(0), n(50), n(35), fx("47.5")),
        ]
    );
    assert_eq!(
        [5, 6, 10, 12].map(|d| (run.rehearsal(d, 5), run.rehearsal(d, 6))),
        [(n(25), n(0)), (n(30), n(5)), (n(50), n(25)), (n(60), n(35))]
    );
    // Force 1 and force 3 hold their 50 until they land: the end of day 11.
    assert_eq!(run.loading(11, 1), n(50));
    assert_eq!(run.loading(11, 3), n(50));
    // The hex of each rehearsal and loading: B, and for unit 4 and force 4 B
    // until day 5 and C from day 6.
    let hex_of = |day: u32, raw: u32| {
        run.at(day).task_forces[&force(raw)]
            .preparation
            .as_ref()
            .map(|p| p.hex)
    };
    for day in 1..=12 {
        assert_eq!(
            hex_of(day, 4),
            Some(if day <= 5 { B } else { C }),
            "day {day}"
        );
        assert_eq!(
            run.rehearsal_hex(day, 4),
            Some(if day <= 5 { B } else { C }),
            "day {day}"
        );
        assert_eq!(hex_of(day, 5), Some(B));
        assert_eq!(run.rehearsal_hex(day, 5), Some(B));
        assert_eq!(
            run.rehearsal_hex(day, 6),
            if day >= 6 { Some(B) } else { None }
        );
        assert_eq!(run.rehearsal_hex(day, 2), None);
    }
    // Rosters: the unit aboard, and for force 5 both units from day 6.
    assert_eq!(run.roster(5, 1), vec![1]);
    assert_eq!(run.roster(10, 4), vec![4]);
    assert_eq!(run.roster(5, 5), vec![5]);
    assert_eq!(run.roster(6, 5), vec![5, 6]);
    assert_eq!(run.roster(12, 5), vec![5, 6]);
}

#[test]
fn force_1s_second_order_on_day_3_is_the_same_order_and_changes_nothing() {
    let run = Run::text(&text());
    let first = run.accepted(1, EntityRef::TaskForce(force(1)));
    let second = run.accepted(3, EntityRef::TaskForce(force(1)));
    assert_ne!(first, second);
    // The standing order is still the first, on every day to the landing.
    for day in 1..=11 {
        let prep = run.at(day).task_forces[&force(1)]
            .preparation
            .as_ref()
            .expect("a preparation");
        assert_eq!(prep.order, first, "day {day}");
    }
    // No loading or rehearsal or roster event of day 3 for force 1 or unit 1
    // but the day's gains, which cite the first order.
    let caused_by_the_order = run.on(3, |k| {
        matches!(
            k,
            EventKind::LoadingLost { .. }
                | EventKind::RehearsalLost { .. }
                | EventKind::LeftRoster { .. }
                | EventKind::PreparationShared { .. }
        )
    });
    assert!(
        caused_by_the_order.is_empty(),
        "an event the same order must not cause: {caused_by_the_order:?}"
    );
    let gained = run.one(
        3,
        |k| matches!(k, EventKind::LoadingGained { task_force, .. } if task_force.raw() == 1),
    );
    assert!(caused_by_order(gained, first));
    assert!(!caused_by_order(gained, second));
    // It is as force 3, which was given the order once.
    for day in 1..=10 {
        assert_eq!(run.loading(day, 1), run.loading(day, 3), "day {day}");
        assert_eq!(run.rehearsal(day, 1), run.rehearsal(day, 3), "day {day}");
    }
}

#[test]
fn force_4_changing_its_landing_place_on_day_6_loses_everything() {
    let run = Run::text(&text());
    let order = run.accepted(6, EntityRef::TaskForce(force(4)));
    let lost = run.one(
        6,
        |k| matches!(k, EventKind::LoadingLost { task_force, .. } if task_force.raw() == 4),
    );
    assert_eq!(
        lost.kind,
        EventKind::LoadingLost {
            task_force: force(4),
            hex: B,
            loading_before: n(25),
            loading: n(0),
            reason: LoadingLoss::NewLandingPlace,
        }
    );
    assert!(caused_by_order(lost, order));
    let rehearsal = run.one(
        6,
        |k| matches!(k, EventKind::RehearsalLost { unit, .. } if unit.raw() == 4),
    );
    assert_eq!(
        rehearsal.kind,
        EventKind::RehearsalLost {
            unit: unit(4),
            hex: B,
            rehearsal_before: n(25),
            reason: RehearsalLoss::NewLandingPlace,
        }
    );
    assert!(caused_by_order(rehearsal, order));
    // It gains 5 that day, at C, and the rehearsal is for C from then on.
    let gained = run.one(
        6,
        |k| matches!(k, EventKind::LoadingGained { task_force, .. } if task_force.raw() == 4),
    );
    assert_eq!(
        gained.kind,
        EventKind::LoadingGained {
            task_force: force(4),
            hex: C,
            loading: n(5),
        }
    );
    let rehearsed = run.one(
        6,
        |k| matches!(k, EventKind::RehearsalGained { unit, .. } if unit.raw() == 4),
    );
    assert_eq!(
        rehearsed.kind,
        EventKind::RehearsalGained {
            unit: unit(4),
            task_force: force(4),
            hex: C,
            rehearsal: n(5),
        }
    );
    assert_eq!(run.rehearsal_hex(5, 4), Some(B));
    assert_eq!(run.rehearsal_hex(6, 4), Some(C));
    // No other rehearsal lost on day 6, and no event for it of "another
    // landing place": the order took the old rehearsal at intake.
    assert_eq!(
        run.on(6, |k| matches!(k, EventKind::RehearsalLost { .. }))
            .len(),
        1
    );
}

#[test]
fn force_5_taking_a_second_unit_on_day_6_keeps_half_its_loading() {
    let run = Run::text(&text());
    let order = run.accepted(6, EntityRef::TaskForce(force(5)));
    let lost = run.one(
        6,
        |k| matches!(k, EventKind::LoadingLost { task_force, .. } if task_force.raw() == 5),
    );
    assert_eq!(
        lost.kind,
        EventKind::LoadingLost {
            task_force: force(5),
            hex: B,
            loading_before: n(25),
            loading: fx("12.5"),
            reason: LoadingLoss::UnitAdded,
        }
    );
    assert!(caused_by_order(lost, order));
    assert_eq!(factor(lost, "loading_before"), n(25));
    assert_eq!(factor(lost, "preparation_kept_on_change"), fx("0.5"));
    // It gains 5 that day: 17.5. Unit 5 keeps its 25 and gains: 30. Unit 6
    // starts at 0, is ashore at A in the force's hex, and gains 5.
    assert_eq!(run.loading(6, 5), fx("17.5"));
    assert_eq!(run.rehearsal(6, 5), n(30));
    assert_eq!(run.rehearsal(6, 6), n(5));
    let rehearsed = run.one(
        6,
        |k| matches!(k, EventKind::RehearsalGained { unit, .. } if unit.raw() == 6),
    );
    assert_eq!(
        rehearsed.kind,
        EventKind::RehearsalGained {
            unit: unit(6),
            task_force: force(5),
            hex: B,
            rehearsal: n(5),
        }
    );
    // Nothing of unit 5's rehearsal was lost.
    assert!(run
        .on(
            6,
            |k| matches!(k, EventKind::RehearsalLost { unit, .. } if unit.raw() == 5)
        )
        .is_empty());
}

#[test]
fn force_2_and_unit_2_have_no_loading_and_no_rehearsal() {
    let run = Run::text(&text());
    assert!(run
        .all(|k| matches!(
            k,
            EventKind::LoadingGained { task_force, .. }
                | EventKind::LoadingLost { task_force, .. } if task_force.raw() == 2
        ))
        .is_empty());
    assert!(run
        .all(|k| matches!(
            k,
            EventKind::RehearsalGained { unit, .. } | EventKind::RehearsalLost { unit, .. }
                if unit.raw() == 2
        ))
        .is_empty());
}

#[test]
fn the_passages_of_forces_1_2_and_3_follow_the_calculation() {
    let run = Run::text(&text());
    // A to B is 160 nm at 10 kn: 16 hours. They arrive at hour 256. A to C is
    // 240 nm at 16 kn: 15 hours, hour 255.
    assert_eq!(run.arrived_at(1), n(256));
    assert_eq!(run.arrived_at(2), n(256));
    assert_eq!(run.arrived_at(3), n(255));
    // Fuel: 16 t each for the troop transports, 30 t for the fast one.
    for day in [11, 12] {
        assert_eq!(run.at(day).ships[&ShipId::new(1)].fuel_t, n(984));
        assert_eq!(run.at(day).ships[&ShipId::new(2)].fuel_t, n(984));
        assert_eq!(run.at(day).ships[&ShipId::new(3)].fuel_t, n(470));
    }
    // Forces 4 and 5 stay at A, full of fuel.
    assert_eq!(run.at(12).ships[&ShipId::new(4)].fuel_t, n(1000));
    assert_eq!(run.at(12).ships[&ShipId::new(5)].fuel_t, n(1000));
}

#[test]
fn the_men_landed_in_each_pulse_follow_the_calculation() {
    let run = Run::text(&text());
    use Pulse::{Afternoon, Morning, Night};
    assert_eq!(
        run.landings(),
        vec![
            // Day 11, morning: force 1 at 15 t an hour for 2 hours, force 2 at
            // 10 for 2, force 3 at 10 for 3.
            (11, Morning, 1, 120),
            (11, Morning, 2, 80),
            (11, Morning, 3, 120),
            // Day 11, afternoon: 6 hours.
            (11, Afternoon, 1, 360),
            (11, Afternoon, 2, 240),
            (11, Afternoon, 3, 80),
            // Day 12, night: the last 120 of unit 1 and the last 280 of
            // unit 2.
            (12, Night, 1, 120),
            (12, Night, 2, 280),
        ]
    );
    // The rate of each landing, from the breakdown of its event: the port's
    // lighter rate 20, the share 0.5 and the loading counted at the hex.
    let landed = run.all(|k| matches!(k, EventKind::MenLanded { .. }));
    assert_eq!(landed.len(), 8);
    for e in landed {
        let ship = match e.kind {
            EventKind::MenLanded { ship, .. } => ship.raw(),
            _ => unreachable!(),
        };
        let (loading, rate, factor_of_rate) = match ship {
            // Force 1: loading 50 for B, 20 x (0.5 + 0.5 x 50/100) = 15.
            1 => (n(50), n(15), fx("0.75")),
            // Force 2: none, 20 x 0.5 = 10.
            2 => (n(0), n(10), fx("0.5")),
            // Force 3: 50 for B, but it lands at C: none counted, 10.
            3 => (n(0), n(10), fx("0.5")),
            other => panic!("ship {other} landed men"),
        };
        assert_eq!(factor(e, "port_rate_t_per_h"), n(20), "{e:?}");
        assert_eq!(factor(e, "unprepared_landing_share"), fx("0.5"), "{e:?}");
        assert_eq!(factor(e, "loading"), loading, "{e:?}");
        assert_eq!(factor(e, "loading_factor"), factor_of_rate, "{e:?}");
        assert_eq!(factor(e, "rate_t_per_h"), rate, "{e:?}");
    }
    // Force 1's 60 t of supply are unloaded at 15 t an hour after the last
    // man: the same rate, the loading holds for the whole unloading.
    let unloaded = run.one(12, |k| matches!(k, EventKind::CargoUnloaded { .. }));
    assert_eq!(
        unloaded.kind,
        EventKind::CargoUnloaded {
            ship: ShipId::new(1),
            task_force: force(1),
            base: base(2),
            cargo: sim_core::orders::Cargo::Supply,
            unloaded_t: n(60),
            aboard_t: n(0),
            base_stock_t: n(156),
        }
    );
    assert_eq!(factor(unloaded, "rate_t_per_h"), n(15));
    assert_eq!(factor(unloaded, "loading_factor"), fx("0.75"));
}

#[test]
fn the_hours_of_completion_are_270_271_and_260() {
    let run = Run::text(&text());
    // Force 1: 120 men take 30 t, then 60 t at 15 t an hour: 90 / 15 = 6
    // hours into the night pulse of day 12, hour 264 + 6.
    assert_eq!(run.completed_at(1), Some(n(270)));
    // Force 2: 280 men take 70 t, at 10 t an hour: 7 hours, hour 271.
    assert_eq!(run.completed_at(2), Some(n(271)));
    // Force 3: 3 hours and then 2: hour 260.
    assert_eq!(run.completed_at(3), Some(n(260)));
    let done = |raw: u32| {
        run.one(
            if raw == 3 { 11 } else { 12 },
            |k| matches!(k, EventKind::UnloadingComplete { task_force, .. } if task_force.raw() == raw),
        )
    };
    assert_eq!(factor(done(1), "work_start_h"), n(264));
    assert_eq!(factor(done(1), "longest_ship_hours"), n(6));
    assert_eq!(factor(done(2), "longest_ship_hours"), n(7));
    assert_eq!(factor(done(3), "work_start_h"), n(258));
    assert_eq!(factor(done(3), "longest_ship_hours"), n(2));
}

#[test]
fn fatigue_on_landing_and_after_each_day_follows_the_calculation() {
    let run = Run::text(&text());
    // On landing, day 11: unit 1 gains 40 x 50/100 = 20, units 2 and 3 gain
    // 40, with no rehearsal counted at the hex they land at.
    let landed = run.on(11, |k| {
        matches!(
            k,
            EventKind::LandUnitFatigue {
                change: LandFatigueChange::Landed,
                ..
            }
        )
    });
    assert_eq!(landed.len(), 3);
    let mut by_unit = std::collections::BTreeMap::new();
    for e in &landed {
        if let EventKind::LandUnitFatigue { unit, fatigue, .. } = e.kind {
            by_unit.insert(unit.raw(), (fatigue, factor(e, "rehearsal")));
            assert_eq!(factor(e, "fatigue_before"), n(0));
            assert_eq!(factor(e, "unprepared_landing_fatigue"), n(40));
            // Each cites the landing of its first men.
            let man_landed = run
                .on(
                    11,
                    |k| matches!(k, EventKind::MenLanded { unit: u, .. } if *u == unit),
                )
                .first()
                .map(|e| e.id)
                .expect("a landing");
            assert!(caused_by_event(e, man_landed));
        }
    }
    assert_eq!(by_unit[&1], (n(20), n(50)));
    assert_eq!(by_unit[&2], (n(40), n(0)));
    assert_eq!(by_unit[&3], (n(40), n(0)));
    // Only the first men of a unit are a landing: no such event on day 12.
    assert!(run
        .on(12, |k| matches!(
            k,
            EventKind::LandUnitFatigue {
                change: LandFatigueChange::Landed,
                ..
            }
        ))
        .is_empty());
    // A unit ashore that neither attacks nor marches rests 5 a day.
    for (raw, day_11, day_12) in [(1, 15, 10), (2, 35, 30), (3, 35, 30)] {
        assert_eq!(run.fatigue(11, raw), n(day_11), "unit {raw}");
        assert_eq!(run.fatigue(12, raw), n(day_12), "unit {raw}");
    }
    for raw in [4, 5, 6] {
        assert_eq!(run.fatigue(12, raw), n(0));
    }
}

#[test]
fn rehearsal_is_0_after_landing_and_the_landed_units_leave_their_rosters() {
    let run = Run::text(&text());
    for raw in [1, 3] {
        assert_eq!(run.rehearsal(10, raw), n(50));
        assert_eq!(run.rehearsal(11, raw), n(0));
        assert_eq!(run.rehearsal_hex(11, raw), None);
        let lost = run.one(
            11,
            |k| matches!(k, EventKind::RehearsalLost { unit, .. } if unit.raw() == raw),
        );
        assert_eq!(
            lost.kind,
            EventKind::RehearsalLost {
                unit: unit(raw),
                hex: B,
                rehearsal_before: n(50),
                reason: RehearsalLoss::Landed,
            }
        );
        let left = run.one(
            11,
            |k| matches!(k, EventKind::LeftRoster { unit, .. } if unit.raw() == raw),
        );
        assert_eq!(
            left.kind,
            EventKind::LeftRoster {
                unit: unit(raw),
                task_force: force(raw),
                reason: RosterLeft::Landed,
            }
        );
        // Both cite the landing of the first men.
        let landing = run.on(
            11,
            |k| matches!(k, EventKind::MenLanded { unit: u, .. } if u.raw() == raw),
        )[0];
        assert!(caused_by_event(lost, landing.id));
        assert!(caused_by_event(left, landing.id));
    }
    // Unit 2 had none to lose and was on no roster.
    assert!(run
        .all(|k| matches!(k, EventKind::LeftRoster { unit, .. } if unit.raw() == 2))
        .is_empty());
    assert_eq!(
        run.all(|k| matches!(k, EventKind::LeftRoster { .. })).len(),
        2
    );
    // Force 1's and force 3's rosters are empty.
    assert!(run.roster(11, 1).is_empty());
    assert!(run.roster(11, 3).is_empty());
}

#[test]
fn force_3_keeps_its_loading_of_50_and_its_order_and_force_1s_is_spent() {
    let run = Run::text(&text());
    let order = run.accepted(1, EntityRef::TaskForce(force(3)));
    for day in [11, 12] {
        let prep = run.at(day).task_forces[&force(3)]
            .preparation
            .as_ref()
            .expect("the order stands");
        assert_eq!(prep.loading, n(50), "day {day}");
        assert_eq!(prep.hex, B);
        assert_eq!(prep.order, order);
        assert!(prep.roster.is_empty());
    }
    // Force 1's order is done at hour 270: the loading is spent, citing the
    // end of its unloading.
    let done = run.one(
        12,
        |k| matches!(k, EventKind::UnloadingComplete { task_force, .. } if task_force.raw() == 1),
    );
    let spent = run.one(
        12,
        |k| matches!(k, EventKind::LoadingLost { task_force, .. } if task_force.raw() == 1),
    );
    assert_eq!(
        spent.kind,
        EventKind::LoadingLost {
            task_force: force(1),
            hex: B,
            loading_before: n(50),
            loading: n(0),
            reason: LoadingLoss::Spent,
        }
    );
    assert!(caused_by_event(spent, done.id));
    assert!(run.at(12).task_forces[&force(1)].preparation.is_none());
    assert_eq!(run.loading(12, 1), n(0));
    // Forces 2 and 3 completed no order of the kind, and force 3 lost nothing.
    assert_eq!(
        run.all(|k| matches!(
            k,
            EventKind::LoadingLost {
                reason: LoadingLoss::Spent,
                ..
            }
        ))
        .len(),
        1
    );
    // The landing's marks are cleared at the end: no force is landing.
    for raw in 1..=3 {
        assert_eq!(run.at(12).task_forces[&force(raw)].landing_at, None);
    }
}

#[test]
fn supply_at_the_bases_and_the_men_left_follow_the_calculation() {
    let run = Run::text(&text());
    // Base 1 feeds unit 6 every night: 300 x 0.005 = 1.5 t, twelve times.
    for day in 1..=12 {
        assert_eq!(
            run.supply(day, 1),
            n(1000) - fx("1.5") * Fixed::from_int(i64::from(day)),
            "day {day}"
        );
    }
    assert_eq!(run.supply(12, 1), n(982));
    // Base 2, night of day 12, before that pulse's loading: 480 men of unit 1
    // and 320 of unit 2 are ashore, 800 x 0.005 = 4 t.
    assert_eq!(run.supply(11, 2), n(100));
    let spent = run.one(
        12,
        |k| matches!(k, EventKind::BaseSupplySpent { base, .. } if base.raw() == 2),
    );
    match spent.kind {
        EventKind::BaseSupplySpent {
            men,
            paid_t,
            left_t,
            ..
        } => {
            assert_eq!((men, paid_t, left_t), (800, n(4), n(96)));
        }
        _ => unreachable!(),
    }
    // It is charged before the pulse's landing: the landing's 60 t come after.
    let landing_of_supply = run.one(12, |k| matches!(k, EventKind::CargoUnloaded { .. }));
    assert!(spent.id < landing_of_supply.id);
    assert_eq!(run.supply(12, 2), n(156));
    // Base 3 at C has no stock; unit 3's 200 men are unfed on the night of
    // day 12: 200 x 0.05 = 10 are lost.
    assert_eq!(run.supply(11, 3), n(0));
    assert_eq!(run.supply(12, 3), n(0));
    let lost = run.one(12, |k| matches!(k, EventKind::MenLostUnfed { .. }));
    assert_eq!(
        lost.kind,
        EventKind::MenLostUnfed {
            unit: unit(3),
            hex: C,
            lost: 10,
            ashore: 190,
        }
    );
    // At the end.
    let end = run.at(12);
    assert_eq!(end.land_units[&unit(1)].men_ashore, 600);
    assert_eq!(end.land_units[&unit(1)].hex, Some(B));
    assert_eq!(end.land_units[&unit(2)].men_ashore, 600);
    assert_eq!(end.land_units[&unit(2)].hex, Some(B));
    assert_eq!(end.land_units[&unit(3)].men_ashore, 190);
    assert_eq!(end.land_units[&unit(3)].hex, Some(C));
    for raw in [1, 2, 3] {
        assert_eq!(end.men_aboard(unit(raw)), 0);
    }
    assert_eq!(end.men_aboard(unit(4)), 600);
    assert_eq!(end.men_aboard(unit(5)), 300);
    assert_eq!(end.land_units[&unit(4)].men_ashore, 0);
    assert_eq!(end.land_units[&unit(5)].men_ashore, 0);
    assert_eq!(end.land_units[&unit(6)].men_ashore, 300);
    assert_eq!(end.land_units[&unit(6)].hex, Some(A));
}

// ------------------------------------------------------------------ Variants

/// Force 5 with a second troop transport, ship 6, empty.
fn with_second_ship_in_force_5(source: &str) -> String {
    with(
        source,
        "[[ship]]\nid = 6\nname = \"Placeholder troop transport 6\"\nclass = 10\nside = \"allied\"\nfuel_t = 1000\ntask_force = 5\n",
    )
}

/// Unit 6 with the given experience.
fn with_experience_6(source: &str, experience: u32) -> String {
    common::replace_once(
        source,
        "men = 300\nq = 0\nr = 0\n",
        &format!("men = 300\nq = 0\nr = 0\nexperience = {experience}\n"),
    )
}

#[test]
fn a_split_force_keeps_its_loading_with_the_units_aboard_the_leaving_ships() {
    // Force 5 has ships 5 (unit 5 aboard) and 6 (empty), and on day 8 ship 5
    // leaves it as a new force.
    let source = with_order(
        &with_second_ship_in_force_5(&text()),
        8,
        "allied",
        "task_force = 5",
        "detach",
        "ships = [5]",
    );
    let split = Run::text(&source);
    let plain = Run::text(&with_second_ship_in_force_5(&text()));
    let shared = split.one(8, |k| matches!(k, EventKind::PreparationShared { .. }));
    let detached = split.one(8, |k| matches!(k, EventKind::ShipsDetached { .. }));
    let new_force = match shared.kind {
        EventKind::PreparationShared {
            parent,
            new_force,
            hex,
            loading,
            ref units,
        } => {
            assert_eq!(parent, force(5));
            assert_eq!(hex, B);
            // After the gain of day 7 the loading was 22.5, and it is the
            // same in both parts.
            assert_eq!(loading, fx("22.5"));
            assert_eq!(units, &vec![unit(5)]);
            new_force
        }
        _ => unreachable!(),
    };
    assert!(caused_by_event(shared, detached.id));
    // Both parts have the loading, the new one the unit aboard its ship, the
    // parent the unit ashore at A, and both gain 5 that day.
    let new_part = &split.at(8).task_forces[&new_force];
    let prep = new_part
        .preparation
        .as_ref()
        .expect("the new part prepares");
    assert_eq!(prep.loading, fx("27.5"));
    assert_eq!(prep.hex, B);
    assert_eq!(
        prep.order,
        split.at(7).task_forces[&force(5)]
            .preparation
            .as_ref()
            .expect("the parent prepared")
            .order
    );
    assert_eq!(split.roster(8, 5), vec![6]);
    assert_eq!(split.loading(8, 5), fx("27.5"));
    assert_eq!(
        split.at(8).task_forces[&new_force]
            .preparation
            .as_ref()
            .expect("the new part prepares")
            .roster
            .iter()
            .map(|u| u.raw())
            .collect::<Vec<_>>(),
        vec![5]
    );
    // Rehearsal is never copied: every unit's rehearsal is what it is without
    // the split, every day.
    for day in 0..=12 {
        for raw in 1..=6 {
            assert_eq!(
                split.rehearsal(day, raw),
                plain.rehearsal(day, raw),
                "unit {raw} day {day}"
            );
        }
        let sum = |run: &Run| {
            (1..=6)
                .map(|raw| run.rehearsal(day, raw))
                .fold(Fixed::ZERO, |a, b| a + b)
        };
        assert_eq!(sum(&split), sum(&plain), "day {day}");
        // No unit is on two rosters.
        let mut seen = std::collections::BTreeSet::new();
        for f in split.at(day).task_forces.values() {
            if let Some(p) = &f.preparation {
                for u in &p.roster {
                    assert!(seen.insert(*u), "unit {u:?} on two rosters, day {day}");
                }
            }
        }
    }
    // The parent's own loading was not changed by the split but for the day's
    // gain, and neither part lost any.
    assert!(split
        .on(8, |k| matches!(k, EventKind::LoadingLost { .. }))
        .is_empty());
}

#[test]
fn a_unit_moved_from_one_roster_to_another_keeps_its_rehearsal_and_the_first_force_loses_nothing() {
    // Day 8: force 3 is ordered to prepare for B with units 3 and 6. Unit 6,
    // ashore at A, was on force 5's roster with a rehearsal of 10 for B.
    let source = with(&text(), &prepare(8, 3, B, "[3, 6]"));
    let run = Run::text(&source);
    let plain = Run::text(&text());
    let order = run.accepted(8, EntityRef::TaskForce(force(3)));
    let left = run.one(8, |k| matches!(k, EventKind::LeftRoster { .. }));
    assert_eq!(
        left.kind,
        EventKind::LeftRoster {
            unit: unit(6),
            task_force: force(5),
            reason: RosterLeft::TakenByOrder,
        }
    );
    assert!(caused_by_order(left, order));
    assert_eq!(run.roster(8, 5), vec![5]);
    assert_eq!(run.roster(8, 3), vec![3, 6]);
    // The first force loses nothing: its loading goes on as it did.
    assert!(run
        .on(
            8,
            |k| matches!(k, EventKind::LoadingLost { task_force, .. } if task_force.raw() == 5)
        )
        .is_empty());
    for day in 7..=10 {
        assert_eq!(run.loading(day, 5), plain.loading(day, 5), "day {day}");
    }
    // The unit keeps its rehearsal and the same hex: 10 after day 7, 15
    // after day 8, as with force 5. No rehearsal lost.
    assert_eq!(run.rehearsal(7, 6), n(10));
    assert_eq!(run.rehearsal(8, 6), n(15));
    assert_eq!(run.rehearsal_hex(8, 6), Some(B));
    assert!(run
        .all(|k| matches!(k, EventKind::RehearsalLost { unit, .. } if unit.raw() == 6))
        .is_empty());
    // The force that took it in has a unit added: it keeps half its loading
    // (35 after day 7) and gains 5.
    let lost = run.one(
        8,
        |k| matches!(k, EventKind::LoadingLost { task_force, .. } if task_force.raw() == 3),
    );
    assert_eq!(
        lost.kind,
        EventKind::LoadingLost {
            task_force: force(3),
            hex: B,
            loading_before: n(35),
            loading: fx("17.5"),
            reason: LoadingLoss::UnitAdded,
        }
    );
    assert_eq!(run.loading(8, 3), fx("22.5"));
    // And it rehearsed with force 3 that day.
    let rehearsed = run.one(
        8,
        |k| matches!(k, EventKind::RehearsalGained { unit, .. } if unit.raw() == 6),
    );
    assert!(matches!(
        rehearsed.kind,
        EventKind::RehearsalGained { task_force, .. } if task_force == force(3)
    ));
}

#[test]
fn a_unit_set_to_train_gains_training_and_no_rehearsal_and_ready_again_gains_rehearsal() {
    // Unit 6, ashore at A on force 5's roster, at experience 1 (below the
    // training ceiling of 2). It trains on days 8 and 9 and is ready again
    // on day 10.
    let source = with_experience_6(&text(), 1);
    let source = with_order(
        &source,
        8,
        "allied",
        "land_unit = 6",
        "set_activity",
        "activity = \"train\"",
    );
    let source = with_order(
        &source,
        10,
        "allied",
        "land_unit = 6",
        "set_activity",
        "activity = \"ready\"",
    );
    let run = Run::text(&source);
    let plain = Run::text(&with_experience_6(&text(), 1));
    let train = run.accepted(8, EntityRef::LandUnit(unit(6)));
    let training = |day: u32| run.at(day).land_units[&unit(6)].training_days;
    // Rehearsal: 10 after day 7, then none gained on days 8 and 9, then 15.
    assert_eq!(
        (7..=10).map(|d| run.rehearsal(d, 6)).collect::<Vec<_>>(),
        vec![n(10), n(10), n(10), n(15)]
    );
    assert_eq!((7..=10).map(training).collect::<Vec<_>>(), vec![0, 1, 2, 2]);
    for day in 8..=9 {
        assert!(run
            .on(
                day,
                |k| matches!(k, EventKind::RehearsalGained { unit, .. } if unit.raw() == 6)
            )
            .is_empty());
        let e = run.one(day, |k| matches!(k, EventKind::TrainingDay { .. }));
        assert_eq!(
            e.kind,
            EventKind::TrainingDay {
                unit: unit(6),
                days: day - 7,
                experience: 1,
            }
        );
        assert!(caused_by_order(e, train));
    }
    assert!(run
        .on(10, |k| matches!(k, EventKind::TrainingDay { .. }))
        .is_empty());
    assert_eq!(
        run.one(
            10,
            |k| matches!(k, EventKind::RehearsalGained { unit, .. } if unit.raw() == 6)
        )
        .kind,
        EventKind::RehearsalGained {
            unit: unit(6),
            task_force: force(5),
            hex: B,
            rehearsal: n(15),
        }
    );
    // The force's loading was untouched, and the unit stayed on the roster.
    for day in 1..=12 {
        assert_eq!(run.loading(day, 5), plain.loading(day, 5), "day {day}");
        assert!(run.roster(day, 5).contains(&6) || day < 6);
    }
    // Experience is still 1: two days of 30.
    assert_eq!(run.at(12).land_units[&unit(6)].experience, 1);
}

/// The map of scenario 27 with a land hex beside A joined to it by a land
/// link, in a file of its own, and the scenario text pointing at it.
fn with_a_land_link(source: &str) -> String {
    let map = std::fs::read_to_string(common::data_dir().join("map/placeholder_preparing.toml"))
        .expect("the map reads");
    let map = format!(
        "{map}\n[[hex]]\nq = 1\nr = 0\nkind = \"coast\"\nport_size = 0\n\n[[land_link]]\na = [0, 0]\nb = [1, 0]\nnote = \"Placeholder: A and its neighbour are on one landmass.\"\n"
    );
    let file = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("map_scenario_27_land_link.toml");
    std::fs::write(&file, map).expect("write the map");
    common::replace_once(
        source,
        "map = \"../map/placeholder_preparing.toml\"",
        &format!("map = {:?}", file.to_string_lossy()),
    )
}

#[test]
fn a_unit_that_marched_gains_neither_rehearsal_nor_training() {
    // Unit 6 marches from A to its neighbour on day 8: three days, 8 to 10.
    for training in [false, true] {
        let mut source = with_a_land_link(&text());
        if training {
            source = with_experience_6(&source, 1);
            source = with_order(
                &source,
                7,
                "allied",
                "land_unit = 6",
                "set_activity",
                "activity = \"train\"",
            );
        }
        let source = with_order(
            &source,
            8,
            "allied",
            "land_unit = 6",
            "march",
            "hex = [1, 0]",
        );
        let run = Run::text(&source);
        let plain = Run::text(&with_a_land_link(&text()));
        assert_eq!(run.len(), 12);
        let unit_6 = |day: u32| &run.at(day).land_units[&unit(6)];
        // The day before the march: unit 6 had 10 after day 7 as on ready, or,
        // training from day 7, the 5 of day 6 and a day of training.
        let marched_days = (8..=10)
            .map(|d| unit_6(d).march.is_some() || unit_6(d).hex != Some(A))
            .collect::<Vec<_>>();
        assert_eq!(marched_days, vec![true, true, true]);
        let kept = if training {
            assert_eq!(unit_6(7).training_days, 1);
            n(5)
        } else {
            assert_eq!(plain.rehearsal(7, 6), n(10));
            assert_eq!(plain.rehearsal(10, 6), n(25));
            n(10)
        };
        assert_eq!(unit_6(7).rehearsal, kept);
        for day in 8..=10 {
            assert_eq!(unit_6(day).rehearsal, kept, "day {day}");
            assert_eq!(unit_6(day).rehearsal_hex, Some(B));
            assert_eq!(unit_6(day).training_days, u32::from(training));
            assert!(run
                .on(day, |k| matches!(
                    k,
                    EventKind::RehearsalGained { unit, .. } | EventKind::TrainingDay { unit, .. }
                        if unit.raw() == 6
                ))
                .is_empty());
        }
        // The force it was on goes on gaining.
        assert_eq!(run.loading(10, 5), n(0) + fx("17.5") + n(5 * 4));
    }
}

#[test]
fn a_unit_whose_hex_was_contested_gains_neither_rehearsal_nor_training() {
    // A Japanese battalion lands from nowhere at A on day 1, 1000 men on
    // attack, against unit 6 ashore there: the hex is contested every day.
    let japanese = "[[land_unit]]\nid = 7\nname = \"Placeholder Japanese battalion 7\"\nside = \"japanese\"\nmen = 1000\nq = 0\nr = 0\nposture = \"attack\"\n";
    let source = with(&with_experience_6(&text(), 1), japanese);
    let source = with_order(
        &source,
        1,
        "allied",
        "land_unit = 6",
        "set_activity",
        "activity = \"train\"",
    );
    let run = Run::text(&source);
    let plain = Run::text(&text());
    assert!(
        run.on(1, |k| matches!(k, EventKind::GroundFight { .. }))
            .len()
            == 1
    );
    for day in 1..=2 {
        assert_eq!(run.rehearsal(day, 6), n(0), "day {day}");
        assert_eq!(run.at(day).land_units[&unit(6)].training_days, 0);
        assert!(run
            .on(day, |k| matches!(
                k,
                EventKind::RehearsalGained { unit, .. } | EventKind::TrainingDay { unit, .. }
                    if unit.raw() == 6
            ))
            .is_empty());
    }
    // Unit 6 on ready, in the contested hex, gains no rehearsal either, on
    // force 5's roster from day 6.
    let source = with(&text(), japanese);
    let run = Run::text(&source);
    for day in 6..=8 {
        assert_eq!(run.rehearsal(day, 6), n(0), "day {day}");
    }
    // Unit 5 is aboard at A with no man ashore: not in a contested hex, it
    // rehearses as ever.
    for day in 1..=8 {
        assert_eq!(run.rehearsal(day, 5), plain.rehearsal(day, 5), "day {day}");
    }
}

// ------------------------------------------------------------ The refusals

/// The scenario with a Japanese battalion 7 at C, with a base to feed it,
/// and an Allied unit 8 of one man at C with no base of its side there:
/// unit 8 is unfed on the night of day 1 and is destroyed.
fn with_a_foreign_and_a_doomed_unit(source: &str) -> String {
    with(
        source,
        "[[base]]\nid = 9\nname = \"Placeholder Japanese base\"\nside = \"japanese\"\nq = 0\nr = 6\nsupply_t = 100\n\n[[land_unit]]\nid = 7\nname = \"Placeholder Japanese battalion 7\"\nside = \"japanese\"\nmen = 100\nq = 0\nr = 6\n\n[[land_unit]]\nid = 8\nname = \"Placeholder doomed company 8\"\nside = \"allied\"\nmen = 1\nq = 0\nr = 6\n",
    )
}

fn refused(source: &str, day: u32) -> Vec<(EntityRef, RejectReason)> {
    Run::text(source).refusals(day)
}

#[test]
fn a_prepare_order_is_refused_for_a_force_that_is_not_the_sides() {
    // The Japanese side orders Allied force 1, and force 77 that no one has.
    let source = with_order(
        &text(),
        2,
        "japanese",
        "task_force = 1",
        "prepare_landing",
        "hex = [4, 0]\nroster = []",
    );
    let run = Run::text(&source);
    assert_eq!(
        run.refusals(2),
        vec![(EntityRef::TaskForce(force(1)), RejectReason::NotYourUnit)]
    );
    // Nothing changed: force 1 is as it is without the order.
    let plain = Run::text(&text());
    for day in 1..=12 {
        assert_eq!(run.loading(day, 1), plain.loading(day, 1));
    }
    let source = with_order(
        &text(),
        2,
        "allied",
        "task_force = 77",
        "prepare_landing",
        "hex = [4, 0]\nroster = []",
    );
    assert_eq!(
        refused(&source, 2),
        vec![(EntityRef::TaskForce(force(77)), RejectReason::NotYourUnit)]
    );
}

#[test]
fn a_prepare_order_is_refused_for_a_unit_that_is_not_the_sides() {
    let source = with_a_foreign_and_a_doomed_unit(&text());
    // The Japanese unit 7, and unit 99 that does not exist, in a roster with
    // unit 1 which is the side's.
    for raw in [7, 99] {
        let source = with_order(
            &source,
            2,
            "allied",
            "task_force = 2",
            "prepare_landing",
            &format!("hex = [4, 0]\nroster = [2, {raw}]"),
        );
        let run = Run::text(&source);
        assert_eq!(
            run.refusals(2),
            vec![(
                EntityRef::TaskForce(force(2)),
                RejectReason::NotYourLandUnit
            )],
            "unit {raw}"
        );
        // Nothing of the order was applied: force 2 prepares nothing, and the
        // roster of another force that unit 2 is not on is as it was.
        assert!(run.at(2).task_forces[&force(2)].preparation.is_none());
        assert!(run.all(|k| matches!(k, EventKind::LeftRoster { .. })).len() == 2);
    }
    // A unit on another force's roster (unit 1) is the side's: accepted.
    let source = with_order(
        &text(),
        2,
        "allied",
        "task_force = 2",
        "prepare_landing",
        "hex = [4, 0]\nroster = [2, 1]",
    );
    let run = Run::text(&source);
    assert!(run.refusals(2).is_empty());
    assert_eq!(run.roster(2, 2), vec![1, 2]);
    assert_eq!(run.roster(2, 1), Vec::<u32>::new());
}

#[test]
fn a_prepare_order_is_refused_for_a_hex_that_is_not_on_the_coast() {
    let block = |hex: &str| {
        with_order(
            &text(),
            2,
            "allied",
            "task_force = 2",
            "prepare_landing",
            &format!("hex = {hex}\nroster = [2]"),
        )
    };
    // Sea, off the map, and the first hex beside A: none is a coast hex.
    for (hex, shown) in [("[2, 0]", Hex::new(2, 0)), ("[50, 50]", Hex::new(50, 50))] {
        let run = Run::text(&block(hex));
        assert_eq!(
            run.refusals(2),
            vec![(
                EntityRef::TaskForce(force(2)),
                RejectReason::NotACoastHex(shown)
            )]
        );
        assert!(run.at(2).task_forces[&force(2)].preparation.is_none());
    }
    // A unit that is not the side's is refused first.
    let source = with_order(
        &text(),
        2,
        "allied",
        "task_force = 2",
        "prepare_landing",
        "hex = [2, 0]\nroster = [99]",
    );
    assert_eq!(
        refused(&source, 2),
        vec![(
            EntityRef::TaskForce(force(2)),
            RejectReason::NotYourLandUnit
        )]
    );
    // A coast hex with no base and no port is fine: C and B and A are all
    // accepted, and an empty roster is accepted.
    for hex in ["[0, 0]", "[4, 0]", "[0, 6]"] {
        for roster in ["[2]", "[]"] {
            let source = with_order(
                &text(),
                2,
                "allied",
                "task_force = 2",
                "prepare_landing",
                &format!("hex = {hex}\nroster = {roster}"),
            );
            assert!(refused(&source, 2).is_empty(), "{hex} {roster}");
        }
    }
}

#[test]
fn a_prepare_order_is_refused_for_a_destroyed_unit() {
    let source = with_a_foreign_and_a_doomed_unit(&text());
    let source = with_order(
        &source,
        2,
        "allied",
        "task_force = 2",
        "prepare_landing",
        "hex = [4, 0]\nroster = [2, 8]",
    );
    let run = Run::text(&source);
    // Unit 8 had one man, no base to feed it, and lost him on the night of
    // day 1.
    assert_eq!(
        run.on(
            1,
            |k| matches!(k, EventKind::LandUnitDestroyed { unit, .. } if unit.raw() == 8)
        )
        .len(),
        1
    );
    assert_eq!(
        run.refusals(2),
        vec![(
            EntityRef::TaskForce(force(2)),
            RejectReason::LandUnitDestroyed
        )]
    );
    assert!(run.at(2).task_forces[&force(2)].preparation.is_none());
}

#[test]
fn a_prepare_order_is_accepted_at_sea_and_with_an_empty_roster() {
    // Force 1 is at B on day 12, its unloading going on (it is complete at
    // hour 270): an order for another landing at C with an empty roster is
    // accepted, and the loading it had is lost, not spent.
    let source = with(&text(), &prepare(12, 1, C, "[]"));
    let run = Run::text(&source);
    assert!(run.refusals(12).is_empty());
    let lost = run.on(
        12,
        |k| matches!(k, EventKind::LoadingLost { task_force, .. } if task_force.raw() == 1),
    );
    assert_eq!(lost.len(), 1);
    assert_eq!(
        lost[0].kind,
        EventKind::LoadingLost {
            task_force: force(1),
            hex: B,
            loading_before: n(50),
            loading: n(0),
            reason: LoadingLoss::NewLandingPlace,
        }
    );
    // The force prepares for C with nobody on its roster, and it did not
    // gain: it is at sea of the day's work.
    let prep = run.at(12).task_forces[&force(1)]
        .preparation
        .as_ref()
        .expect("the new order stands");
    assert_eq!((prep.hex, prep.loading), (C, n(0)));
    assert!(prep.roster.is_empty());
}

#[test]
fn a_set_activity_order_is_refused_for_a_unit_that_is_not_the_sides_or_is_destroyed() {
    let source = with_a_foreign_and_a_doomed_unit(&text());
    // The Allied side orders the Japanese unit 7 and the nonexistent 77; the
    // Japanese side orders Allied unit 1; and the Allied side orders the
    // destroyed unit 8 on day 2.
    let source = with_order(
        &source,
        2,
        "allied",
        "land_unit = 7",
        "set_activity",
        "activity = \"train\"",
    );
    let source = with_order(
        &source,
        2,
        "allied",
        "land_unit = 77",
        "set_activity",
        "activity = \"train\"",
    );
    let source = with_order(
        &source,
        2,
        "japanese",
        "land_unit = 1",
        "set_activity",
        "activity = \"train\"",
    );
    let source = with_order(
        &source,
        2,
        "allied",
        "land_unit = 8",
        "set_activity",
        "activity = \"train\"",
    );
    let run = Run::text(&source);
    let mut refusals = run.refusals(2);
    refusals.sort_by_key(|(target, _)| format!("{target:?}"));
    assert_eq!(
        refusals,
        vec![
            (EntityRef::LandUnit(unit(1)), RejectReason::NotYourLandUnit),
            (EntityRef::LandUnit(unit(7)), RejectReason::NotYourLandUnit),
            (EntityRef::LandUnit(unit(77)), RejectReason::NotYourLandUnit),
            (
                EntityRef::LandUnit(unit(8)),
                RejectReason::LandUnitDestroyed
            ),
        ]
    );
    for raw in [1, 7, 8] {
        assert_eq!(
            run.at(2).land_units[&unit(raw)].activity,
            sim_core::orders::Activity::Ready
        );
    }
}

#[test]
fn a_set_activity_order_is_accepted_for_a_unit_with_every_man_aboard() {
    // Unit 1 is aboard force 1's ship: not refused as having no men ashore.
    let source = with_order(
        &text(),
        2,
        "allied",
        "land_unit = 1",
        "set_activity",
        "activity = \"train\"",
    );
    let run = Run::text(&source);
    assert!(run.refusals(2).is_empty());
    assert_eq!(
        run.at(2).land_units[&unit(1)].activity,
        sim_core::orders::Activity::Train
    );
    // It stays aboard: no training day, and no rehearsal gained while it is
    // set to train.
    assert_eq!(run.at(2).land_units[&unit(1)].training_days, 0);
    assert_eq!(run.rehearsal(1, 1), n(5));
    assert_eq!(run.rehearsal(2, 1), n(5));
}

// ------------------------------------------------------------- The properties

#[test]
fn property_more_days_of_preparation_never_land_a_forces_men_later() {
    // Force 1 is given its order on day 1, 3, 5, 7, 9 and 10: 10, 8, 6, 4, 2
    // and 1 days of preparation, and at last none.
    let hours_and_men = |start: Option<u32>| -> (Fixed, u32) {
        let source = match start {
            Some(day) => with(&without_force_1_orders(&text()), &prepare(day, 1, B, "[1]")),
            None => without_force_1_orders(&text()),
        };
        let run = Run::text(&source);
        let landed_by_day_11: u32 = run
            .landings()
            .iter()
            .filter(|(day, _, raw, _)| *day == 11 && *raw == 1)
            .map(|(_, _, _, men)| men)
            .sum();
        (
            run.completed_at(1).expect("force 1 completes"),
            landed_by_day_11,
        )
    };
    let mut previous: Option<(Fixed, u32)> = None;
    // From fewest days to most.
    for start in [None, Some(10), Some(9), Some(7), Some(5), Some(3), Some(1)] {
        let now = hours_and_men(start);
        if let Some(before) = previous {
            assert!(now.0 <= before.0, "{start:?}: {now:?} after {before:?}");
            assert!(now.1 >= before.1, "{start:?}: {now:?} after {before:?}");
        }
        previous = Some(now);
    }
    // And it does matter: none is slower than ten days.
    assert!(hours_and_men(None).0 > hours_and_men(Some(1)).0);
    assert_eq!(hours_and_men(Some(1)).0, n(270));
}

#[test]
fn property_loading_and_rehearsal_stay_within_0_to_100_and_are_never_copied() {
    // 30 days with no sailing: forces 1 and 3 and unit 1 and 3 pass 100 on
    // day 20 and stay at 100.
    let source = text()
        .split("# Day 11:")
        .next()
        .expect("the text before the sailing orders")
        .to_string();
    let source = common::replace_once(&source, "days = 12", "days = 35");
    let run = Run::text(&source);
    assert_eq!(run.len(), 35);
    for day in 0..=35u32 {
        let truth = run.at(day);
        for f in truth.task_forces.values() {
            if let Some(p) = &f.preparation {
                assert!(
                    p.loading >= n(0) && p.loading <= n(100),
                    "force {:?} day {day}",
                    f.id
                );
            }
        }
        for u in truth.land_units.values() {
            assert!(
                u.rehearsal >= n(0) && u.rehearsal <= n(100),
                "{:?} day {day}",
                u.id
            );
            assert_eq!(
                u.rehearsal_hex.is_some(),
                u.rehearsal > n(0),
                "{:?} day {day}",
                u.id
            );
        }
    }
    assert_eq!(run.loading(20, 1), n(100));
    assert_eq!(run.loading(35, 1), n(100));
    assert_eq!(run.rehearsal(35, 1), n(100));
    assert_eq!(run.loading(35, 4), n(100));
    // Day 35: 100 reached at day 25 for force 4, which started over on day 6.
    assert_eq!(run.loading(24, 4), n(95));
    assert_eq!(run.loading(25, 4), n(100));
    // Rehearsal is not copied by a split: see the split test; here, on a
    // longer run, a split on day 20 leaves each unit's rehearsal as it was.
    let split_source = with_order(
        &with_second_ship_in_force_5(&source),
        20,
        "allied",
        "task_force = 5",
        "detach",
        "ships = [5]",
    );
    let split = Run::text(&split_source);
    let plain = Run::text(&with_second_ship_in_force_5(&source));
    for day in 0..=35 {
        for raw in 1..=6 {
            assert_eq!(split.rehearsal(day, raw), plain.rehearsal(day, raw));
        }
        for f in split.at(day).task_forces.values() {
            if let Some(p) = &f.preparation {
                assert!(p.loading <= n(100));
            }
        }
    }
}

#[test]
fn property_the_enemys_view_is_the_same_in_two_worlds_that_differ_only_in_a_prepare_order() {
    // The Japanese side has a battalion at C, where force 3 lands on day 11,
    // so that it sees something. Force 1's orders to prepare are taken out
    // of one world.
    let japanese = "[[base]]\nid = 9\nname = \"Placeholder Japanese base\"\nside = \"japanese\"\nq = 0\nr = 6\nsupply_t = 1000\n\n[[land_unit]]\nid = 7\nname = \"Placeholder Japanese battalion 7\"\nside = \"japanese\"\nmen = 100\nq = 0\nr = 6\n";
    let prepared = Run::text(&with(&text(), japanese));
    let unprepared = Run::text(&with(&without_force_1_orders(&text()), japanese));
    // The worlds differ: force 1 landed at another rate.
    assert_ne!(prepared.completed_at(1), unprepared.completed_at(1));
    assert_ne!(
        digest(&prepared.scenario, Side::Allied),
        digest(&unprepared.scenario, Side::Allied)
    );
    let seen = digest(&prepared.scenario, Side::Japanese);
    assert_eq!(seen, digest(&unprepared.scenario, Side::Japanese));
    // And it is not nothing: the Japanese side sees its own events and
    // ground reports of the landing at C.
    assert!(
        seen.contains("GroundReport") || seen.contains("Whole"),
        "{seen}"
    );
    // It sees nothing of the Allied preparation: no event of the kinds of
    // 0095, and no cause of the Allied orders.
    for e in prepared.scenario.world.side_view(Side::Japanese).events() {
        if let SideEventKind::Whole(kind) = &e.kind {
            assert!(!is_preparation_event(kind), "{kind:?}");
            assert!(
                !matches!(kind, EventKind::LandUnitFatigue { .. }),
                "{kind:?}"
            );
        }
    }
    // The Allied side sees all its own.
    let own: Vec<_> = prepared
        .scenario
        .world
        .side_view(Side::Allied)
        .events()
        .into_iter()
        .filter_map(|e| match e.kind {
            SideEventKind::Whole(kind) if is_preparation_event(&kind) => Some(kind),
            _ => None,
        })
        .collect();
    assert_eq!(
        own.len(),
        prepared.all(is_preparation_event).len(),
        "every event of the kinds is seen whole by its own side"
    );
}

#[test]
fn property_with_the_frozen_values_the_rule_is_off_and_the_calculation_runs_as_before() {
    // The frozen figures as they are: share 1, fatigue 0.
    let run = Run::of(common::load(&path()).expect("loads on the frozen figures"));
    let g = run.scenario.world.ref_data().ground.expect("figures");
    assert_eq!(g.unprepared_landing_share, n(1));
    assert_eq!(g.unprepared_landing_fatigue, n(0));
    // Every landing is at the port's rate, 20 t an hour: force 1's 600 men
    // and 60 t take 210 t, 10.5 hours from hour 256; force 2's 150 t take
    // 7.5; force 3's 50 t take 2.5 from hour 255.
    assert_eq!(run.completed_at(1), Some(fx("266.5")));
    assert_eq!(run.completed_at(2), Some(fx("263.5")));
    assert_eq!(run.completed_at(3), Some(fx("257.5")));
    // No landing tires anyone: nothing is recorded, and so no fatigue at
    // all, with the rest of 5 a day below 0 leaving them at 0.
    for day in 0..=12 {
        for raw in 1..=6 {
            assert_eq!(run.fatigue(day, raw), n(0), "unit {raw} day {day}");
        }
    }
    assert!(run
        .all(|k| matches!(
            k,
            EventKind::LandUnitFatigue {
                change: LandFatigueChange::Landed,
                ..
            }
        ))
        .is_empty());
    // No breakdown names the share or the loading factor: they are as before.
    for e in &run.events {
        assert!(!has_factor(e, "unprepared_landing_share"), "{e:?}");
        assert!(!has_factor(e, "loading_factor"), "{e:?}");
    }
    // The rest of the rule is not a switch: the loading and rehearsal grow,
    // and the men leave their rosters when they land.
    assert_eq!(run.loading(10, 1), n(50));
    assert_eq!(run.rehearsal(10, 1), n(50));
    assert_eq!(run.rehearsal(11, 1), n(0));
}

#[test]
fn property_with_the_frozen_values_every_scenario_with_land_units_records_none_of_the_new_rules() {
    // The meaning of "gives what it gave before apart from the hash version"
    // without a run from before: on the frozen figures, every scenario that
    // has land units and none of the new orders (prepare_landing and
    // set_activity) records no event of the new rules but the two a fight
    // always records (a day of fighting counted and, after ten, a step), no
    // fatigue of landing, and no breakdown naming a figure of the new rules;
    // every shot of a fight is at the side's chance, whatever the unit's
    // experience, and every landing is at the port's rate. The last is
    // checked in scenario 27's frozen run above.
    let mut checked = Vec::new();
    for entry in std::fs::read_dir(common::data_dir().join("scenarios")).expect("the folder reads")
    {
        let file = entry.expect("an entry").path();
        if file.extension().is_none_or(|e| e != "toml") {
            continue;
        }
        let source = std::fs::read_to_string(&file).expect("reads");
        if !source.contains("[[land_unit]]")
            || source.contains("prepare_landing")
            || source.contains("set_activity")
        {
            continue;
        }
        let mut scenario = common::load(&file).expect("loads on the frozen figures");
        let g = scenario.world.ref_data().ground.expect("ground figures");
        while scenario.next_day().expect("a day resolves").is_some() {}
        let name = file
            .file_name()
            .expect("a name")
            .to_string_lossy()
            .to_string();
        for e in scenario.world.events().iter() {
            assert!(!is_preparation_event(&e.kind), "{name}: {e:?}");
            assert!(
                !matches!(
                    e.kind,
                    EventKind::TrainingDay { .. }
                        | EventKind::LandUnitFatigue {
                            change: LandFatigueChange::Landed,
                            ..
                        }
                ),
                "{name}: {e:?}"
            );
            for factor in [
                "unprepared_landing_share",
                "loading_factor",
                "experience_factor",
                "side_chance",
            ] {
                assert!(!has_factor(e, factor), "{name}: {e:?}");
            }
            if let EventKind::GroundFire { chance, .. } = e.kind {
                assert!(
                    chance == g.hit_chance || chance == g.defender_hit_chance,
                    "{name}: {e:?}"
                );
            }
        }
        checked.push(name);
    }
    assert!(
        checked.len() >= 4,
        "scenarios 21, 23, 24 and 25 at least: {checked:?}"
    );
}
