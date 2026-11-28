//! Scenario 35: days of supply. The figure a side reads for a hex it holds:
//! its stock there over its planned use for the day. With no use there is
//! no figure; a unit on its way counts from the day it is due; a surge of
//! sorties shows the day after it is ordered; repairs count; and a base
//! taken gives a figure to the side that took it and none to the side that
//! lost it.
//!
//! Proves decisions 31 to 36 of decision 0107 (step 3, stage 1) against a
//! hand calculation. The data is placeholder data, in three scenario files
//! so that no case has dice: `data/scenarios/35a_days_of_supply.toml` (men,
//! no aircraft), `35b_days_of_supply_sorties.toml` (aircraft that fly, with
//! the air combat and air supply figures and no wear or fatigue figures)
//! and `35c_days_of_supply_repairs.toml` (aircraft that do not fly, with
//! the wear and repair and the air supply figures). They run on the frozen
//! step 1 figures; 35a's test sets `weight_strength` to 0.25, as the test
//! of scenario 29 does.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly: no case has a random draw. If the code disagrees, the code or
//! the rule is wrong: do not change a number here to make a test pass. A
//! mismatch goes back to the architect.
//!
//! ## The rule
//!
//! The figure is for a side in a hex where it holds a base. It is read at
//! the moment a staff is asked: at the start of a day, after ships in port
//! have refuelled and before the day's orders are taken in. So it reads the
//! stock as the day before left it, and the missions and postures as the
//! day before left them.
//!
//! It is the supply stock of all the side's bases in the hex, over the
//! side's planned use there for that day:
//!
//! - the men's supply for its men ashore in the hex, each unit at its
//!   weight, and for the men aboard its own forces that are ordered to land
//!   there and are due that day or were due earlier and have not landed;
//! - 0.2 t for each of its aircraft there, ready or damaged;
//! - for each of its bases there, the supply of the repairs it would make
//!   that night: the smaller of its repairs a day and the damaged aircraft
//!   at it, at 1 t a repair;
//! - for each of its groups there on search or on patrol, the sorties of
//!   the day's two daylight pulses with the aircraft able to fly. A patrol
//!   sortie costs 0.5 t an aircraft. A strike mission counts nothing.
//!
//! With no planned use there is no figure. A figure is cut at six decimals.
//!
//! ## Figures
//!
//! `daily_supply_t_per_man` 0.005, `weight_strength` 0.25 (a unit of weight
//! 4 costs 1.5 times as much a man), `daily_supply_t_per_aircraft` 0.2,
//! `repair_supply_t` 1, `sortie_supply_t` 0.5. From the earlier rules:
//! `unfed_loss_share_per_day` 0.05, `captured_stock_share` 0.5. The men's
//! supply and the aircraft's upkeep are paid at the start of each night
//! pulse, which is the first of a day; repairs are made, and paid for,
//! before them.
//!
//! ## 35a: men
//!
//! A placeholder map of coast hexes far apart with no land link: A, B, C,
//! D, G and H. C is 14 hexes, 560 nm, from D on one axis. The game is 6
//! days. Every unit has weight 2 but as stated.
//!
//! | Base | Side     | Hex | Supply |
//! | ---- | -------- | --- | ------ |
//! | 1    | Allied   | A   | 500 t  |
//! | 2    | Allied   | B   | 600 t  |
//! | 3    | Allied   | C   | 300 t  |
//! | 4    | Allied   | D   | 5000 t |
//! | 7    | Japanese | G   | 200 t  |
//! | 8    | Allied   | H   | 200 t  |
//! | 9    | Japanese | H   | 50 t   |
//!
//! | Unit | Side     | Kind     | Men  | Where                     | Stated   | Posture |
//! | ---- | -------- | -------- | ---- | ------------------------- | -------- | ------- |
//! | 1    | Allied   | fighting | 2000 | B                         |          | defend  |
//! | 2    | Allied   | fighting | 1000 | C                         |          | defend  |
//! | 3    | Allied   | fighting | 3000 | aboard force 1, at base 4 | Weight 4 | defend  |
//! | 4    | Allied   | fighting | 100  | G                         |          | attack  |
//! | 6    | Allied   | working  | 100  | H                         |          | defend  |
//! | 7    | Japanese | fighting | 100  | H                         |          | attack  |
//!
//! Force 1 is five placeholder troop transports (600 men each, 10 kn at
//! cruise). On day 1 it is ordered to C at cruise with an unload order. 560
//! nm at 10 kn is 56 hours: it arrives at hour 56, in the night pulse of
//! day 3, so it is due on day 3. Each ship has 600 men to land, 150 t of
//! work at 20 t an hour: all are ashore 7.5 hours later, on day 3.
//!
//! ### At A: no use
//!
//! Base 1 holds 500 t and the Allied side has no man and no aircraft there.
//! There is no figure on any day.
//!
//! ### At B: men
//!
//! Unit 1 costs 2000 x 0.005 = 10 t a day.
//!
//! | Read on day | 1   | 2   | 3   | 4   | 5   | 6   |
//! | ----------- | --- | --- | --- | --- | --- | --- |
//! | Stock       | 600 | 590 | 580 | 570 | 560 | 550 |
//! | Figure      | 60  | 59  | 58  | 57  | 56  | 55  |
//!
//! ### At C: a unit arriving
//!
//! Unit 2 costs 5 t a day. Unit 3 costs 3000 x 0.005 x 1.5 = 22.5 t a day,
//! and counts from day 3, the day it is due. It is ashore on day 3 after
//! that night's feeding, so base 3 first pays for it on the night of day 4.
//!
//! | Read on day | Stock   | Planned use | Figure    |
//! | ----------- | ------- | ----------- | --------- |
//! | 1           | 300     | 5           | 60        |
//! | 2           | 295     | 5           | 59        |
//! | 3           | 290     | 27.5        | 10.545454 |
//! | 4           | 285     | 27.5        | 10.363636 |
//! | 5           | 257.5   | 27.5        | 9.363636  |
//! | 6           | 230     | 27.5        | 8.363636  |
//!
//! On day 1 the figure is 60 days, and the regiment that will cut it to ten
//! is already at sea. That is what the projection of decision 35 is for:
//! worked on day 1 against an intent of 30 days, the first day it falls
//! below is day 3. The projection is the staff's and is asserted in the
//! tests of stage 1, step 1; this file asserts the figure.
//!
//! ### At G: a base taken
//!
//! Day 1. The Allied side holds no base at G: it has no figure. The
//! Japanese side holds base 7 and has no man there: no figure. Unit 4 is
//! unfed on the night of day 1 and loses 100 x 0.05 = 5 men: 95. In the
//! ground combat phase the Japanese side, with a base and no man, is beaten
//! without a fight, and base 7 becomes Allied with 200 x 0.5 = 100 t.
//!
//! Unit 4 costs 95 x 0.005 = 0.475 t a day from then.
//!
//! | Read on day | Allied stock | Allied figure | Japanese figure |
//! | ----------- | ------------ | ------------- | --------------- |
//! | 1           | none         | none          | none            |
//! | 2           | 100          | 210.526315    | none            |
//! | 3           | 99.525       | 209.526315    | none            |
//! | 4           | 99.05        | 208.526315    | none            |
//!
//! ### At H: a base lost
//!
//! Day 1. Allied: 200 / (100 x 0.005) = 400. Japanese: 50 / 0.5 = 100.
//!
//! Night of day 1: base 8 pays 0.5 t and holds 199.5 t; base 9 pays 0.5 t
//! and holds 49.5 t. In the ground combat phase the Allied side, with only
//! working men, is beaten without a fight. Base 8 becomes Japanese with
//! 199.5 x 0.5 = 99.75 t. H has no land link and unit 6 surrenders.
//!
//! From day 2 the Japanese side holds two bases at H and the figure counts
//! both. Base 8 is the lower numbered and pays the 0.5 t a night.
//!
//! | Read on day | Allied figure | Japanese stock          | Japanese figure |
//! | ----------- | ------------- | ----------------------- | --------------- |
//! | 1           | 400           | 50                      | 100             |
//! | 2           | none          | 99.75 + 49.5 = 149.25   | 298.5           |
//! | 3           | none          | 148.75                  | 297.5           |
//!
//! ## 35b: a surge of sorties
//!
//! One coast hex, E. Base 5, Allied, 400 t, with an airfield. Air group 1:
//! 20 placeholder fighters ready, no mission. No enemy. The game is 6 days.
//! On day 3 the group is ordered to patrol.
//!
//! With no mission its use is its upkeep, 20 x 0.2 = 4 t a day. On patrol
//! it is 4 + 20 x 0.5 x 2 = 24 t a day. The order of day 3 is taken in
//! after the figure of day 3 is read, so the surge shows on day 4.
//!
//! | Read on day | Stock | Planned use | Figure    | That day the base pays       |
//! | ----------- | ----- | ----------- | --------- | ---------------------------- |
//! | 1           | 400   | 4           | 100       | 4                            |
//! | 2           | 396   | 4           | 99        | 4                            |
//! | 3           | 392   | 4           | 98        | 4, and 10 in each daylight pulse |
//! | 4           | 368   | 24          | 15.333333 | 24                           |
//! | 5           | 344   | 24          | 14.333333 | 24                           |
//! | 6           | 320   | 24          | 13.333333 | 24                           |
//!
//! ## 35c: repairs
//!
//! One coast hex, F. Base 6, Allied, 100 t, with an airfield and 3 repairs
//! a day. Air group 2: 10 ready and 5 damaged, no mission. The game is 4
//! days. Nothing flies, so nothing wears.
//!
//! | Read on day | Damaged | Stock | Upkeep | Repairs that night | Planned use | Figure    |
//! | ----------- | ------- | ----- | ------ | ------------------ | ----------- | --------- |
//! | 1           | 5       | 100   | 3      | 3                  | 6           | 16.666666 |
//! | 2           | 2       | 94    | 3      | 2                  | 5           | 18.8      |
//! | 3           | 0       | 89    | 3      | 0                  | 3           | 29.666666 |
//! | 4           | 0       | 86    | 3      | 0                  | 3           | 28.666666 |
//!
//! # Properties
//!
//! - The figure is the same in two worlds that differ only in what the
//!   side cannot see: the enemy's men, bases and stocks in other hexes, and
//!   in this one.
//! - A side is never given a figure for a hex where it holds no base, and
//!   never the enemy's figure.
//! - Setting a group to rest lowers the planned use, and the figure says in
//!   its parts what it counted, so that the reason can be read.
//! - Where the planned use is 0 the tools say that there is no figure, and
//!   never show a number.

// ---------------------------------------------------------------- The tests
//
// The figure is read where the staff would be asked: at the start of each
// day, before that day's orders are taken in. The tests resolve each day with
// `resolve_day_watched(Staffs::none(), ..)` and read at `Moment::Orders {
// day }` (after the refuelling in port, before the orders are taken in), with
// the day's scripted orders already handed to the world as the scenario's
// `next_day` hands them. Reading before each `next_day` gives the same.

use crate::common;

use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::common::values::{fx, n};
use sim_core::choices::Moment;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::state::Side;
use sim_core::{DaysOfSupply, Staffs};
use sim_data::Scenario;

// A to H as the placeholder map has them.
const A: Hex = Hex::new(0, 4);
const B: Hex = Hex::new(3, 4);
const C: Hex = Hex::new(0, 0);
const G: Hex = Hex::new(6, 4);
const H: Hex = Hex::new(9, 4);
const E: Hex = Hex::new(12, 4);
const F: Hex = Hex::new(15, 4);
const D: Hex = Hex::new(14, 0);

fn path(file: &str) -> PathBuf {
    common::data_dir().join("scenarios").join(file)
}

fn text(file: &str) -> String {
    std::fs::read_to_string(path(file)).expect("the scenario file reads")
}

/// 35a's figures: the frozen ones with the weight strength of the
/// calculation.
fn working() -> sim_data::LoadOptions {
    common::altered_reference("days_of_supply", |dir| {
        common::edit_file_by(
            dir,
            "ground.toml",
            &[("weight_strength = 0", "weight_strength = \"0.25\"")],
        );
    })
}

fn scenario_a(source: &str) -> Scenario {
    common::from_text_with(source, &path("35a_days_of_supply.toml"), &working()).expect("35a loads")
}

fn scenario_b(source: &str) -> Scenario {
    common::from_text(source, &path("35b_days_of_supply_sorties.toml")).expect("35b loads")
}

fn scenario_c(source: &str) -> Scenario {
    common::from_text(source, &path("35c_days_of_supply_repairs.toml")).expect("35c loads")
}

/// Every figure of both sides in each of `hexes`, read at the start of each
/// day before its orders, by day. Also the stock of each base at that moment,
/// by the truth, to check what the figure says it holds.
struct Readings {
    figures: BTreeMap<(u32, Side, Hex), Option<DaysOfSupply>>,
    truth_stock: BTreeMap<(u32, Side, Hex), Fixed>,
}

impl Readings {
    fn of(mut scenario: Scenario, hexes: &[Hex]) -> Readings {
        let script = scenario.scripted_orders().clone();
        let mut figures = BTreeMap::new();
        let mut truth_stock = BTreeMap::new();
        for day in 1..=scenario.days {
            for (side, request) in script.get(&day).cloned().unwrap_or_default() {
                scenario.world.submit_orders(side, vec![request]);
            }
            scenario
                .world
                .resolve_day_watched(Staffs::none(), &mut |world, moment| {
                    let Moment::Orders { day: now } = moment else {
                        return;
                    };
                    assert_eq!(now, day);
                    for side in [Side::Allied, Side::Japanese] {
                        let view = world.side_view(side);
                        for hex in hexes {
                            figures.insert((day, side, *hex), view.days_of_supply(*hex));
                        }
                        for base in view.bases() {
                            *truth_stock
                                .entry((day, side, base.hex))
                                .or_insert(Fixed::ZERO) += base.supply_t;
                        }
                    }
                })
                .expect("a day resolves");
        }
        Readings {
            figures,
            truth_stock,
        }
    }

    fn at(&self, day: u32, side: Side, hex: Hex) -> Option<DaysOfSupply> {
        self.figures[&(day, side, hex)]
    }

    /// The figure of a side that holds a base in the hex, with its stock
    /// checked against the bases' own.
    fn some(&self, day: u32, side: Side, hex: Hex) -> DaysOfSupply {
        let figure = self
            .at(day, side, hex)
            .unwrap_or_else(|| panic!("a figure for {side:?} at {hex:?} on day {day}"));
        assert_eq!(
            figure.stock_t,
            self.truth_stock[&(day, side, hex)],
            "the stock is the bases' own, day {day}"
        );
        figure
    }
}

const ALL_HEXES: [Hex; 9] = [A, B, C, D, G, H, E, F, Hex::new(1, 1)];

fn readings_a() -> Readings {
    Readings::of(scenario_a(&text("35a_days_of_supply.toml")), &ALL_HEXES)
}

fn readings_b() -> Readings {
    Readings::of(
        scenario_b(&text("35b_days_of_supply_sorties.toml")),
        &ALL_HEXES,
    )
}

fn readings_c() -> Readings {
    Readings::of(
        scenario_c(&text("35c_days_of_supply_repairs.toml")),
        &ALL_HEXES,
    )
}

/// A check of one reading: the stock, the planned use in its parts, the
/// figure.
#[allow(clippy::too_many_arguments)]
fn check(
    got: DaysOfSupply,
    day: u32,
    stock: &str,
    ashore: &str,
    due: &str,
    upkeep: &str,
    repairs: &str,
    sorties: &str,
    figure: Option<&str>,
) {
    assert_eq!(got.stock_t, fx(stock), "stock, day {day}");
    let used = got.planned_use;
    assert_eq!(used.men_ashore_t, fx(ashore), "men ashore, day {day}");
    assert_eq!(used.men_due_t, fx(due), "men due, day {day}");
    assert_eq!(used.aircraft_upkeep_t, fx(upkeep), "upkeep, day {day}");
    assert_eq!(used.repairs_t, fx(repairs), "repairs, day {day}");
    assert_eq!(used.sorties_t, fx(sorties), "sorties, day {day}");
    assert_eq!(got.days, figure.map(fx), "figure, day {day}");
}

// -------------------------------------------------------------- 35a: men

#[test]
fn at_a_there_is_no_use_and_so_no_figure_on_any_day() {
    let read = readings_a();
    for day in 1..=6 {
        let got = read.some(day, Side::Allied, A);
        check(got, day, "500", "0", "0", "0", "0", "0", None);
    }
}

#[test]
fn at_b_the_figure_falls_a_day_by_ten_tons() {
    let read = readings_a();
    for day in 1..=6u32 {
        let stock = 600 - 10 * i64::from(day - 1);
        let got = read.some(day, Side::Allied, B);
        check(
            got,
            day,
            &stock.to_string(),
            "10",
            "0",
            "0",
            "0",
            "0",
            Some(&(60 - (day - 1)).to_string()),
        );
    }
}

#[test]
fn at_c_a_regiment_at_sea_counts_from_the_day_it_is_due() {
    let read = readings_a();
    // Day, stock, men ashore, men due, figure.
    let table = [
        (1, "300", "5", "0", "60"),
        (2, "295", "5", "0", "59"),
        (3, "290", "5", "22.5", "10.545454"),
        (4, "285", "27.5", "0", "10.363636"),
        (5, "257.5", "27.5", "0", "9.363636"),
        (6, "230", "27.5", "0", "8.363636"),
    ];
    for (day, stock, ashore, due, figure) in table {
        let got = read.some(day, Side::Allied, C);
        check(got, day, stock, ashore, due, "0", "0", "0", Some(figure));
    }
}

#[test]
fn the_staffs_projection_at_c_worked_on_day_1_first_falls_below_the_floor_on_day_3() {
    // The force's route is ordered on day 1, so the view at the staff's own
    // moment of day 1 does not yet see it. "Worked on day 1" is the view
    // after the day's orders are taken in: the start of the night pulse. The
    // projection counts the regiment from the day it is due, day 3, as the
    // table above does: 295 t on day 2 over 5 t is 59 days; on day 3 the
    // stock is 290 t over 5 + 22.5 t, below the floor of 30 days.
    let mut scenario = scenario_a(&text("35a_days_of_supply.toml"));
    let mut projection = None;
    scenario
        .next_day_watched(Staffs::none(), &mut |world, moment| {
            if moment
                == (Moment::PulseBegins {
                    day: 1,
                    pulse: sim_core::time::Pulse::Night,
                })
            {
                projection = sim_staff::project(&world.side_view(Side::Allied), C, 30, 6);
            }
        })
        .expect("day 1 resolves");
    let projection = projection.expect("the side holds a base at C");
    assert_eq!(projection.hex, C);
    assert_eq!(projection.floor_days, 30);
    assert_eq!(projection.readings.len(), 7);
    let first = &projection.readings[0];
    assert_eq!(
        (first.day, first.stock_t, first.days),
        (1, n(300), Some(n(60)))
    );
    let second = &projection.readings[1];
    assert_eq!(
        (second.day, second.stock_t, second.days),
        (2, n(295), Some(n(59)))
    );
    let third = &projection.readings[2];
    assert_eq!(third.day, 3);
    assert_eq!(third.stock_t, n(290));
    assert!(third.days.expect("a figure") < n(30));
    assert_eq!(projection.first_day_below, Some(3));
}

#[test]
fn the_men_due_by_day_lists_a_force_on_its_way_with_its_due_day_and_agrees_with_the_men_due() {
    let mut scenario = scenario_a(&text("35a_days_of_supply.toml"));
    let script = scenario.scripted_orders().clone();
    let mut seen: BTreeMap<u32, Vec<(u32, Fixed)>> = BTreeMap::new();
    for day in 1..=scenario.days {
        for (side, request) in script.get(&day).cloned().unwrap_or_default() {
            scenario.world.submit_orders(side, vec![request]);
        }
        scenario
            .world
            .resolve_day_watched(Staffs::none(), &mut |world, moment| {
                let Moment::Orders { day: now } = moment else {
                    return;
                };
                let view = world.side_view(Side::Allied);
                let due = view.men_due_by_day(C);
                let counted = due
                    .iter()
                    .filter(|(due_day, _)| *due_day <= now)
                    .fold(Fixed::ZERO, |sum, (_, charge)| sum + *charge);
                let figure = view.days_of_supply(C).expect("a base at C");
                assert_eq!(counted, figure.planned_use.men_due_t, "day {now}");
                seen.insert(now, due);
            })
            .expect("a day resolves");
    }
    // Day 1: the orders of the day are not yet taken in, so nothing is on
    // its way. From day 2 the regiment is on its way and due on day 3, at
    // 22.5 t a day.
    assert!(seen[&1].is_empty());
    assert_eq!(seen[&2], vec![(3, fx("22.5"))]);
    assert_eq!(seen[&3], vec![(3, fx("22.5"))]);
    // Once it is ashore nothing is due.
    assert!(seen[&4].is_empty());
}

#[test]
fn at_g_a_base_taken_gives_a_figure_to_the_side_that_took_it() {
    let read = readings_a();
    // Day 1: the Allied side holds no base there; the Japanese side holds
    // base 7 and has no man there.
    assert_eq!(read.at(1, Side::Allied, G), None);
    let japanese = read.some(1, Side::Japanese, G);
    check(japanese, 1, "200", "0", "0", "0", "0", "0", None);
    // From day 2 the Allied side holds it, and the Japanese side does not.
    let table = [
        (2, "100", "210.526315"),
        (3, "99.525", "209.526315"),
        (4, "99.05", "208.526315"),
    ];
    for (day, stock, figure) in table {
        let got = read.some(day, Side::Allied, G);
        check(got, day, stock, "0.475", "0", "0", "0", "0", Some(figure));
        assert_eq!(read.at(day, Side::Japanese, G), None, "day {day}");
    }
}

#[test]
fn at_h_a_base_lost_gives_no_figure_and_the_other_side_counts_both_bases() {
    let read = readings_a();
    let allied = read.some(1, Side::Allied, H);
    check(allied, 1, "200", "0.5", "0", "0", "0", "0", Some("400"));
    let japanese = read.some(1, Side::Japanese, H);
    check(japanese, 1, "50", "0.5", "0", "0", "0", "0", Some("100"));
    let table = [(2, "149.25", "298.5"), (3, "148.75", "297.5")];
    for (day, stock, figure) in table {
        assert_eq!(read.at(day, Side::Allied, H), None, "day {day}");
        let got = read.some(day, Side::Japanese, H);
        check(got, day, stock, "0.5", "0", "0", "0", "0", Some(figure));
    }
}

// ------------------------------------------------ 35b: a surge of sorties

#[test]
fn a_surge_of_sorties_shows_the_day_after_the_order() {
    let read = readings_b();
    // Day, stock, upkeep, sorties, figure.
    let table = [
        (1, "400", "4", "0", "100"),
        (2, "396", "4", "0", "99"),
        (3, "392", "4", "0", "98"),
        (4, "368", "4", "20", "15.333333"),
        (5, "344", "4", "20", "14.333333"),
        (6, "320", "4", "20", "13.333333"),
    ];
    for (day, stock, upkeep, sorties, figure) in table {
        let got = read.some(day, Side::Allied, E);
        check(
            got,
            day,
            stock,
            "0",
            "0",
            upkeep,
            "0",
            sorties,
            Some(figure),
        );
    }
}

// ------------------------------------------------------- 35c: repairs

#[test]
fn repairs_count_the_smaller_of_the_base_figure_and_the_damaged_aircraft() {
    let read = readings_c();
    // Day, stock, repairs that night, figure (upkeep is 3 on every day).
    let table = [
        (1, "100", "3", "16.666666"),
        (2, "94", "2", "18.8"),
        (3, "89", "0", "29.666666"),
        (4, "86", "0", "28.666666"),
    ];
    for (day, stock, repairs, figure) in table {
        let got = read.some(day, Side::Allied, F);
        check(got, day, stock, "0", "0", "3", repairs, "0", Some(figure));
    }
}

// ------------------------------------------------------------ Properties

#[test]
fn two_worlds_that_differ_only_in_what_the_side_cannot_see_give_one_figure() {
    // The Japanese side's stocks and its man at H differ (an enemy man, an
    // enemy base's stock), and so does an enemy base with its man in a hex
    // where the Allied side holds a base and no man. None of it changes an
    // Allied figure.
    let plain = text("35a_days_of_supply.toml");
    let mut changed = plain.clone();
    changed = changed.replace("supply_t = 200\n", "supply_t = 9000\n");
    changed = changed.replace("supply_t = 50\n", "supply_t = 77\n");
    assert_ne!(plain, changed);
    // Unit 7's men: 100 to 90.
    let unit7 = "name = \"Placeholder unit 7\"\nside = \"japanese\"\nmen = 100";
    assert!(changed.contains(unit7));
    changed = changed.replace(
        unit7,
        "name = \"Placeholder unit 7\"\nside = \"japanese\"\nmen = 90",
    );
    // The Japanese side also holds a base with 5000 t and 500 men, on defence,
    // at A, where the Allied side holds base 1 and has no man.
    changed.push_str(
        "\n[[base]]\nid = 10\nname = \"Placeholder base 10\"\nside = \"japanese\"\nq = 0\nr = 4\nsupply_t = 5000\n\n[[land_unit]]\nid = 8\nname = \"Placeholder unit 8\"\nside = \"japanese\"\nmen = 500\nq = 0\nr = 4\nposture = \"defend\"\n",
    );
    let one = Readings::of(scenario_a(&plain), &ALL_HEXES);
    let two = Readings::of(scenario_a(&changed), &ALL_HEXES);
    // The hexes whose Allied figure no enemy fight or capture reaches.
    for day in 1..=6 {
        for hex in [B, C, D] {
            assert_eq!(
                one.at(day, Side::Allied, hex),
                two.at(day, Side::Allied, hex),
                "{hex:?} on day {day}"
            );
        }
    }
    // The Japanese side's own figure does see its own stock and its own men.
    assert_ne!(one.at(1, Side::Japanese, G), two.at(1, Side::Japanese, G));
}

#[test]
fn a_side_never_has_a_figure_for_a_hex_without_a_base_of_its_own() {
    let a = readings_a();
    // The Allied side holds no base at G on day 1, nor the Japanese at A, B,
    // C or D on any day; neither has one for a hex with no base at all.
    assert_eq!(a.at(1, Side::Allied, G), None);
    for day in 1..=6 {
        for hex in [A, B, C, D] {
            assert_eq!(a.at(day, Side::Japanese, hex), None, "{hex:?}, day {day}");
        }
        for hex in [E, F, Hex::new(1, 1)] {
            assert_eq!(a.at(day, Side::Allied, hex), None);
            assert_eq!(a.at(day, Side::Japanese, hex), None);
        }
    }
    // And never the enemy's: at H on day 1 each side gets its own.
    assert_eq!(a.some(1, Side::Allied, H).days, Some(n(400)));
    assert_eq!(a.some(1, Side::Japanese, H).days, Some(n(100)));
    // No figure at all for the other side in 35b and 35c.
    for day in 1..=6 {
        assert_eq!(readings_b().at(day, Side::Japanese, E), None);
    }
}

#[test]
fn resting_a_group_lowers_the_planned_use_and_the_figure_says_what_it_counted() {
    let patrol = text("35b_days_of_supply_sorties.toml");
    let rest = patrol.replace("mission = \"combat_air_patrol\"", "mission = \"rest\"");
    assert_ne!(patrol, rest);
    let on = Readings::of(scenario_b(&patrol), &ALL_HEXES);
    let off = Readings::of(scenario_b(&rest), &ALL_HEXES);
    for day in 4..=6 {
        let flying = on.some(day, Side::Allied, E).planned_use;
        let resting = off.some(day, Side::Allied, E).planned_use;
        assert!(resting.total_t() < flying.total_t(), "day {day}");
        assert_eq!(resting.sorties_t, Fixed::ZERO);
        assert_eq!(flying.sorties_t, n(20));
        // Only the sorties differ: the rest of what it counted is the same.
        assert_eq!(resting.aircraft_upkeep_t, flying.aircraft_upkeep_t);
    }
}

#[test]
fn where_the_planned_use_is_nothing_there_is_no_figure() {
    // 35a at A: a base and no use.
    let a = readings_a();
    assert_eq!(a.some(1, Side::Allied, A).days, None);
    // 35b with no air group at all: the base holds 400 t and uses nothing.
    let source = text("35b_days_of_supply_sorties.toml");
    let empty = source
        .split("[[air_group]]")
        .next()
        .expect("text before the group")
        .to_string();
    let read = Readings::of(scenario_b(&empty), &ALL_HEXES);
    for day in 1..=6 {
        let got = read.some(day, Side::Allied, E);
        check(got, day, "400", "0", "0", "0", "0", "0", None);
    }
}
