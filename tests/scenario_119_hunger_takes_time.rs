//! Scenario 119: hunger takes time. Men land with rations, a unit counts
//! its nights of hunger, and men are lost only from a unit that has been
//! hungry for many nights.
//!
//! Proves decision 0205. The statements are the architect's, written
//! before the code. A mismatch goes back to the architect, and nobody else
//! changes a statement or a number here.
//!
//! # The setting
//!
//! A placeholder scenario made for the test, on a placeholder map, with
//! no real unit. Its ground figures are in a file of its own: the daily
//! supply for a man is 0.005 t, `rations_landed_nights` is 3,
//! `hungry_below_share` is 0.5, `hunger_nights_before_loss` is 14 and
//! `unfed_loss_share_per_day` is 0.05. Every land unit has the standard
//! weight. Nothing fights and nothing is bombed.
//!
//! Side A has a base Q, with no aircraft, and a land unit V of 1,000 men
//! ashore there at the start. The men's charge at Q is 5 t a night. Q's
//! stock is what each statement gives it, and nothing is landed there
//! unless a statement says so. Side B has nothing ashore.
//!
//! # Statements: nights of hunger
//!
//! 1. **Hungry, and nobody lost.** With no stock at Q, V's fed share is 0
//!    every night. Its count is 1 after the first night and 14 after the
//!    fourteenth, and through those fourteen nights V loses no man and
//!    has 1,000.
//! 2. **The fifteenth night.** That night the count is 15, above 14. V's
//!    unfed are 1,000 and it loses 1,000 times 0.05, 50 men; 950 remain.
//!    On the sixteenth it loses 950 times 0.05, 47.5 cut to 47; 903
//!    remain.
//! 3. **By today's rule,** with the same scenario on a ground file
//!    without the three figures, V loses 50 men on the first night.
//! 4. **Short rations are not hunger.** With 3 t at Q, the first night
//!    feeds 600 of the 1,000: a fed share of 0.6, not below a half. The
//!    count stays at nothing and nobody is lost. With 2.5 t the share is
//!    exactly 0.5 and the count stays. With 2 t the share is 0.4 and the
//!    count is 1.
//! 5. **The count falls only on a full night.** With no stock for five
//!    nights the count is 5. Supply for V then reaches Q on day 6: 3.5 t,
//!    then 5 t on each of days 7 and 8. On the sixth night 700 are fed, a
//!    share of 0.7, and the count stays 5. On the seventh and the eighth
//!    all are fed, and the count is 4 and then 3.
//! 6. **A count given at the start.** With V given a count of 14 at the
//!    start and no stock at Q, V loses 50 men on the first night.
//!
//! # Statements: who is lost
//!
//! 7. **Only the units past the count.** V, 1,000 men with a count of 14
//!    at the start, and a second unit W, 500 men with none, are at Q with
//!    no stock. On the first night V's count is 15 and W's is 1. The loss
//!    is worked on V's men alone: 1,000 unfed, 50 lost, all from V. W
//!    loses nobody. By today's rule the hex would lose 1,500 times 0.05,
//!    75: 50 from V and 25 from W.
//! 8. **One man at least.** With V of 10 men and a count of 14 at the
//!    start and no stock, the first night's loss is 10 times 0.05, 0.5,
//!    cut to nothing, and one man at least: V loses 1 and has 9.
//! 9. **A unit aboard keeps its count, and a unit divided takes it.**
//!    With V's count at 5, V put aboard ship keeps 5 while it has no man
//!    ashore; a unit made by dividing V starts with 5.
//!
//! # Statements: rations landed
//!
//! Side A has a force T of two transports with a land unit U of 3,000
//! men aboard, and a hex P where A has no base.
//!
//! 10. **The landing brings three nights.** When T lands its first man at
//!     P a base is formed there, and 3,000 times 3 times 0.005 t, 45 t of
//!     supply, are in its stock at that moment, whatever number of men
//!     are ashore by then. The record of it names 3,000 men, 3 nights and
//!     45 t, and cites the landing.
//! 11. **A base is given rations once.** A second force with 1,000 men
//!     landing at P on a later day adds nothing to P's stock but what it
//!     unloads. A force landing 1,000 men at Q, a base at the start at
//!     which no men have been landed, brings 1,000 times 3 times 0.005
//!     t, 15 t, at its first man; a force landing men at Q after that
//!     brings none. (Corrected by the architect on Nov 21, 2026: as first
//!     written a base that stood got none, which left the 39 days'
//!     landing at Lunga, where the Allied base stands before the men
//!     land, with no rations at all.)
//! 12. **With the rations, the first nights are fed.** With all of U
//!     ashore at P before the first night, the charge is 15 t a night:
//!     the three nights after the landing are fed in full, the stock is
//!     then nothing, and U's count is still nothing after them.
//! 13. **Without the figures, a landing brings nothing,** and the base is
//!     formed with no supply, as today.
//!
//! # Properties
//!
//! - **No game on file moves.** Every scenario and recorded game on file
//!   gives the events it gave before, and the daily hashes it gave before
//!   but for the state's layout version.
//! - **Nothing is drawn.** The same game played twice gives the same
//!   events.
//! - **Each side sees its own.** B's view gives none of A's rations, A's
//!   counts or A's losses to hunger.
//! - **The table's copies are true copies.** Each scenario the table
//!   plays differs from the file it was made from in the line that names
//!   its ground figures and in nothing else.

// The architect's statement lists are list items in the doc comment above,
// which is left as written.
#![allow(clippy::doc_lazy_continuation)]

// ---------------------------------------------------------------- The harness
//
// How the scenario is set up (data/scenarios/hunger-takes-time/
// 119_hunger_takes_time.toml, on scenario 21's placeholder map,
// data/map/placeholder_landing.toml):
//
// - Side A is the Allied side. Its base Q is base 1 at (10,0), a coast hex of
//   port size 0, with no stock, and its land unit V is land unit 1, 1,000 men
//   ashore there. Side B, the Japanese, has nothing. The game is 16 days.
// - P is the coast hex (8,0), two hexes from Q, where A has no base. A
//   statement of landing adds a force T, task force 1, of five placeholder
//   troop transports (class 10, 600 men each, ships 1 to 5) with U, land unit
//   2, 3,000 men, aboard, stopped at sea at (7,0), one hex from P, with the
//   orders on day 1 to go to P and unload. It lands in the night pulse of day
//   1, after the night's charge. The base it forms is base 2.
// - A night of a day is that day's night pulse. The men's supply is charged
//   first in it, before movement, and a supply line delivers before the
//   charge. "The count after night N" is read at the start of day N's
//   morning pulse.
// - The ground figures are scenario 119's file, which is the frozen step 1
//   `ground.toml` with the three figures of hunger added (the test loads it
//   from `data/reference/step1`). "Today's rule" is the same file without the
//   three figures, made by `without_the_figures`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use sim_core::events::{Cause, Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, EventId, LandUnitId};
use sim_core::side_events::SideEventKind;
use sim_core::state::{Side, TruthState};
use sim_core::time::Pulse;
use sim_core::Staffs;
use sim_data::{LoadOptions, Scenario};

mod common;

/// P, the empty shore, and Q, the base at the start.
const P: Hex = Hex::new(8, 0);
const Q: Hex = Hex::new(10, 0);

fn path() -> PathBuf {
    common::data_dir().join("scenarios/hunger-takes-time/119_hunger_takes_time.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn changed(text: &str, original: &str, replacement: &str) -> String {
    common::replace_once(text, original, replacement)
}

/// Adds `what` at the mark `# MARK: name`; the mark stays.
fn at(text: &str, name: &str, what: &str) -> String {
    let mark = format!("# MARK: {name}\n");
    changed(text, &mark, &format!("{what}{mark}"))
}

fn fx(text: &str) -> Fixed {
    text.parse::<Fixed>().expect("a decimal figure")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn unit(raw: u32) -> LandUnitId {
    LandUnitId::new(raw)
}

fn base(raw: u32) -> BaseId {
    BaseId::new(raw)
}

// ---------------------------------------------------------------- Variants

/// Q's stock at the start, in tons (a whole or decimal figure).
fn with_stock(text: &str, tons: &str) -> String {
    changed(text, "supply_t = 0\n", &format!("supply_t = \"{tons}\"\n"))
}

/// V given a count of hunger at the start.
fn with_count(text: &str, nights: u32) -> String {
    at(text, "v", &format!("hunger_nights = {nights}\n"))
}

/// V's men, not 1,000.
fn with_men(text: &str, men: u32) -> String {
    changed(text, "men = 1000\n", &format!("men = {men}\n"))
}

/// A second land unit at Q.
fn with_unit_at_q(text: &str, id: u32, men: u32, nights: u32) -> String {
    at(
        text,
        "units",
        &format!(
            "[[land_unit]]\nid = {id}\nname = \"Placeholder unit {id}\"\nside = \"allied\"\nmen = {men}\nq = 10\nr = 0\nhunger_nights = {nights}\n\n"
        ),
    )
}

/// The game is `days` days, not 16.
fn with_days(text: &str, days: u32) -> String {
    changed(text, "days = 16\n", &format!("days = {days}\n"))
}

/// A supply line to Q.
fn with_line(text: &str, tons: &str, first: u32, last: u32) -> String {
    at(
        text,
        "lines",
        &format!(
            "[[supply_line]]\nbase = 1\nsupply_t = \"{tons}\"\nfirst_day = {first}\nlast_day = {last}\n\n"
        ),
    )
}

/// A force of `ships` placeholder troop transports (class 10), the first
/// numbered `first_ship`, at (q,0), with `cargo_t` of supply in its first ship.
fn with_force(text: &str, tf: u32, first_ship: u32, ships: u32, q: i32, cargo_t: u32) -> String {
    let mut block = format!(
        "[[task_force]]\nid = {tf}\nname = \"Placeholder force {tf}\"\nside = \"allied\"\nq = {q}\nr = 0\nmission = \"transport\"\n\n"
    );
    for i in 0..ships {
        let id = first_ship + i;
        let cargo = if i == 0 && cargo_t > 0 {
            format!("supply_t = {cargo_t}\n")
        } else {
            String::new()
        };
        block.push_str(&format!(
            "[[ship]]\nid = {id}\nname = \"Placeholder troop transport {id}\"\nclass = 10\nside = \"allied\"\nfuel_t = 1000\n{cargo}task_force = {tf}\n\n"
        ));
    }
    at(text, "forces", &block)
}

/// A land unit aboard a force.
fn with_unit_aboard(text: &str, id: u32, men: u32, tf: u32) -> String {
    at(
        text,
        "forces",
        &format!(
            "[[land_unit]]\nid = {id}\nname = \"Placeholder unit {id}\"\nside = \"allied\"\nmen = {men}\ntask_force = {tf}\n\n"
        ),
    )
}

/// The orders on `day` for a force: route to (dest,0) at cruise and unload.
fn with_go(text: &str, tf: u32, day: u32, dest: i32) -> String {
    let order = |kind: &str, extra: &str| {
        format!(
            "[[order]]\nday = {day}\nside = \"allied\"\ntask_force = {tf}\nkind = \"{kind}\"\n{extra}\n"
        )
    };
    at(
        text,
        "orders",
        &format!(
            "{}{}{}",
            order("set_route", &format!("waypoints = [[{dest}, 0]]\n")),
            order("set_speed", "speed = \"cruise\"\n"),
            order("unload", "")
        ),
    )
}

/// A load order on `day`: the force takes men of `unit`, and no cargo.
fn with_load(text: &str, tf: u32, day: u32, unit: u32) -> String {
    at(
        text,
        "orders",
        &format!(
            "[[order]]\nday = {day}\nside = \"allied\"\ntask_force = {tf}\nkind = \"load\"\nunit = {unit}\ncargo = \"none\"\n\n"
        ),
    )
}

/// T and U of the statements of landing: T lands U at P in the night of day 1.
fn with_t_and_u(text: &str) -> String {
    let text = with_force(text, 1, 1, 5, 7, 0);
    let text = with_unit_aboard(&text, 2, 3000, 1);
    with_go(&text, 1, 1, 8)
}

/// The scenario's ground figures without the three of hunger: today's rule.
fn without_the_figures() -> LoadOptions {
    common::altered_reference("no_hunger", |root| {
        common::edit_file_by(
            root,
            "ground_placeholder_scenario_119.toml",
            &[
                ("rations_landed_nights = 3\n", ""),
                ("hungry_below_share = \"0.5\"\n", ""),
                ("hunger_nights_before_loss = 14\n", ""),
            ],
        );
    })
}

// ---------------------------------------------------------------- Playing

/// A game played to its end: the events, the truth at the start (`ends[0]`)
/// and at the end of each day, and the truth at the start of each pulse.
struct Played {
    events: Vec<Event>,
    ends: Vec<TruthState>,
    pulses: BTreeMap<(u32, Pulse), TruthState>,
    hashes: Vec<u64>,
    scenario: Scenario,
}

fn load_with(text: &str, options: &LoadOptions) -> Scenario {
    common::from_text_with(text, &path(), options).expect("the scenario loads")
}

fn play(text: &str) -> Played {
    play_with(text, &common::frozen())
}

fn play_with(text: &str, options: &LoadOptions) -> Played {
    play_scenario(load_with(text, options))
}

fn play_scenario(mut scenario: Scenario) -> Played {
    let mut ends = vec![scenario.world.umpire().clone()];
    let mut pulses = BTreeMap::new();
    let mut hashes = Vec::new();
    loop {
        let summary = scenario
            .next_day_watched(Staffs::none(), &mut |world, moment| {
                if let sim_core::choices::Moment::PulseBegins { day, pulse } = moment {
                    pulses.insert((day, pulse), world.umpire().clone());
                }
            })
            .expect("a day resolves");
        match summary {
            Some(summary) => hashes.push(summary.state_hash),
            None => break,
        }
        ends.push(scenario.world.umpire().clone());
    }
    Played {
        events: scenario.world.events().iter().cloned().collect(),
        ends,
        pulses,
        hashes,
        scenario,
    }
}

impl Played {
    /// The truth at the end of `day` (0 is the start).
    fn end(&self, day: usize) -> &TruthState {
        &self.ends[day]
    }

    /// The truth at the start of the pulse (so after every earlier pulse).
    fn before(&self, day: u32, pulse: Pulse) -> &TruthState {
        &self.pulses[&(day, pulse)]
    }

    /// The count of nights of hunger of `u` at the end of `day`.
    fn count(&self, day: usize, u: u32) -> u32 {
        self.end(day).land_units[&unit(u)].hunger_nights
    }

    fn men(&self, day: usize, u: u32) -> u32 {
        self.end(day).land_units[&unit(u)].men_ashore
    }

    fn stock(&self, day: usize, b: u32) -> Fixed {
        self.end(day).bases[&base(b)].supply_t
    }

    /// The `MenFed` of the hex as (day, men, fed, unfed), in order.
    fn fed(&self, hex: Hex) -> Vec<(u32, u32, u32, u32)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::MenFed {
                    hex: h,
                    men,
                    fed,
                    unfed,
                    ..
                } if *h == hex => Some((e.day, *men, *fed, *unfed)),
                _ => None,
            })
            .collect()
    }

    /// The `HungerNightsChanged` of unit `u` as (day, before, nights).
    fn changes(&self, u: u32) -> Vec<(u32, u32, u32)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::HungerNightsChanged {
                    unit: who,
                    before,
                    nights,
                    ..
                } if *who == unit(u) => Some((e.day, *before, *nights)),
                _ => None,
            })
            .collect()
    }

    /// The `MenLostUnfed` of unit `u` as (day, lost, ashore).
    fn lost(&self, u: u32) -> Vec<(u32, u32, u32)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::MenLostUnfed {
                    unit: who,
                    lost,
                    ashore,
                    ..
                } if *who == unit(u) => Some((e.day, *lost, *ashore)),
                _ => None,
            })
            .collect()
    }

    /// Every `RationsLanded` as the event and its (base, men, nights, tons).
    fn rations(&self) -> Vec<(&Event, BaseId, u32, Fixed, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::RationsLanded {
                    base,
                    men,
                    nights,
                    supply_t,
                    ..
                } => Some((e, *base, *men, *nights, *supply_t)),
                _ => None,
            })
            .collect()
    }

    fn event(&self, id: EventId) -> &Event {
        self.events
            .iter()
            .find(|e| e.id == id)
            .expect("the cited event is in the log")
    }

    /// The `BaseFormed` events as (event, base, hex).
    fn formed(&self) -> Vec<(&Event, BaseId, Hex)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::BaseFormed { base, hex, .. } => Some((e, *base, *hex)),
                _ => None,
            })
            .collect()
    }
}

/// The ids an event cites as causes.
fn cited(event: &Event) -> Vec<EventId> {
    event
        .causes
        .iter()
        .filter_map(|c| match c {
            Cause::Event(id) => Some(*id),
            Cause::Order(_) => None,
        })
        .collect()
}

/// The setting is what the comment says.
#[test]
fn the_setting_is_what_the_comment_says() {
    let scenario = load_with(&text(), &common::frozen());
    let truth = scenario.world.umpire();
    let figures = scenario
        .world
        .ref_data()
        .ground
        .as_ref()
        .expect("ground figures");
    assert_eq!(figures.daily_supply_t_per_man, fx("0.005"));
    assert_eq!(figures.unfed_loss_share_per_day, fx("0.05"));
    let hunger = figures
        .hunger
        .as_ref()
        .expect("the three figures are given");
    assert_eq!(hunger.rations_landed_nights, n(3));
    assert_eq!(hunger.hungry_below_share, fx("0.5"));
    assert_eq!(hunger.hunger_nights_before_loss, 14);
    assert_eq!(truth.bases.len(), 1);
    assert_eq!(truth.bases[&base(1)].hex, Q);
    assert_eq!(truth.bases[&base(1)].supply_t, Fixed::ZERO);
    assert_eq!(truth.land_units.len(), 1);
    assert_eq!(truth.land_units[&unit(1)].men_ashore, 1000);
    assert_eq!(truth.land_units[&unit(1)].hunger_nights, 0);
    assert_eq!(scenario.days, 16);
}

// ---------------------------------------------------------------- Statements: nights of hunger

/// 1. Hungry, and nobody lost: with no stock the count is 1 after the first
/// night and 14 after the fourteenth, and V has 1,000 through those nights.
#[test]
fn statement_1_hungry_and_nobody_lost() {
    let played = play(&text());
    for day in 1..=14usize {
        assert_eq!(played.count(day, 1), day as u32, "count after night {day}");
        assert_eq!(played.men(day, 1), 1000, "men after night {day}");
    }
    let fed = played.fed(Q);
    for (i, row) in fed.iter().enumerate().take(14) {
        assert_eq!(*row, (i as u32 + 1, 1000, 0, 1000), "night {}", i + 1);
    }
    let changes = played.changes(1);
    for day in 1..=14u32 {
        assert_eq!(changes[day as usize - 1], (day, day - 1, day));
    }
    assert!(
        played.lost(1).iter().all(|l| l.0 > 14),
        "no loss before the fifteenth night"
    );
}

/// 1, cited: a change of count cites that night's `MenFed`.
#[test]
fn statement_1_a_change_cites_the_nights_men_fed() {
    let played = play(&with_days(&text(), 3));
    for e in played
        .events
        .iter()
        .filter(|e| matches!(e.kind, EventKind::HungerNightsChanged { .. }))
    {
        assert_eq!(e.pulse, Some(Pulse::Night));
        let causes = cited(e);
        assert_eq!(causes.len(), 1);
        let cause = played.event(causes[0]);
        assert!(matches!(cause.kind, EventKind::MenFed { hex: Q, .. }));
        assert_eq!(cause.day, e.day);
    }
}

/// 2. The fifteenth night: the count is 15, V's unfed are 1,000 and it loses
/// 50; on the sixteenth it loses 47 of its 950.
#[test]
fn statement_2_the_fifteenth_night() {
    let played = play(&text());
    assert_eq!(played.count(15, 1), 15);
    assert_eq!(played.lost(1)[0], (15, 50, 950));
    assert_eq!(played.men(15, 1), 950);
    assert_eq!(played.count(16, 1), 16);
    assert_eq!(played.lost(1)[1], (16, 47, 903));
    assert_eq!(played.men(16, 1), 903);
    assert_eq!(played.lost(1).len(), 2);
    // The `MenFed` of the fifteenth night is of 1,000 men, all unfed.
    assert_eq!(played.fed(Q)[14], (15, 1000, 0, 1000));
    assert_eq!(played.fed(Q)[15], (16, 950, 0, 950));
}

/// 3. By today's rule, with the same scenario on a ground file without the
/// three figures, V loses 50 men on the first night.
#[test]
fn statement_3_by_todays_rule() {
    let played = play_with(&with_days(&text(), 3), &without_the_figures());
    assert!(played
        .scenario
        .world
        .ref_data()
        .ground
        .as_ref()
        .unwrap()
        .hunger
        .is_none());
    assert_eq!(played.lost(1)[0], (1, 50, 950));
    assert_eq!(played.men(1, 1), 950);
    // No count is kept or recorded.
    assert!(played.changes(1).is_empty());
    assert_eq!(played.count(3, 1), 0);
}

/// 4. Short rations are not hunger: with 3 t the first night feeds 600 and
/// the count stays at nothing; with 2.5 t the share is exactly 0.5 and the
/// count stays; with 2 t the share is 0.4 and the count is 1.
#[test]
fn statement_4_short_rations_are_not_hunger() {
    for (stock, fed, count) in [("3", 600, 0), ("2.5", 500, 0), ("2", 400, 1)] {
        let played = play(&with_days(&with_stock(&text(), stock), 2));
        assert_eq!(
            played.fed(Q)[0],
            (1, 1000, fed, 1000 - fed),
            "the first night with {stock} t"
        );
        assert_eq!(played.count(1, 1), count, "the count with {stock} t");
        let first_night: Vec<_> = played.changes(1).into_iter().filter(|c| c.0 == 1).collect();
        assert_eq!(first_night.len() as u32, count);
        // The loss is not worked: nobody is past the count.
        assert!(played.lost(1).is_empty());
        assert_eq!(played.men(1, 1), 1000);
        assert_eq!(played.stock(1, 1), Fixed::ZERO);
    }
}

/// 5. The count falls only on a full night: with no stock for five nights
/// the count is 5; supply reaches Q on day 6 (3.5 t) and on days 7 and 8
/// (5 t); on the sixth night 700 are fed and the count stays 5; on the
/// seventh and eighth all are fed and the count is 4 and then 3.
#[test]
fn statement_5_the_count_falls_only_on_a_full_night() {
    let text = with_line(&with_line(&text(), "3.5", 6, 6), "5", 7, 8);
    let played = play(&with_days(&text, 8));
    for day in 1..=5usize {
        assert_eq!(played.count(day, 1), day as u32);
    }
    // The supply is delivered in the night, before the men are fed.
    let night6: Vec<&Event> = played
        .events
        .iter()
        .filter(|e| e.day == 6 && e.pulse == Some(Pulse::Night))
        .collect();
    let received = night6
        .iter()
        .position(|e| matches!(e.kind, EventKind::StockReceived { base: b, .. } if b == base(1)))
        .expect("the stock is received on night 6");
    let fed = night6
        .iter()
        .position(|e| matches!(e.kind, EventKind::MenFed { .. }))
        .expect("the men are fed on night 6");
    assert!(received < fed, "the supply is in before the men are fed");
    assert_eq!(played.fed(Q)[5], (6, 1000, 700, 300));
    assert_eq!(played.count(6, 1), 5);
    assert_eq!(played.fed(Q)[6], (7, 1000, 1000, 0));
    assert_eq!(played.count(7, 1), 4);
    assert_eq!(played.fed(Q)[7], (8, 1000, 1000, 0));
    assert_eq!(played.count(8, 1), 3);
    // The count changed on nights 1 to 5, not 6, and on 7 and 8.
    let days: Vec<u32> = played.changes(1).iter().map(|c| c.0).collect();
    assert_eq!(days, vec![1, 2, 3, 4, 5, 7, 8]);
    assert_eq!(played.changes(1)[5], (7, 5, 4));
    assert_eq!(played.changes(1)[6], (8, 4, 3));
    // Nobody is lost.
    assert!(played.lost(1).is_empty());
}

/// 6. A count given at the start: with V's count 14 at the start and no stock,
/// V loses 50 men on the first night.
#[test]
fn statement_6_a_count_given_at_the_start() {
    let played = play(&with_days(&with_count(&text(), 14), 2));
    assert_eq!(played.end(0).land_units[&unit(1)].hunger_nights, 14);
    assert_eq!(played.changes(1)[0], (1, 14, 15));
    assert_eq!(played.lost(1)[0], (1, 50, 950));
    assert_eq!(played.men(1, 1), 950);
}

// ---------------------------------------------------------------- Statements: who is lost

/// 7. Only the units past the count: V (1,000 men, count 14) and W (500, none)
/// at Q with no stock. On the first night V's count is 15 and W's is 1; the
/// loss is worked on V's men alone: 50, all from V; W loses nobody. By today's
/// rule the hex loses 75: 50 from V and 25 from W.
#[test]
fn statement_7_only_the_units_past_the_count() {
    let text = with_days(&with_unit_at_q(&with_count(&text(), 14), 3, 500, 0), 2);
    let played = play(&text);
    assert_eq!(played.fed(Q)[0], (1, 1500, 0, 1500));
    assert_eq!(played.count(1, 1), 15);
    assert_eq!(played.count(1, 3), 1);
    assert_eq!(played.lost(1)[0], (1, 50, 950));
    assert_eq!(played.men(1, 1), 950);
    assert_eq!(played.men(1, 3), 500);
    assert!(
        played.lost(3).is_empty(),
        "W loses nobody: no loss is recorded for it"
    );
    // By today's rule.
    let old = play_with(&text, &without_the_figures());
    assert_eq!(old.lost(1)[0], (1, 50, 950));
    assert_eq!(old.lost(3)[0], (1, 25, 475));
    assert_eq!(old.men(1, 1), 950);
    assert_eq!(old.men(1, 3), 475);
}

/// 8. One man at least: V of 10 men with a count of 14 and no stock: 10 times
/// 0.05 is 0.5, cut to nothing, and one man at least: V loses 1 and has 9.
#[test]
fn statement_8_one_man_at_least() {
    let text = with_days(&with_count(&with_men(&text(), 10), 14), 2);
    let played = play(&text);
    assert_eq!(played.fed(Q)[0], (1, 10, 0, 10));
    assert_eq!(played.lost(1)[0], (1, 1, 9));
    assert_eq!(played.men(1, 1), 9);
}

/// 9. A unit aboard keeps its count: V's count is 5 and V is wholly aboard
/// ship, with no man ashore; the count stays 5 over the nights and nothing
/// is recorded.
#[test]
fn statement_9_a_unit_aboard_keeps_its_count() {
    let text = with_count(&text(), 5);
    // V aboard a force of two transports lying at sea at (9,0), no orders.
    let text = changed(
        &text,
        "men = 1000\nq = 10\nr = 0\n",
        "men = 1000\ntask_force = 1\n",
    );
    let text = with_force(&text, 1, 1, 2, 9, 0);
    let played = play(&with_days(&text, 4));
    for day in 0..=4usize {
        assert_eq!(played.count(day, 1), 5, "the count at the end of day {day}");
        assert_eq!(played.men(day, 1), 0, "no man of V is ashore");
    }
    assert!(played.changes(1).is_empty());
    assert!(played.fed(Q).is_empty());
}

/// 9, divided: V's count is 5 at Q; 600 of its men are loaded into one
/// transport (V has 1,000: fewer berths than men), carried to P and landed;
/// the unit made by dividing V starts with 5.
///
/// V is kept at 5 by the stock: 3 t at the start (night 1: 600 of 1,000 fed,
/// a share of 0.6, the count stays) and a line of 1.5 t on day 2 (night 2:
/// 300 of the 400 ashore fed, 0.75, the count stays). The men are loaded in
/// night 1 after the charge; the force goes in night 2 after the charge, 80
/// nm in 8 hours, and lands in the last 4.
#[test]
fn statement_9_a_unit_divided_takes_the_count() {
    let text = with_count(&with_stock(&text(), "3"), 5);
    let text = with_line(&text, "1.5", 2, 2);
    let text = with_force(&text, 1, 1, 1, 10, 0);
    let text = with_load(&text, 1, 1, 1);
    let text = with_go(&text, 1, 2, 8);
    let played = play(&with_days(&text, 3));
    // V's nights 1 and 2 leave its count at 5.
    assert_eq!(played.fed(Q)[0], (1, 1000, 600, 400));
    assert_eq!(played.fed(Q)[1], (2, 400, 300, 100));
    assert_eq!(played.count(1, 1), 5);
    // The division, in night 2.
    let divided: Vec<(&Event, u32)> = played
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::UnitDivided {
                from,
                unit: new,
                men,
                ..
            } if *from == unit(1) => Some((e, *men)),
            _ => None,
        })
        .collect();
    assert_eq!(divided.len(), 1);
    assert_eq!(divided[0].0.day, 2);
    assert_eq!(divided[0].1, 600);
    let new = match &divided[0].0.kind {
        EventKind::UnitDivided { unit: new, .. } => *new,
        _ => unreachable!(),
    };
    assert_eq!(new, unit(2));
    let after = played.before(2, Pulse::Morning);
    assert_eq!(after.land_units[&new].hunger_nights, 5);
    // 4 hours are left of the pulse: 80 t of work, 320 men landed of the 600.
    assert_eq!(after.land_units[&new].men_ashore, 320);
    assert_eq!(after.land_units[&new].hex, Some(P));
    assert_eq!(after.land_units[&unit(1)].hunger_nights, 5);
    assert_eq!(after.land_units[&unit(1)].men_ashore, 400);
}

// ---------------------------------------------------------------- Statements: rations landed

/// 10. The landing brings three nights: when T lands its first man at P a
/// base is formed there and 3,000 x 3 x 0.005 t, 45 t, are in its stock at
/// that moment. The record names 3,000 men, 3 nights and 45 t, and cites the
/// landing.
#[test]
fn statement_10_the_landing_brings_three_nights() {
    let played = play(&with_days(&with_t_and_u(&text()), 2));
    let formed = played.formed();
    assert_eq!(formed.len(), 1);
    assert_eq!(formed[0].1, base(2));
    assert_eq!(formed[0].2, P);
    assert_eq!(formed[0].0.day, 1);
    assert_eq!(formed[0].0.pulse, Some(Pulse::Night));
    let rations = played.rations();
    assert_eq!(rations.len(), 1);
    let (event, b, men, nights, tons) = &rations[0];
    assert_eq!(*b, base(2));
    assert_eq!(*men, 3000);
    assert_eq!(*nights, n(3));
    assert_eq!(*tons, n(45));
    assert_eq!(event.day, 1);
    assert_eq!(event.pulse, Some(Pulse::Night));
    assert_eq!(
        cited(event),
        vec![formed[0].0.id],
        "it cites the BaseFormed"
    );
    // The stock the pulse leaves: 45 t, before any night's charge there.
    assert_eq!(
        played.before(1, Pulse::Morning).bases[&base(2)].supply_t,
        n(45)
    );
    assert_eq!(
        played.before(1, Pulse::Morning).bases[&base(2)].side,
        Side::Allied
    );
}

/// 10, whatever number of men are ashore by then: a force that takes more
/// than a pulse to land. T starts at (4,0), 4 hexes from P: it arrives in the
/// morning of day 1 with 2 hours of the pulse left, and lands the rest in the
/// afternoon. The base is formed in the morning with 45 t at once, with fewer
/// than 3,000 men ashore.
#[test]
fn statement_10_whatever_number_of_men_are_ashore_by_then() {
    let text = with_force(&text(), 1, 1, 5, 4, 0);
    let text = with_unit_aboard(&text, 2, 3000, 1);
    let text = with_go(&text, 1, 1, 8);
    let played = play(&with_days(&text, 2));
    let formed = played.formed();
    assert_eq!(formed.len(), 1);
    assert_eq!(formed[0].0.pulse, Some(Pulse::Morning));
    let rations = played.rations();
    assert_eq!(rations.len(), 1);
    assert_eq!(rations[0].2, 3000);
    assert_eq!(rations[0].4, n(45));
    assert_eq!(rations[0].0.pulse, Some(Pulse::Morning));
    assert_eq!(cited(rations[0].0), vec![formed[0].0.id]);
    let ashore_by_afternoon = played.before(1, Pulse::Afternoon).land_units[&unit(2)].men_ashore;
    assert!(
        0 < ashore_by_afternoon && ashore_by_afternoon < 3000,
        "{ashore_by_afternoon} of the 3,000 are ashore after the pulse of the landing"
    );
    assert_eq!(
        played.before(1, Pulse::Afternoon).bases[&base(2)].supply_t,
        n(45)
    );
    // Once: the stock is 45 t until the first night's charge.
    assert_eq!(played.end(1).land_units[&unit(2)].men_ashore, 3000);
    assert_eq!(played.stock(1, 2), n(45));
}

/// 11. A base is given rations once: a second force with 1,000 men landing at
/// P on a later day (day 3) adds nothing to P's stock but what it unloads.
#[test]
fn statement_11_a_base_is_given_rations_once_at_p() {
    // T2: two transports at (7,0) with W, 1,000 men, and 10 t of supply in
    // its first ship, going to P on day 3.
    let text = with_t_and_u(&text());
    let text = with_force(&text, 2, 6, 2, 7, 10);
    let text = with_unit_aboard(&text, 3, 1000, 2);
    let text = with_go(&text, 2, 3, 8);
    let played = play(&with_days(&text, 4));
    // U's men are charged 15 t a night: 45 at the end of day 1, 30, then 15
    // after night 3's charge, and T2's 10 t come in after it.
    assert_eq!(played.stock(1, 2), n(45));
    assert_eq!(played.stock(2, 2), n(30));
    assert_eq!(
        played.before(3, Pulse::Morning).bases[&base(2)].supply_t,
        n(25)
    );
    // W's landing is the whole of what is added: 15 + 10.
    assert_eq!(
        played.before(3, Pulse::Night).bases[&base(2)].supply_t,
        n(30)
    );
    assert_eq!(played.end(3).land_units[&unit(3)].men_ashore, 1000);
    // One record of rations in the game, for the base T formed; one base.
    assert_eq!(played.rations().len(), 1);
    assert_eq!(played.rations()[0].1, base(2));
    assert_eq!(played.formed().len(), 1);
}

/// 11, at Q: a force landing 1,000 men at Q, a base at the start at which no
/// men have been landed, brings 15 t at its first man; a force landing men at
/// Q after that brings none.
#[test]
fn statement_11_a_base_is_given_rations_once_at_q() {
    // Force 1: two transports at (9,0) with X, 1,000 men (600 and 400),
    // going to Q on day 1. Force 3: one transport at (9,0) with Y, 600
    // men, going to Q on day 2.
    let text = with_force(&text(), 1, 1, 2, 9, 0);
    let text = with_unit_aboard(&text, 4, 1000, 1);
    let text = with_go(&text, 1, 1, 10);
    let text = with_force(&text, 3, 3, 1, 9, 0);
    let text = with_unit_aboard(&text, 5, 600, 3);
    let text = with_go(&text, 3, 2, 10);
    let played = play(&with_days(&text, 3));
    assert!(played.formed().is_empty(), "no base is formed at Q");
    // One record of rations in the game: Q's, for force 1's 1,000 men at its
    // first man: 1,000 x 3 x 0.005 t, 15 t.
    let rations = played.rations();
    assert_eq!(rations.len(), 1, "a base is given rations once");
    let (event, at, men, nights, tons) = rations[0];
    assert_eq!((at, men, nights, tons), (base(1), 1000, n(3), n(15)));
    assert_eq!(event.day, 1);
    // It cites the landing at a base that stands: the unload order (and the
    // force's arrival), no base formed.
    assert!(
        event.causes.iter().any(|c| matches!(c, Cause::Order(_))),
        "it cites the unload order"
    );
    assert!(cited(event)
        .iter()
        .all(|id| !matches!(played.event(*id).kind, EventKind::BaseFormed { .. })));
    // The 15 t are in Q's stock from that pulse: nothing else came in and
    // no night's charge has fallen since.
    let pulse = event.pulse.expect("in a pulse");
    let next = match pulse {
        Pulse::Night => (1, Pulse::Morning),
        Pulse::Morning => (1, Pulse::Afternoon),
        Pulse::Afternoon => (2, Pulse::Night),
    };
    assert_eq!(
        played.before(next.0, next.1).bases[&base(1)].supply_t,
        n(15)
    );
    assert!(played.before(next.0, next.1).bases[&base(1)].rations_landed);
    // Force 3's men are landed at Q after that and bring none.
    let landed = played
        .events
        .iter()
        .any(|e| matches!(e.kind, EventKind::MenLanded { hex: Q, unit: u, .. } if u == unit(5)));
    assert!(landed, "force 3's men are landed at Q");
    assert!(!played.end(0).bases[&base(1)].rations_landed);
}

/// 12. With the rations, the first nights are fed: with all of U ashore at P
/// before the first night the charge is 15 t a night; the three nights after
/// the landing are fed in full, the stock is then nothing and U's count is
/// still nothing after them.
#[test]
fn statement_12_with_the_rations_the_first_nights_are_fed() {
    let played = play(&with_days(&with_t_and_u(&text()), 5));
    // All of U is ashore before the first night after the landing.
    assert_eq!(
        played.before(1, Pulse::Morning).land_units[&unit(2)].men_ashore,
        3000
    );
    let at_p = played.fed(P);
    assert_eq!(
        at_p[..3].to_vec(),
        vec![(2, 3000, 3000, 0), (3, 3000, 3000, 0), (4, 3000, 3000, 0)]
    );
    assert_eq!(played.stock(2, 2), n(30));
    assert_eq!(played.stock(3, 2), n(15));
    assert_eq!(played.stock(4, 2), Fixed::ZERO);
    for day in 1..=4usize {
        assert_eq!(played.count(day, 2), 0, "U's count after night {day}");
    }
    assert!(
        played.changes(2).iter().all(|c| c.0 > 4),
        "no change of U's count in the three nights"
    );
    // The fourth night after the landing is the first unfed.
    assert_eq!(at_p[3], (5, 3000, 0, 3000));
    assert_eq!(played.count(5, 2), 1);
}

/// 13. Without the figures a landing brings nothing, and the base is formed
/// with no supply, as today.
#[test]
fn statement_13_without_the_figures_a_landing_brings_nothing() {
    let played = play_with(
        &with_days(&with_t_and_u(&text()), 3),
        &without_the_figures(),
    );
    assert_eq!(played.formed().len(), 1);
    assert_eq!(played.formed()[0].1, base(2));
    assert!(played.rations().is_empty());
    assert_eq!(
        played.before(1, Pulse::Morning).bases[&base(2)].supply_t,
        Fixed::ZERO
    );
    // As today: the men are unfed the night after.
    assert_eq!(played.fed(P)[0], (2, 3000, 0, 3000));
    assert!(played.changes(2).is_empty());
}

// ---------------------------------------------------------------- Properties

fn scenarios_dir() -> PathBuf {
    common::data_dir().join("scenarios")
}

/// Every `.toml` under `dir`, recursively, in order.
fn tomls_under(dir: &Path, into: &mut Vec<PathBuf>) {
    let mut entries: Vec<PathBuf> = std::fs::read_dir(dir)
        .expect("the folder reads")
        .map(|e| e.expect("an entry reads").path())
        .collect();
    entries.sort();
    for entry in entries {
        if entry.is_dir() {
            tomls_under(&entry, into);
        } else if entry.extension().is_some_and(|x| x == "toml") {
            into.push(entry);
        }
    }
}

/// The value of a top-level `ground = "..."` line of a scenario file.
fn ground_line(text: &str) -> Option<String> {
    text.lines()
        .find(|l| l.starts_with("ground = \""))
        .map(|l| {
            l.trim_start_matches("ground = \"")
                .trim_end_matches('"')
                .to_string()
        })
}

/// Properties: no game on file moves. No scenario on file names a ground file
/// with the hunger figures but scenario 119's own, scenario 122's own (decision
/// 0208) and the table's copies, and none of the figures files in
/// `data/reference` (live or frozen) has them but those scenarios' and
/// `ground_hunger.toml`. (One test by the architect's ruling of Nov 21, 2026:
/// the property's words stand, and the list of files that may give the figures
/// gains scenario 122's, and in the same way scenario 123's, decision 0209.)
#[test]
fn property_no_game_on_file_moves() {
    let mut files = Vec::new();
    tomls_under(&scenarios_dir(), &mut files);
    let mut named = 0;
    for file in &files {
        let rel = file
            .strip_prefix(scenarios_dir())
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let own = rel.starts_with("hunger-takes-time/")
            || rel.starts_with("hungry-men-are-slow-to-mend/")
            || rel.starts_with("a-fight-costs-supply/")
            || rel.starts_with("table-hunger/");
        let text = std::fs::read_to_string(file).expect("the scenario reads");
        let Some(ground) = ground_line(&text) else {
            continue;
        };
        let ground_path = file.parent().unwrap().join(&ground);
        let figures = sim_data::load_ground_figures(&ground_path)
            .unwrap_or_else(|e| panic!("{rel} names {ground}: {}", e.message));
        named += 1;
        if !own {
            assert!(
                figures.hunger.is_none(),
                "{rel} names {ground}, which gives the figures of hunger"
            );
        }
    }
    assert!(named > 50, "{named} scenarios name a ground file");
    // The figures files themselves.
    for dir in [
        common::data_dir().join("reference"),
        common::data_dir().join("reference/step1"),
    ] {
        for entry in std::fs::read_dir(&dir).expect("the folder reads") {
            let path = entry.expect("an entry reads").path();
            let name = path.file_name().unwrap().to_string_lossy().into_owned();
            if !(name.starts_with("ground") && name.ends_with(".toml")) {
                continue;
            }
            if name == "ground_placeholder_scenario_119.toml"
                || name == "ground_placeholder_scenario_122.toml"
                || name == "ground_placeholder_scenario_123.toml"
                || name == "ground_hunger.toml"
            {
                continue;
            }
            let figures = sim_data::load_ground_figures(&path).expect("a ground file loads");
            assert!(
                figures.hunger.is_none(),
                "{} gives the figures",
                path.display()
            );
        }
    }
    // Scenario 119's own file has all three.
    let own = sim_data::load_ground_figures(
        &common::data_dir().join("reference/ground_placeholder_scenario_119.toml"),
    )
    .unwrap();
    assert!(own.hunger.is_some());
}

/// Properties: nothing is drawn. The same game played twice gives the same
/// events, and a game with another seed gives the same events too.
#[test]
fn property_nothing_is_drawn() {
    let text = with_unit_at_q(
        &with_count(&with_days(&with_t_and_u(&text()), 6), 14),
        3,
        500,
        0,
    );
    let first = play(&text);
    let second = play(&text);
    assert_eq!(first.events, second.events);
    assert_eq!(first.hashes, second.hashes);
    assert!(first.events.len() > 20);
    // Another seed: the same kinds, days and pulses, and the same causes.
    let other = play_scenario(
        load_with(&text, &common::frozen())
            .with_seed(7)
            .expect("a seed can be set"),
    );
    let key = |p: &Played| -> Vec<(u32, Option<Pulse>, String, Vec<Cause>)> {
        p.events
            .iter()
            .map(|e| (e.day, e.pulse, format!("{:?}", e.kind), e.causes.clone()))
            .collect()
    };
    assert_eq!(key(&first), key(&other));
}

/// Properties: each side sees its own. B's view gives none of A's rations, A's
/// counts or A's losses to hunger, and A's gives them whole.
#[test]
fn property_each_side_sees_its_own() {
    // T lands U and forms P's base; V, count 14, is hungry and loses men.
    let text = with_count(&with_days(&with_t_and_u(&text()), 4), 14);
    let played = play(&text);
    let truth_count =
        |wanted: fn(&EventKind) -> bool| played.events.iter().filter(|e| wanted(&e.kind)).count();
    let rations = truth_count(|k| matches!(k, EventKind::RationsLanded { .. }));
    let changes = truth_count(|k| matches!(k, EventKind::HungerNightsChanged { .. }));
    let losses = truth_count(|k| matches!(k, EventKind::MenLostUnfed { .. }));
    assert_eq!(rations, 1);
    assert!(changes >= 4, "{changes} changes of count");
    assert!(losses >= 4, "{losses} losses");

    let own = played.scenario.world.side_view(Side::Allied).events();
    let whole = |wanted: fn(&EventKind) -> bool| {
        own.iter()
            .filter(|e| matches!(&e.kind, SideEventKind::Whole(k) if wanted(k)))
            .count()
    };
    assert_eq!(
        whole(|k| matches!(k, EventKind::RationsLanded { .. })),
        rations
    );
    assert_eq!(
        whole(|k| matches!(k, EventKind::HungerNightsChanged { .. })),
        changes
    );
    assert_eq!(
        whole(|k| matches!(k, EventKind::MenLostUnfed { .. })),
        losses
    );

    let enemy = played.scenario.world.side_view(Side::Japanese).events();
    for kind in ["RationsLanded", "HungerNightsChanged", "MenLostUnfed"] {
        assert_eq!(
            enemy
                .iter()
                .filter(|e| format!("{:?}", e.kind).contains(kind))
                .count(),
            0,
            "B is told of {kind}"
        );
    }
}

fn repo() -> PathBuf {
    common::data_dir().join("..")
}

/// Properties: the table's copies are true copies. Each scenario in
/// `data/scenarios/table-hunger/` differs from the one file of its name in
/// another folder of `data/scenarios` (not the folder itself, whose files are
/// of the proofs' scenarios at the top, and not `step2-frozen/`, which holds
/// frozen copies for the tests) in the line that names its ground figures
/// and in nothing else, and the script that makes them, asked to check, says
/// they are what it makes.
#[test]
fn property_the_tables_copies_are_true_copies() {
    let folder = scenarios_dir().join("table-hunger");
    assert!(folder.is_dir(), "the table's copies are on file");
    let mut all = Vec::new();
    tomls_under(&scenarios_dir(), &mut all);
    let mut copies = Vec::new();
    tomls_under(&folder, &mut copies);
    assert!(!copies.is_empty(), "the table has copies");
    for copy in &copies {
        let name = copy.file_name().unwrap();
        let sources: Vec<&PathBuf> = all
            .iter()
            .filter(|p| {
                p.file_name() == Some(name)
                    && !p.starts_with(&folder)
                    && p.parent() != Some(scenarios_dir().as_path())
                    && !p.starts_with(scenarios_dir().join("step2-frozen"))
            })
            .collect();
        assert_eq!(sources.len(), 1, "one source for {name:?}: {sources:?}");
        let source_text = std::fs::read_to_string(sources[0]).expect("the source reads");
        let copy_text = std::fs::read_to_string(copy).expect("the copy reads");
        let a: Vec<&str> = source_text.lines().collect();
        let b: Vec<&str> = copy_text.lines().collect();
        assert_eq!(a.len(), b.len(), "{name:?} has the lines of its source");
        let differing: Vec<usize> = (0..a.len()).filter(|&i| a[i] != b[i]).collect();
        assert_eq!(
            differing.len(),
            1,
            "{name:?} differs in one line: {differing:?}"
        );
        let i = differing[0];
        assert_eq!(
            b[i], "ground = \"../../reference/ground_hunger.toml\"",
            "{name:?}"
        );
        assert!(a[i].starts_with("ground = "), "{name:?}: {}", a[i]);
    }
    let out = std::process::Command::new("python3")
        .arg(repo().join("scripts/table-copies/make_copies.py"))
        .arg("--check")
        .current_dir(repo())
        .output()
        .expect("python3 runs the script");
    assert!(
        out.status.success(),
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}
