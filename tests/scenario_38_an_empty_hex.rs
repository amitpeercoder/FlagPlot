//! Scenario 38: a side's men report an empty hex. Where a side holds an
//! estimate of the enemy ashore in a hex, has fighting men there, and the
//! enemy has no man there, it is told that its men find none and the
//! estimate is cleared. It is cleared on the day the last enemy man is gone
//! and not before; a side with no fighting men in the hex keeps its old
//! estimate, shown with its day.
//!
//! Proves decision 0110, items 10 and 11, against a hand calculation. The
//! data is placeholder data: see `data/scenarios/38_an_empty_hex.toml`. It
//! runs on the frozen step 1 figures.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. An estimate's size is drawn:
//! the calculation gives the men it is drawn on and the range it must fall
//! in, and the tests read it from the run. Everything else is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Figures
//!
//! `report_error_low` 0.5, `report_error_high` 1.5: a ground report's
//! estimate is from half to one and a half times the enemy's men ashore in
//! the hex, working men included. From the earlier rules:
//! `daily_supply_t_per_man` 0.005, `unfed_loss_share_per_day` 0.05,
//! `beaten_fatigue` 30, `march_days_per_hex` 3, `captured_stock_share` 0.5.
//!
//! In the ground combat phase the marches come first; then, hex by hex, the
//! fight, fatigue, a side beaten, its bases changing side and its men
//! falling back or surrendering; and last the ground reports of the hex. So
//! a report sees the hex as the day leaves it.
//!
//! ## The rule
//!
//! A ground report is made, as before, for a side with men ashore in a hex
//! where the enemy has men ashore. Besides that: where a side holds an
//! estimate for a hex, has men of a fighting unit ashore there, and the
//! enemy has no man ashore there, it is given a report that its men find
//! none, and the estimate is cleared. With no estimate held, no report is
//! made. Every estimate is shown with the day it was made; an opening
//! estimate is of day 0.
//!
//! ## Inputs
//!
//! A placeholder map. Every hex is a coast hex and clear. The game is 6
//! days.
//!
//! - P at (0,0), with a land link to R at (1,0).
//! - Q at (10,0), S at (20,0) and V at (30,0), with no land link.
//!
//! | Base | Side     | Hex | Supply |
//! | ---- | -------- | --- | ------ |
//! | 1    | Japanese | P   | 1000 t |
//! | 2    | Japanese | R   | 100 t  |
//! | 3    | Allied   | P   | 1000 t |
//! | 4    | Allied   | Q   | 1000 t |
//! | 5    | Allied   | S   | 1000 t |
//! | 6    | Japanese | V   | 1000 t |
//! | 7    | Allied   | V   | 1000 t |
//!
//! | Unit | Side     | Kind     | Men | Hex | Posture |
//! | ---- | -------- | -------- | --- | --- | ------- |
//! | 1    | Japanese | working  | 300 | P   | defend  |
//! | 2    | Allied   | fighting | 500 | P   | attack  |
//! | 3    | Allied   | fighting | 100 | Q   | defend  |
//! | 4    | Allied   | working  | 100 | S   | defend  |
//! | 5    | Japanese | working  | 200 | V   | defend  |
//! | 6    | Allied   | fighting | 100 | V   | attack  |
//!
//! Opening estimates held by the Allied side: 400 Japanese men at Q, 400
//! at S and 300 at V. It holds none at P. The Japanese side holds none
//! anywhere. There is no order.
//!
//! ## At P: cleared on the day the last man is gone
//!
//! Day 1. The Japanese side has only working men at P: it is beaten without
//! a fight, base 1 becomes Allied, and unit 1 is set to fall back to R,
//! where base 2 is. It is still at P when the reports are made.
//!
//! | Day | Unit 1's men at P when the reports are made | The Allied report at P | The Allied estimate at P |
//! | --- | ------------------------------------------- | ---------------------- | ------------------------ |
//! | 1   | 300                                         | men: from 150 to 450   | that figure, of day 1    |
//! | 2   | 285, after the night's unfed loss of 15     | from 142.5 to 427.5    | that figure, of day 2    |
//! | 3   | 271, after a loss of 14                     | from 135.5 to 406.5    | that figure, of day 3    |
//! | 4   | none: it marched its third day and is at R, with 258 men | its men find none | none |
//! | 5   | none                                        | no report              | none                     |
//!
//! The estimate is not cleared on days 1 to 3, though the garrison is
//! beaten and leaving: its men are still in the hex. On day 4 one report
//! says the hex is empty. On days 5 and 6 the Allied side holds no estimate
//! for P, so no report is made.
//!
//! The Japanese side: on days 1 to 3 its men at P report the Allied men
//! there, 500, as a figure from 250 to 750, replaced each day. From day 4
//! it has no man at P and gets no report: it still holds its estimate of
//! day 3, shown as of day 3, to the end.
//!
//! The Allied side has no man at R and is told nothing of it.
//!
//! ## At Q: an old estimate of men who were never there
//!
//! The Allied side holds an opening estimate of 400 at Q, of day 0, and has
//! fighting men there. No Japanese man is at Q. On day 1 its men report
//! that they find none, and the estimate is cleared. No report is made
//! after.
//!
//! ## At S: working men do not report
//!
//! The Allied side holds an opening estimate of 400 at S and has only a
//! working unit there. No report is made on any day. At the end it still
//! holds 400, shown as of day 0.
//!
//! ## At V: a surrender
//!
//! Day 1. The Japanese side has only working men at V: it is beaten without
//! a fight and base 6 becomes Allied. V has no land link and unit 5
//! surrenders, its 200 men lost, before the reports are made. The Allied
//! side holds an opening estimate of 300 and has fighting men there: its
//! men report that they find none, and the estimate is cleared on day 1.
//!
//! # Properties
//!
//! - A report that a hex is empty is made only where it is true: on no day
//!   is a side told a hex is empty while an enemy man is ashore in it.
//! - A side is never told that a hex is empty where it has no fighting man.
//! - The enemy is told nothing of the report.
//! - With no estimate held anywhere, a scenario gives what it gave before,
//!   apart from the hash version. A scenario whose events change is listed
//!   for the architect before its baseline is remade.

use crate::common;

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::events::{Cause, Event, EventKind};
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, LandUnitId};
use sim_core::side_events::SideEventKind;
use sim_core::state::{Side, TruthState};
use sim_data::Scenario;

// ------------------------------------------------------------------ Helpers

const P: Hex = Hex::new(0, 0);
const R: Hex = Hex::new(1, 0);
const Q: Hex = Hex::new(10, 0);
const S: Hex = Hex::new(20, 0);
const V: Hex = Hex::new(30, 0);

fn path() -> PathBuf {
    common::data_dir().join("scenarios/38_an_empty_hex.toml")
}

/// What a side holds for each hex, as (men, day), at the end of a day.
type Held = BTreeMap<Hex, (u32, u32)>;

/// A run: the truth and what each side holds at the end of each day (day 0
/// is the start), and the log.
struct Run {
    scenario: Scenario,
    days: Vec<TruthState>,
    held: Vec<[Held; 2]>,
    events: Vec<Event>,
}

fn held_of(scenario: &Scenario, side: Side) -> Held {
    scenario
        .world
        .side_view(side)
        .ground_estimates()
        .map(|e| (e.hex, (e.men, e.day)))
        .collect()
}

fn index(side: Side) -> usize {
    match side {
        Side::Allied => 0,
        Side::Japanese => 1,
    }
}

impl Run {
    fn live() -> Run {
        let text = std::fs::read_to_string(path()).expect("the scenario file reads");
        let mut scenario =
            common::from_text(&text, &path()).expect("the scenario loads on the frozen figures");
        let mut days = vec![scenario.world.umpire().clone()];
        let mut held = vec![[
            held_of(&scenario, Side::Allied),
            held_of(&scenario, Side::Japanese),
        ]];
        while scenario.next_day().expect("a day resolves").is_some() {
            days.push(scenario.world.umpire().clone());
            held.push([
                held_of(&scenario, Side::Allied),
                held_of(&scenario, Side::Japanese),
            ]);
        }
        let events = scenario.world.events().iter().cloned().collect();
        Run {
            scenario,
            days,
            held,
            events,
        }
    }

    fn held(&self, day: u32, side: Side, hex: Hex) -> Option<(u32, u32)> {
        self.held[day as usize][index(side)].get(&hex).copied()
    }

    fn men(&self, day: u32, unit: u32) -> (u32, Option<Hex>) {
        let unit = &self.days[day as usize].land_units[&LandUnitId::new(unit)];
        (unit.men_ashore, unit.hex)
    }

    /// The ground reports of a side at a hex on a day: (men, replaced).
    fn reports(&self, day: u32, side: Side, hex: Hex) -> Vec<(u32, bool)> {
        self.events
            .iter()
            .filter(|e| e.day == day)
            .filter_map(|e| match e.kind {
                EventKind::GroundReportReceived {
                    side: s,
                    hex: h,
                    men,
                    replaced,
                    ..
                } if s == side && h == hex => Some((men, replaced)),
                _ => None,
            })
            .collect()
    }

    /// The reports that a hex is empty, of a side at a hex: (day, men
    /// cleared, day of that estimate).
    fn empties(&self, side: Side, hex: Hex) -> Vec<(u32, u32, u32)> {
        self.events
            .iter()
            .filter_map(|e| match e.kind {
                EventKind::GroundFoundEmpty {
                    side: s,
                    hex: h,
                    men,
                    reported_day,
                } if s == side && h == hex => Some((e.day, men, reported_day)),
                _ => None,
            })
            .collect()
    }
}

fn in_range(value: u32, low: u32, high: u32, what: &str) {
    assert!(
        (low..=high).contains(&value),
        "{what}: {value} is not within {low}..={high}"
    );
}

// ------------------------------------------------------------ At P

#[test]
fn p_the_allied_estimate_is_replaced_each_day_then_cleared_the_day_the_last_man_is_gone() {
    let run = Run::live();
    // Unit 1's men when the reports are made (end of day truth, the march
    // being before the reports): 300, 285, 271, then 258 at R.
    assert_eq!(run.men(1, 1), (300, Some(P)));
    assert_eq!(run.men(2, 1), (285, Some(P)));
    assert_eq!(run.men(3, 1), (271, Some(P)));
    assert_eq!(run.men(4, 1), (258, Some(R)));
    // Days 1 to 3: one Allied report of the day, within the stated range,
    // and the estimate is that figure, of that day.
    for (day, low, high) in [(1, 150, 450), (2, 142, 427), (3, 135, 406)] {
        let reports = run.reports(day, Side::Allied, P);
        assert_eq!(reports.len(), 1, "day {day}");
        in_range(
            reports[0].0,
            low,
            high,
            &format!("Allied report, day {day}"),
        );
        assert_eq!(reports[0].1, day > 1, "replaced from day 2");
        assert_eq!(run.held(day, Side::Allied, P), Some((reports[0].0, day)));
        assert!(run.empties(Side::Allied, P).iter().all(|e| e.0 >= 4));
    }
    // Day 4: one report that the hex is empty, citing the estimate of day 3,
    // and the estimate is gone. Days 5 and 6: none, and none held.
    let day3 = run.reports(3, Side::Allied, P)[0].0;
    assert_eq!(run.empties(Side::Allied, P), [(4, day3, 3)]);
    for day in 4..=6 {
        assert_eq!(run.held(day, Side::Allied, P), None, "day {day}");
    }
    for day in [4, 5, 6] {
        assert!(run.reports(day, Side::Allied, P).is_empty(), "day {day}");
    }
    // The event cites the report that made the estimate it clears.
    let found = run
        .events
        .iter()
        .find(|e| matches!(e.kind, EventKind::GroundFoundEmpty { .. }) && e.day == 4)
        .expect("the report that P is empty");
    let cited = run
        .events
        .iter()
        .find(|e| {
            e.day == 3
                && matches!(
                    e.kind,
                    EventKind::GroundReportReceived {
                        side: Side::Allied,
                        hex: P,
                        ..
                    }
                )
        })
        .expect("the report of day 3");
    assert!(found.causes.contains(&Cause::Event(cited.id)));
    assert!(found.breakdown.is_none());
}

#[test]
fn p_the_japanese_side_is_replaced_for_three_days_and_keeps_its_estimate_of_day_3() {
    let run = Run::live();
    for day in 1..=3 {
        let reports = run.reports(day, Side::Japanese, P);
        assert_eq!(reports.len(), 1, "day {day}");
        in_range(
            reports[0].0,
            250,
            750,
            &format!("Japanese report, day {day}"),
        );
        assert_eq!(run.held(day, Side::Japanese, P), Some((reports[0].0, day)));
    }
    let day3 = run.reports(3, Side::Japanese, P)[0].0;
    for day in 4..=6 {
        assert!(run.reports(day, Side::Japanese, P).is_empty(), "day {day}");
        assert_eq!(
            run.held(day, Side::Japanese, P),
            Some((day3, 3)),
            "day {day}"
        );
    }
    // It has no man at P from day 4, so it is told nothing of the hex.
    assert!(run.empties(Side::Japanese, P).is_empty());
    assert_eq!(run.held(6, Side::Japanese, P).map(|h| h.1), Some(3));
}

#[test]
fn r_the_allied_side_is_told_nothing_of_it() {
    let run = Run::live();
    assert!(run.reports(4, Side::Allied, R).is_empty());
    for day in 0..=6 {
        assert_eq!(run.held(day, Side::Allied, R), None);
    }
    for side in [Side::Allied, Side::Japanese] {
        assert!(run.empties(side, R).is_empty());
        for day in 1..=6 {
            assert!(run.reports(day, side, R).is_empty());
        }
    }
}

// ------------------------------------------------------------ At Q, S, V

#[test]
fn q_an_old_estimate_of_men_never_there_is_cleared_on_day_1_and_no_report_follows() {
    let run = Run::live();
    assert_eq!(run.held(0, Side::Allied, Q), Some((400, 0)));
    assert_eq!(run.empties(Side::Allied, Q), [(1, 400, 0)]);
    for day in 1..=6 {
        assert_eq!(run.held(day, Side::Allied, Q), None, "day {day}");
        assert!(run.reports(day, Side::Allied, Q).is_empty());
    }
}

#[test]
fn s_working_men_do_not_report() {
    let run = Run::live();
    for day in 0..=6 {
        assert_eq!(run.held(day, Side::Allied, S), Some((400, 0)), "day {day}");
    }
    assert!(run.empties(Side::Allied, S).is_empty());
    for day in 1..=6 {
        assert!(run.reports(day, Side::Allied, S).is_empty());
    }
}

#[test]
fn v_the_garrison_surrenders_and_the_allied_men_find_none() {
    let run = Run::live();
    let truth = &run.days[1];
    assert_eq!(truth.bases[&BaseId::new(6)].side, Side::Allied);
    let five = &truth.land_units[&LandUnitId::new(5)];
    assert_eq!(five.men_ashore, 0);
    assert!(run
        .events
        .iter()
        .any(|e| e.day == 1 && matches!(e.kind, EventKind::MenSurrenderedAshore { .. })));
    assert!(run.events.iter().any(|e| matches!(
        &e.kind,
        EventKind::UnitSurrendered { unit, men: 200, .. } if unit.raw() == 5
    )));
    assert_eq!(run.held(0, Side::Allied, V), Some((300, 0)));
    assert_eq!(run.empties(Side::Allied, V), [(1, 300, 0)]);
    for day in 1..=6 {
        assert_eq!(run.held(day, Side::Allied, V), None, "day {day}");
    }
}

// -------------------------------------------------------------- Properties

#[test]
fn a_hex_is_reported_empty_only_where_true_and_only_to_a_side_with_fighting_men() {
    let run = Run::live();
    let mut seen = 0;
    for event in &run.events {
        let EventKind::GroundFoundEmpty { side, hex, .. } = event.kind else {
            continue;
        };
        seen += 1;
        // The truth at the end of the day is the truth when the reports are
        // made: the marches come before them and nothing moves after.
        let truth = &run.days[event.day as usize];
        let ashore = |wanted: Side| {
            truth
                .land_units
                .values()
                .filter(move |u| u.side == wanted && u.hex == Some(hex) && u.men_ashore > 0)
        };
        assert_eq!(
            ashore(match side {
                Side::Allied => Side::Japanese,
                Side::Japanese => Side::Allied,
            })
            .count(),
            0,
            "{event:?}"
        );
        assert!(ashore(side).any(|u| !u.working), "{event:?}");
    }
    assert_eq!(seen, 3);
}

#[test]
fn no_report_that_a_hex_is_empty_is_made_while_an_enemy_man_is_ashore() {
    let run = Run::live();
    for day in 1..=6u32 {
        let truth = &run.days[day as usize];
        for side in [Side::Allied, Side::Japanese] {
            for hex in [P, R, Q, S, V] {
                let enemy_ashore = truth
                    .land_units
                    .values()
                    .any(|u| u.side != side && u.hex == Some(hex) && u.men_ashore > 0);
                if enemy_ashore {
                    assert!(
                        !run.empties(side, hex).iter().any(|e| e.0 == day),
                        "day {day} {side:?} {hex}"
                    );
                }
            }
        }
    }
}

#[test]
fn the_enemy_is_told_nothing_of_the_report() {
    let run = Run::live();
    let is_empty = |e: &sim_core::side_events::SideEvent| {
        matches!(
            &e.kind,
            SideEventKind::Whole(EventKind::GroundFoundEmpty { .. })
        )
    };
    let allied = run.scenario.world.side_view(Side::Allied).events();
    let japanese = run.scenario.world.side_view(Side::Japanese).events();
    assert_eq!(allied.iter().filter(|e| is_empty(e)).count(), 3);
    assert_eq!(japanese.iter().filter(|e| is_empty(e)).count(), 0);
    for e in &allied {
        if let SideEventKind::Whole(EventKind::GroundFoundEmpty { side, .. }) = &e.kind {
            assert_eq!(*side, Side::Allied);
        }
    }
}
