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
//! **Not built.** Each test below is its statement and nothing more.
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

/// Statement 1: the file as the player is given it.
#[test]
#[ignore = "statement written, not built"]
fn the_file_as_given() {
    todo!()
}

/// Statement 2: case A, two runs of 25 t, ordered on days 3 and 8.
#[test]
#[ignore = "statement written, not built"]
fn two_runs_of_twenty_five() {
    todo!()
}

/// Statement 3: case A, the stocks of each night and every man fed.
#[test]
#[ignore = "statement written, not built"]
fn every_man_fed_every_night() {
    todo!()
}

/// Statement 4: case A, the fuel, and "cannot meet" on days 13 and 14 alone.
#[test]
#[ignore = "statement written, not built"]
fn fuel_for_two_trips() {
    todo!()
}

/// Statement 5: case B, too much sent, and the source unfed on two nights.
#[test]
#[ignore = "statement written, not built"]
fn too_much_starves_the_source() {
    todo!()
}

/// Statement 6: case C, too little sent, and the forward garrison unfed on
/// the last night.
#[test]
#[ignore = "statement written, not built"]
fn too_little_starves_the_forward_garrison() {
    todo!()
}

/// Statement 7: case D, 1 and 1, six nights unfed.
#[test]
#[ignore = "statement written, not built"]
fn far_too_little() {
    todo!()
}

/// Statement 8: case E, a hold with no number fails as case B does.
#[test]
#[ignore = "statement written, not built"]
fn a_hold_with_no_number() {
    todo!()
}

/// Statement 9: case F, the two edges of the band.
#[test]
#[ignore = "statement written, not built"]
fn the_edges_of_the_band() {
    todo!()
}

/// Statement 10: case G, a floor of nothing is refused.
#[test]
#[ignore = "statement written, not built"]
fn a_floor_of_nothing_is_refused() {
    todo!()
}

/// Statement 11: nobody is lost, and the game is the same played twice.
#[test]
#[ignore = "statement written, not built"]
fn nobody_lost_and_the_same_twice() {
    todo!()
}
