//! Scenario 138: one ship. The first rung of the ladder (decision 0237): a
//! game for play with one decision, the floor and the interval of one supply
//! order, and two ways to get it wrong. Too little sent starves the forward
//! garrison; too much starves the source's own men.
//!
//! The data is placeholder data: see
//! `data/scenarios/ladder/138_one_ship.toml` and
//! `data/map/placeholder_ladder.toml`. No rule is proved here that another
//! test does not prove: what is proved is that the game the player is given
//! has the answers the record says it has.
//!
//! **It runs on the live figures,** the ones the file names, through
//! `common::load_live` or `common::from_text_with` with the live options,
//! and asserts numbers that depend on them. That is decision 0237's named
//! exception to the rule that hand calculations run on the frozen figures
//! (question 8, confirmed by the designer on Nov 27, 2026): a game for play
//! that is no battle's is proved on the figures it is played on. A tuning
//! that moves a number below comes back to the architect.
//!
//! **Built.** Each test below asserts its statement, on the file as given
//! with the case's commands added at its mark `# orders`. The last day's
//! hash of each game is pinned in `tests/golden/pinned_hashes.txt`.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the scenario is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! Every stock, load and day below was worked by hand from the rules cited
//! and then checked on the engine as it stood on Nov 27, 2026 (`sim-cli
//! run` on this file with the commands of each case added at its mark).
//!
//! ## The figures read
//!
//! - `ground_hunger.toml: daily_supply_t_per_man` 0.005: each garrison of
//!   1,000 men is charged 5 t a night.
//! - Class 12 (`ship_classes_placeholder_fast_transport_cargo.csv`): 30 t of
//!   cargo, 16 kn at cruise, 500 t of fuel over 4,000 nm, so 0.125 t a mile.
//! - `port_sizes.csv`: size 1 (S) has one berth at 50 t an hour and fuels a
//!   ship at 200 t an hour; size 0 (T) works by lighter at 20 t an hour.
//! - The map: S (0,0) to T (5,0) is 5 hexes, 200 nm.
//!
//! ## What every case shares
//!
//! - **The order of a day** (chapter 1): the staff is asked, then the night
//!   pulse, the morning pulse and the afternoon pulse. The men's supply is in
//!   the night pulse, and a ship in port loads and fuels after it in the same
//!   pulse. So a night's meal comes before that night's loading at S, and
//!   before anything landed at T that day.
//! - **A passage** is 200 nm at 16 kn = 12.5 hours, 1 day, and burns
//!   200 x 0.125 = 25 t; a round trip burns 50 t.
//! - **Fuel.** The ship holds 60 t and S holds 50 t: 110 t, two round trips
//!   and 10 t over. On the night she first loads she takes the 50 t (110 t
//!   aboard, S out of fuel). After the first trip she holds 60 t, after the
//!   second 10 t, and 10 t is less than the 50 t of a third.
//! - **A run ordered on day d** (chapter 8, The doctrine, A new run) loads on
//!   day d (under an hour at the berth, 1 day), sails on day d + 1 and is due
//!   on day d + 1 (the sail day, plus the passage of 1 day, less one). She
//!   halts at T 12.5 hours into day d + 1, in the morning pulse (the engine
//!   gives the time as 06:30), and unloads there (at most 30 t at 20 t an
//!   hour, 1.5 hours). She is back at S early on day d + 2. Her ship is
//!   committed from day d to day d + 3 and free on day d + 4.
//! - **When a run is needed.** With R the stock at T on the morning of day d
//!   and nothing on its way, the reading of day d + 2 is R - 10 t. A run is
//!   needed where that is below the floor's stock, the floor in days x 5 t.
//! - **Its load** is the floor's stock less the reading of the day it is due
//!   (R - 5 t), not below 0, plus the interval's days x 5 t, cut to the
//!   ship's 30 t.
//! - **The need for the game.** S's men eat 14 x 5 = 70 t of its 120 t, so
//!   50 t may leave. T's men eat 70 t and T holds 25 t, so 45 t must arrive.
//!   Between 45 t and 50 t in two runs feeds every man.
//!
//! In each table "T" and "S" are the stock after the night's meal and after
//! that day's loading and landing, in tons.
//!
//! ## Case A: a floor of 2 days and an interval of 5. The mission is met
//!
//! The floor's stock is 10 t. A run is needed where R - 10 < 10, R below
//! 20 t.
//!
//! - Day 1, R = 25: 15, not below. Day 2, R = 20: 10, not below. No order.
//! - Day 3, R = 15: the reading of day 5 is 5 t, below 10: run 1. The reading
//!   of day 4 is 10 t; load = max(0, 10 - 10) + 5 x 5 = 25 t. Loading day 3,
//!   due day 4, committed days 3 to 6.
//! - Night of day 3 at S: the meal, 110 - 5 = 105; then 25 t loaded, 80 t.
//! - Day 4: after the night's meal T holds 5 t; 25 t landed, 30 t.
//! - Mornings at T: day 5, 30; day 6, 25; day 7, 20; day 8, 15.
//! - Day 8, R = 15: run 2, 25 t, loading day 8, due day 9, committed days 8
//!   to 11. Night of day 8 at S: 60 - 5 = 55, then 30 t. Day 9: T 5 + 25 = 30.
//! - Day 13, R = 15: the reading of day 15 is 5 t, below 10, and the ship
//!   holds 10 t of fuel against 50: no run, the staff hands in "cannot meet",
//!   and the standing interrupt fires. Day 14, R = 10: the same notice. The
//!   mission is not touched: the game ends with T at 5 t.
//!
//! | Night of day | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 |
//! | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
//! | T | 20 | 15 | 10 | 30 | 25 | 20 | 15 | 10 | 30 | 25 | 20 | 15 | 10 | 5 |
//! | S | 115 | 110 | 80 | 75 | 70 | 65 | 60 | 30 | 25 | 20 | 15 | 10 | 5 | 0 |
//!
//! Every night 1,000 of 1,000 are fed at each place. At the end S holds 0 t
//! of supply and 0 t of fuel, T holds 5 t, and the ship holds 10 t of fuel.
//!
//! ## Case B: 2 and 6. Too much: the source goes unfed
//!
//! The same floor, so run 1 is ordered on day 3; its load is 0 + 6 x 5 = 30 t.
//!
//! - Night of day 3 at S: 105, then 75. Day 4: T 5 + 30 = 35.
//! - Mornings at T: day 5, 35; 6, 30; 7, 25; 8, 20; 9, 15. Day 9: run 2,
//!   30 t, loading day 9, due day 10.
//! - S: nights 4 to 9 take 30 t, 45 t; 30 t loaded, 15 t. Nights 10, 11 and
//!   12 take it to 0.
//! - **Nights of days 13 and 14: 0 of 1,000 fed at S, 1,000 unfed.** T is fed
//!   every night and ends at 15 t: 25 + 60 - 70.
//!
//! ## Case C: 2 and 4. Too little: the forward garrison goes unfed
//!
//! Run 1 is ordered on day 3 with 0 + 4 x 5 = 20 t.
//!
//! - Day 4: T 5 + 20 = 25. Mornings: day 5, 25; 6, 20; 7, 15. Day 7: run 2,
//!   20 t (the ship is free that morning), loading day 7, due day 8. Day 8:
//!   T 5 + 20 = 25.
//! - Mornings: day 9, 25; 10, 20; 11, 15. Day 11: a run is needed and the
//!   ship holds 10 t of fuel: "cannot meet", on days 11, 12, 13 and 14.
//! - T after the nights of days 9 to 13: 20, 15, 10, 5, 0.
//! - **Night of day 14: 0 of 1,000 fed at T, 1,000 unfed.** S is fed every
//!   night and ends at 120 - 70 - 40 = 10 t.
//!
//! ## Case D: 1 and 1. Far too little
//!
//! The floor's stock is 5 t. A run is needed where R is below 15 t.
//!
//! - Day 4, R = 10: run 1. The reading of day 5 is 5 t; load = 0 + 5 = 5 t.
//!   Loading day 4, due day 5, committed days 4 to 7.
//! - Day 5: after the night's meal T holds 0; 5 t landed. Night of day 6: 0.
//! - Days 5, 6 and 7: a run is needed and the ship is not free: "cannot
//!   meet".
//! - Day 8, R = 0: run 2. The reading of day 9 is 0; load = (5 - 0) + 5 =
//!   10 t. Due day 9: 10 t landed after that night's meal. Nights of days 10
//!   and 11 take it to 0. From day 10 the ship has not the fuel.
//! - **Nights of days 7, 8, 9, 12, 13 and 14: 1,000 unfed at T.** Six
//!   nights. S is fed every night and ends at 120 - 70 - 15 = 35 t.
//!
//! ## Case E: "hold the place", with no number given
//!
//! In place of the supply order: "hold base 2", the same forces limit and
//! the same leaves (chapter 8, Holding a place).
//!
//! - Day 1: the staff forms under it intent 2, "keep base 2 supplied at 5
//!   days", with no interval, so 15 days. The floor's stock is 25 t.
//! - Day 2, R = 20: the reading of day 4 is 10 t, below 25: run 1. The
//!   reading of day 3 is 15 t; load = (25 - 15) + 15 x 5 = 85 t, cut to the
//!   ship's 30 t. Loading day 2, due day 3, committed days 2 to 5.
//! - Night of day 2 at S: 110, then 80. Day 3: T 10 + 30 = 40.
//! - Mornings at T: day 4, 40; 5, 35; 6, 30. Day 6: the reading of day 8 is
//!   20 t, below 25: run 2, cut to 30 t, due day 7. S: nights 3 to 6 take
//!   20 t, 60 t; 30 t loaded, 30 t. Day 7: T 20 + 30 = 50.
//! - S: nights 7 to 12 take it to 0.
//! - **Nights of days 13 and 14: 1,000 unfed at S.** T is fed every night
//!   and ends at 15 t. So a hold with no number fails the mission, the way
//!   case B does.
//!
//! ## Case F: two edges of the band, each met
//!
//! - **4 and 5.** The floor's stock is 20 t; needed where R is below 30. Day
//!   1, R = 25: run 1, load = max(0, 20 - 20) + 25 = 25 t, due day 2: T
//!   15 + 25 = 40. Mornings: day 3, 40; 4, 35; 5, 30; 6, 25. Day 6: run 2,
//!   25 t, due day 7: T 15 + 25 = 40. T ends at 5 t. S: 115 then 90 on the
//!   night of day 1; 65 then 40 on the night of day 6; 0 after the night of
//!   day 14. Nobody unfed.
//! - **8 and 1.** The floor's stock is 40 t. Day 1: load = (40 - 20) + 5 =
//!   25 t, due day 2: T 40. The ship is free on day 5. Day 5, R = 30: the
//!   reading of day 6 is 25; load = (40 - 25) + 5 = 20 t, due day 6: T
//!   20 + 20 = 40. T ends at 0 t after the night of day 14, fed. S ends at
//!   120 - 70 - 45 = 5 t. Nobody unfed.
//!
//! ## Case G: a floor of nothing
//!
//! "Keep base 2 supplied" with a floor of 0 days is refused: "a number of
//! days is 0". The forces limit after it is refused too, since intent 1 is
//! not in force. No order is in force and no run is sent. T is fed on the
//! nights of days 1 to 5 and holds 0 after the fifth; **1,000 are unfed at T
//! on each night from day 6 to day 14,** nine nights. S ends at 50 t.
//!
//! ## In every case
//!
//! No man is lost: `hunger_nights_before_loss` is 14 and no unit has more
//! than nine hungry nights. There is no enemy, and no random number is
//! drawn. The game with no command at all is case G without its refusals.
//!
//! # Statements
//!
//! 1. The file loads on the figures it names; it has two Allied bases, two
//!    Allied land units of 1,000 men, one ship and no command; the Japanese
//!    side has nothing; S to T is 200 nm by sea.
//! 2. Case A: the two runs, each of 25 t, are added on days 3 and 8 with the
//!    loading and due days above and the ship committed for days 3 to 6 and
//!    8 to 11; no run is added on any other day.
//! 3. Case A: the stock of T and of S after each day is the table's; `MenFed`
//!    shows 1,000 of 1,000 at each place on all 14 nights.
//! 4. Case A: the ship takes 50 t of fuel on the night of day 3 and holds
//!    10 t at the end; the staff hands in "cannot meet" on days 13 and 14
//!    and on no other day, and its reason of day 13 names the fuel.
//! 5. Case B: runs of 30 t on days 3 and 9; 1,000 unfed at S on the nights
//!    of days 13 and 14 and on no other; nobody unfed at T.
//! 6. Case C: runs of 20 t on days 3 and 7; 1,000 unfed at T on the night of
//!    day 14 and on no other; nobody unfed at S.
//! 7. Case D: runs of 5 t on day 4 and 10 t on day 8; 1,000 unfed at T on
//!    the nights of days 7, 8, 9, 12, 13 and 14 and on no other; nobody
//!    unfed at S.
//! 8. Case E: the staff forms "keep base 2 supplied at 5 days" under the
//!    hold on day 1; runs of 30 t on days 2 and 6; 1,000 unfed at S on the
//!    nights of days 13 and 14 and on no other; nobody unfed at T.
//! 9. Case F: with 4 and 5, runs of 25 t on days 1 and 6; with 8 and 1, runs
//!    of 25 t on day 1 and 20 t on day 5; in each nobody is unfed at either
//!    place on any night, and the stocks at the end are those above.
//! 10. Case G: both commands are refused, with the reasons above; 1,000
//!     unfed at T on the nights of days 6 to 14; nobody unfed at S.
//! 11. In every case no `MenLostUnfed` is recorded, each unit has 1,000 men
//!     at the end, and the game played twice gives the same hash each day.

use std::path::PathBuf;

use sim_core::command::{CommandKind, IntentKind, NoticeKind};
use sim_core::events::EventKind;
use sim_core::fixed::Fixed;
use sim_core::state::Side;
use sim_data::LoadOptions;

use crate::common::played::{play, Played};
use crate::common::values::{base, hex, n, ship, text_of, unit};
use crate::common::{self, data_dir};

fn path() -> PathBuf {
    data_dir().join("scenarios/ladder/138_one_ship.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

/// The scenario as given, on the live figures, with `orders` added at its
/// mark `# orders`.
fn load(orders: &str) -> sim_data::Scenario {
    let t = common::replace_once(&text(), "# orders", &format!("# orders\n{orders}"));
    common::from_text_with(&t, &path(), &LoadOptions::default()).expect("the scenario loads")
}

fn command(day: u32, body: &str) -> String {
    format!("\n[[command]]\nday = {day}\nside = \"allied\"\n{body}\n")
}

fn leaves() -> String {
    command(1, "kind = \"set_leave\"\nleave = \"prepare\"\ngiven = true")
        + &command(1, "kind = \"set_leave\"\nleave = \"launch\"\ngiven = true")
}

fn limit() -> String {
    command(
        1,
        "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\nships = [1]\nsource = 1",
    )
}

/// "Keep base 2 supplied", with a floor and an interval, the ship and the
/// source, and both leaves.
fn supply(floor: u32, interval: u32) -> String {
    leaves()
        + &command(
            1,
            &format!(
                "kind = \"set_intent\"\nintent = 1\nintent_kind = \"keep_supplied\"\nbase = 2\nfloor_days = {floor}\ninterval_days = {interval}"
            ),
        )
        + &limit()
}

fn hold() -> String {
    leaves()
        + &command(
            1,
            "kind = \"set_intent\"\nintent = 1\nintent_kind = \"hold\"\nbase = 2",
        )
        + &limit()
}

fn game(orders: &str) -> Played {
    play(load(orders))
}

fn supply_game(floor: u32, interval: u32) -> Played {
    game(&supply(floor, interval))
}

const S: u32 = 1;
const T: u32 = 2;

/// A run the staff added: the day, its load and its days.
struct RunAdded {
    day: u32,
    load: Fixed,
    days: sim_core::command::RunDays,
}

trait Reads {
    fn runs(&self) -> Vec<RunAdded>;
    fn stock(&self, day: u32, base_raw: u32) -> Fixed;
    fn fed(&self, base_raw: u32) -> Vec<(u32, u32, u32)>;
    fn unfed_days(&self, base_raw: u32) -> Vec<u32>;
    fn cannot_meet_days(&self) -> Vec<u32>;
    fn lost(&self) -> usize;
}

impl Reads for Played {
    fn runs(&self) -> Vec<RunAdded> {
        let mut out = Vec::new();
        for (day, d) in &self.handed[&Side::Allied] {
            for c in &d.commands {
                if let CommandKind::AddRun { run, .. } = &c.kind {
                    out.push(RunAdded {
                        day: *day,
                        load: run.load_t,
                        days: run.days,
                    });
                }
            }
        }
        out
    }

    fn stock(&self, day: u32, base_raw: u32) -> Fixed {
        self.truth[day as usize].bases[&base(base_raw)].supply_t
    }

    /// (day, fed, unfed) of the men at base `base_raw`'s hex, each night:
    /// one record a night, of 1,000 men, all fed or all unfed.
    fn fed(&self, base_raw: u32) -> Vec<(u32, u32, u32)> {
        let at = if base_raw == S { hex(0, 0) } else { hex(5, 0) };
        self.scenario
            .world
            .events()
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::MenFed {
                    hex, fed, unfed, ..
                } if *hex == at => Some((e.day, *fed, *unfed)),
                _ => None,
            })
            .collect::<Vec<_>>()
    }

    fn unfed_days(&self, base_raw: u32) -> Vec<u32> {
        let nights = self.fed(base_raw);
        let days: Vec<u32> = (1..=14).collect();
        assert_eq!(
            nights.iter().map(|(d, _, _)| *d).collect::<Vec<_>>(),
            days,
            "one record a night"
        );
        for (day, fed, unfed) in &nights {
            assert_eq!(fed + unfed, 1000, "day {day}");
            assert!(*unfed == 0 || *unfed == 1000, "day {day}: {unfed} unfed");
        }
        nights
            .into_iter()
            .filter(|(_, _, unfed)| *unfed > 0)
            .map(|(day, _, _)| day)
            .collect()
    }

    fn cannot_meet_days(&self) -> Vec<u32> {
        self.handed[&Side::Allied]
            .iter()
            .filter(|(_, d)| d.notices.iter().any(|x| x.kind == NoticeKind::CannotMeet))
            .map(|(day, _)| *day)
            .collect()
    }

    fn lost(&self) -> usize {
        self.scenario
            .world
            .events()
            .iter()
            .filter(|e| matches!(e.kind, EventKind::MenLostUnfed { .. }))
            .count()
    }
}

/// A run's load and day, plainly.
fn runs_of(g: &Played) -> Vec<(u32, Fixed)> {
    g.runs().iter().map(|r| (r.day, r.load)).collect()
}

fn stocks(g: &Played, b: u32) -> Vec<Fixed> {
    (1..=14).map(|d| g.stock(d, b)).collect()
}

fn tons(list: &[i64]) -> Vec<Fixed> {
    list.iter().map(|t| n(*t)).collect()
}

/// Every game of the file, by case.
fn every_case() -> Vec<(&'static str, String)> {
    vec![
        ("A", supply(2, 5)),
        ("B", supply(2, 6)),
        ("C", supply(2, 4)),
        ("D", supply(1, 1)),
        ("E", hold()),
        ("F 4 and 5", supply(4, 5)),
        ("F 8 and 1", supply(8, 1)),
        ("G", supply(0, 5)),
        ("no command", String::new()),
    ]
}

/// Statement 1: the file as the player is given it.
#[test]
fn the_file_as_given() {
    let t = text();
    assert!(!t.contains("[[command]]"), "no command in the file");
    assert!(t.contains("../../reference/ground_hunger.toml"));
    assert!(!t.contains("step1"), "the live figures");
    let scenario = load("");
    let truth = scenario.world.umpire();
    assert_eq!(truth.bases.len(), 2);
    assert!(truth.bases.values().all(|b| b.side == Side::Allied));
    assert_eq!(truth.land_units.len(), 2);
    for raw in [1, 2] {
        let u = &truth.land_units[&unit(raw)];
        assert_eq!((u.side, u.men_ashore), (Side::Allied, 1000));
    }
    assert_eq!(truth.ships.len(), 1);
    assert_eq!(truth.ships[&ship(1)].side, Side::Allied);
    assert!(truth.air_groups.is_empty());
    assert!(scenario
        .world
        .command_state(Side::Allied)
        .intents
        .is_empty());
    assert!(scenario
        .world
        .command_state(Side::Japanese)
        .intents
        .is_empty());
    // The Japanese side has nothing.
    assert!(truth.bases.values().all(|b| b.side != Side::Japanese));
    assert!(truth.ships.values().all(|s| s.side != Side::Japanese));
    assert!(truth.land_units.values().all(|u| u.side != Side::Japanese));
    // S to T is 200 nm by sea: 5 hexes of 40 nm, in a straight line.
    let (s, t) = (hex(0, 0), hex(5, 0));
    assert_eq!(truth.bases[&base(S)].hex, s);
    assert_eq!(truth.bases[&base(T)].hex, t);
    assert_eq!(scenario.world.map().sea_distance(s, t), Some(5));
    assert_eq!(s.air_distance_nm(t), 200);
}

/// Statement 2: case A, two runs of 25 t, ordered on days 3 and 8.
#[test]
fn two_runs_of_twenty_five() {
    let g = supply_game(2, 5);
    assert_eq!(runs_of(&g), vec![(3, n(25)), (8, n(25))]);
    let days: Vec<_> = g
        .runs()
        .iter()
        .map(|r| (r.days.load, r.days.sail, r.days.arrive, r.days.free))
        .collect();
    assert_eq!(days, vec![(3, 4, 4, 7), (8, 9, 9, 12)]);
    // The ship is committed for days 3 to 6 and 8 to 11.
    let mut commits = Vec::new();
    for (day, d) in &g.handed[&Side::Allied] {
        for c in &d.commands {
            if let CommandKind::CommitShips {
                ships,
                from_day,
                to_day,
                ..
            } = &c.kind
            {
                assert!(ships.contains(&ship(1)) && ships.len() == 1);
                commits.push((*day, *from_day, *to_day));
            }
        }
    }
    assert_eq!(commits, vec![(3, 3, 6), (8, 8, 11)]);
}

/// Statement 3: case A, the stocks of each night and every man fed.
#[test]
fn every_man_fed_every_night() {
    let g = supply_game(2, 5);
    assert_eq!(
        stocks(&g, T),
        tons(&[20, 15, 10, 30, 25, 20, 15, 10, 30, 25, 20, 15, 10, 5])
    );
    assert_eq!(
        stocks(&g, S),
        tons(&[115, 110, 80, 75, 70, 65, 60, 30, 25, 20, 15, 10, 5, 0])
    );
    for b in [S, T] {
        assert_eq!(
            g.fed(b),
            (1..=14).map(|d| (d, 1000, 0)).collect::<Vec<_>>(),
            "base {b}"
        );
    }
}

/// Statement 4: case A, the fuel, and "cannot meet" on days 13 and 14 alone.
#[test]
fn fuel_for_two_trips() {
    let g = supply_game(2, 5);
    let ship_fuel = |d: usize| g.truth[d].ships[&ship(1)].fuel_t;
    let source_fuel = |d: usize| g.truth[d].bases[&base(S)].fuel_t;
    assert_eq!((ship_fuel(2), source_fuel(2)), (n(60), n(50)));
    // The night of day 3: she takes the 50 t.
    assert_eq!((ship_fuel(3), source_fuel(3)), (n(110), n(0)));
    assert_eq!(ship_fuel(5), n(60));
    assert_eq!(ship_fuel(10), n(10));
    assert_eq!((ship_fuel(14), source_fuel(14)), (n(10), n(0)));
    assert_eq!(g.cannot_meet_days(), vec![13, 14]);
    let day_13 = &g.handed[&Side::Allied][&13];
    let all: String = day_13
        .no_orders
        .iter()
        .map(text_of)
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        all.contains("fuel"),
        "the reason of day 13 names the fuel: {all}"
    );
}

/// Statement 5: case B, too much sent, and the source unfed on two nights.
#[test]
fn too_much_starves_the_source() {
    let g = supply_game(2, 6);
    assert_eq!(runs_of(&g), vec![(3, n(30)), (9, n(30))]);
    assert_eq!(g.unfed_days(S), vec![13, 14]);
    assert!(g.unfed_days(T).is_empty());
    assert_eq!(g.stock(14, T), n(15));
}

/// Statement 6: case C, too little sent, and the forward garrison unfed on
/// the last night.
#[test]
fn too_little_starves_the_forward_garrison() {
    let g = supply_game(2, 4);
    assert_eq!(runs_of(&g), vec![(3, n(20)), (7, n(20))]);
    assert_eq!(g.unfed_days(T), vec![14]);
    assert!(g.unfed_days(S).is_empty());
    assert_eq!(g.cannot_meet_days(), vec![11, 12, 13, 14]);
    let after: Vec<_> = (9..=13).map(|d| g.stock(d, T)).collect();
    assert_eq!(after, tons(&[20, 15, 10, 5, 0]));
    assert_eq!(g.stock(14, S), n(10));
}

/// Statement 7: case D, 1 and 1, six nights unfed.
#[test]
fn far_too_little() {
    let g = supply_game(1, 1);
    assert_eq!(runs_of(&g), vec![(4, n(5)), (8, n(10))]);
    assert_eq!(g.unfed_days(T), vec![7, 8, 9, 12, 13, 14]);
    assert!(g.unfed_days(S).is_empty());
    assert_eq!(g.stock(14, S), n(35));
}

/// Statement 8: case E, a hold with no number fails as case B does.
#[test]
fn a_hold_with_no_number() {
    let g = game(&hold());
    let formed: Vec<_> = g.handed[&Side::Allied]
        .iter()
        .flat_map(|(day, d)| d.commands.iter().map(move |c| (*day, &c.kind)))
        .filter_map(|(day, k)| match k {
            CommandKind::FormIntent { parent, kind } => Some((day, *parent, *kind)),
            _ => None,
        })
        .collect();
    assert_eq!(
        formed,
        vec![(
            1,
            sim_core::ids::IntentId::new(1),
            IntentKind::KeepSupplied {
                base: base(T),
                floor_days: 5,
                interval_days: None
            }
        )]
    );
    assert_eq!(runs_of(&g), vec![(2, n(30)), (6, n(30))]);
    assert_eq!(g.unfed_days(S), vec![13, 14]);
    assert!(g.unfed_days(T).is_empty());
}

/// Statement 9: case F, the two edges of the band.
#[test]
fn the_edges_of_the_band() {
    let g = supply_game(4, 5);
    assert_eq!(runs_of(&g), vec![(1, n(25)), (6, n(25))]);
    assert!(g.unfed_days(S).is_empty() && g.unfed_days(T).is_empty());
    assert_eq!(g.stock(14, T), n(5));
    assert_eq!(
        (g.stock(1, S), g.stock(6, S), g.stock(14, S)),
        (n(90), n(40), n(0))
    );

    let g = supply_game(8, 1);
    assert_eq!(runs_of(&g), vec![(1, n(25)), (5, n(20))]);
    assert!(g.unfed_days(S).is_empty() && g.unfed_days(T).is_empty());
    assert_eq!((g.stock(2, T), g.stock(6, T)), (n(40), n(40)));
    assert_eq!((g.stock(14, T), g.stock(14, S)), (n(0), n(5)));
}

/// Statement 10: case G, a floor of nothing is refused.
#[test]
fn a_floor_of_nothing_is_refused() {
    let g = supply_game(0, 5);
    let refused: Vec<(u32, String)> = g
        .scenario
        .world
        .events()
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::CommandRefused { reason, .. } => Some((e.day, reason.to_string())),
            _ => None,
        })
        .collect();
    assert_eq!(
        refused,
        vec![
            (1, "a number of days is 0".to_string()),
            (1, "intent:1 is not in force".to_string()),
        ]
    );
    assert!(g.runs().is_empty());
    assert_eq!(g.unfed_days(T), (6..=14).collect::<Vec<_>>());
    assert!(g.unfed_days(S).is_empty());
    assert_eq!(g.stock(5, T), n(0));
    assert_eq!(g.stock(14, S), n(50));
}

/// Statement 11: nobody is lost, and the game is the same played twice.
#[test]
fn nobody_lost_and_the_same_twice() {
    for (name, orders) in every_case() {
        let g = game(&orders);
        assert_eq!(g.lost(), 0, "case {name}");
        for raw in [1, 2] {
            assert_eq!(
                g.truth[14].land_units[&unit(raw)].men_ashore,
                1000,
                "case {name}, unit {raw}"
            );
        }
        let again = game(&orders);
        assert_eq!(g.scenario.world.day_hashes().len(), 14, "case {name}");
        let last = *g.scenario.world.day_hashes().last().expect("a day hash");
        let pin = format!("scenario_138_{}", name.to_lowercase().replace(' ', "_"));
        assert_eq!(common::pinned_hash(&pin, last), last, "case {name}");
        assert_eq!(
            g.scenario.world.day_hashes(),
            again.scenario.world.day_hashes(),
            "case {name}"
        );
    }
}
