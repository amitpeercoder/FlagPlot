//! Scenario 57: going back to rearm. Scenario 54's force, with one of its
//! two cruisers short of rounds: the staff keeps the force in port on the
//! first night, she rearms, and the force sorties on the second.
//!
//! Proves decision 0127 against a hand calculation, with decision 0122's
//! rule beneath it. It stands on scenario 54's for the sortie's hours,
//! places and fuel. The data is placeholder data: see
//! `data/scenarios/57_going_back_to_rearm.toml`. It runs on the frozen
//! step 1 figures with scenario 52's rearming rates and weights. There is
//! no enemy unit and no random draw.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Figures
//!
//! Scenario 54's, and scenario 52's: a size 1 port has one berth, where
//! ammunition is worked at 5 t an hour; a round weighs 0.1 t for a
//! cruiser, so a cruiser at the berth takes 50 rounds an hour; a torpedo
//! weighs 2 t. The placeholder cruiser's outfit is 800 rounds, and she has
//! 8 tubes.
//!
//! ## Inputs
//!
//! Scenario 54's map, base 1 at S with port size 1, the waters at (3,0),
//! task force 1 of ships 1 and 2, the contact of 3 ships at (10,0), the
//! intent and its forces limit, 4 days. Balanced.
//!
//! Changed: base 1 holds 100 t of ammunition. Ship 1 has 300 rounds
//! aboard; ship 2 has her 800. No tube of either has fired.
//!
//! ## Day 1
//!
//! There is a threat, and the force has the fuel. At balanced a ship is
//! short below a half of her outfit: 400 rounds. Ship 1 has 300: the force
//! is short and does not sortie.
//!
//! Ship 1 lacks 500 rounds, 50 t. She is the only ship at base 1 with
//! work and has the berth: 500 rounds in 10 hours. She will be no longer
//! short at hour 2, with 400 aboard, and full at hour 10. The reason gives
//! 300 against 400 and day 2 as the day from which the force may be used.
//!
//! The staff's orders of day 1 are two: the home port, and the rules of
//! engagement. The force is not set to battle readiness: it does not
//! sortie.
//!
//! In the night of day 1 ship 1 takes 500 rounds, 50 t, and is full at
//! hour 10. Base 1 holds 50 t of ammunition.
//!
//! ## Day 2
//!
//! The contact is held, at age 3. The force is short of nothing. It
//! sorties, as scenario 54's first sortie a day later: two orders, battle
//! readiness and the route; it halts at (3,0) at hour 31.5, steams on at
//! hour 36 and is at S, stopped and full of fuel, at hour 43.5.
//!
//! ## Days 3 and 4
//!
//! No contact is held. On day 3, one order: normal readiness. On day 4,
//! none.
//!
//! The staff gives 2, 2, 1 and 0 orders: 5 in all. No notice is handed
//! in.
//!
//! # Properties
//!
//! - **Press.** A ship is short below a quarter, 200 rounds. Ship 1 has
//!   300: the force sorties on day 1 and on every night after. She sails
//!   at the start of the night of day 1 before any work is done, and has
//!   taken no round when she sails.
//! - **Preserve the force.** With the contact reporting 2 ships, so that
//!   the threat counts: a ship is short below three quarters, 600 rounds.
//!   The force is short on day 1 and sorties on day 2.
//! - **No ammunition ashore.** With base 1 holding none: the force does
//!   not sortie on day 1 or day 2, the reason gives no day, and the staff
//!   hands in on day 1 that it cannot meet intent 2.
//! - **Torpedoes.** With ship 1 full of rounds and 5 of her 8 tubes fired:
//!   at balanced more than a half are fired and the force is short. She
//!   takes 5 torpedoes, 10 t, in 2 hours of the night of day 1, and the
//!   force sorties on day 2. With 4 fired she is not short and the force
//!   sorties on day 1. At preserve the force one fired tube makes her
//!   short; at press no number does.
//! - **The later of fuel and ammunition.** With ship 1 at 300 rounds and
//!   both ships at 70 t of fuel, against the 72 the sortie needs: the
//!   reason of day 1 gives both shortages and day 2.
//! - **A short escort is not free.** In scenario 56, with ship 3 at a
//!   tenth of her rounds: the first run sails alone, and its reason gives
//!   her rounds against the level.
//! - **The marker.** A force whose ships carry 9,999 rounds a gun is never
//!   short of rounds.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see give the same orders and reasons.

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use sim_core::command::{NoticeKind, Reason, StaffDecisions, StaffOrder};
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::orders::{OrderKind, Readiness, Stance, StragglerInstruction, TaskForceOrder};
use sim_core::state::{HaltWork, Side, TruthState};
use sim_core::time::Pulse;
use sim_core::world::SimConfig;
use sim_data::{LoadOptions, Scenario};
use sim_staff::{as_staffs, staffs};

use crate::common;
use crate::common::replace_once as changed;
use crate::common::values::{base, command_table, force, fx, hex, n, ship};

fn path() -> PathBuf {
    common::data_dir().join("scenarios/57_going_back_to_rearm.toml")
}

fn path_56() -> PathBuf {
    common::data_dir().join("scenarios/56_an_escorted_run.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn with(extra: &str) -> String {
    format!("{}{}", text(), extra)
}

fn risk(level: &str) -> String {
    command_table(
        1,
        &format!("kind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"{level}\""),
    )
}

/// Ship 1 full of rounds, with `fired` of her tubes fired.
fn tubes(fired: u32) -> String {
    changed(
        &text(),
        "gun_rounds = 300",
        &format!("tubes_fired = {fired}"),
    )
}

/// The contact reporting 2 ships, so that the threat counts at preserve.
fn two_ships(text: &str) -> String {
    changed(text, "ship_count = 3", "ship_count = 2")
}

/// A game played to its end with the doctrine's staffs, breakdowns recorded
/// (which changes nothing that happens).
struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    truth: Vec<TruthState>,
    events: Vec<Event>,
    state: sim_core::command::CommandState,
}

fn play(text: &str) -> Played {
    play_at(text, &path(), &common::frozen())
}

fn play_at(text: &str, at: &Path, options: &LoadOptions) -> Played {
    let mut scenario: Scenario =
        common::from_text_with(text, at, options).expect("the scenario loads");
    scenario.world.set_config(SimConfig {
        record_breakdowns: true,
    });
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

fn tf_order(o: &StaffOrder) -> TaskForceOrder {
    match &o.kind {
        OrderKind::TaskForce { target, order } => {
            assert_eq!(*target, force(1), "an order to another force");
            order.clone()
        }
        other => panic!("the doctrine gave a non-task-force order: {other:?}"),
    }
}

fn route(waypoints: &[(i32, i32)], stand: &[(i32, i32)]) -> TaskForceOrder {
    TaskForceOrder::SetRoute {
        waypoints: waypoints.iter().map(|(q, r)| hex(*q, *r)).collect(),
        act_at: Default::default(),
        stand_at: stand.iter().map(|(q, r)| hex(*q, *r)).collect(),
    }
}

fn sortie() -> TaskForceOrder {
    route(&[(3, 0), (0, 0)], &[(3, 0)])
}

fn engage(break_off: &str) -> TaskForceOrder {
    TaskForceOrder::SetRulesOfEngagement {
        stance: Stance::Engage,
        break_off: fx(break_off),
        stragglers: StragglerInstruction::Keep,
        keep_share: Fixed::ZERO,
    }
}

const BATTLE: TaskForceOrder = TaskForceOrder::SetReadiness(Readiness::Battle);
const NORMAL: TaskForceOrder = TaskForceOrder::SetReadiness(Readiness::Normal);

fn home_port() -> TaskForceOrder {
    TaskForceOrder::SetHomePort(base(1))
}

fn night_work() -> HaltWork {
    HaltWork {
        bombard: false,
        cargo: false,
        night: true,
    }
}

/// One rearming, as the `ShipRearmed` event records it.
#[derive(Clone, Debug)]
struct Rearmed {
    day: u32,
    pulse: Option<Pulse>,
    ship: u32,
    rounds: u32,
    torpedoes: u32,
    taken_t: Fixed,
    rounds_aboard: Fixed,
    tubes_fired: u32,
    base_ammunition_t: Fixed,
    rate: Fixed,
}

impl Played {
    fn decisions(&self, day: u32) -> &StaffDecisions {
        &self.handed[&day]
    }

    fn orders(&self, day: u32) -> Vec<TaskForceOrder> {
        self.decisions(day).orders.iter().map(tf_order).collect()
    }

    fn order_count(&self) -> Vec<usize> {
        (1..=self.handed.len() as u32)
            .map(|d| self.decisions(d).orders.len())
            .collect()
    }

    fn nothing_refused(&self) {
        let refused: Vec<&Event> = self
            .events
            .iter()
            .filter(|e| {
                matches!(
                    e.kind,
                    EventKind::OrderRejected { .. } | EventKind::CommandRefused { .. }
                )
            })
            .collect();
        assert!(refused.is_empty(), "something was refused: {refused:?}");
    }

    fn halts(&self) -> Vec<(u32, Option<Pulse>, Hex, HaltWork, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::TaskForceHalted {
                    hex,
                    work,
                    elapsed_hours,
                    ..
                } => Some((e.day, e.pulse, *hex, *work, *elapsed_hours)),
                _ => None,
            })
            .collect()
    }

    fn steamed_on(&self) -> Vec<(u32, Option<Pulse>, Hex)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::TaskForceSteamedOn { hex, .. } => Some((e.day, e.pulse, *hex)),
                _ => None,
            })
            .collect()
    }

    fn steamed_in(&self, day: u32) -> usize {
        self.events
            .iter()
            .filter(|e| e.day == day && matches!(e.kind, EventKind::TaskForceSteamed { .. }))
            .count()
    }

    fn arrivals(&self) -> Vec<(u32, Option<Pulse>, Hex, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::TaskForceArrived {
                    hex, elapsed_hours, ..
                } => Some((e.day, e.pulse, *hex, *elapsed_hours)),
                _ => None,
            })
            .collect()
    }

    fn refuelled(&self) -> Vec<(u32, Option<Pulse>, u32, Fixed, Fixed, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::ShipRefuelled {
                    ship,
                    taken_t,
                    fuel_t,
                    base_fuel_t,
                    ..
                } => Some((e.day, e.pulse, ship.raw(), *taken_t, *fuel_t, *base_fuel_t)),
                _ => None,
            })
            .collect()
    }

    fn rearmed(&self) -> Vec<Rearmed> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::ShipRearmed {
                    ship,
                    rounds,
                    torpedoes,
                    taken_t,
                    rounds_aboard,
                    tubes_fired,
                    base_ammunition_t,
                    ..
                } => {
                    let rate = e
                        .breakdown
                        .as_ref()
                        .expect("breakdown recorded")
                        .factors
                        .iter()
                        .find(|f| f.name == "rate_t_per_h")
                        .expect("the rate")
                        .value;
                    Some(Rearmed {
                        day: e.day,
                        pulse: e.pulse,
                        ship: ship.raw(),
                        rounds: *rounds,
                        torpedoes: *torpedoes,
                        taken_t: *taken_t,
                        rounds_aboard: *rounds_aboard,
                        tubes_fired: *tubes_fired,
                        base_ammunition_t: *base_ammunition_t,
                        rate,
                    })
                }
                _ => None,
            })
            .collect()
    }

    fn force_at_end_of(&self, day: usize) -> &sim_core::state::TaskForce {
        &self.truth[day].task_forces[&force(1)]
    }

    fn cannot_meet(&self, day: u32) -> usize {
        self.decisions(day)
            .notices
            .iter()
            .filter(|n| n.kind == NoticeKind::CannotMeet)
            .count()
    }

    /// The reasons of the decisions to give no order on a day.
    fn none(&self, day: u32) -> Vec<String> {
        self.decisions(day).no_orders.iter().map(said).collect()
    }

    fn no_sortie(&self) {
        assert!(self.halts().is_empty(), "{:?}", self.halts());
        assert!(self.arrivals().is_empty());
        for day in 1..=self.handed.len() as u32 {
            assert_eq!(self.steamed_in(day), 0, "day {day}");
        }
    }
}

/// A reason's text: its calculation and its evidence.
fn said(reason: &Reason) -> String {
    let evidence: Vec<&str> = reason.evidence.iter().map(|e| e.what.as_str()).collect();
    format!("{} | {}", reason.worked, evidence.join(" | "))
}

// ------------------------------------------------------- The hand calculation

#[test]
fn the_commands_and_the_figures_of_the_file_are_the_ones_these_tests_vary() {
    let t = text();
    assert!(t.contains("port_sizes = \"../reference/port_sizes_placeholder_rearming.csv\""));
    assert!(
        t.contains("surface_combat = \"../reference/surface_combat_placeholder_rearming.toml\"")
    );
    assert!(t.contains("fuel_t = 5000\nammunition_t = 100\n"));
    assert_eq!(t.matches("gun_rounds = 300").count(), 1);
    assert_eq!(t.matches("[[command]]").count(), 4);
    assert!(!t.contains("risk ="));
    assert!(t.contains("placeholder = true"));
}

#[test]
fn day_1_the_force_is_short_300_rounds_against_400_and_stays_in_port_two_orders() {
    let played = play(&text());
    played.nothing_refused();
    // Day 1: the home port and the rules of engagement. Not battle
    // readiness: the force does not sortie.
    assert_eq!(played.orders(1), vec![home_port(), engage("0.5")]);
    assert!(!played.orders(1).contains(&BATTLE));
    // The reason gives 300 against 400 and day 2.
    let none = played.none(1);
    assert!(
        none.iter().any(|r| r
            .contains("ship:1 has 300 rounds against the 400 of a 0.5 of her outfit of 800")
            && r.contains("the force is short of ammunition")
            && r.contains("it rearms at")
            && r.contains("may be used from day 2")),
        "{none:?}"
    );
    // Ship 2 has her 800 rounds: she is not named as short of them.
    assert!(
        none.iter().all(|r| !r.contains("ship:2 has 800 rounds")),
        "{none:?}"
    );
    assert_eq!(played.cannot_meet(1), 0);
    played.no_sortie_on(1);
}

impl Played {
    fn no_sortie_on(&self, day: u32) {
        assert_eq!(self.steamed_in(day), 0, "day {day}");
        assert!(!self.halts().iter().any(|h| h.0 == day), "day {day}");
    }
}

#[test]
fn in_the_night_of_day_1_ship_1_takes_500_rounds_50_t_in_10_hours_and_base_1_holds_50_t() {
    let played = play(&text());
    let rearmed = played.rearmed();
    assert_eq!(rearmed.len(), 1, "{rearmed:?}");
    let one = &rearmed[0];
    assert_eq!((one.day, one.pulse), (1, Some(Pulse::Night)));
    assert_eq!(one.ship, 1);
    assert_eq!((one.rounds, one.torpedoes), (500, 0));
    assert_eq!(one.taken_t, n(50));
    assert_eq!(one.rounds_aboard, n(800));
    assert_eq!(one.tubes_fired, 0);
    assert_eq!(one.base_ammunition_t, n(50));
    // The hours are the tons taken over the rate: 50 t at 5 t an hour, 10
    // hours, full at hour 10.
    assert_eq!(one.rate, n(5));
    assert_eq!(one.taken_t / one.rate, n(10));
    // The end of day 1 and of every day after.
    for day in 1..=4 {
        assert_eq!(
            played.truth[day].ships[&ship(1)].gun_rounds,
            n(800),
            "{day}"
        );
        assert_eq!(
            played.truth[day].ships[&ship(2)].gun_rounds,
            n(800),
            "{day}"
        );
        assert_eq!(
            played.truth[day].bases[&base(1)].ammunition_t,
            n(50),
            "{day}"
        );
    }
    assert_eq!(played.truth[0].ships[&ship(1)].gun_rounds, n(300));
    assert_eq!(played.truth[0].bases[&base(1)].ammunition_t, n(100));
}

#[test]
fn day_2_the_force_sorties_two_orders_halts_at_3_0_at_hour_31_and_a_half() {
    let played = play(&text());
    assert_eq!(played.orders(2), vec![BATTLE, sortie()]);
    // The contact is held at age 3 on day 2.
    assert_eq!(
        played.halts(),
        vec![(2, Some(Pulse::Night), hex(3, 0), night_work(), fx("31.5"))]
    );
    // Steams on at hour 36, the start of the morning of day 2.
    assert_eq!(
        played.steamed_on(),
        vec![(2, Some(Pulse::Morning), hex(3, 0))]
    );
    // At S at hour 43.5, stopped and full of fuel (as scenario 54's).
    assert_eq!(
        played.arrivals(),
        vec![(2, Some(Pulse::Afternoon), hex(0, 0), fx("43.5"))]
    );
    let f = played.force_at_end_of(2);
    assert_eq!(f.position(), hex(0, 0));
    assert_eq!(f.toward_hex, None);
    assert!(!matches!(
        f.status,
        sim_core::state::TaskForceStatus::Underway | sim_core::state::TaskForceStatus::Halted
    ));
    assert_eq!(played.truth[2].ships[&ship(1)].fuel_t, n(2000));
    assert_eq!(played.truth[2].ships[&ship(2)].fuel_t, n(2000));
    // Day 1 it does not move.
    assert_eq!(played.force_at_end_of(1).position(), hex(0, 0));
    assert_eq!(played.truth[1].ships[&ship(1)].fuel_t, n(2000));
    // Each ship takes the 60 t she burned, in the afternoon of day 2.
    let filled = played.refuelled();
    assert_eq!(filled.len(), 2, "{filled:?}");
    for (day, pulse, _, taken, fuel, _) in &filled {
        assert_eq!((*day, *pulse), (2, Some(Pulse::Afternoon)));
        assert_eq!(*taken, n(60));
        assert_eq!(*fuel, n(2000));
    }
    // No more rearming after the night of day 1: she fired nothing.
    assert_eq!(played.rearmed().len(), 1);
}

#[test]
fn days_3_and_4_one_order_and_none_2_2_1_0_in_all_5_and_no_notice() {
    let played = play(&text());
    played.nothing_refused();
    assert_eq!(played.orders(3), vec![NORMAL]);
    assert!(played.orders(4).is_empty());
    assert_eq!(played.order_count(), vec![2, 2, 1, 0]);
    assert_eq!(played.order_count().iter().sum::<usize>(), 5);
    for day in 1..=4 {
        assert!(played.decisions(day).notices.is_empty(), "day {day}");
    }
    for day in [3, 4] {
        assert_eq!(played.steamed_in(day), 0, "day {day}");
    }
    assert_eq!(played.force_at_end_of(2).readiness, Readiness::Battle);
    assert_eq!(played.force_at_end_of(3).readiness, Readiness::Normal);
    assert_eq!(played.force_at_end_of(4).home_port, Some(base(1)));
}

// --------------------------------------------------------------- Properties

#[test]
fn press_a_ship_is_short_below_200_so_the_force_sorties_every_night_and_takes_no_round_first() {
    let played = play(&with(&risk("press")));
    played.nothing_refused();
    assert_eq!(
        played.orders(1),
        vec![home_port(), engage("0.75"), BATTLE, sortie()]
    );
    for day in 2..=4 {
        assert_eq!(played.orders(day), vec![sortie()], "day {day}");
    }
    assert_eq!(played.halts().len(), 4);
    // She sails at the start of the night before any work is done and has
    // taken no round when she sails: nothing is rearmed in the night of day
    // 1.
    assert!(
        !played
            .rearmed()
            .iter()
            .any(|r| r.day == 1 && r.pulse == Some(Pulse::Night)),
        "{:?}",
        played.rearmed()
    );
    assert!(played.rearmed().iter().all(|r| r.ship == 1));
}

#[test]
fn preserve_the_force_a_contact_of_2_ships_short_below_600_so_day_1_in_port_and_the_sortie_on_day_2(
) {
    let played = play(&two_ships(&with(&risk("preserve"))));
    played.nothing_refused();
    assert_eq!(played.orders(1), vec![home_port(), engage("0.25")]);
    let none = played.none(1);
    assert!(
        none.iter().any(|r| r
            .contains("ship:1 has 300 rounds against the 600 of a 0.75 of her outfit of 800")
            && r.contains("may be used from day 2")),
        "{none:?}"
    );
    assert_eq!(played.orders(2), vec![BATTLE, sortie()]);
    assert_eq!(played.halts().len(), 1);
    assert_eq!(played.halts()[0].0, 2);
    assert_eq!(played.cannot_meet(1), 0);
}

#[test]
fn no_ammunition_ashore_no_day_the_notice_on_day_1_and_no_sortie_on_day_1_or_2() {
    let played = play(&changed(&text(), "ammunition_t = 100", "ammunition_t = 0"));
    played.nothing_refused();
    played.no_sortie();
    assert!(played.rearmed().is_empty());
    assert_eq!(played.orders(1), vec![home_port(), engage("0.5")]);
    let none = played.none(1);
    assert!(
        none.iter()
            .any(|r| r.contains("ship:1 has 300 rounds against the 400")
                && r.contains("there is no day from which it may be used")
                && r.contains("cannot meet intent:2")),
        "{none:?}"
    );
    assert_eq!(played.cannot_meet(1), 1);
    assert_eq!(played.truth[2].ships[&ship(1)].gun_rounds, n(300));
}

#[test]
fn torpedoes_5_fired_at_balanced_is_short_5_torpedoes_10_t_in_2_hours_and_the_sortie_on_day_2() {
    let played = play(&tubes(5));
    played.nothing_refused();
    assert_eq!(played.orders(1), vec![home_port(), engage("0.5")]);
    let none = played.none(1);
    assert!(
        none.iter().any(|r| r
            .contains("ship:1 has 5 of her 8 tubes fired, more than the 4 allowed")
            && r.contains("may be used from day 2")),
        "{none:?}"
    );
    let rearmed = played.rearmed();
    assert_eq!(rearmed.len(), 1, "{rearmed:?}");
    let one = &rearmed[0];
    assert_eq!((one.day, one.pulse, one.ship), (1, Some(Pulse::Night), 1));
    assert_eq!((one.rounds, one.torpedoes), (0, 5));
    assert_eq!(one.taken_t, n(10));
    assert_eq!(one.tubes_fired, 0);
    assert_eq!(one.base_ammunition_t, n(90));
    assert_eq!(one.taken_t / one.rate, n(2));
    assert_eq!(played.orders(2), vec![BATTLE, sortie()]);
    assert_eq!(played.halts().len(), 1);
    assert_eq!(played.halts()[0].0, 2);
}

#[test]
fn torpedoes_4_fired_is_not_short_and_the_force_sorties_on_day_1() {
    let played = play(&tubes(4));
    played.nothing_refused();
    assert_eq!(
        played.orders(1),
        vec![home_port(), engage("0.5"), BATTLE, sortie()]
    );
    assert_eq!(played.halts()[0].0, 1);
    assert!(!played
        .rearmed()
        .iter()
        .any(|r| r.day == 1 && r.pulse == Some(Pulse::Night)));
}

#[test]
fn torpedoes_preserve_the_force_one_fired_tube_makes_her_short() {
    let played = play(&two_ships(&format!("{}{}", tubes(1), risk("preserve"))));
    played.nothing_refused();
    assert_eq!(played.orders(1), vec![home_port(), engage("0.25")]);
    let none = played.none(1);
    assert!(
        none.iter().any(|r| r
            .contains("ship:1 has 1 of her 8 tubes fired, more than the 0 allowed")
            && r.contains("may be used from day 2")),
        "{none:?}"
    );
    assert_eq!(played.orders(2), vec![BATTLE, sortie()]);
}

#[test]
fn torpedoes_press_never_short_not_even_with_every_tube_fired() {
    let played = play(&format!("{}{}", tubes(8), risk("press")));
    played.nothing_refused();
    assert_eq!(
        played.orders(1),
        vec![home_port(), engage("0.75"), BATTLE, sortie()]
    );
    assert_eq!(played.halts().len(), 4);
    assert_eq!(played.halts()[0].0, 1);
}

#[test]
fn the_later_of_fuel_and_ammunition_70_t_each_against_72_and_300_rounds_gives_both_and_day_2() {
    let played = play(&text().replace("fuel_t = 2000", "fuel_t = 70"));
    played.nothing_refused();
    assert_eq!(played.orders(1), vec![home_port(), engage("0.5")]);
    let none = played.none(1);
    assert!(
        none.iter()
            .any(|r| r.contains("ship:1 has 70 t aboard against 72 t needed")
                && r.contains("ship:2 has 70 t aboard against 72 t needed")
                && r.contains("ship:1 has 300 rounds against the 400")
                && r.contains("it fuels and rearms at")
                && r.contains("may be used from day 2")),
        "{none:?}"
    );
    assert_eq!(played.cannot_meet(1), 0);
}

fn ship_3_at_a_tenth_of_her_rounds() -> String {
    let t = std::fs::read_to_string(path_56()).expect("scenario 56 reads");
    changed(
        &t,
        "name = \"Placeholder destroyer 3\"\nclass = 2\nside = \"allied\"\nfuel_t = 500\n",
        "name = \"Placeholder destroyer 3\"\nclass = 2\nside = \"allied\"\nfuel_t = 500\ngun_rounds = 60\n",
    )
}

#[test]
fn a_short_escort_is_not_free_the_first_run_sails_alone_and_the_reason_gives_her_rounds() {
    let played = play_at(
        &ship_3_at_a_tenth_of_her_rounds(),
        &path_56(),
        &common::frozen(),
    );
    played.nothing_refused();
    // Ship 3's outfit is 600 rounds (4 guns of 150): a half is 300.
    // Day 2: the run is routed and no order is given to force 1.
    let day_2: Vec<&StaffOrder> = played.decisions(2).orders.iter().collect();
    assert_eq!(day_2.len(), 2, "{day_2:?}");
    for o in &day_2 {
        if let OrderKind::TaskForce { target, order } = &o.kind {
            assert_ne!(*target, force(1), "an order to the escort: {order:?}");
        }
    }
    let worked: Vec<String> = day_2.iter().map(|o| said(&o.reason)).collect();
    assert!(
        worked
            .iter()
            .any(|r| r.contains("ship:3 has 60 rounds against the 300")
                && r.contains("task_force:1 is not free")
                && r.contains("so the run sails alone")),
        "{worked:?}"
    );
    for day in 1..=18 {
        for o in &played.decisions(day).orders {
            if let OrderKind::TaskForce { target, .. } = &o.kind {
                assert_ne!(*target, force(1), "day {day}");
            }
        }
    }
}

#[test]
fn the_marker_a_class_with_9999_rounds_a_gun_is_never_short_of_rounds() {
    let options = common::altered_reference("going_back_marker", |dir| {
        common::edit_file(dir, "ship_classes.csv", |text| {
            common::replace_once(
                &text,
                "1,Placeholder cruiser,cruiser,16,32,2000,8000,0,8,3,100,20,8,8,200,400,0,0,0",
                "1,Placeholder cruiser,cruiser,16,32,2000,8000,0,8,3,9999,20,8,8,200,400,0,0,0",
            )
        });
    });
    let played = play_at(&text(), &path(), &options);
    played.nothing_refused();
    assert_eq!(
        played.orders(1),
        vec![home_port(), engage("0.5"), BATTLE, sortie()]
    );
    assert_eq!(played.halts()[0].0, 1);
    assert!(played.decisions(1).no_orders.is_empty());
}

// ------------------------------------------------------------ Determinism

#[test]
fn the_same_start_gives_the_same_orders() {
    let a = play(&text());
    let b = play(&text());
    assert_eq!(a.handed, b.handed);
    assert_eq!(a.state, b.state);
    assert_eq!(a.events.len(), b.events.len());
}

#[test]
fn two_worlds_that_differ_only_in_what_the_allied_side_cannot_see_give_the_same_decisions() {
    let hidden = format!(
        "{}\n[[base]]\nid = 3\nname = \"Placeholder base 3\"\nside = \"japanese\"\nq = 10\nr = 5\nsupply_t = 50\non_map = false\n\n[[air_group]]\nid = 6\nname = \"Placeholder Japanese fighters 6\"\nside = \"japanese\"\nbase = 3\naircraft_type = 4\nready = 20\n",
        text()
    );
    let plain = play(&text());
    let other = play(&hidden);
    assert_ne!(
        plain.truth.last().map(|t| t.bases.len()),
        other.truth.last().map(|t| t.bases.len()),
        "the two worlds do not differ"
    );
    assert_eq!(plain.handed, other.handed);
    assert_eq!(plain.state, other.state);
}
