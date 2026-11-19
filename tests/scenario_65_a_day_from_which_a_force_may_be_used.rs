//! Scenario 65: a day from which a force may be used. A carrier force
//! that its side may use from day 2 is left alone on day 1 and played from
//! day 2 exactly as it would have been played from day 1.
//!
//! Proves decision 0140, step 1 of stage 3, against a hand calculation.
//! The data is placeholder data: `data/scenarios/65_a_day_from_which_a_force_may_be_used.toml`
//! is scenario 58's file with no contact, a game of 4 days, intent 1
//! "operate task force 1 from (6,0)" with no risk set, so balanced, and
//! task force 1 available from day 2. It runs on the frozen step 1
//! figures as scenario 58 does. There is no Japanese unit, so no die is
//! rolled.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## The game without the day
//!
//! "The same file without the day" is this scenario with task force 1's
//! day taken out. By scenario 58's property "speed alone", at balanced
//! she is ordered to full speed on day 1 and arrives at (6,0) at hour 8.
//! Call the orders the staff gives on its days 1, 2 and 3 the first,
//! second and third days' orders.
//!
//! ## The game with the day
//!
//! - **Day 1.** Task force 1 may not be used. The staff gives no order to
//!   her or to groups 1, 2 and 3, and hands in one reason: that intent 1
//!   is to be served by task force 1, which may be used from day 2. She
//!   lies stopped at (0,0) through day 1 and burns no fuel under way.
//! - **Day 2.** The staff gives the first day's orders. She arrives at
//!   (6,0) at hour 32: hour 8 and a day.
//! - **Day 3.** The second day's orders.
//! - **Day 4.** The third day's orders.
//!
//! Every order is the same order, to the same unit, with the same reason
//! but for the day it names; the count on each day is the count a day
//! earlier in the game without the day.
//!
//! # Properties
//!
//! - **An order before the day is refused.** A commander's order on day 1
//!   that routes task force 1 is refused with the day, 2, and changes
//!   nothing. The same order on day 2 is taken.
//! - **A group's own day.** With task force 1 available from day 1 and
//!   group 3, the dive bombers, from day 3: on days 1 and 2 the staff
//!   gives group 3 no order and every other order is as in the game
//!   without the day; on day 3 it gives group 3 the orders it gives it on
//!   day 1 in that game. A commander's order to group 3 on day 2 is
//!   refused with the day, 3.
//! - **The later of two days.** With task force 1 from day 2 and ship 1
//!   from day 3: nothing is ordered before day 3, and the reason names
//!   day 3. With task force 1 from day 3 and group 3 from day 2: group 3
//!   is given no order before day 3.
//! - **A land unit.** In a variant with a land unit ashore at base 1,
//!   available from day 3: a commander's `set_posture` on day 2 is refused
//!   with the day, and on day 3 is taken.
//! - **A ship in a pool.** In scenario 36, keeping a base supplied, with
//!   the second ship of the pool available from day 5: the staff commits
//!   it to no run and loads nothing on it before day 5, and where a reason
//!   lists the pool it names the ship with its day.
//! - **A side knows its own days and not the enemy's.** The Allied side's
//!   view gives task force 1's day. Two worlds that differ only in the day
//!   of a Japanese force give the Allied side the same view, the same
//!   orders and the same reasons.
//! - **No day, no change.** Every scenario built before this one gives the
//!   events it gave.
//! - **Formations.** In a variant with two land units in "Placeholder
//!   Battalion 1" and "Placeholder Battalion 2", both under "Placeholder
//!   Regiment", under "Placeholder Division": the side's view gives each
//!   unit's formation and each formation's parent, to the division, which
//!   has none. The loader refuses a parent that is not there and a
//!   formation that is its own ancestor. No order and no event differs
//!   from the same file without the formations.
//! - **The staff's setup of the 39 days** loads, runs its 39 days with no
//!   order from anyone and none refused, and gives the events that
//!   `data/scenarios/07_full_slice.toml` gives with its orders taken out.
//!   Every unit with a day in it has the day of its first scripted order
//!   in the scripted file, or the day the architect ruled.
//! - The same start gives the same game.

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use sim_core::choices::Moment;
use sim_core::command::{CommandKind, Maker, Reason, StaffDecisions, StaffOrder};
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{
    AirGroupId, BaseId, EntityRef, FormationId, IntentId, LandUnitId, ShipId, TaskForceId,
};
use sim_core::orders::{
    AirGroupOrder, AirMission, OrderKind, Posture, RejectReason, SpeedSetting, TaskForceOrder,
};
use sim_core::state::{Side, TruthState};
use sim_core::time::Pulse;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

mod common;

const COMMANDS_MARKER: &str = "# The Allied commander's commands";

fn path() -> PathBuf {
    common::data_dir().join("scenarios/65_a_day_from_which_a_force_may_be_used.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn group(raw: u32) -> AirGroupId {
    AirGroupId::new(raw)
}

fn intent(raw: u32) -> IntentId {
    IntentId::new(raw)
}

fn base(raw: u32) -> BaseId {
    BaseId::new(raw)
}

fn force(raw: u32) -> TaskForceId {
    TaskForceId::new(raw)
}

fn ship(raw: u32) -> ShipId {
    ShipId::new(raw)
}

fn hex(q: i32, r: i32) -> Hex {
    Hex { q, r }
}

fn changed(text: &str, original: &str, replacement: &str) -> String {
    common::replace_once(text, original, replacement)
}

/// The scenario's own figures: scenario 53's table of aircraft for each ship,
/// added after the frozen file's empty `[strike_aircraft_per_ship]` table.
fn figures() -> sim_data::LoadOptions {
    common::altered_reference("s65", |dir| {
        common::edit_file(dir, "air_combat.toml", |text| {
            format!("{text}\ncruiser = 12\ndestroyer = 6\ntransport = 6\nauxiliary = 4\n")
        })
    })
}

/// A game played to its end with the doctrine's staffs: what the Allied staff
/// handed in each day, the truth at the end of each day (index 0 is the
/// start), the log and the side's command state at the end.
struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    truth: Vec<TruthState>,
    events: Vec<Event>,
    state: sim_core::command::CommandState,
}

fn play(text: &str) -> Played {
    play_at(text, &path(), &figures())
}

/// As [`play`], for a scenario text that sits at `at` and loads with `options`.
fn play_at(text: &str, at: &Path, options: &sim_data::LoadOptions) -> Played {
    let mut scenario: Scenario =
        common::from_text_with(text, at, options).expect("the scenario loads");
    let held = staffs();
    let mut handed = BTreeMap::new();
    let mut truth = vec![scenario.world.umpire().clone()];
    loop {
        let day = scenario.world.day() + 1;
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
        handed.insert(
            day,
            scenario
                .world
                .staff_handed_in()
                .get(&Side::Allied)
                .cloned()
                .unwrap_or_default(),
        );
        truth.push(scenario.world.umpire().clone());
    }
    Played {
        handed,
        truth,
        events: scenario.world.events().iter().cloned().collect(),
        state: scenario.world.command_state(Side::Allied).clone(),
    }
}

fn mission(m: AirMission) -> AirGroupOrder {
    AirGroupOrder::SetAirMission(m)
}

fn sizing(on: bool) -> AirGroupOrder {
    AirGroupOrder::SetStrikeSizing { on }
}

const STRIKE: AirMission = AirMission::NavalStrike;
/// An order of the staff: to a force or to an air group.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Given {
    Force(u32, TaskForceOrder),
    Air(u32, AirGroupOrder),
}

fn given(o: &StaffOrder) -> Given {
    match &o.kind {
        OrderKind::TaskForce { target, order } => Given::Force(target.raw(), order.clone()),
        OrderKind::AirGroup { target, order } => Given::Air(target.raw(), order.clone()),
        other => panic!("the doctrine gave an order of another kind: {other:?}"),
    }
}

fn route(waypoints: &[(i32, i32)]) -> TaskForceOrder {
    TaskForceOrder::SetRoute {
        waypoints: waypoints.iter().map(|(q, r)| hex(*q, *r)).collect(),
        act_at: Default::default(),
        stand_at: Default::default(),
    }
}

impl Played {
    fn decisions(&self, day: u32) -> &StaffDecisions {
        &self.handed[&day]
    }

    fn orders(&self, day: u32) -> Vec<Given> {
        self.decisions(day).orders.iter().map(given).collect()
    }

    fn order_count(&self) -> Vec<usize> {
        (1..=self.handed.len() as u32)
            .map(|d| self.decisions(d).orders.len())
            .collect()
    }

    fn all_orders(&self) -> Vec<(u32, &StaffOrder)> {
        self.handed
            .iter()
            .flat_map(|(day, d)| d.orders.iter().map(move |o| (*day, o)))
            .collect()
    }

    fn all(&self, pick: impl Fn(&EventKind) -> bool) -> Vec<&Event> {
        self.events.iter().filter(|e| pick(&e.kind)).collect()
    }

    fn nothing_refused(&self) {
        let refused = self.all(|k| {
            matches!(
                k,
                EventKind::OrderRejected { .. } | EventKind::CommandRefused { .. }
            )
        });
        assert!(refused.is_empty(), "something was refused: {refused:?}");
    }

    fn mission_at_end(&self, raw: u32) -> AirMission {
        self.truth.last().expect("a state").air_groups[&group(raw)].mission
    }

    fn force_at_end(&self) -> &sim_core::state::TaskForce {
        &self.truth.last().expect("a state").task_forces[&force(1)]
    }

    /// Every reason's chain, followed to its end, reaches an intent the
    /// commander made.
    fn every_chain_ends_with_the_commander(&self) {
        for (day, order) in self.all_orders() {
            let reason = &order.reason;
            let served = reason.serves.intent.expect("a reason serves an intent");
            match reason.serves.chain.last() {
                Some(link) => {
                    assert_eq!(link.maker, Maker::Commander, "day {day}: {reason:?}");
                    assert!(!link.statement.is_empty(), "day {day}");
                    assert_eq!(
                        Some(link.intent),
                        Some(self.state.root_of(served)),
                        "day {day}"
                    );
                }
                None => assert_eq!(
                    self.state.intents[&served].maker,
                    Maker::Commander,
                    "day {day}"
                ),
            }
        }
    }
}

/// A reason's text: its calculation and its evidence.
fn said(reason: &Reason) -> String {
    let evidence: Vec<&str> = reason.evidence.iter().map(|e| e.what.as_str()).collect();
    format!("{} | {}", reason.worked, evidence.join(" | "))
}

// ------------------------------------------------------- The variants

fn full() -> Given {
    Given::Force(1, TaskForceOrder::SetSpeed(SpeedSetting::Full))
}

const TF_DAY: &str = "# Task force 1 may be used from day 2 (decision 0140).\navailable_from = 2\n";
const GROUP_3: &str = "aircraft_type = 3\nready = 10\n";
const SHIP_1: &str = "fuel_t = 3000\ntask_force = 1\n";
const AIR_COMBAT: &str = "air_combat = \"../reference/air_combat.toml\"\n";

/// The same file with task force 1's day taken out: "the game without the
/// day".
fn without_day(text: &str) -> String {
    changed(text, TF_DAY, "")
}

fn force_day(text: &str, day: u32) -> String {
    changed(text, TF_DAY, &format!("available_from = {day}\n"))
}

fn group_3_day(text: &str, day: u32) -> String {
    changed(text, GROUP_3, &format!("{GROUP_3}available_from = {day}\n"))
}

fn ship_1_day(text: &str, day: u32) -> String {
    changed(text, SHIP_1, &format!("{SHIP_1}available_from = {day}\n"))
}

/// The scenario's text before the commander's commands: no intent, so no
/// staff order.
fn without_commands(text: &str) -> String {
    let at = text.find(COMMANDS_MARKER).expect("the commands marker");
    text[..at].to_string()
}

/// A commander's order table of the Allied side.
fn order_table(day: u32, body: &str) -> String {
    format!("\n[[order]]\nday = {day}\nside = \"allied\"\n{body}\n")
}

// ------------------------------------------------------- Comparing games

/// The numbers that follow the word "day " in a text, in order.
fn days_named(text: &str) -> Vec<u32> {
    text.match_indices("day ")
        .filter_map(|(at, word)| {
            let digits: String = text[at + word.len()..]
                .chars()
                .take_while(char::is_ascii_digit)
                .collect();
            digits.parse().ok()
        })
        .collect()
}

/// The text with the number after each "day " replaced by "#".
fn days_blanked(text: &str) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some(at) = rest.find("day ") {
        let (before, after) = rest.split_at(at + 4);
        out.push_str(before);
        out.push('#');
        rest = after.trim_start_matches(|c: char| c.is_ascii_digit());
    }
    out.push_str(rest);
    out
}

/// Two texts are the same but for the day they name: blanked of their day
/// numbers they are equal, and each day number of `later` is the one of
/// `earlier` or that one plus `shift`. A difference in anything but a day
/// is a mismatch.
fn same_text_but_for_the_day(earlier: &str, later: &str, shift: u32, what: &str) {
    assert_eq!(
        days_blanked(earlier),
        days_blanked(later),
        "{what}: the texts differ in more than a day"
    );
    let (a, b) = (days_named(earlier), days_named(later));
    assert_eq!(a.len(), b.len(), "{what}");
    for (x, y) in a.iter().zip(&b) {
        assert!(
            y == x || *y == x + shift,
            "{what}: day {x} against day {y}, a shift of {shift}"
        );
    }
}

/// Two reasons are the same but for the day they name: the intent and the
/// chain it serves, the calculation, the binding, what would change it and
/// the evidence are each the same text but for a day number, and the day an
/// evidence is of is the same or `shift` later.
fn same_reason_but_for_the_day(earlier: &Reason, later: &Reason, shift: u32, what: &str) {
    assert_eq!(earlier.serves, later.serves, "{what}");
    same_text_but_for_the_day(&earlier.worked, &later.worked, shift, what);
    assert_eq!(earlier.binding.len(), later.binding.len(), "{what}");
    for (a, b) in earlier.binding.iter().zip(&later.binding) {
        same_text_but_for_the_day(a, b, shift, what);
    }
    assert_eq!(earlier.would_change, later.would_change, "{what}");
    assert_eq!(earlier.evidence.len(), later.evidence.len(), "{what}");
    for (a, b) in earlier.evidence.iter().zip(&later.evidence) {
        same_text_but_for_the_day(&a.what, &b.what, shift, what);
        assert_eq!(b.day, a.day + shift, "{what}: the day of the evidence");
    }
}

/// What the staff handed in on `earlier_day` of `earlier` is what it handed
/// in on `later_day` of `later` but for the day: the same orders, to the
/// same units, serving the same intent, in the same order, with the same
/// reasons but for the day they name, and the same decisions to give no
/// order and the same notices. The staff's commands are not compared: the
/// two approvals of the plan that the staff hands in on its first day in both
/// games are not orders, and are checked on their own.
fn assert_same_day(earlier: &Played, earlier_day: u32, later: &Played, later_day: u32) {
    let (a, b) = (earlier.decisions(earlier_day), later.decisions(later_day));
    let shift = later_day - earlier_day;
    let what = format!("day {later_day} against day {earlier_day}");
    assert_eq!(
        a.orders.len(),
        b.orders.len(),
        "{what}: the count of orders"
    );
    for (x, y) in a.orders.iter().zip(&b.orders) {
        assert_eq!(given(x), given(y), "{what}");
        assert_eq!(x.intent, y.intent, "{what}");
        same_reason_but_for_the_day(&x.reason, &y.reason, shift, &what);
    }
    assert_eq!(a.notices, b.notices, "{what}");
    assert_eq!(a.no_orders.len(), b.no_orders.len(), "{what}: no_orders");
    for (x, y) in a.no_orders.iter().zip(&b.no_orders) {
        same_reason_but_for_the_day(x, y, shift, &what);
    }
}

fn refusals(played: &Played) -> Vec<(u32, EntityRef, RejectReason)> {
    played
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::OrderRejected { target, reason, .. } => {
                Some((e.day, *target, reason.clone()))
            }
            _ => None,
        })
        .collect()
}

fn acceptances(played: &Played) -> Vec<(u32, EntityRef)> {
    played
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::OrderAccepted { target, .. } => Some((e.day, *target)),
            _ => None,
        })
        .collect()
}

fn arrivals(played: &Played) -> Vec<(Hex, Fixed)> {
    played
        .all(|k| matches!(k, EventKind::TaskForceArrived { .. }))
        .iter()
        .map(|e| match &e.kind {
            EventKind::TaskForceArrived {
                hex, elapsed_hours, ..
            } => (*hex, *elapsed_hours),
            _ => unreachable!(),
        })
        .collect()
}

/// The staff's orders to one air group on a day.
fn orders_to_group(played: &Played, day: u32, raw: u32) -> Vec<AirGroupOrder> {
    played
        .orders(day)
        .into_iter()
        .filter_map(|g| match g {
            Given::Air(a, o) if a == raw => Some(o),
            _ => None,
        })
        .collect()
}

/// Every order the staff gave on a day, not to the group.
fn orders_but_group(played: &Played, day: u32, raw: u32) -> Vec<Given> {
    played
        .orders(day)
        .into_iter()
        .filter(|g| !matches!(g, Given::Air(a, _) if *a == raw))
        .collect()
}

/// The reasons of a day's decisions to give no order, as text.
fn no_order_reasons(played: &Played, day: u32) -> Vec<String> {
    played.decisions(day).no_orders.iter().map(said).collect()
}

// ------------------------------------------------------- The hand calculation

#[test]
fn the_file_is_the_one_these_tests_vary() {
    let t = text();
    assert!(t.contains("days = 4\n"));
    assert_eq!(t.matches("[[task_force]]").count(), 1);
    assert_eq!(
        t.matches("available_from").count(),
        2,
        "the header names it too"
    );
    assert!(t.contains(TF_DAY));
    assert!(t.contains(GROUP_3) && t.contains(SHIP_1) && t.contains(AIR_COMBAT));
    assert_eq!(t.matches("[[contact]]").count(), 0);
    assert_eq!(t.matches("[[command]]").count(), 3);
    let c = &t[t.find(COMMANDS_MARKER).expect("the marker")..];
    assert!(c.contains("intent_kind = \"operate_carriers\"\ntask_force = 1\nhex = [6, 0]\n"));
    assert!(!c.contains("limit = \"risk\""), "no risk limit: balanced");
    assert_eq!(c.matches("given = true").count(), 2);
}

#[test]
fn day_1_gives_no_order_and_one_reason_and_the_force_lies_stopped_burning_nothing() {
    let played = play(&text());
    played.nothing_refused();
    let d1 = played.decisions(1);
    assert!(d1.orders.is_empty(), "{:?}", played.orders(1));
    // The commands of the day are not orders: the two approvals of the plan
    // that the staff hands in on its first day with the day or without it.
    let kinds = |p: &Played, day: u32| -> Vec<CommandKind> {
        p.decisions(day)
            .commands
            .iter()
            .map(|c| c.kind.clone())
            .collect()
    };
    let without = play(&without_day(&text()));
    assert_eq!(kinds(&played, 1), kinds(&without, 1));
    assert_eq!(kinds(&played, 1).len(), 2);
    for day in 2..=4 {
        assert!(kinds(&played, day).is_empty(), "day {day}");
    }
    assert!(d1.notices.is_empty());
    assert_eq!(d1.no_orders.len(), 1, "{:?}", no_order_reasons(&played, 1));
    let reason = &d1.no_orders[0];
    assert_eq!(
        reason.worked,
        "intent:1, operating the carrier force task_force:1 from (6,0): intent:1 is to be served by task_force:1, which may be used from day 2, so the staff gives task_force:1 and its groups no order."
    );
    assert!(said(reason).contains("task_force:1 may be used from day 2"));
    assert_eq!(reason.serves.intent, Some(intent(1)));
    // She lies stopped at (0,0) through day 1 and burns no fuel under way.
    assert_eq!(played.truth[1].task_forces[&force(1)].position(), hex(0, 0));
    assert_eq!(played.truth[1].task_forces[&force(1)].toward_hex, None);
    assert_eq!(played.truth[1].ships[&ship(1)].fuel_t, n(3000));
    assert!(
        played
            .events
            .iter()
            .filter(|e| e.day == 1)
            .all(|e| !matches!(
                e.kind,
                EventKind::TaskForceSteamed { .. } | EventKind::FuelBurned { .. }
            )),
        "something moved or burnt on day 1"
    );
}

#[test]
fn days_2_3_and_4_are_the_orders_of_days_1_2_and_3_of_the_game_without_the_day() {
    let with = play(&text());
    let without = play(&without_day(&text()));
    with.nothing_refused();
    without.nothing_refused();
    // The orders of the game without the day: day 1 the force's orders and
    // the groups', day 2 her readiness, day 3 none.
    assert_eq!(without.orders(1).len(), 9, "{:?}", without.orders(1));
    assert_eq!(without.orders(2).len(), 1, "{:?}", without.orders(2));
    assert_eq!(without.orders(3).len(), 0, "{:?}", without.orders(3));
    // The count on each day is the count a day earlier in the game without
    // the day.
    assert_eq!(without.order_count()[..3], [9, 1, 0]);
    assert_eq!(with.order_count(), vec![0, 9, 1, 0]);
    assert_eq!(with.all_orders().len(), 10);
    for day in 2..=4 {
        assert_same_day(&without, day - 1, &with, day);
        // The orders themselves, exactly.
        assert_eq!(with.orders(day), without.orders(day - 1), "day {day}");
    }
    // And the day-1 orders are the speed-alone orders of scenario 58:
    // full speed and a route to (6,0).
    assert!(with.orders(2).contains(&full()));
    assert!(with.orders(2).contains(&Given::Force(1, route(&[(6, 0)]))));
    with.every_chain_ends_with_the_commander();
}

#[test]
fn she_arrives_at_6_0_at_hour_32_with_the_day_and_at_hour_8_without() {
    let with = play(&text());
    let without = play(&without_day(&text()));
    assert_eq!(arrivals(&with), vec![(hex(6, 0), n(32))]);
    assert_eq!(arrivals(&without), vec![(hex(6, 0), n(8))]);
    assert_eq!(with.force_at_end().position(), hex(6, 0));
    assert_eq!(without.force_at_end().position(), hex(6, 0));
    // Hour 32 is hour 8 and a day.
    assert_eq!(n(32), n(8) + n(24));
    let at = force_by_moment(&text());
    assert_eq!(at["1 Night"], hex(0, 0));
    assert_eq!(at["1 Afternoon"], hex(0, 0));
    assert_eq!(at["2 Night"], hex(0, 0));
    assert_eq!(at["2 Morning"], hex(6, 0));
    assert_eq!(at["3 Night"], hex(6, 0));
}

/// Where the force is at the start of each pulse, as scenario 58's test
/// reads it: "1 Morning" is the start of the morning of day 1.
fn force_by_moment(text: &str) -> BTreeMap<String, Hex> {
    let mut held: BTreeMap<String, Hex> = BTreeMap::new();
    let mut scenario: Scenario =
        common::from_text_with(text, &path(), &figures()).expect("the scenario loads");
    let staffs = staffs();
    loop {
        let summary = scenario
            .next_day_watched(as_staffs(&staffs), &mut |world, moment| {
                if let Moment::PulseBegins { day, pulse } = moment {
                    held.insert(
                        format!("{day} {pulse:?}"),
                        world.umpire().task_forces[&force(1)].position(),
                    );
                }
            })
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
    }
    held
}

// --------------------------------------------------------------- Properties

#[test]
fn an_order_before_the_day_is_refused_with_the_day_and_changes_nothing() {
    // No intent: the only order is the commander's.
    let route_order = "task_force = 1\nkind = \"set_route\"\nwaypoints = [[3, 0]]\n";
    let t = format!(
        "{}{}",
        without_commands(&text()),
        order_table(1, route_order)
    );
    let refused = play(&t);
    assert_eq!(
        refusals(&refused),
        vec![(
            1,
            EntityRef::TaskForce(force(1)),
            RejectReason::NotYetAvailable {
                unit: EntityRef::TaskForce(force(1)),
                day: 2
            }
        )]
    );
    assert!(acceptances(&refused).is_empty());
    // It changed nothing: the force never moved and has no route.
    assert!(arrivals(&refused).is_empty());
    assert_eq!(refused.force_at_end().position(), hex(0, 0));
    assert!(refused.force_at_end().route.waypoints.is_empty());
    assert!(refused
        .all(|k| matches!(k, EventKind::TaskForceSteamed { .. }))
        .is_empty());
    // The refusal reads with the day.
    let reason = RejectReason::NotYetAvailable {
        unit: EntityRef::TaskForce(force(1)),
        day: 2,
    };
    assert!(reason.to_string().contains("may be used from day 2"));
    // The same order on day 2 is taken.
    let t = format!(
        "{}{}",
        without_commands(&text()),
        order_table(2, route_order)
    );
    let taken = play(&t);
    assert!(refusals(&taken).is_empty());
    assert_eq!(
        acceptances(&taken),
        vec![(2, EntityRef::TaskForce(force(1)))]
    );
    assert_eq!(taken.force_at_end().position(), hex(3, 0));
    assert_eq!(arrivals(&taken).len(), 1);
    assert_eq!(arrivals(&taken)[0].0, hex(3, 0));
}

#[test]
fn a_groups_own_day_group_3_from_day_3_is_left_out_on_days_1_and_2() {
    // Task force 1 available from day 1 (no key) and group 3 from day 3.
    let t = group_3_day(&without_day(&text()), 3);
    let played = play(&t);
    let game = play(&without_day(&text()));
    played.nothing_refused();
    // On days 1 and 2 the staff gives group 3 no order and every other order
    // is as in the game without the day.
    for day in 1..=2 {
        assert!(
            orders_to_group(&played, day, 3).is_empty(),
            "day {day}: {:?}",
            played.orders(day)
        );
        assert_eq!(
            played.orders(day),
            orders_but_group(&game, day, 3),
            "day {day}"
        );
    }
    // The other orders' reasons are the same too.
    let kept: Vec<&StaffOrder> = game
        .decisions(1)
        .orders
        .iter()
        .filter(|o| !matches!(given(o), Given::Air(3, _)))
        .collect();
    assert_eq!(kept.len(), played.decisions(1).orders.len());
    for (x, y) in kept.iter().zip(&played.decisions(1).orders) {
        same_reason_but_for_the_day(&x.reason, &y.reason, 0, "day 1, a group left out");
    }
    // On day 3 it gives group 3 the orders it gives it on day 1 in that game.
    let on_day_1 = orders_to_group(&game, 1, 3);
    assert_eq!(on_day_1, vec![mission(STRIKE), sizing(true)]);
    assert_eq!(orders_to_group(&played, 3, 3), on_day_1);
    // Its reason on days 1 and 2 names the day.
    assert!(
        no_order_reasons(&played, 1)
            .iter()
            .any(|r| r.contains("air_group:3 may be used from day 3")),
        "{:?}",
        no_order_reasons(&played, 1)
    );
    // A commander's order to group 3 on day 2 is refused with the day, 3;
    // on day 3 it is taken.
    let strike = "air_group = 3\nkind = \"set_air_mission\"\nmission = \"naval_strike\"\n";
    let refused = play(&format!(
        "{}{}",
        without_commands(&group_3_day(&without_day(&text()), 3)),
        order_table(2, strike)
    ));
    assert_eq!(
        refusals(&refused),
        vec![(
            2,
            EntityRef::AirGroup(group(3)),
            RejectReason::NotYetAvailable {
                unit: EntityRef::AirGroup(group(3)),
                day: 3
            }
        )]
    );
    assert_eq!(refused.mission_at_end(3), AirMission::Rest);
    let taken = play(&format!(
        "{}{}",
        without_commands(&group_3_day(&without_day(&text()), 3)),
        order_table(3, strike)
    ));
    assert!(refusals(&taken).is_empty());
    assert_eq!(taken.mission_at_end(3), STRIKE);
}

#[test]
fn the_later_of_two_days_task_force_from_2_and_ship_1_from_3() {
    let played = play(&ship_1_day(&text(), 3));
    played.nothing_refused();
    // Nothing is ordered before day 3.
    assert!(played.orders(1).is_empty(), "{:?}", played.orders(1));
    assert!(played.orders(2).is_empty(), "{:?}", played.orders(2));
    // The reason names day 3.
    for day in 1..=2 {
        let reasons = no_order_reasons(&played, day);
        assert!(
            reasons.iter().any(|r| r.contains("may be used from day 3")),
            "day {day}: {reasons:?}"
        );
        assert!(
            !reasons.iter().any(|r| r.contains("may be used from day 2")),
            "day {day}: {reasons:?}"
        );
    }
    // From day 3 it is the game without the day, two days late.
    let without = play(&without_day(&text()));
    assert_eq!(played.orders(3), without.orders(1));
    assert_eq!(played.orders(4), without.orders(2));
    assert_eq!(played.force_at_end().position(), hex(6, 0));
    assert_eq!(arrivals(&played)[0].0, hex(6, 0));
    assert_eq!(arrivals(&played)[0].1, n(8) + n(48));
}

#[test]
fn the_later_of_two_days_task_force_from_3_and_group_3_from_2() {
    let t = group_3_day(&force_day(&text(), 3), 2);
    let played = play(&t);
    played.nothing_refused();
    for day in 1..=2 {
        assert!(
            played.orders(day).is_empty(),
            "day {day}: {:?}",
            played.orders(day)
        );
        assert!(orders_to_group(&played, day, 3).is_empty());
    }
    let reasons = no_order_reasons(&played, 1);
    assert!(
        reasons.iter().any(|r| r.contains("may be used from day 3")),
        "{reasons:?}"
    );
    // Group 3 is given orders on day 3, with the rest of the force.
    assert!(!orders_to_group(&played, 3, 3).is_empty());
    let without = play(&without_day(&text()));
    assert_eq!(played.orders(3), without.orders(1));
}

#[test]
fn a_land_unit_from_day_3_a_posture_order_on_day_2_is_refused_and_on_day_3_taken() {
    let land = "\n[[land_unit]]\nid = 1\nname = \"Placeholder unit 1\"\nside = \"allied\"\nmen = 2000\nq = 0\nr = 5\nposture = \"defend\"\navailable_from = 3\n";
    let posture = "land_unit = 1\nkind = \"set_posture\"\nposture = \"attack\"\n";
    let ground = format!("{AIR_COMBAT}ground = \"../reference/ground.toml\"\n");
    let build = |day: u32| {
        format!(
            "{}{land}{}",
            changed(&without_commands(&text()), AIR_COMBAT, &ground),
            order_table(day, posture)
        )
    };
    let refused = play(&build(2));
    assert_eq!(
        refusals(&refused),
        vec![(
            2,
            EntityRef::LandUnit(LandUnitId::new(1)),
            RejectReason::NotYetAvailable {
                unit: EntityRef::LandUnit(LandUnitId::new(1)),
                day: 3
            }
        )]
    );
    assert!(acceptances(&refused).is_empty());
    assert_eq!(
        refused.truth.last().expect("a state").land_units[&LandUnitId::new(1)].posture,
        Posture::Defend
    );
    let taken = play(&build(3));
    assert!(refusals(&taken).is_empty());
    assert_eq!(
        acceptances(&taken),
        vec![(3, EntityRef::LandUnit(LandUnitId::new(1)))]
    );
    assert_eq!(
        taken.truth.last().expect("a state").land_units[&LandUnitId::new(1)].posture,
        Posture::Attack
    );
}

// ----------------------------------------------------- A ship in a pool

fn path_36() -> PathBuf {
    common::data_dir().join("scenarios/36_one_convoy.toml")
}

#[test]
fn a_ship_in_a_pool_ship_2_from_day_5_is_committed_to_no_run_and_loaded_with_nothing_before() {
    let original = std::fs::read_to_string(path_36()).expect("scenario 36 reads");
    let ship_2 = "name = \"Placeholder cargo ship 2\"\nclass = 3\nside = \"allied\"\nfuel_t = 1000\nport = 1\n";
    let t = changed(&original, ship_2, &format!("{ship_2}available_from = 5\n"));
    let played = play_at(&t, &path_36(), &common::frozen());
    played.nothing_refused();
    let two = format!("{:?}", ship(2));
    let mut listed = 0;
    for day in 1..=18u32 {
        let d = played.decisions(day);
        let on_day: Vec<String> = d
            .commands
            .iter()
            .map(|c| format!("{:?}", c.kind))
            .chain(d.orders.iter().map(|o| format!("{:?}", o.kind)))
            .collect();
        if day < 5 {
            // No run commits it and nothing is loaded on it.
            for each in &on_day {
                assert!(!each.contains(&two), "day {day}: {each}");
            }
            for c in &d.commands {
                if let CommandKind::AddRun { run, .. } = &c.kind {
                    assert!(!run.ships.contains(&ship(2)), "day {day}");
                }
            }
            // Where a reason lists the pool, it names the ship with its day.
            let reasons: Vec<String> = d
                .commands
                .iter()
                .map(|c| said(&c.reason))
                .chain(d.orders.iter().map(|o| said(&o.reason)))
                .chain(d.no_orders.iter().map(said))
                .collect();
            for r in reasons.iter().filter(|r| r.contains("ship:2")) {
                assert!(
                    r.contains("ship:2 may be used from day 5"),
                    "day {day}: {r}"
                );
                listed += 1;
            }
        }
    }
    // (No reason of days 1 to 4 does name ship 2 here: ship 1 is free and
    // the reasons list the ships the run takes, which is ship 1 alone. The
    // next test has no ship free and the reason lists the pool.)
    assert_eq!(listed, 0);
    // Nothing is on ship 2 before day 5: it stays in port with no cargo.
    for day in 1..=4usize {
        let ship_2 = &played.truth[day].ships[&ship(2)];
        assert_eq!(
            ship_2.location,
            sim_core::state::ShipLocation::Port(base(1)),
            "day {day}"
        );
        assert_eq!(ship_2.supply_t, Fixed::ZERO, "day {day}");
        assert_eq!(ship_2.cargo_fuel_t, Fixed::ZERO, "day {day}");
    }
}

#[test]
fn a_ship_in_a_pool_with_no_ship_free_the_reason_lists_the_pool_with_each_ships_day() {
    let original = std::fs::read_to_string(path_36()).expect("scenario 36 reads");
    let ship_1 = "name = \"Placeholder cargo ship 1\"\nclass = 3\nside = \"allied\"\nfuel_t = 1000\nport = 1\n";
    let ship_2 = "name = \"Placeholder cargo ship 2\"\nclass = 3\nside = \"allied\"\nfuel_t = 1000\nport = 1\n";
    let t = changed(&original, ship_1, &format!("{ship_1}available_from = 3\n"));
    let t = changed(&t, ship_2, &format!("{ship_2}available_from = 5\n"));
    let played = play_at(&t, &path_36(), &common::frozen());
    played.nothing_refused();
    for day in 1..=2u32 {
        let d = played.decisions(day);
        let reasons: Vec<String> = d
            .commands
            .iter()
            .map(|c| said(&c.reason))
            .chain(d.orders.iter().map(|o| said(&o.reason)))
            .chain(d.no_orders.iter().map(said))
            .collect();
        assert!(
            reasons.iter().any(|r| r
                .contains("ship:1 may be used from day 3, ship:2 may be used from day 5")),
            "day {day}: {reasons:?}"
        );
        assert!(!d
            .commands
            .iter()
            .any(|c| matches!(c.kind, CommandKind::AddRun { .. })));
        assert!(d.orders.is_empty());
    }
}

// ---------------------------------- A side knows its own days, not the enemy's

#[test]
fn a_side_knows_its_own_days_and_the_allied_view_gives_task_force_1s() {
    let scenario: Scenario =
        common::from_text_with(&text(), &path(), &figures()).expect("the scenario loads");
    let view = scenario.world.side_view(Side::Allied);
    let own = EntityRef::TaskForce(force(1));
    assert_eq!(view.available_from(own), Some(2));
    assert!(!view.may_use(own));
    // Her ship takes the later of its own day and hers.
    assert_eq!(view.available_from(EntityRef::Ship(ship(1))), Some(2));
    assert_eq!(view.available_from(EntityRef::AirGroup(group(3))), Some(2));
    // The enemy's view of her gives nothing.
    let other = scenario.world.side_view(Side::Japanese);
    assert_eq!(other.available_from(own), None);
    assert!(!other.may_use(own));
    // The day comes: after day 1 she may be used on the orders of day 2.
    let mut scenario = scenario;
    let held = staffs();
    scenario
        .next_day_watched(as_staffs(&held), &mut |_, _| {})
        .expect("a day resolves");
    let view = scenario.world.side_view(Side::Allied);
    assert_eq!(view.available_from(own), Some(2));
    assert!(view.may_use(own));
}

/// A Japanese task force of one carrier far away, with the day given.
fn with_japanese_force(day: u32) -> String {
    format!(
        "{}\n[[task_force]]\nid = 2\nname = \"Placeholder Japanese force 2\"\nside = \"japanese\"\nq = 30\nr = 0\nmission = \"carrier\"\navailable_from = {day}\n\n[[ship]]\nid = 2\nname = \"Placeholder Japanese carrier 2\"\nclass = 4\nside = \"japanese\"\nfuel_t = 3000\ntask_force = 2\n",
        text()
    )
}

#[test]
fn two_worlds_that_differ_only_in_the_day_of_a_japanese_force_give_the_allied_side_the_same_game() {
    let (a, b) = (with_japanese_force(3), with_japanese_force(7));
    let mut scenarios: Vec<Scenario> = [&a, &b]
        .iter()
        .map(|t| common::from_text_with(t, &path(), &figures()).expect("the scenario loads"))
        .collect();
    // The two worlds do differ: the Japanese force's day.
    let japanese = |s: &Scenario| s.world.umpire().task_forces[&force(2)].available_from;
    assert_eq!(japanese(&scenarios[0]), Some(3));
    assert_eq!(japanese(&scenarios[1]), Some(7));
    // The Allied view gives the Japanese force's day to nobody.
    for s in &scenarios {
        let view = s.world.side_view(Side::Allied);
        assert_eq!(view.available_from(EntityRef::TaskForce(force(2))), None);
        assert_eq!(view.available_from(EntityRef::TaskForce(force(1))), Some(2));
    }
    let held = staffs();
    let mut handed: Vec<Vec<StaffDecisions>> = vec![Vec::new(), Vec::new()];
    for (i, s) in scenarios.iter_mut().enumerate() {
        loop {
            let summary = s
                .next_day_watched(as_staffs(&held), &mut |_, _| {})
                .expect("a day resolves");
            if summary.is_none() {
                break;
            }
            handed[i].push(
                s.world
                    .staff_handed_in()
                    .get(&Side::Allied)
                    .cloned()
                    .unwrap_or_default(),
            );
        }
    }
    // The same orders and the same reasons.
    assert_eq!(handed[0], handed[1]);
    assert_eq!(handed[0].len(), 4);
    assert_eq!(
        scenarios[0].world.command_state(Side::Allied),
        scenarios[1].world.command_state(Side::Allied)
    );
    // The same view: what the Allied side may know of the events and of its
    // own units.
    let views: Vec<_> = scenarios
        .iter()
        .map(|s| s.world.side_view(Side::Allied))
        .collect();
    assert_eq!(views[0].events(), views[1].events());
    let day_of = |v: &sim_core::view::SideView<'_>| {
        v.ships()
            .map(|x| (x.id, v.available_from(EntityRef::Ship(x.id))))
            .collect::<Vec<_>>()
    };
    assert_eq!(day_of(&views[0]), day_of(&views[1]));
    // And the same game as the world with no Japanese force, for the Allied
    // side's decisions.
    let plain = play(&text());
    assert_eq!(
        handed[0],
        plain.handed.values().cloned().collect::<Vec<_>>()
    );
}

// ---------------------------------------------------- No day, no change

/// The scenario files of `data/scenarios`, the top level only: the folders
/// `staff`, `step2-frozen` and the other proofs are not walked, as the other
/// loops that walk `data/scenarios/*.toml` do not walk them (decision 0072).
fn top_level_scenarios() -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = std::fs::read_dir(common::data_dir().join("scenarios"))
        .expect("the scenarios folder reads")
        .map(|entry| entry.expect("an entry").path())
        .filter(|p| p.extension().is_some_and(|e| e == "toml"))
        .collect();
    files.sort();
    files
}

/// A scenario built before this one has no `available_from` key and no
/// formation: a unit with no day may be used from day 1 and a game with no
/// formation has none, so nothing in it differs from before. That the events
/// are unchanged is what the run baselines and the pinned hashes check
/// (`scenario_07_determinism`, `tests/golden/pinned_hashes.txt` and the
/// scenarios' own hand calculations); this test checks the cause, that no
/// earlier scenario sets either.
#[test]
fn no_day_no_change_no_earlier_scenario_has_a_day_or_a_formation() {
    let mut checked = 0;
    for file in top_level_scenarios() {
        // Scenario 77 also gives a force a day (decision 0152), and scenarios
        // 80 and 96 a unit (decisions 0159 and 0177).
        if file == path()
            || file.file_name().is_some_and(|name| {
                let name = name.to_string_lossy();
                name.starts_with("77_") || name.starts_with("80_") || name.starts_with("96_")
            })
        {
            continue;
        }
        let text = std::fs::read_to_string(&file).expect("a scenario reads");
        for line in text.lines() {
            let line = line.trim_start();
            assert!(
                !line.starts_with("available_from"),
                "{} sets a day: {line}",
                file.display()
            );
            assert!(
                !line.starts_with("[[formation]]") && !line.starts_with("formation"),
                "{} has a formation: {line}",
                file.display()
            );
        }
        checked += 1;
    }
    assert!(checked >= 50, "only {checked} scenarios");
}

// ------------------------------------------------------------ Formations

const FORMATIONS: &str = r#"
[[formation]]
id = 1
name = "Placeholder Division"
side = "allied"

[[formation]]
id = 2
name = "Placeholder Regiment"
side = "allied"
parent = 1

[[formation]]
id = 3
name = "Placeholder Battalion 1"
side = "allied"
parent = 2

[[formation]]
id = 4
name = "Placeholder Battalion 2"
side = "allied"
parent = 2

[[formation]]
id = 5
name = "Placeholder Japanese Division"
side = "japanese"
"#;

fn two_land_units(formations: bool) -> String {
    let unit = |id: u32, formation: u32| {
        let tail = if formations {
            format!("formation = {formation}\n")
        } else {
            String::new()
        };
        format!(
            "\n[[land_unit]]\nid = {id}\nname = \"Placeholder unit {id}\"\nside = \"allied\"\nmen = 1000\nq = 0\nr = 5\nposture = \"defend\"\n{tail}"
        )
    };
    format!("{}{}", unit(1, 3), unit(2, 4))
}

/// The file with two land units ashore at base 1, and the formations of
/// the doc comment if `formations`; with the commands, so that the staff
/// has something to do.
fn with_land_units(formations: bool) -> String {
    let ground = format!("{AIR_COMBAT}ground = \"../reference/ground.toml\"\n");
    format!(
        "{}{}{}",
        changed(&text(), AIR_COMBAT, &ground),
        two_land_units(formations),
        if formations { FORMATIONS } else { "" }
    )
}

#[test]
fn formations_the_view_gives_each_units_formation_and_each_formations_parent() {
    let scenario: Scenario = common::from_text_with(&with_land_units(true), &path(), &figures())
        .expect("the scenario loads");
    let view = scenario.world.side_view(Side::Allied);
    let tree: Vec<(u32, String, Option<u32>)> = view
        .formations()
        .map(|f| (f.id.raw(), f.name.clone(), f.parent.map(|p| p.raw())))
        .collect();
    assert_eq!(
        tree,
        vec![
            (1, "Placeholder Division".to_string(), None),
            (2, "Placeholder Regiment".to_string(), Some(1)),
            (3, "Placeholder Battalion 1".to_string(), Some(2)),
            (4, "Placeholder Battalion 2".to_string(), Some(2)),
        ],
        "the Japanese division is not the Allied side's to know"
    );
    let units: Vec<(u32, Option<FormationId>)> = view
        .land_units()
        .map(|u| (u.id.raw(), u.formation))
        .collect();
    assert_eq!(
        units,
        vec![
            (1, Some(FormationId::new(3))),
            (2, Some(FormationId::new(4)))
        ]
    );
    // The division has no parent: the tree from a unit up ends there.
    let formations: BTreeMap<FormationId, _> = view.formations().map(|f| (f.id, f)).collect();
    let mut at = FormationId::new(3);
    let mut up = vec![at.raw()];
    while let Some(parent) = formations[&at].parent {
        up.push(parent.raw());
        at = parent;
    }
    assert_eq!(up, vec![3, 2, 1]);
    // The Japanese side's view gives its own and not the Allied side's.
    let other: Vec<u32> = scenario
        .world
        .side_view(Side::Japanese)
        .formations()
        .map(|f| f.id.raw())
        .collect();
    assert_eq!(other, vec![5]);
}

#[test]
fn formations_the_loader_refuses_a_parent_that_is_not_there_and_a_loop() {
    let load = |formations: &str| {
        let t = format!("{}{formations}", with_land_units(false));
        common::from_text_with(&t, &path(), &figures())
            .err()
            .map(|e| format!("{e:?}"))
    };
    let missing = load(
        "\n[[formation]]\nid = 1\nname = \"Placeholder Regiment\"\nside = \"allied\"\nparent = 9\n",
    );
    let missing = missing.expect("a parent that is not there is refused");
    assert!(missing.contains("not there"), "{missing}");
    let own = load(
        "\n[[formation]]\nid = 1\nname = \"Placeholder Regiment\"\nside = \"allied\"\nparent = 1\n",
    );
    let own = own.expect("a formation under itself is refused");
    assert!(own.contains("ancestor"), "{own}");
    let two = load(
        "\n[[formation]]\nid = 1\nname = \"Placeholder A\"\nside = \"allied\"\nparent = 2\n\n[[formation]]\nid = 2\nname = \"Placeholder B\"\nside = \"allied\"\nparent = 1\n",
    );
    let two = two.expect("two formations each under the other are refused");
    assert!(two.contains("ancestor"), "{two}");
    // A good one loads.
    assert!(load(
        "\n[[formation]]\nid = 1\nname = \"Placeholder A\"\nside = \"allied\"\n\n[[formation]]\nid = 2\nname = \"Placeholder B\"\nside = \"allied\"\nparent = 1\n"
    )
    .is_none());
}

#[test]
fn formations_change_no_order_and_no_event() {
    let with = play(&with_land_units(true));
    let without = play(&with_land_units(false));
    with.nothing_refused();
    assert_eq!(with.handed, without.handed);
    assert_eq!(with.state, without.state);
    // The events, kind by kind and in order, with their days and pulses.
    let kinds = |p: &Played| -> Vec<(u32, Option<Pulse>, String)> {
        p.events
            .iter()
            .map(|e| (e.day, e.pulse, format!("{:?}", e.kind)))
            .collect()
    };
    assert!(!kinds(&with).is_empty());
    assert_eq!(kinds(&with), kinds(&without));
    // Nothing else of the log differs either: the ids and causes.
    assert_eq!(with.events, without.events);
}

// ------------------------------------------- The staff's setup of the 39 days

fn staff_file() -> PathBuf {
    common::data_dir().join("scenarios/staff/07_full_slice.toml")
}

fn scripted_file() -> PathBuf {
    common::data_dir().join("scenarios/07_full_slice.toml")
}

/// The text of scenario 7 with every `[[order]]` table taken out: each from
/// its header to the next line that opens another table. Comments between an
/// order and the next table go with it; they change nothing.
fn without_orders(text: &str) -> String {
    let mut out = String::new();
    let mut inside = false;
    for line in text.lines() {
        if line.starts_with("[[order]]") {
            inside = true;
            continue;
        }
        if inside && line.starts_with('[') {
            inside = false;
        }
        if !inside {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

/// The events of a log as (day, pulse, kind), in order.
fn kinds_of(events: &[Event]) -> Vec<(u32, Option<Pulse>, EventKind)> {
    events
        .iter()
        .map(|e| (e.day, e.pulse, e.kind.clone()))
        .collect()
}

/// A game played to its end with every side's staff, as `play` does: what
/// every side's staff handed in each day, and the log.
struct PlayedAll {
    orders: usize,
    commands: usize,
    events: Vec<Event>,
}

fn play_all(mut scenario: Scenario) -> PlayedAll {
    let held = staffs();
    let (mut orders, mut commands) = (0, 0);
    loop {
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
        for decisions in scenario.world.staff_handed_in().values() {
            orders += decisions.orders.len();
            commands += decisions.commands.len();
        }
    }
    PlayedAll {
        orders,
        commands,
        events: scenario.world.events().iter().cloned().collect(),
    }
}

// The full slice is run here as `scenario_07_determinism` runs it, in the
// ordinary test run, with no `#[ignore]`: that file marks none of its
// 39-day runs, and this test runs two of them.
#[test]
fn the_staffs_setup_of_the_39_days_gives_the_events_of_the_scripted_file_without_its_orders() {
    let staff = common::load_live(&staff_file()).expect("the staff's setup loads");
    assert_eq!(staff.days, 39);
    assert!(
        staff.scripted_orders().values().all(Vec::is_empty),
        "the staff's setup holds a scripted order"
    );
    assert!(staff.scripted_commands().values().all(Vec::is_empty));
    let staff = play_all(staff);
    // No order from anyone and none refused.
    assert_eq!(staff.orders, 0);
    assert_eq!(staff.commands, 0);
    let refused: Vec<_> = staff
        .events
        .iter()
        .filter(|e| {
            matches!(
                e.kind,
                EventKind::OrderRejected { .. }
                    | EventKind::OrderAccepted { .. }
                    | EventKind::CommandRefused { .. }
            )
        })
        .collect();
    assert!(refused.is_empty(), "{refused:?}");
    // The events scenario 7 gives with its orders taken out.
    let text = std::fs::read_to_string(scripted_file()).expect("scenario 7 reads");
    let bare = without_orders(&text);
    assert!(bare.len() < text.len());
    let scenario = Scenario::from_text(&bare, &scripted_file()).expect("it loads");
    assert!(scenario.scripted_orders().values().all(Vec::is_empty));
    let bare = play_all(scenario);
    assert_eq!(bare.orders, 0);
    assert!(!staff.events.is_empty());
    assert_eq!(kinds_of(&staff.events), kinds_of(&bare.events));
}

/// Units whose day in the staff's setup is not the day of their first
/// scripted order, because the architect ruled another: (kind, id, day),
/// kind one of "task_force", "ship", "air_group", "land_unit". Day 1 is no
/// key. The architect's rulings of Nov 17, 2026 (decision 0140, As built): a
/// unit's day is the day the rows have it there to use, and an air group
/// whose aircraft arrive in the file takes the day of that arrival.
const RULED: &[(&str, u32, u32)] = &[
    // On station by engagement_positions 334, 335, 336, 350 and 334, 365, 369.
    ("task_force", 3, 17),
    ("task_force", 4, 17),
    // Flying on 7 August by air_unit_facts 18, 362, 363, 56, 355 and 82, 83.
    ("air_group", 1, 1),
    ("air_group", 2, 1),
    // The day their aircraft arrive in the file.
    ("air_group", 3, 25),
    ("air_group", 10, 14),
    ("air_group", 11, 14),
    ("air_group", 13, 24),
    ("air_group", 14, 24),
    ("air_group", 15, 36),
    ("air_group", 16, 38),
    ("air_group", 17, 38),
];

/// The units an order names, as intake reads them.
fn units_an_order_names(kind: &OrderKind) -> Vec<EntityRef> {
    let mut named = vec![kind.target()];
    match kind {
        OrderKind::Base {
            order: sim_core::orders::BaseOrder::FormTaskForce { ships },
            ..
        } => named.extend(ships.iter().map(|s| EntityRef::Ship(*s))),
        OrderKind::TaskForce {
            order: TaskForceOrder::LoadUnit { unit, .. },
            ..
        } => named.push(EntityRef::LandUnit(*unit)),
        _ => {}
    }
    named
}

#[test]
fn every_unit_with_a_day_in_the_staffs_setup_has_the_day_of_its_first_scripted_order_or_the_ruled_day(
) {
    let scripted = common::load_live(&scripted_file()).expect("scenario 7 loads");
    let mut first: BTreeMap<EntityRef, u32> = BTreeMap::new();
    for (day, orders) in scripted.scripted_orders() {
        for (_, request) in orders {
            for unit in units_an_order_names(&request.kind) {
                first.entry(unit).or_insert(*day);
            }
        }
    }
    assert!(!first.is_empty());
    let staff = common::load_live(&staff_file()).expect("the staff's setup loads");
    let truth = staff.world.umpire();
    let mut with_days: Vec<(&str, u32, u32)> = Vec::new();
    for (id, f) in &truth.task_forces {
        if let Some(day) = f.available_from {
            with_days.push(("task_force", id.raw(), day));
        }
    }
    for (id, s) in &truth.ships {
        if let Some(day) = s.available_from {
            with_days.push(("ship", id.raw(), day));
        }
    }
    for (id, g) in &truth.air_groups {
        if let Some(day) = g.available_from {
            with_days.push(("air_group", id.raw(), day));
        }
    }
    for (id, u) in &truth.land_units {
        if let Some(day) = u.available_from {
            with_days.push(("land_unit", id.raw(), day));
        }
    }
    assert!(
        !with_days.is_empty(),
        "no unit has a day in the staff's setup"
    );
    let mut wrong = Vec::new();
    for (kind, raw, day) in &with_days {
        let unit = match *kind {
            "task_force" => EntityRef::TaskForce(TaskForceId::new(*raw)),
            "ship" => EntityRef::Ship(ShipId::new(*raw)),
            "air_group" => EntityRef::AirGroup(AirGroupId::new(*raw)),
            _ => EntityRef::LandUnit(LandUnitId::new(*raw)),
        };
        let ruled = RULED.iter().find(|(k, r, _)| k == kind && r == raw);
        let expected = match ruled {
            Some((_, _, ruled_day)) => Some(*ruled_day),
            None => first.get(&unit).copied(),
        };
        if expected != Some(*day) {
            wrong.push(format!(
                "{kind}:{raw} has day {day}, expected {expected:?} (first scripted order {:?}, ruled {ruled:?})",
                first.get(&unit)
            ));
        }
    }
    // Every ruled day is the file's own key, day 1 being no key.
    for (kind, raw, day) in RULED {
        let own = with_days
            .iter()
            .find(|(k, r, _)| k == kind && r == raw)
            .map(|(_, _, d)| *d);
        let wanted = (*day > 1).then_some(*day);
        if own != wanted {
            wrong.push(format!("{kind}:{raw} has {own:?}, ruled {day}"));
        }
    }
    assert!(
        wrong.is_empty(),
        "{} units:\n{}",
        wrong.len(),
        wrong.join("\n")
    );
}

// ------------------------------------------------------ The same start

#[test]
fn the_same_start_gives_the_same_game() {
    let a = play(&text());
    let b = play(&text());
    assert_eq!(a.handed, b.handed);
    assert_eq!(a.state, b.state);
    assert_eq!(a.events, b.events);
    assert_eq!(a.truth, b.truth);
}
