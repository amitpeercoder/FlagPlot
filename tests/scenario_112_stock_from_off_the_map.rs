//! Scenario 112: stock from off the map. A base with a supply line
//! receives tons of supply and fuel each night while it is its side's.
//!
//! Proves decision 0196. The arithmetic is the architect's, written
//! before the code. A mismatch goes back to the architect, and nobody else
//! changes a statement or a number here.
//!
//! # The setting
//!
//! A placeholder scenario of ten days made for the test. Side J has a base
//! R with 100 t of supply and 50 t of fuel, no aircraft, no men and no
//! ships, so that nothing draws on it, and a supply line to R of 30 t of
//! supply and 12.5 t of fuel a day (written "12.5") from day 3 to day 6,
//! with no side given, so it is J's. A day's night comes first; a base
//! changes side in the fight at the day's end. Side A has a base
//! with no line.
//!
//! # Statements
//!
//! 1. **Before its first day,** at the end of days 1 and 2: 100 t and
//!    50 t.
//! 2. **Each night of its days.** At the end of day 3: 100 + 30 = 130 t
//!    and 50 + 12.5 = 62.5 t. Of day 6: 100 + 4 x 30 = 220 t and 50 + 4 x
//!    12.5 = 100 t. One `StockReceived` on each of days 3 to 6, each of
//!    30 t and 12.5 t at R.
//! 3. **After its last day,** at the end of day 10: 220 t and 100 t.
//! 4. **Who sees it.** J sees each event whole. A is given nothing.
//! 5. **Not to a base that has changed hands.** With R taken by A at the
//!    end of day 3: `StockReceived` is recorded on the night of day 3 and
//!    on no night after. With R taken back by J at the end of day 5: it
//!    is recorded on the nights of days 3 and 6 and no other. A is given
//!    none of them and receives nothing while it holds R.
//! 5a. **The night's tons feed the night's men.** R with no supply at
//!    all, 1,000 men, and a line of 30 t from day 1: on the night of day
//!    1 the men draw 5 t and none goes unfed; R ends the night with 25 t.
//! 5b. **A line of its own side.** A line to R with `side = "allied"`:
//!    nothing is received while J holds R; with R taken by A at the end
//!    of day 3, A receives from the night of day 4.
//! 6. **Two lines add.** A second line to R of 10 t of supply on days 5
//!    to 10: at the end of day 10, 220 + 6 x 10 = 280 t.
//! 7. **It feeds what draws on the base.** With 1,000 men at R drawing
//!    1,000 x 0.005 = 5 t a day, at the end of day 6: 100 - 6 x 5 + 4 x
//!    30 = 190 t.
//! 8. **Told in the briefing.** J's briefing of day 1 names the line, its
//!    tons and its days, under what is due. A's says nothing of it.
//! 8a. **The staff counts the line.** J keeps a second base supplied
//!    from R. With R at 100 t, no reserve, and the runs taking 20 t a
//!    day, the staff's reason gives no day on which R falls to its
//!    reserve while the line gives 30 t a day; with the line taken out it
//!    gives the staff's today and five days more (100 / 20): day 7 on the
//!    view after day 1, where the staff's today is 2. (Corrected by the
//!    architect on Nov 21, 2026: "day 5" as first written counted from
//!    day 0, which no view has.)
//! 9. **Refused scenarios:** a line to a base the scenario has not; one
//!    with neither supply nor fuel; a last day before its first; a first
//!    day of 0; a day past the game's last; tons below 0. A line with no
//!    days gives from the night of day 1.
//! 10. **No line, no change.** Scenario 5 and the 39 days' first week
//!     give the events they gave before.
//!
//! # Properties
//!
//! - A base's stock at the end of a day is its stock at the start, less
//!   what the day's rules took, plus what its lines gave: nothing else.
//! - The same seed gives the same daily hashes twice.
//!
//! The tests are written by the build under these statements.

// The architect's doc comment above is kept as he wrote it; its numbered
// items with letters (5a, 5b, 8a) read to clippy as lazy list continuations.
#![allow(clippy::doc_lazy_continuation)]

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde_json::{json, Value};
use sim_agent::{Seat, Session};
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::ids::{BaseId, IntentId};
use sim_core::side_events::SideEventKind;
use sim_core::state::{Side, TruthState};
use sim_core::Staff;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs, SupplyStaff};

use crate::common;

const A: Side = Side::Allied;
const J: Side = Side::Japanese;

/// R, base 2, J's; B, base 1, A's.
const R: BaseId = BaseId::new(2);
const B: BaseId = BaseId::new(1);

fn path() -> PathBuf {
    common::data_dir().join("scenarios/stock-from-off-the-map/112_stock_from_off_the_map.toml")
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

fn load(text: &str) -> Scenario {
    common::from_text(text, &path()).expect("the scenario loads")
}

fn load_error(text: &str) -> String {
    match common::from_text(text, &path()) {
        Ok(_) => panic!("the scenario is refused"),
        Err(error) => error.message,
    }
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

/// A decimal figure such as "12.5".
fn fx(text: &str) -> Fixed {
    text.parse::<Fixed>().expect("a decimal figure")
}

/// A game played to its end: the events, the daily hashes, and the truth at
/// the start (index 0) and at the end of each day.
struct Played {
    events: Vec<Event>,
    hashes: Vec<u64>,
    truth: Vec<TruthState>,
    /// The scenario, after its last day.
    scenario: Scenario,
}

fn play(text: &str) -> Played {
    play_seeded(text, None)
}

fn play_seeded(text: &str, seed: Option<u64>) -> Played {
    let mut scenario = load(text);
    if let Some(seed) = seed {
        scenario = scenario.with_seed(seed).expect("a seed can be set");
    }
    let mut truth = vec![scenario.world.umpire().clone()];
    let mut hashes = Vec::new();
    while let Some(summary) = scenario.next_day().expect("a day resolves") {
        hashes.push(summary.state_hash);
        truth.push(scenario.world.umpire().clone());
    }
    Played {
        events: scenario.world.events().iter().cloned().collect(),
        hashes,
        truth,
        scenario,
    }
}

impl Played {
    fn on(&self, day: u32) -> Vec<&Event> {
        self.events.iter().filter(|e| e.day == day).collect()
    }

    /// The `StockReceived` events as (day, base, side, supply, fuel).
    fn received(&self) -> Vec<(u32, BaseId, Side, Fixed, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::StockReceived {
                    base,
                    side,
                    supply_t,
                    fuel_t,
                } => Some((e.day, *base, *side, *supply_t, *fuel_t)),
                _ => None,
            })
            .collect()
    }

    /// The days on which base `b` received, in order.
    fn received_days(&self, b: BaseId) -> Vec<u32> {
        self.received()
            .into_iter()
            .filter(|r| r.1 == b)
            .map(|r| r.0)
            .collect()
    }

    /// A base's supply at the end of `day` (0 is the start).
    fn supply(&self, day: usize, b: BaseId) -> Fixed {
        self.truth[day].bases[&b].supply_t
    }

    fn fuel(&self, day: usize, b: BaseId) -> Fixed {
        self.truth[day].bases[&b].fuel_t
    }

    /// What the rules took from base `b`'s supply on `day`: the tons paid by
    /// each `BaseSupplySpent` of the day.
    fn spent(&self, day: u32, b: BaseId) -> Fixed {
        self.events
            .iter()
            .filter(|e| e.day == day)
            .filter_map(|e| match &e.kind {
                EventKind::BaseSupplySpent { base, paid_t, .. } if *base == b => Some(*paid_t),
                _ => None,
            })
            .fold(Fixed::ZERO, |a, x| a + x)
    }

    /// What `b`'s lines gave on `day`, supply and fuel.
    fn given(&self, day: u32, b: BaseId) -> (Fixed, Fixed) {
        self.received()
            .into_iter()
            .filter(|r| r.0 == day && r.1 == b)
            .fold((Fixed::ZERO, Fixed::ZERO), |(s, f), r| (s + r.3, f + r.4))
    }

    fn base_changes(&self) -> Vec<(u32, BaseId, Side)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::BaseChangedSide { base, to, .. } => Some((e.day, *base, *to)),
                _ => None,
            })
            .collect()
    }
}

/// What `side` is told, through its view's events, of `StockReceived`, as
/// (the event, whether it came whole).
fn told_to(scenario: &Scenario, side: Side) -> Vec<(BaseId, Side, Fixed, Fixed)> {
    scenario
        .world
        .side_view(side)
        .events()
        .into_iter()
        .filter_map(|e| match &e.kind {
            SideEventKind::Whole(EventKind::StockReceived {
                base,
                side,
                supply_t,
                fuel_t,
            }) => Some((*base, *side, *supply_t, *fuel_t)),
            _ => None,
        })
        .collect()
}

/// Every side event of the view that is a `StockReceived` in any form (whole
/// or in part): what the side is told of the kind at all.
fn told_of_the_kind(scenario: &Scenario, side: Side) -> usize {
    scenario
        .world
        .side_view(side)
        .events()
        .into_iter()
        .filter(|e| format!("{:?}", e.kind).contains("StockReceived"))
        .count()
}

/// J's garrison at R: `men` men of a placeholder unit, land unit 5, on
/// defend, each drawing 0.005 t a night from R.
fn garrison(text: &str, men: u32) -> String {
    at(
        text,
        "units",
        &format!(
            "[[land_unit]]\nid = 5\nname = \"Placeholder J garrison\"\nside = \"japanese\"\nmen = {men}\nq = 14\nr = 0\n\n"
        ),
    )
}

/// An Allied unit T, land unit 3, 300 men ashore at R on defend, and the order
/// that sets it to attack on `attack_day`: with no Japanese man at R, base 2
/// falls to A at the end of that day (as scenario 111's `t_takes_r`).
fn t_takes_r(text: &str, attack_day: u32) -> String {
    let t = at(
        text,
        "units",
        "[[land_unit]]\nid = 3\nname = \"Placeholder Allied raiders T\"\nside = \"allied\"\nmen = 300\nq = 14\nr = 0\n\n",
    );
    at(
        &t,
        "orders",
        &format!(
            "[[order]]\nday = {attack_day}\nside = \"allied\"\nland_unit = 3\nkind = \"set_posture\"\nposture = \"attack\"\n\n"
        ),
    )
}

/// J's land unit M, land unit 4, 3,000 men on attack at R2, (15,0), beside R
/// and joined to it by a land link, with J's base 3 there, and a march to R
/// ordered on `march_day`: it fights T two days later and takes R back (as
/// scenario 111's `m_marches_on_r`).
fn m_marches_on_r(text: &str, march_day: u32) -> String {
    let t = at(
        text,
        "bases",
        "[[base]]\nid = 3\nname = \"Placeholder base R2\"\nside = \"japanese\"\nq = 15\nr = 0\nsupply_t = 500\n\n",
    );
    let t = at(
        &t,
        "units",
        "[[land_unit]]\nid = 4\nname = \"Placeholder M\"\nside = \"japanese\"\nmen = 3000\nq = 15\nr = 0\nposture = \"attack\"\n\n",
    );
    at(
        &t,
        "orders",
        &format!(
            "[[order]]\nday = {march_day}\nside = \"japanese\"\nland_unit = 4\nkind = \"march\"\nhex = [14, 0]\n\n"
        ),
    )
}

/// The base text's line, written out, for a variant that replaces it.
const THE_LINE: &str = "# J's supply line to R: 30 t of supply and 12.5 t of fuel a night, nights of\n# days 3 to 6; the side is the base's at the start, J.\n[[supply_line]]\nbase = 2\nsupply_t = 30\nfuel_t = \"12.5\"\nfirst_day = 3\nlast_day = 6\n\n";

/// The text with its line taken out.
fn without_the_line(text: &str) -> String {
    changed(text, THE_LINE, "")
}

/// The text with its line replaced by `line`'s keys (a `[[supply_line]]`
/// table of its own).
fn with_the_line(text: &str, line: &str) -> String {
    changed(text, THE_LINE, &format!("[[supply_line]]\n{line}\n\n"))
}

fn session(text: &str) -> Session {
    let scenario = load(text);
    let name = scenario.name.clone();
    Session::new(
        scenario.world,
        &name,
        scenario.days,
        Seat::Player,
        Seat::Player,
    )
}

fn ask(session: &mut Session, side: Side, tool: &str, args: Value) -> Value {
    session
        .call(side, tool, &args)
        .unwrap_or_else(|e| panic!("{tool} was refused: {e}"))
}

// ------------------------------------------------------------------ Statements

#[test]
fn the_setting_is_what_the_comment_says() {
    let scenario = load(&text());
    assert!(scenario.placeholder);
    assert_eq!(scenario.days, 10);
    let truth = scenario.world.umpire();
    // R: J's, 100 t and 50 t. B: A's, with no line.
    assert_eq!(truth.bases[&R].side, J);
    assert_eq!(truth.bases[&R].supply_t, n(100));
    assert_eq!(truth.bases[&R].fuel_t, n(50));
    assert_eq!(truth.bases[&B].side, A);
    // No aircraft, no men and no ships, so nothing draws on R.
    assert!(truth.ships.is_empty());
    assert!(truth.air_groups.is_empty());
    assert!(truth.land_units.is_empty());
    // One line: to R, 30 t and 12.5 t, from day 3 to day 6, J's.
    assert_eq!(truth.supply_lines.len(), 1);
    let line = &truth.supply_lines[0];
    assert_eq!(line.base, R);
    assert_eq!(line.side, J);
    assert_eq!(line.supply_t, n(30));
    assert_eq!(line.fuel_t, fx("12.5"));
    assert_eq!((line.first_day, line.last_day), (3, 6));
    // The day is a night, three pulses, and a game with no unit plays on to
    // its tenth day.
    let played = play(&text());
    assert_eq!(played.hashes.len(), 10);
}

#[test]
fn statement_1_before_its_first_day_nothing_is_received() {
    let played = play(&text());
    for day in [1usize, 2] {
        assert_eq!(played.supply(day, R), n(100), "end of day {day}");
        assert_eq!(played.fuel(day, R), n(50), "end of day {day}");
    }
    // And no event: the line gives on none of those nights.
    assert!(played.received().iter().all(|r| r.0 >= 3));
}

#[test]
fn statement_2_each_night_of_its_days() {
    let played = play(&text());
    // The end of day 3: 100 + 30 = 130 t and 50 + 12.5 = 62.5 t.
    assert_eq!(played.supply(3, R), n(130));
    assert_eq!(played.fuel(3, R), fx("62.5"));
    // The end of day 6: 100 + 4 x 30 = 220 t and 50 + 4 x 12.5 = 100 t.
    assert_eq!(played.supply(6, R), n(100) + n(4) * n(30));
    assert_eq!(played.supply(6, R), n(220));
    assert_eq!(played.fuel(6, R), n(50) + n(4) * fx("12.5"));
    assert_eq!(played.fuel(6, R), n(100));
    // The days between, one night at a time.
    for (day, supply, fuel) in [(4usize, 160, "75"), (5, 190, "87.5")] {
        assert_eq!(played.supply(day, R), n(supply), "day {day}");
        assert_eq!(played.fuel(day, R), fx(fuel), "day {day}");
    }
    // One `StockReceived` on each of days 3 to 6, each of 30 t and 12.5 t at
    // R, and no other.
    let received = played.received();
    assert_eq!(received.len(), 4);
    for (event, day) in received.iter().zip(3u32..=6) {
        assert_eq!(
            *event,
            (day, R, J, n(30), fx("12.5")),
            "the event of day {day}"
        );
    }
    // It cites no cause (rule 11) and is recorded in the night pulse.
    for event in played
        .events
        .iter()
        .filter(|e| matches!(e.kind, EventKind::StockReceived { .. }))
    {
        assert!(event.causes.is_empty(), "no cause: {event:?}");
        assert_eq!(event.pulse, Some(sim_core::time::Pulse::Night));
    }
}

#[test]
fn statement_3_after_its_last_day_nothing_is_received() {
    let played = play(&text());
    assert_eq!(played.supply(10, R), n(220));
    assert_eq!(played.fuel(10, R), n(100));
    for day in [7usize, 8, 9] {
        assert_eq!(played.supply(day, R), n(220), "day {day}");
        assert_eq!(played.fuel(day, R), n(100), "day {day}");
    }
    assert!(played.received().iter().all(|r| r.0 <= 6));
}

#[test]
fn statement_4_who_sees_it() {
    let played = play(&text());
    // J sees each event whole.
    let to_j = told_to(&played.scenario, J);
    assert_eq!(to_j.len(), 4);
    for event in &to_j {
        assert_eq!(*event, (R, J, n(30), fx("12.5")));
    }
    // A is given nothing: not whole and not in part.
    assert!(told_to(&played.scenario, A).is_empty());
    assert_eq!(told_of_the_kind(&played.scenario, A), 0);
    assert_eq!(told_of_the_kind(&played.scenario, J), 4);
}

/// Base R taken by A at the end of `taken` (an Allied land unit of 300 men,
/// set to attack on `taken`), the base's J line of the file as it is.
fn taken_at_the_end_of(taken: u32) -> String {
    t_takes_r(&text(), taken)
}

#[test]
fn statement_5_not_to_a_base_that_has_changed_hands_taken_at_the_end_of_day_3() {
    let played = play(&taken_at_the_end_of(3));
    // R is taken by A at the end of day 3 and not before.
    assert_eq!(played.base_changes(), vec![(3, R, A)]);
    assert_eq!(played.truth[2].bases[&R].side, J);
    assert_eq!(played.truth[3].bases[&R].side, A);
    // Recorded on the night of day 3 and on no night after.
    assert_eq!(played.received_days(R), vec![3]);
    // A base taken keeps only a share of its stock (0.5 here): 130 t and
    // 62.5 t at the night's end halve. (Checked in the events and the
    // stock, as the statement asks.)
    assert_eq!(played.supply(3, R), n(65));
    assert_eq!(played.fuel(3, R), fx("31.25"));
    // A receives nothing while it holds R: R's stock only falls, by what the
    // Allied men there are fed (nothing draws on it that is not a man).
    assert!(played.supply(10, R) <= played.supply(3, R));
    assert_eq!(played.fuel(10, R), played.fuel(3, R));
    // A is given none of the events and J is given the one.
    assert!(told_to(&played.scenario, A).is_empty());
    assert_eq!(told_of_the_kind(&played.scenario, A), 0);
    assert_eq!(told_to(&played.scenario, J).len(), 1);
}

#[test]
fn statement_5_not_to_a_base_that_has_changed_hands_taken_back_at_the_end_of_day_5() {
    // T takes R at the end of day 3; M, ordered to march on day 3, fights T
    // on day 5 and J holds R again from day 6 (scenario 111 has the same
    // march: ordered on a day, it fights two days later).
    let played = play(&m_marches_on_r(&taken_at_the_end_of(3), 3));
    assert_eq!(played.base_changes(), vec![(3, R, A), (5, R, J)]);
    assert_eq!(played.truth[3].bases[&R].side, A);
    assert_eq!(played.truth[4].bases[&R].side, A);
    assert_eq!(played.truth[5].bases[&R].side, J);
    // Recorded on the nights of days 3 and 6 and no other.
    assert_eq!(played.received_days(R), vec![3, 6]);
    // A is given none of them and receives nothing while it holds R.
    assert!(told_to(&played.scenario, A).is_empty());
    assert_eq!(told_of_the_kind(&played.scenario, A), 0);
    assert_eq!(told_to(&played.scenario, J).len(), 2);
    for event in played.received() {
        assert_eq!(event.2, J);
    }
}

#[test]
fn statement_5a_the_nights_tons_feed_the_nights_men() {
    // R with no supply at all, 1,000 men, and a line of 30 t from day 1.
    let t = changed(
        &text(),
        "supply_t = 100\nfuel_t = 50\n",
        "supply_t = 0\nfuel_t = 50\n",
    );
    let t = with_the_line(&t, "base = 2\nsupply_t = 30\nfirst_day = 1");
    let played = play(&garrison(&t, 1000));
    assert_eq!(played.supply(0, R), n(0));
    // On the night of day 1 the line gives 30 t, the men draw 5 t and none
    // goes unfed.
    assert_eq!(played.given(1, R), (n(30), n(0)));
    let night: Vec<_> = played
        .on(1)
        .into_iter()
        .filter_map(|e| match &e.kind {
            EventKind::BaseSupplySpent {
                base,
                men,
                asked_t,
                paid_t,
                left_t,
                ..
            } if *base == R => Some((*men, *asked_t, *paid_t, *left_t)),
            _ => None,
        })
        .collect();
    assert_eq!(night, vec![(1000, n(5), n(5), n(25))]);
    let fed: Vec<_> = played
        .on(1)
        .into_iter()
        .filter_map(|e| match &e.kind {
            EventKind::MenFed {
                men, fed, unfed, ..
            } => Some((*men, *fed, *unfed)),
            _ => None,
        })
        .collect();
    assert_eq!(fed, vec![(1000, 1000, 0)]);
    // R ends the night, and the day, with 25 t.
    assert_eq!(played.supply(1, R), n(25));
    // The event of the line comes before the men are fed, on the same night.
    let order: Vec<&str> = played
        .on(1)
        .into_iter()
        .filter_map(|e| match &e.kind {
            EventKind::StockReceived { .. } => Some("received"),
            EventKind::MenFed { .. } => Some("fed"),
            _ => None,
        })
        .collect();
    assert_eq!(order, vec!["received", "fed"]);
}

#[test]
fn statement_5b_a_line_of_its_own_side_while_j_holds_r() {
    // A line to R with side = "allied": nothing is received while J holds R.
    let t = with_the_line(
        &text(),
        "base = 2\nside = \"allied\"\nsupply_t = 30\nfuel_t = 5",
    );
    let played = play(&t);
    assert!(played.received().is_empty());
    assert_eq!(played.supply(10, R), n(100));
    assert_eq!(played.fuel(10, R), n(50));
}

#[test]
fn statement_5b_a_line_of_its_own_side_a_receives_from_the_night_after_it_takes_r() {
    // With R taken by A at the end of day 3, A receives from the night of
    // day 4 (and not on night 3, when R was still J's).
    let t = with_the_line(
        &text(),
        "base = 2\nside = \"allied\"\nsupply_t = 30\nfuel_t = 5",
    );
    let played = play(&t_takes_r(&t, 3));
    assert_eq!(played.base_changes(), vec![(3, R, A)]);
    assert_eq!(played.received_days(R), (4..=10).collect::<Vec<u32>>());
    for event in played.received() {
        assert_eq!((event.2, event.3, event.4), (A, n(30), n(5)));
    }
    // A is told of each whole, and J of none.
    assert_eq!(told_to(&played.scenario, A).len(), 7);
    assert_eq!(told_of_the_kind(&played.scenario, J), 0);
    // The stock rises by 30 each night, less the Allied men fed at R (300 men
    // at 0.005 t, 1.5 t a night).
    let fuel_after_taking = played.fuel(3, R);
    assert_eq!(played.fuel(10, R), fuel_after_taking + n(7) * n(5));
}

#[test]
fn statement_6_two_lines_add() {
    let t = at(
        &text(),
        "lines",
        "[[supply_line]]\nbase = 2\nsupply_t = 10\nfirst_day = 5\nlast_day = 10\n\n",
    );
    let played = play(&t);
    assert_eq!(played.supply(10, R), n(220) + n(6) * n(10));
    assert_eq!(played.supply(10, R), n(280));
    // Fuel is the first line's alone.
    assert_eq!(played.fuel(10, R), n(100));
    // Both are recorded on a night both give (days 5 and 6): two events.
    assert_eq!(
        played
            .on(5)
            .iter()
            .filter(|e| matches!(e.kind, EventKind::StockReceived { .. }))
            .count(),
        2
    );
    assert_eq!(
        played
            .on(8)
            .iter()
            .filter(|e| matches!(e.kind, EventKind::StockReceived { .. }))
            .count(),
        1
    );
}

#[test]
fn statement_7_it_feeds_what_draws_on_the_base() {
    // 1,000 men at R draw 1,000 x 0.005 = 5 t a day.
    let played = play(&garrison(&text(), 1000));
    for day in 1..=10 {
        assert_eq!(played.spent(day, R), n(5), "day {day}");
    }
    // At the end of day 6: 100 - 6 x 5 + 4 x 30 = 190 t.
    assert_eq!(played.supply(6, R), n(100) - n(6) * n(5) + n(4) * n(30));
    assert_eq!(played.supply(6, R), n(190));
    // The men were fed every night.
    assert!(played
        .events
        .iter()
        .all(|e| !matches!(e.kind, EventKind::MenLostUnfed { .. })));
}

#[test]
fn statement_8_told_in_the_briefing() {
    let mut s = session(&text());
    let briefing = ask(&mut s, J, "get_briefing", json!({}));
    let lines = briefing["arrivals"]["supply_lines"]
        .as_array()
        .unwrap_or_else(|| panic!("J's briefing names the line: {briefing}"));
    assert_eq!(lines.len(), 1, "{briefing}");
    assert_eq!(lines[0]["base"]["name"], "Placeholder base R");
    assert_eq!(lines[0]["supply_t"], "30");
    assert_eq!(lines[0]["fuel_t"], "12.5");
    assert_eq!(lines[0]["first_day"], 3);
    assert_eq!(lines[0]["last_day"], 6);
    let words = ask(&mut s, J, "get_briefing", json!({"text": true}));
    let words = words["text"].as_str().expect("text");
    assert!(words.contains("Supply line"), "{words}");
    // A's says nothing of it.
    let allied = ask(&mut s, A, "get_briefing", json!({}));
    assert!(
        !allied["arrivals"]
            .as_object()
            .expect("arrivals")
            .contains_key("supply_lines"),
        "{allied}"
    );
    assert!(!allied.to_string().contains("12.5"));
    let words = ask(&mut s, A, "get_briefing", json!({"text": true}));
    assert!(!words["text"]
        .as_str()
        .expect("text")
        .contains("Supply line"));
}

#[test]
fn statement_8_a_game_with_no_line_has_no_such_key() {
    let mut s = session(&without_the_line(&text()));
    for side in [A, J] {
        let briefing = ask(&mut s, side, "get_briefing", json!({}));
        assert!(!briefing["arrivals"]
            .as_object()
            .expect("arrivals")
            .contains_key("supply_lines"));
    }
}

// ---------------------------------------------------------------- Statement 8a

/// J keeps base 3, R2 at (15,0), supplied from R. R2 holds 4,000 men, which
/// use 4,000 x 0.005 = 20 t a day; a cargo ship, ship 1, lies in port at R;
/// the intent gives no leave to prepare or to launch, so the staff orders
/// nothing and R stays at 100 t: with no reserve, the staff has J's
/// forecast of the day R falls to its reserve.
fn keeping_r2_supplied(text: &str) -> String {
    let t = at(
        text,
        "bases",
        "[[base]]\nid = 3\nname = \"Placeholder base R2\"\nside = \"japanese\"\nq = 15\nr = 0\nsupply_t = 100\n\n",
    );
    let t = at(
        &t,
        "units",
        "[[land_unit]]\nid = 6\nname = \"Placeholder R2 garrison\"\nside = \"japanese\"\nmen = 4000\nq = 15\nr = 0\nposture = \"defend\"\n\n",
    );
    let t = at(
        &t,
        "forces",
        "[[ship]]\nid = 1\nname = \"Placeholder cargo ship 1\"\nclass = 3\nside = \"japanese\"\nfuel_t = 1000\nport = 2\n\n",
    );
    at(
        &t,
        "commands",
        "[[command]]\nday = 1\nside = \"japanese\"\nkind = \"set_intent\"\nintent = 1\nbase = 3\nfloor_days = 30\ninterval_days = 15\n\n[[command]]\nday = 1\nside = \"japanese\"\nkind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\nships = [1]\ntask_forces = []\nsource = 2\n\n[[command]]\nday = 1\nside = \"japanese\"\nkind = \"set_leave\"\nleave = \"prepare\"\ngiven = false\n\n[[command]]\nday = 1\nside = \"japanese\"\nkind = \"set_leave\"\nleave = \"launch\"\ngiven = false\n\n",
    )
}

/// J's staff's shortfall for intent 1 on the view after day 1: the day it says
/// R falls to its reserve, the day it is working from (the staff's today),
/// and R's stock the view holds.
fn dry_day(text: &str) -> (Option<u32>, u32, Fixed) {
    let mut scenario = load(text);
    let held = staffs();
    scenario.next_day_staffed(as_staffs(&held)).expect("day 1");
    let view = scenario.world.side_view(J);
    let stock = view.bases().find(|b| b.id == R).expect("R is J's").supply_t;
    let proposal = SupplyStaff
        .propose(view, IntentId::new(1))
        .expect("a proposal");
    let shortfall = proposal.shortfall.expect("a shortfall");
    (shortfall.source_dry_day, view.day() + 1, stock)
}

#[test]
fn statement_8a_the_staff_counts_the_line_with_the_line_no_day() {
    let (with, today, stock) = dry_day(&keeping_r2_supplied(&text()));
    assert_eq!(stock, n(100), "R holds 100 t on the view");
    assert_eq!(today, 2);
    // With R at 100 t, no reserve and the runs taking 20 t a day, the reason
    // gives no day on which R falls to its reserve while the line gives 30 t
    // a day.
    assert_eq!(with, None);
}

#[test]
fn statement_8a_without_the_line_the_day_is_today_plus_floor_of_stock_over_rate() {
    // What does hold: with the line taken out the day is the staff's today
    // plus floor(100 / 20), as the code states (doctrine.rs `shortfall`).
    let (without, today, stock) = dry_day(&keeping_r2_supplied(&without_the_line(&text())));
    assert_eq!(stock, n(100));
    assert_eq!(without, Some(today + 5));
    assert_eq!(without, Some(7));
}

// ------------------------------------------------------------------ Statement 9

#[test]
fn statement_9_refused_scenarios() {
    // A line to a base the scenario has not.
    let m = load_error(&with_the_line(&text(), "base = 9\nsupply_t = 1"));
    assert!(
        m.contains("supply line 1") && m.contains("unknown base 9"),
        "{m}"
    );
    // One with neither supply nor fuel (left out, or both 0).
    let m = load_error(&with_the_line(&text(), "base = 2"));
    assert!(
        m.contains("supply line 1") && m.contains("gives nothing"),
        "{m}"
    );
    let m = load_error(&with_the_line(
        &text(),
        "base = 2\nsupply_t = 0\nfuel_t = \"0\"",
    ));
    assert!(m.contains("gives nothing"), "{m}");
    // A last day before its first.
    let m = load_error(&with_the_line(
        &text(),
        "base = 2\nsupply_t = 1\nfirst_day = 5\nlast_day = 4",
    ));
    assert!(m.contains("last day 4 before its first day 5"), "{m}");
    // A first day of 0.
    let m = load_error(&with_the_line(
        &text(),
        "base = 2\nsupply_t = 1\nfirst_day = 0",
    ));
    assert!(m.contains("first day below 1"), "{m}");
    // A day past the game's last (ten days): the first and the last.
    let m = load_error(&with_the_line(
        &text(),
        "base = 2\nsupply_t = 1\nlast_day = 11",
    ));
    assert!(m.contains("past the game's last day 10"), "{m}");
    let m = load_error(&with_the_line(
        &text(),
        "base = 2\nsupply_t = 1\nfirst_day = 11\nlast_day = 11",
    ));
    assert!(m.contains("past the game's last day 10"), "{m}");
    // Tons below 0: supply and fuel, whole and decimal.
    let m = load_error(&with_the_line(
        &text(),
        "base = 2\nsupply_t = -1\nfuel_t = 5",
    ));
    assert!(m.contains("supply_t below 0"), "{m}");
    let m = load_error(&with_the_line(
        &text(),
        "base = 2\nsupply_t = 5\nfuel_t = \"-0.5\"",
    ));
    assert!(m.contains("fuel_t below 0"), "{m}");
    // A line's number is its place in the file.
    let two = at(
        &text(),
        "lines",
        "[[supply_line]]\nbase = 9\nsupply_t = 1\n\n",
    );
    assert!(load_error(&two).contains("supply line 2"));
    // A TOML float is not a quantity.
    load_error(&with_the_line(&text(), "base = 2\nsupply_t = 1.5"));
    // The limits themselves load: day 1 to day 10, and a day of one.
    for days in [
        "first_day = 1\nlast_day = 10",
        "first_day = 10\nlast_day = 10",
    ] {
        load(&with_the_line(
            &text(),
            &format!("base = 2\nsupply_t = 1\n{days}"),
        ));
    }
}

#[test]
fn statement_9_a_line_with_no_days_gives_from_the_night_of_day_1() {
    let played = play(&with_the_line(
        &text(),
        "base = 2\nsupply_t = 10\nfuel_t = 2",
    ));
    assert_eq!(played.received_days(R), (1..=10).collect::<Vec<u32>>());
    assert_eq!(played.supply(1, R), n(110));
    assert_eq!(played.fuel(1, R), n(52));
    assert_eq!(played.supply(10, R), n(200));
    assert_eq!(played.fuel(10, R), n(70));
    let line = &played.truth[0].supply_lines[0];
    assert_eq!((line.first_day, line.last_day), (1, 10));
    assert_eq!(line.side, J, "the base's side at the start");
}

// ----------------------------------------------------------------- Statement 10

fn repo_root() -> PathBuf {
    common::data_dir().join("..")
}

/// A recorded game's saved file replayed on the live figures as `sim-cli
/// replay` replays it: the day-by-day hashes the file holds, which were
/// written before decision 0196 was built, against the hashes of today's
/// code. Returns the scenario's text and the hashes replayed.
fn replayed(save: &str) -> (String, Vec<(u32, String, String)>) {
    let file = repo_root()
        .join("crates/sim-cli/tests/recorded_games")
        .join(save);
    let saved = sim_cli::save::read(file.to_str().expect("a path in words")).expect("reads");
    let scenario_path = repo_root().join(&saved.scenario_file);
    let scenario = Scenario::load(&scenario_path).expect("the scenario loads");
    // Nothing given by a line.
    assert!(scenario.world.umpire().supply_lines.is_empty(), "{save}");
    let text = std::fs::read_to_string(&scenario_path).expect("the file reads");
    let mut scenario = scenario.with_seed(saved.seed).expect("seeded");
    scenario.replace_script(saved.script.clone());
    scenario.replace_script_inside_day(saved.script_inside_day.clone());
    scenario.replace_commands(saved.commands.clone());
    let mut out = Vec::new();
    for (day, hash) in &saved.hashes {
        for (side, decisions) in saved.staff.get(day).into_iter().flatten() {
            scenario.world.submit_staff(*side, decisions.clone());
        }
        let summary = scenario
            .next_day_watched(sim_core::Staffs::none(), &mut |_, _| {})
            .expect("the day resolves")
            .expect("the day exists");
        out.push((*day, hash.clone(), format!("{:016x}", summary.state_hash)));
    }
    (text, out)
}

#[test]
fn statement_10_scenario_5_and_the_39_days_first_week_give_the_hashes_they_gave() {
    // The pinned expectations are the daily hashes of the recorded games'
    // saved files (Eastern Solomons, two players; the full slice's first
    // five days; the second proof's week), written before decision 0196 and
    // unchanged by it: replayed on today's code they are the same.
    for save in [
        "eastern_solomons_haiku/save.json",
        "eastern_solomons_sonnet/save.json",
        "full_slice_five_days_sonnet/save.json",
        "second_proof_week_sonnet/save.json",
    ] {
        let (text, days) = replayed(save);
        assert!(!days.is_empty(), "{save}");
        assert!(
            !text.contains("[[supply_line]]"),
            "{save}: no line in the file"
        );
        for (day, saved, replayed) in &days {
            assert_eq!(saved, replayed, "{save}, day {day}: the daily hash");
        }
    }
}

#[test]
fn statement_10_the_scenarios_hold_no_line() {
    for name in ["05_eastern_solomons.toml", "07_full_slice.toml"] {
        let file = common::data_dir().join("scenarios").join(name);
        let scenario = common::load_live(&file).expect("the scenario loads");
        assert!(scenario.world.umpire().supply_lines.is_empty(), "{name}");
        let text = std::fs::read_to_string(&file).expect("the file reads");
        assert!(!text.contains("[[supply_line]]"), "{name}");
    }
}

#[test]
fn statement_10_a_scenario_with_no_line_is_the_same_with_the_line_logic_present() {
    // This scenario with its line taken out: nothing is received, nothing
    // changes in either base, and the same seed gives the same hashes.
    let none = without_the_line(&text());
    let a = play(&none);
    assert!(a.received().is_empty());
    for day in 0..=10 {
        assert_eq!(a.supply(day, R), n(100));
        assert_eq!(a.fuel(day, R), n(50));
        assert_eq!(a.supply(day, B), n(1000));
    }
    assert_eq!(a.hashes, play(&none).hashes);
}

// ------------------------------------------------------------------ Properties

/// The property of a game: for each base and each day, the stock at the end
/// of the day is the stock at the start, less what the day's rules took (the
/// tons paid by each `BaseSupplySpent`) and plus what the lines gave (the
/// `StockReceived` of the day); fuel, which nothing takes in these games, is
/// the start's plus what the lines gave.
fn check_stock_is_start_less_taken_plus_given(played: &Played, label: &str) {
    for b in [R, B] {
        for day in 1..=played.hashes.len() {
            let d = u32::try_from(day).expect("a day");
            let (supply_given, fuel_given) = played.given(d, b);
            assert_eq!(
                played.supply(day, b),
                played.supply(day - 1, b) - played.spent(d, b) + supply_given,
                "{label}: supply of {b} on day {d}"
            );
            assert_eq!(
                played.fuel(day, b),
                played.fuel(day - 1, b) + fuel_given,
                "{label}: fuel of {b} on day {d}"
            );
        }
    }
}

#[test]
fn property_a_bases_stock_is_its_start_less_what_the_rules_took_plus_what_its_lines_gave() {
    let mut variants: Vec<(String, String)> = vec![("the file as it is".into(), text())];
    // A few tonnages and days; with and without men to draw on R; two seeds
    // for each.
    for (supply, fuel, first, last) in [
        ("30", "\"12.5\"", 1, 10),
        ("7", "\"0.25\"", 2, 9),
        ("0", "3", 4, 4),
        ("\"0.5\"", "0", 1, 3),
    ] {
        let line = format!(
            "base = 2\nsupply_t = {supply}\nfuel_t = {fuel}\nfirst_day = {first}\nlast_day = {last}"
        );
        variants.push((
            format!("line {supply} {fuel} {first}..{last}"),
            with_the_line(&text(), &line),
        ));
        variants.push((
            format!("line {supply} {fuel} {first}..{last} with 1,000 men"),
            garrison(&with_the_line(&text(), &line), 1000),
        ));
    }
    // Two lines to R; and a garrison that outruns the stock, so that some of
    // what it is owed is not paid and the stock hits 0.
    variants.push((
        "two lines".into(),
        at(
            &text(),
            "lines",
            "[[supply_line]]\nbase = 2\nsupply_t = 10\nfirst_day = 5\n\n",
        ),
    ));
    variants.push(("a hungry garrison".into(), garrison(&text(), 9000)));
    for (label, variant) in &variants {
        for seed in [1942u64, 7] {
            let played = play_seeded(variant, Some(seed));
            assert_eq!(played.hashes.len(), 10, "{label}");
            check_stock_is_start_less_taken_plus_given(&played, &format!("{label}, seed {seed}"));
        }
    }
}

#[test]
fn property_the_stock_property_holds_where_a_base_changes_hands_apart_from_the_share_kept() {
    // On a day a base changes side the end-of-day stock is cut to the share
    // its taker keeps; every other day the property holds. Played with R
    // taken at the end of day 3 and retaken at the end of day 5.
    let played = play(&m_marches_on_r(&taken_at_the_end_of(3), 3));
    let changed: Vec<u32> = played.base_changes().iter().map(|c| c.0).collect();
    assert_eq!(changed, vec![3, 5]);
    for day in 1..=10usize {
        let d = u32::try_from(day).expect("a day");
        if changed.contains(&d) {
            continue;
        }
        let (supply_given, fuel_given) = played.given(d, R);
        assert_eq!(
            played.supply(day, R),
            played.supply(day - 1, R) - played.spent(d, R) + supply_given,
            "supply of R on day {d}"
        );
        assert_eq!(
            played.fuel(day, R),
            played.fuel(day - 1, R) + fuel_given,
            "fuel of R on day {d}"
        );
    }
}

#[test]
fn property_the_same_seed_gives_the_same_daily_hashes_twice() {
    for text in [
        text(),
        garrison(&text(), 1000),
        taken_at_the_end_of(3),
        m_marches_on_r(&taken_at_the_end_of(3), 3),
        with_the_line(&text(), "base = 2\nside = \"allied\"\nsupply_t = 30"),
        without_the_line(&text()),
    ] {
        let first = play(&text);
        let second = play(&text);
        assert_eq!(first.hashes.len(), 10);
        assert_eq!(first.hashes, second.hashes);
        assert_eq!(first.events, second.events);
    }
    // A line changes the hash: the same game with and without it differs.
    let with = play(&text()).hashes;
    let without = play(&without_the_line(&text())).hashes;
    assert_ne!(with, without);
    // And the hashes of one game are distinct from day to day where a line
    // gives (days 3 to 6).
    let distinct: std::collections::BTreeSet<_> = with[2..6].iter().collect();
    assert_eq!(distinct.len(), 4);
    // By a seed.
    let seeded = |seed| play_seeded(&text(), Some(seed)).hashes;
    assert_eq!(seeded(5), seeded(5));
    let _ = BTreeMap::<u8, u8>::new();
}
