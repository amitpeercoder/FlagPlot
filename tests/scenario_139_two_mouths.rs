//! Scenario 139: two mouths, two ships. The second rung of the ladder
//! (decision 0237): a game for play with one decision, which of two unlike
//! ships is named to which of two supply orders. The far garrison's food is
//! gone after the night of day 3: the fast ship lands there on day 3 and the
//! slow one on day 4, hours after that night's meal.
//!
//! The data is placeholder data: see
//! `data/scenarios/ladder/139_two_mouths.toml` and
//! `data/map/placeholder_ladder.toml`. No rule is proved here that another
//! test does not prove: what is proved is that the game the player is given
//! has the answers the record says it has, and that the floor and the
//! interval of any right answer of rung 1 decide nothing in it.
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
//! and then checked on the engine as it stood on Nov 28, 2026 (`sim-cli
//! run` on this file with the commands of each case added at its mark).
//!
//! ## The figures read
//!
//! - `ground_hunger.toml: daily_supply_t_per_man` 0.005: the 1,000 men at T
//!   are charged 5 t a night and the 400 men at U 2 t.
//! - `ground_hunger.toml: hunger_nights_before_loss` 14: nobody can be lost
//!   in a game of 8 days.
//! - Class 3 (`ship_classes.csv`), ship 1: 3,000 t of cargo, 10 kn at
//!   cruise, 1,000 t of fuel over 10,000 nm, so 0.1 t a mile.
//! - Class 12 (`ship_classes_placeholder_fast_transport_cargo.csv`), ship 2:
//!   30 t of cargo, 16 kn at cruise, 500 t of fuel over 4,000 nm, so 0.125 t
//!   a mile.
//! - `port_sizes.csv`: size 1 (S) has one berth at 50 t an hour and fuels a
//!   ship at 200 t an hour; size 0 (T and U) works by lighter at 20 t an
//!   hour.
//! - The map: S (0,0) to T (5,0) is 5 hexes, 200 nm; S to U (0,14) is 14
//!   hexes in a straight line, 560 nm.
//!
//! ## What every case shares
//!
//! - **The order of a day** (chapter 1): the commander's commands and the
//!   staff, then the night pulse of 12 hours, the morning pulse and the
//!   afternoon pulse of 6 hours each. The men's supply is the first thing
//!   of the night pulse, and a ship in port loads after it in the same
//!   pulse. So a night's meal comes before anything landed that day,
//!   whatever the hour it lands.
//! - **The need of the game.** U holds 6 t and eats 2 t: after the night of
//!   day 3 it holds nothing, and food must land there on day 3 at the
//!   latest. T holds 25 t and eats 5 t: after the night of day 5 it holds
//!   nothing, and food must land there on day 5 at the latest. S holds
//!   500 t and no man eats from it.
//! - **A run ordered on day d** (chapter 8, The doctrine, A new run) loads
//!   on day d (every load here is under two hours at the berth, 1 day) and
//!   sails at the start of day d + 1. By ship and place:
//!
//!   | Ship, to | Passage | Due | Halts there | Committed | Free |
//!   | --- | --- | --- | --- | --- | --- |
//!   | 2, U | 560 / 16 = 35 h, 2 days | d + 2 | 11 h into day d + 2, the night pulse (05:00) | d to d + 5 | d + 6 |
//!   | 1, U | 560 / 10 = 56 h, 3 days | d + 3 | 8 h into day d + 3, the night pulse (02:00) | d to d + 7 | d + 8 |
//!   | 1, T | 200 / 10 = 20 h, 1 day | d + 1 | 20 h into day d + 1, the afternoon pulse (14:00) | d to d + 3 | d + 4 |
//!   | 2, T | 200 / 16 = 12.5 h, 1 day | d + 1 | 12.5 h into day d + 1, the morning pulse (06:30) | d to d + 3 | d + 4 |
//!
//!   The due day is the sail day plus the passage in whole days, less one.
//!   The free day is the sail day plus the two passages and one day of
//!   unloading, in whole days; the fuelling at S adds under two hours and
//!   moves no day. Every halt is after that day's meal, so **a run due on
//!   day a first feeds the night of day a + 1.**
//! - **When a run is needed:** where, with no new run, the reading of the
//!   day after it would be due is below the floor's stock, the floor in days
//!   times the day's use. The reading of a day is the stock on its morning,
//!   before the night's meal.
//! - **Its load** is the floor's stock less the reading of the day it is
//!   due, not below 0, plus the interval's days of use, cut to the ships'
//!   room.
//! - **A run on its way** is counted in the projection on the day after its
//!   due day (chapter 8, The projection), and a day's landing is in the next
//!   day's reading: so its load is in the readings from the second day after
//!   it is due. Once it has landed the stock itself has it.
//! - **Fuel** binds nothing. Each ship's tanks are full. A round trip burns
//!   140 t in ship 2 to U and 50 t to T, 112 t in ship 1 to U and 40 t to T.
//! - **The commands.** Every case but G gives leave to prepare and leave to
//!   launch on day 1. Unless a case says otherwise, intent 1 is "keep base 2
//!   supplied" (T) and intent 2 "keep base 3 supplied" (U), each with a
//!   forces limit that names its ships and base 1 as the source; plan 1 is
//!   intent 1's and plan 2 intent 2's. The intents are served in number
//!   order (chapter 8, Each day, for each intent).
//!
//! In each table "T", "U" and "S" are the stocks of bases 2, 3 and 1 after
//! the night's meal and after that day's loading and landing, in tons.
//!
//! ## Case A: the right ships, 2 and 5 on both orders. The mission is met
//!
//! Ship 1 is named to T's order and ship 2 to U's. The floor's stock is
//! 10 t at T and 4 t at U.
//!
//! - Day 1, U: a run by ship 2 would be due on day 3; the reading of day 4
//!   is 0, below 4: run 1 of plan 2. The reading of day 3 is 2 t; load =
//!   (4 - 2) + 5 x 2 = 12 t. Loading day 1, sailing day 2, due day 3, free
//!   day 7; ship 2 committed for days 1 to 6. T: the reading of day 3 is
//!   15 t, not below 10: no run.
//! - Day 2, T: the reading of day 4 is 10 t, not below: no run.
//! - Day 3, T: the reading of day 5 is 5 t, below 10: run 1 of plan 1. The
//!   reading of day 4 is 10 t; load = 0 + 5 x 5 = 25 t. Loading day 3,
//!   sailing and due day 4, free day 7; ship 1 committed for days 3 to 6.
//!   U: after the night's meal it holds 0; 12 t landed, 12 t.
//! - Day 4: T holds 5 t after the meal; 25 t landed, 30 t. U, reading 12: a
//!   run ordered today would be due on day 6 and the reading of day 7 is
//!   6 t: no run is needed.
//! - Day 5, U, reading 10: the reading of day 8 is 4 t, not below 4.
//! - Day 6, U, reading 8: the reading of day 9 is 2 t, below 4, and ship 2
//!   is committed to the end of day 6: the staff hands in "cannot meet". The
//!   mission is not touched: U has food to the night of day 9.
//! - Day 7, U, reading 6: ship 2 is free. Due day 9; the reading of day 10
//!   is 0; the reading of day 9 is 2 t; run 2 of plan 2, 12 t, loading day
//!   7, sailing day 8, due day 9, free day 13, committed for days 7 to 12.
//! - Day 8, T, reading 15: the reading of day 10 is 5 t, below 10: run 2 of
//!   plan 1, 25 t, loading day 8, sailing and due day 9, free day 12,
//!   committed for days 8 to 11. It loads and the game ends.
//!
//! | Night of day | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
//! | --- | --- | --- | --- | --- | --- | --- | --- | --- |
//! | T | 20 | 15 | 10 | 30 | 25 | 20 | 15 | 10 |
//! | U | 4 | 2 | 12 | 10 | 8 | 6 | 4 | 2 |
//! | S | 488 | 488 | 463 | 463 | 463 | 463 | 451 | 426 |
//!
//! Every night 1,000 of 1,000 are fed at T and 400 of 400 at U. At the end
//! ship 1 holds 1,000 - 40 = 960 t of fuel and 25 t of supply. Ship 2 has
//! made one round trip, 140 t, and steamed the whole of day 8, 24 x 16 =
//! 384 nm, 48 t: she holds 312 t of fuel and 12 t of supply.
//!
//! ## Case B: the right ships, 1 and 5 on both orders. The mission is met
//!
//! The lowest right answer of rung 1. The floor's stock is 5 t at T and 2 t
//! at U.
//!
//! - Day 1, U: the reading of day 4 is 0, below 2: run 1 of plan 2. The
//!   reading of day 3 is 2 t; load = 0 + 10 = 10 t. Due day 3: 10 t landed.
//! - Day 4, T, reading 10: the reading of day 6 is 0, below 5: run 1 of
//!   plan 1. The reading of day 5 is 5 t; load = 0 + 25 = 25 t. Due day 5:
//!   T holds 0 after that night's meal, and 25 t are landed.
//! - Day 6, U, reading 6: the reading of day 9 is 0, below 2, and ship 2 is
//!   committed: "cannot meet". Day 7, reading 4: the reading of day 9 is 0;
//!   run 2 of plan 2, (2 - 0) + 10 = 12 t, loading day 7, due day 9.
//! - Day 8, T, reading 15: the reading of day 10 is 5 t, not below 5: no
//!   run.
//!
//! | Night of day | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
//! | --- | --- | --- | --- | --- | --- | --- | --- | --- |
//! | T | 20 | 15 | 10 | 5 | 25 | 20 | 15 | 10 |
//! | U | 4 | 2 | 10 | 8 | 6 | 4 | 2 | 0 |
//! | S | 490 | 490 | 490 | 465 | 465 | 465 | 453 | 453 |
//!
//! Nobody is unfed on any night. U is fed on the night of day 8 with its
//! last 2 t. Were the game longer, the men at U would go unfed on the night
//! of day 9: run 2 lands that day, after the meal. That is why the game is
//! 8 days.
//!
//! ## Case C: the right ships, 8 and 2 on both orders. The mission is met
//!
//! The largest loads of rung 1's right answers. The floor's stock is 40 t
//! at T and 16 t at U.
//!
//! - Day 1, T: the reading of day 3 is 15 t, below 40: run 1 of plan 1. The
//!   reading of day 2 is 20 t; load = (40 - 20) + 2 x 5 = 30 t. Due day 2,
//!   free day 5. U: the reading of day 4 is 0: run 1 of plan 2, load =
//!   (16 - 2) + 2 x 2 = 18 t, inside ship 2's 30 t. Due day 3.
//! - Day 2: T holds 15 t after the meal; 30 t landed, 45 t. Day 3: U holds
//!   0; 18 t landed.
//! - Day 5, T, reading 35: ship 1 is free. The reading of day 7 is 25 t,
//!   below 40; the reading of day 6 is 30 t; run 2 of plan 1, (40 - 30) +
//!   10 = 20 t, due day 6: T holds 25 t after the meal, and 45 t.
//! - Day 7, U, reading 12: ship 2 is free. The reading of day 10 is 6 t,
//!   below 16; the reading of day 9 is 8 t; run 2 of plan 2, (16 - 8) + 4 =
//!   12 t, loading day 7.
//!
//! | Night of day | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
//! | --- | --- | --- | --- | --- | --- | --- | --- | --- |
//! | T | 20 | 45 | 40 | 35 | 30 | 45 | 40 | 35 |
//! | U | 4 | 2 | 18 | 16 | 14 | 12 | 10 | 8 |
//! | S | 452 | 452 | 452 | 452 | 432 | 432 | 420 | 420 |
//!
//! Nobody is unfed on any night. The staff hands in "cannot meet" on several
//! days, since neither ship can keep a floor of 8 days; no statement holds
//! those days.
//!
//! ## Case D: the ships exchanged, 2 and 5. The far garrison goes unfed
//!
//! Ship 2 is named to T's order and ship 1 to U's.
//!
//! - Day 1, U: a run by ship 1 would be due on day 4; the reading of day 5
//!   is 0, below 4: run 1 of plan 2. The reading of day 4 is 0; load =
//!   (4 - 0) + 10 = 14 t. Loading day 1, sailing day 2, due day 4, free day
//!   9; ship 1 committed for days 1 to 8. Day 1 is the first day an order
//!   can act, so nothing sends her sooner.
//! - Day 3, T: as case A, run 1 of plan 1, 25 t, by ship 2: loading day 3,
//!   sailing and due day 4, free day 7, committed for days 3 to 6.
//! - **Night of day 4: 0 of 400 fed at U, 400 unfed.** Ship 1 halts there 8
//!   hours into the day, after the meal, and lands 14 t. T holds 5 t after
//!   the meal; 25 t landed in the morning, 30 t.
//! - Day 5, U, reading 14: the reading of day 9 is 6 t: no run. Day 6: the
//!   reading of day 10 is 4 t, not below 4. Days 7 and 8: the readings of
//!   days 11 and 12 are 2 t and 0, below 4, and ship 1 is committed:
//!   "cannot meet" on each.
//! - Day 8, T, reading 15: run 2 of plan 1, 25 t, by ship 2, loading day 8,
//!   due day 9, free day 12, committed for days 8 to 11.
//!
//! | Night of day | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
//! | --- | --- | --- | --- | --- | --- | --- | --- | --- |
//! | T | 20 | 15 | 10 | 30 | 25 | 20 | 15 | 10 |
//! | U | 4 | 2 | 0 | 14 | 12 | 10 | 8 | 6 |
//! | S | 486 | 486 | 461 | 461 | 461 | 461 | 461 | 436 |
//!
//! T is fed every night. At the end ship 1 holds 1,000 - 112 = 888 t of
//! fuel and ship 2 500 - 50 = 450 t.
//!
//! ## Case E: both ships named to both orders. The staff chooses, and the
//! outcome turns on which order was given first
//!
//! Each order's forces limit names ships 1 and 2. A run takes the pool's
//! free ships in number order until their room covers its load, and ship 1
//! has room for any load here: so the first order that needs a run on a
//! morning takes ship 1, the slow one, if she is free.
//!
//! **E1: T's order is intent 1, 2 and 5 on both.**
//!
//! - Day 1: T needs no run (case A). U: the free ships in number order
//!   begin with ship 1; due day 4; run 1 of plan 2, 14 t, as case D; ship 1
//!   committed for days 1 to 8.
//! - Day 2: T needs no run. U: run 1 sails; ship 2 is free, and a run by
//!   her would be due on day 4. Run 1 is counted from the reading of day 6,
//!   so the reading of day 5 is 0, below 4: run 2 of plan 2. The reading of
//!   day 4 is 0; load = 14 t. Loading day 2, sailing day 3, due day 4, free
//!   day 8; ship 2 committed for days 2 to 7. So both ships go to U.
//! - Days 3 to 7, T: a run is needed and neither ship is free.
//! - **Night of day 4: 400 unfed at U.** Both runs halt there that day
//!   after the meal, 8 and 11 hours into it, and land 28 t.
//! - T holds 0 after the night of day 5. **Nights of days 6, 7 and 8: 1,000
//!   unfed at T.**
//! - Day 8, T, reading 0: ship 2 is free. Load = (10 - 0) + 25 = 35 t, cut
//!   to her 30 t; run 1 of plan 1, loading day 8, due day 9.
//!
//! | Night of day | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
//! | --- | --- | --- | --- | --- | --- | --- | --- | --- |
//! | T | 20 | 15 | 10 | 5 | 0 | 0 | 0 | 0 |
//! | U | 4 | 2 | 0 | 28 | 26 | 24 | 22 | 20 |
//! | S | 486 | 472 | 472 | 472 | 472 | 472 | 472 | 442 |
//!
//! **E2: T's order is intent 1, 4 and 5 on both. Met, by the order of the
//! orders.** The floor's stock is 20 t at T and 8 t at U.
//!
//! - Day 1, T, served first: the reading of day 3 is 15 t, below 20: run 1
//!   of plan 1 takes ship 1. The reading of day 2 is 20 t; load = 25 t.
//!   Due day 2, free day 5, committed for days 1 to 4. U: ship 1 is taken
//!   that morning, so the run takes ship 2: due day 3; load = (8 - 2) + 10 =
//!   16 t; free day 7, committed for days 1 to 6. So each ship goes where
//!   the right answer sends her.
//! - Day 2: T holds 15 t after the meal; 25 t landed, 40 t. Day 3: U holds
//!   0; 16 t landed.
//! - Day 5: ship 1 is free. T, reading 30: the reading of day 7 is 20 t,
//!   not below 20: no run. U, reading 14: a run by ship 1 would be due on
//!   day 8; the reading of day 9 is 6 t, below 8; the reading of day 8 is
//!   8 t; run 2 of plan 2, 0 + 10 = 10 t, by ship 1, loading day 5, sailing
//!   day 6, due day 8, free day 13, committed for days 5 to 12.
//! - Day 6, T, reading 25: the reading of day 8 is 15 t, below 20, and no
//!   ship is free. Day 7, reading 20: ship 2 is free. The reading of day 9
//!   is 10 t; the reading of day 8 is 15 t; run 2 of plan 1, (20 - 15) +
//!   25 = 30 t, by ship 2, loading day 7, sailing and due day 8, free day
//!   11, committed for days 7 to 10.
//! - Day 8: U holds 6 t after the meal; 10 t landed, 16 t. T holds 10 t
//!   after the meal; 30 t landed, 40 t.
//!
//! | Night of day | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
//! | --- | --- | --- | --- | --- | --- | --- | --- | --- |
//! | T | 20 | 40 | 35 | 30 | 25 | 20 | 15 | 40 |
//! | U | 4 | 2 | 16 | 14 | 12 | 10 | 8 | 16 |
//! | S | 459 | 459 | 459 | 459 | 449 | 449 | 419 | 419 |
//!
//! Nobody is unfed on any night.
//!
//! **E3: the same two orders with U's given first.** Intent 1 is "keep base
//! 3 supplied" and intent 2 "keep base 2 supplied", 4 and 5 on both, each
//! naming ships 1 and 2; plan 1 is U's and plan 2 is T's.
//!
//! - Day 1, U, served first: run 1 of plan 1 takes ship 1. Due day 4; the
//!   reading of day 4 is 0; load = 8 + 10 = 18 t; free day 9, committed for
//!   days 1 to 8. T: run 1 of plan 2 takes ship 2, 25 t, due day 2, free day
//!   5, committed for days 1 to 4.
//! - Day 2: T holds 15 t after the meal; 25 t landed, 40 t.
//! - **Night of day 4: 400 unfed at U.** Ship 1 lands 18 t that day, after
//!   the meal.
//! - Day 5: ship 2 is free. U, reading 18: a run by her would be due on day
//!   7, and the reading of day 8 is 12 t: no run. T, reading 30: no run.
//!   Day 6, U, reading 16: the reading of day 9 is 10 t, not below 8. T,
//!   reading 25: the reading of day 8 is 15 t, below 20; run 2 of plan 2,
//!   25 t, by ship 2, loading day 6, sailing and due day 7, free day 10,
//!   committed for days 6 to 9. Day 7: T holds 15 t after the meal; 25 t
//!   landed.
//!
//! | Night of day | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
//! | --- | --- | --- | --- | --- | --- | --- | --- | --- |
//! | T | 20 | 40 | 35 | 30 | 25 | 20 | 40 | 35 |
//! | U | 4 | 2 | 0 | 18 | 16 | 14 | 12 | 10 |
//! | S | 457 | 457 | 457 | 457 | 457 | 432 | 432 | 432 |
//!
//! T is fed every night. So the same two orders, with the same ships named,
//! meet the mission or fail it by which was given first: with both ships
//! named to both orders the ships do not decide, and the commander has not.
//!
//! ## Case F: every right answer of rung 1, on each order
//!
//! The nine pairs of floor and interval that rung 1 proves or shows met: 1
//! and 5, 2 and 5, 3 and 5, 4 and 5, 5 and 4, 6 and 3, 6 and 4, 8 and 1, 8
//! and 2. In each the floor F and the interval I sum to between 6 and 10.
//! T's order takes one of the nine and U's one of the nine, 81 games for
//! each choice of ships.
//!
//! **The right ships: nobody is unfed in any of the 81.**
//!
//! - U, by ship 2: on day 1 the reading of day 4 is 0, below any floor: a
//!   run. The reading of day 3 is 2 t; load = (2F - 2) + 2I = 2 (F + I - 1),
//!   at most 18 t, inside her 30 t. It lands on day 3, when U holds 0: food
//!   for F + I - 1 nights, the nights of days 4 to F + I + 2, which is day
//!   8 or later.
//! - T, by ship 1. With a floor of 1 to 4 days (and an interval of 5) the
//!   run is ordered on day 5 - F, the first day whose reading less 10 t is
//!   below 5F; it is due on day 6 - F, when the reading is 5F: load = 25 t.
//!   T then holds 5F - 5 + 25 t, food for the nights to day 10. With a
//!   floor of 5 or more the run is ordered on day 1 and due on day 2, when
//!   the reading is 20 t: load = (5F - 20) + 5I, at most 30 t; T then holds
//!   5 (F + I - 1) t, at least 40 t, food for the nights to day 10 or later.
//! - A later run only adds, and S has the stock for all of them.
//!
//! **The ships exchanged: in each of the 81, 400 are unfed at U on the
//! night of day 4 and on no other, and nobody is unfed at T.**
//!
//! - U, by ship 1: the run of day 1 is due on day 4, when the reading is 0:
//!   load = 2F + 2I, at least 12 t. It lands after the meal of day 4, which
//!   nobody eats, and feeds the nights of days 5 to 8 (8 t). No order can
//!   send her sooner than day 1.
//! - T, by ship 2: her passage is 1 day as ship 1's is, so every day above
//!   stands, and no load is above her 30 t.
//!
//! ## Case G: no command
//!
//! The file as given, with nothing added: no order and no leave. No run is
//! sent. **400 are unfed at U on each night from day 4 to day 8,
//! and 1,000 at T on each night from day 6 to day 8.** S holds 500 t at the
//! end.
//!
//! ## Case H: two holds, with the right ships
//!
//! In place of the supply orders: "hold base 2" as intent 1 with ship 1 and
//! base 1, and "hold base 3" as intent 2 with ship 2 and base 1 (chapter 8,
//! Holding a place).
//!
//! - Day 1: the staff forms under intent 1 "keep base 2 supplied at 5
//!   days" (intent 3) and under intent 2 "keep base 3 supplied at 5 days"
//!   (intent 4), each with no interval, so 15 days. They are served from the
//!   next morning. The floor's stock is 25 t at T and 10 t at U.
//! - Day 2, T, reading 20: the reading of day 4 is 10 t, below 25. The
//!   reading of day 3 is 15 t; load = (25 - 15) + 15 x 5 = 85 t, by ship 1,
//!   a run of plan 1, the hold's (plan 2 is the other hold's): loading day
//!   2 (1.7 hours), sailing and due day 3, free day 6. U,
//!   reading 4: due day 4; the reading of day 5 is 0, below 10; the reading
//!   of day 4 is 0; load = 10 + 15 x 2 = 40 t, cut to ship 2's 30 t.
//!   Loading day 2, sailing day 3, due day 4, free day 8.
//! - Day 3: ship 1 halts at T with 4 hours of the day left and lands 80 t;
//!   the last 5 t are landed in the night of day 4, after the meal.
//! - **Night of day 4: 400 unfed at U.** The hold cost a day: ship 2 lands
//!   her 30 t that day, after the meal.
//!
//! | Night of day | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
//! | --- | --- | --- | --- | --- | --- | --- | --- | --- |
//! | T | 20 | 15 | 90 | 90 | 85 | 80 | 75 | 70 |
//! | U | 4 | 2 | 0 | 30 | 28 | 26 | 24 | 22 |
//! | S | 500 | 385 | 385 | 385 | 385 | 385 | 385 | 385 |
//!
//! T is fed every night. So a hold with no number is no answer here, as on
//! rung 1.
//!
//! ## Case I: the right ships, a day late. The far garrison goes unfed
//!
//! Case A's two orders and their forces limits, given on day 2 in place of
//! day 1; the leaves on day 1 as before.
//!
//! - Day 1: no order is in force. Day 2, U, reading 4: a run by ship 2
//!   would be due on day 4; the reading of day 5 is 0, below 4: run 1 of
//!   plan 2. The reading of day 4 is 0; load = (4 - 0) + 10 = 14 t. Loading
//!   day 2, sailing day 3, due day 4, free day 8; ship 2 committed for days
//!   2 to 7. T: the reading of day 4 is 10 t, not below 10: no run.
//! - Day 3, T: run 1 of plan 1, 25 t, as case A.
//! - **Night of day 4: 0 of 400 fed at U, 400 unfed.** Ship 2 halts there 11
//!   hours into the day, after the meal, and lands 14 t. It is the night,
//!   and the count, of case D: the right ships a day late read as the wrong
//!   ships do.
//! - Day 7, U, reading 10: the reading of day 10 is 4 t, not below 4. Day
//!   8: T, reading 15: run 2 of plan 1, 25 t, as case A. U, reading 8: ship
//!   2 is free; due day 10; the reading of day 11 is 2 t, below 4; the
//!   reading of day 10 is 4 t; run 2 of plan 2, 0 + 10 = 10 t, loading day
//!   8, sailing day 9, due day 10, free day 14, committed for days 8 to 13.
//!   The staff hands in "cannot meet" on no day.
//!
//! | Night of day | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
//! | --- | --- | --- | --- | --- | --- | --- | --- | --- |
//! | T | 20 | 15 | 10 | 30 | 25 | 20 | 15 | 10 |
//! | U | 4 | 2 | 0 | 14 | 12 | 10 | 8 | 6 |
//! | S | 500 | 486 | 461 | 461 | 461 | 461 | 461 | 426 |
//!
//! T is fed every night. U has no day to spare: its food is gone after the
//! night of day 3 and ship 2 lands on the second day after she is ordered.
//!
//! ## In every case
//!
//! No man is lost: `hunger_nights_before_loss` is 14 and no unit has more
//! than five hungry nights. There is no enemy, and no random number is
//! drawn.
//!
//! # Statements
//!
//! 1. The file loads on the figures it names; it has three Allied bases, of
//!    which base 1 holds 500 t of supply and no fuel; two Allied land units,
//!    of 1,000 men at (5,0) and 400 men at (0,14); two ships in port at base
//!    1 in no task force, ship 1 of class 3 with 1,000 t of fuel and ship 2
//!    of class 12 with 500 t; no command; the Japanese side has nothing; S
//!    to T is 5 hexes by sea and 200 nm, and S to U 14 hexes and 560 nm.
//! 2. Case A: plan 2 has runs of 12 t added on days 1 and 7, with loading,
//!    sailing, due and free days (1, 2, 3, 7) and (7, 8, 9, 13), and ship 2
//!    committed for days 1 to 6 and 7 to 12; plan 1 has runs of 25 t added
//!    on days 3 and 8, with days (3, 4, 4, 7) and (8, 9, 9, 12), and ship 1
//!    committed for days 3 to 6 and 8 to 11; no run is added on any other
//!    day.
//! 3. Case A: the stock of T, of U and of S after each day is the table's;
//!    `MenFed` shows 1,000 of 1,000 at (5,0) and 400 of 400 at (0,14) on all
//!    8 nights.
//! 4. Case A: the staff hands in "cannot meet" on day 6 and on no other
//!    day; at the end ship 1 holds 960 t of fuel and 25 t of supply, and
//!    ship 2 312 t of fuel and 12 t of supply.
//! 5. Case B: runs of 10 t on day 1 and 12 t on day 7 in plan 2, and one of
//!    25 t on day 4 in plan 1; the stocks after each day are the table's;
//!    nobody is unfed at either place on any night; U holds 0 at the end.
//! 6. Case C: runs of 30 t on day 1 and 20 t on day 5 in plan 1, and of 18 t
//!    on day 1 and 12 t on day 7 in plan 2; the stocks after each day are
//!    the table's; nobody is unfed at either place on any night.
//! 7. Case D: plan 2 has one run, of 14 t, added on day 1 with days (1, 2,
//!    4, 9) and ship 1 committed for days 1 to 8; plan 1 has runs of 25 t on
//!    days 3 and 8 with ship 2 committed for days 3 to 6 and 8 to 11; 400
//!    unfed at (0,14) on the night of day 4 and on no other; nobody unfed at
//!    (5,0); the stocks after each day are the table's; "cannot meet" is
//!    handed in on days 7 and 8 and on no other day; ship 1 holds 888 t of
//!    fuel at the end and ship 2 450 t.
//! 8. Case E1: plan 2 has runs of 14 t added on day 1, with ship 1
//!    committed for days 1 to 8, and on day 2, with ship 2 committed for
//!    days 2 to 7; plan 1 has one run, of 30 t, added on day 8; 400 unfed at
//!    (0,14) on the night of day 4 and on no other; 1,000 unfed at (5,0) on
//!    the nights of days 6, 7 and 8 and on no other; the stocks after each
//!    day are the table's.
//! 9. Case E2: plan 1 has runs of 25 t on day 1 by ship 1 and 30 t on day 7
//!    by ship 2; plan 2 has runs of 16 t on day 1 by ship 2 and 10 t on day
//!    5 by ship 1; nobody is unfed at either place on any night; the stocks
//!    after each day are the table's. Case E3: plan 1 has one run, of 18 t
//!    on day 1 by ship 1; plan 2 has runs of 25 t on days 1 and 6 by ship 2;
//!    400 unfed at (0,14) on the night of day 4 and on no other; nobody
//!    unfed at (5,0); the stocks after each day are the table's.
//! 10. Case F: in each of the 81 games with the right ships nobody is unfed
//!     at either place on any night; in each of the 81 with the ships
//!     exchanged 400 are unfed at (0,14) on the night of day 4 and on no
//!     other, and nobody is unfed at (5,0).
//! 11. Case G: no run is added; 400 unfed at (0,14) on the nights of days 4
//!     to 8 and 1,000 at (5,0) on the nights of days 6 to 8, and on no
//!     other; base 1 holds 500 t at the end. Case H: the staff forms "keep
//!     base 2 supplied at 5 days" under intent 1 and "keep base 3 supplied
//!     at 5 days" under intent 2 on day 1, each with no interval; a run of
//!     85 t is added to plan 1 and one of 30 t to plan 2 on day 2, and no
//!     other run is added; 400 unfed at (0,14) on the night of day 4
//!     and on no other; nobody unfed at (5,0); the stocks after each day are
//!     the table's.
//! 12. In each of cases A, B, C, D, E1, E2, E3, G, H and I no `MenLostUnfed`
//!     is recorded, unit 1 has 1,000 men and unit 2 400 at the end, and the
//!     game played twice gives the same hash each day; the last day's hash
//!     of each is pinned.
//! 13. Case I: plan 2 has runs of 14 t added on day 2, with days (2, 3, 4,
//!     8) and ship 2 committed for days 2 to 7, and of 10 t on day 8, with
//!     days (8, 9, 10, 14); plan 1 has runs of 25 t on days 3 and 8; no run
//!     is added on day 1; 400 unfed at (0,14) on the night of day 4 and on
//!     no other; nobody unfed at (5,0); the stocks after each day are the
//!     table's; "cannot meet" is handed in on no day.

use std::collections::BTreeSet;
use std::path::PathBuf;

use sim_core::command::{CommandKind, IntentKind, NoticeKind, RunDays};
use sim_core::events::EventKind;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{IntentId, ShipId};
use sim_core::state::Side;
use sim_data::LoadOptions;

use crate::common::played::{play, Played};
use crate::common::values::{base, hex, n, ship, unit};
use crate::common::{self, data_dir};

fn path() -> PathBuf {
    data_dir().join("scenarios/ladder/139_two_mouths.toml")
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

fn limit(day: u32, intent: u32, ships: &[u32]) -> String {
    let list: Vec<String> = ships.iter().map(|s| s.to_string()).collect();
    command(
        day,
        &format!(
            "kind = \"set_limit\"\nlimit = \"forces\"\nintent = {intent}\nships = [{}]\nsource = 1",
            list.join(", ")
        ),
    )
}

/// "Keep base `base` supplied" as intent `intent`, with a floor, an interval
/// and the ships its forces limit names, all given on `day`.
fn supply(day: u32, intent: u32, base: u32, floor: u32, interval: u32, ships: &[u32]) -> String {
    command(
        day,
        &format!(
            "kind = \"set_intent\"\nintent = {intent}\nintent_kind = \"keep_supplied\"\nbase = {base}\nfloor_days = {floor}\ninterval_days = {interval}"
        ),
    ) + &limit(day, intent, ships)
}

/// "Hold base `base`" as intent `intent`, with the ships its forces limit
/// names, both given on day 1.
fn hold(intent: u32, base: u32, ships: &[u32]) -> String {
    command(
        1,
        &format!("kind = \"set_intent\"\nintent = {intent}\nintent_kind = \"hold\"\nbase = {base}"),
    ) + &limit(1, intent, ships)
}

/// T's order as intent 1 and U's as intent 2, on day 1, with both leaves.
fn two_orders(t: (u32, u32, &[u32]), u: (u32, u32, &[u32])) -> String {
    leaves() + &supply(1, 1, 2, t.0, t.1, t.2) + &supply(1, 2, 3, u.0, u.1, u.2)
}

/// Ship 1 to T and ship 2 to U: the right ships.
fn right(t: (u32, u32), u: (u32, u32)) -> String {
    two_orders((t.0, t.1, &[1]), (u.0, u.1, &[2]))
}

/// Ship 2 to T and ship 1 to U: the ships exchanged.
fn exchanged(t: (u32, u32), u: (u32, u32)) -> String {
    two_orders((t.0, t.1, &[2]), (u.0, u.1, &[1]))
}

fn game(orders: &str) -> Played {
    play(load(orders))
}

/// Case A: the right ships, 2 and 5 on both orders.
fn case_a() -> String {
    right((2, 5), (2, 5))
}

/// Case I: case A's two orders and their limits as commands of day 2.
fn case_i() -> String {
    leaves() + &supply(2, 1, 2, 2, 5, &[1]) + &supply(2, 2, 3, 2, 5, &[2])
}

const S: u32 = 1;
const T: u32 = 2;
const U: u32 = 3;

const DAYS: u32 = 8;

fn t_hex() -> Hex {
    hex(5, 0)
}

fn u_hex() -> Hex {
    hex(0, 14)
}

/// A run the staff added: the plan, the day, its ships, its load and its
/// days.
struct RunAdded {
    plan: u32,
    day: u32,
    ships: Vec<u32>,
    load: Fixed,
    days: RunDays,
}

/// A commitment of ships the staff handed in: the plan, the day, the ships
/// and the days.
struct Commit {
    plan: u32,
    day: u32,
    ships: Vec<u32>,
    from: u32,
    to: u32,
}

fn raws(ships: &BTreeSet<ShipId>) -> Vec<u32> {
    ships.iter().map(|s| s.raw()).collect()
}

trait Reads {
    fn runs(&self) -> Vec<RunAdded>;
    fn commits(&self) -> Vec<Commit>;
    fn stock(&self, day: u32, base_raw: u32) -> Fixed;
    fn fed(&self, at: Hex) -> Vec<(u32, u32, u32)>;
    fn unfed_days(&self, at: Hex, men: u32) -> Vec<u32>;
    fn cannot_meet_days(&self) -> Vec<u32>;
    fn formed(&self) -> Vec<(u32, IntentId, IntentKind)>;
    fn lost(&self) -> usize;
}

impl Reads for Played {
    fn runs(&self) -> Vec<RunAdded> {
        let mut out = Vec::new();
        for (day, d) in &self.handed[&Side::Allied] {
            for c in &d.commands {
                if let CommandKind::AddRun { plan, run } = &c.kind {
                    out.push(RunAdded {
                        plan: plan.raw(),
                        day: *day,
                        ships: raws(&run.ships),
                        load: run.load_t,
                        days: run.days,
                    });
                }
            }
        }
        out
    }

    fn commits(&self) -> Vec<Commit> {
        let mut out = Vec::new();
        for (day, d) in &self.handed[&Side::Allied] {
            for c in &d.commands {
                if let CommandKind::CommitShips {
                    plan,
                    ships,
                    from_day,
                    to_day,
                    ..
                } = &c.kind
                {
                    out.push(Commit {
                        plan: plan.raw(),
                        day: *day,
                        ships: raws(ships),
                        from: *from_day,
                        to: *to_day,
                    });
                }
            }
        }
        out
    }

    fn stock(&self, day: u32, base_raw: u32) -> Fixed {
        self.truth[day as usize].bases[&base(base_raw)].supply_t
    }

    /// (day, fed, unfed) of the men at `at`, each night.
    fn fed(&self, at: Hex) -> Vec<(u32, u32, u32)> {
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
            .collect()
    }

    /// The nights the men at `at` (`men` of them) go unfed: one record a
    /// night, all fed or all unfed.
    fn unfed_days(&self, at: Hex, men: u32) -> Vec<u32> {
        let nights = self.fed(at);
        assert_eq!(
            nights.iter().map(|(d, _, _)| *d).collect::<Vec<_>>(),
            (1..=DAYS).collect::<Vec<_>>(),
            "one record a night"
        );
        for (day, fed, unfed) in &nights {
            assert_eq!(fed + unfed, men, "day {day}");
            assert!(*unfed == 0 || *unfed == men, "day {day}: {unfed} unfed");
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

    fn formed(&self) -> Vec<(u32, IntentId, IntentKind)> {
        self.handed[&Side::Allied]
            .iter()
            .flat_map(|(day, d)| d.commands.iter().map(move |c| (*day, &c.kind)))
            .filter_map(|(day, k)| match k {
                CommandKind::FormIntent { parent, kind } => Some((day, *parent, *kind)),
                _ => None,
            })
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

/// The runs of a plan, as (day, load, load day, sail day, due day, free
/// day).
fn plan_runs(g: &Played, plan: u32) -> Vec<(u32, Fixed, u32, u32, u32, u32)> {
    g.runs()
        .iter()
        .filter(|r| r.plan == plan)
        .map(|r| {
            (
                r.day,
                r.load,
                r.days.load,
                r.days.sail,
                r.days.arrive,
                r.days.free,
            )
        })
        .collect()
}

/// The runs of a plan, as (day, ships, load).
fn plan_loads(g: &Played, plan: u32) -> Vec<(u32, Vec<u32>, Fixed)> {
    g.runs()
        .iter()
        .filter(|r| r.plan == plan)
        .map(|r| (r.day, r.ships.clone(), r.load))
        .collect()
}

/// The commitments of a plan, as (day, ships, from, to).
fn plan_commits(g: &Played, plan: u32) -> Vec<(u32, Vec<u32>, u32, u32)> {
    g.commits()
        .iter()
        .filter(|c| c.plan == plan)
        .map(|c| (c.day, c.ships.clone(), c.from, c.to))
        .collect()
}

fn stocks(g: &Played, b: u32) -> Vec<Fixed> {
    (1..=DAYS).map(|d| g.stock(d, b)).collect()
}

fn tons(list: &[i64]) -> Vec<Fixed> {
    list.iter().map(|t| n(*t)).collect()
}

/// The stocks after each day of T, U and S, against a table.
fn assert_stocks(g: &Played, t: &[i64], u: &[i64], s: &[i64]) {
    assert_eq!(stocks(g, T), tons(t), "T");
    assert_eq!(stocks(g, U), tons(u), "U");
    assert_eq!(stocks(g, S), tons(s), "S");
}

/// Nobody is unfed at either place on any night.
fn assert_all_fed(g: &Played) {
    assert!(g.unfed_days(t_hex(), 1000).is_empty());
    assert!(g.unfed_days(u_hex(), 400).is_empty());
}

/// 400 unfed at U on the night of day 4 and on no other, and nobody at T.
fn assert_only_u_day_4(g: &Played) {
    assert_eq!(g.unfed_days(u_hex(), 400), vec![4]);
    assert!(g.unfed_days(t_hex(), 1000).is_empty());
}

fn ship_fuel(g: &Played, raw: u32) -> Fixed {
    g.truth[DAYS as usize].ships[&ship(raw)].fuel_t
}

fn ship_supply(g: &Played, raw: u32) -> Fixed {
    g.truth[DAYS as usize].ships[&ship(raw)].supply_t
}

/// Every game of statement 12, by case.
fn every_case() -> Vec<(&'static str, String)> {
    vec![
        ("A", case_a()),
        ("B", right((1, 5), (1, 5))),
        ("C", right((8, 2), (8, 2))),
        ("D", exchanged((2, 5), (2, 5))),
        ("E1", two_orders((2, 5, &[1, 2]), (2, 5, &[1, 2]))),
        ("E2", two_orders((4, 5, &[1, 2]), (4, 5, &[1, 2]))),
        (
            "E3",
            leaves() + &supply(1, 1, 3, 4, 5, &[1, 2]) + &supply(1, 2, 2, 4, 5, &[1, 2]),
        ),
        ("G", String::new()),
        ("H", leaves() + &hold(1, 2, &[1]) + &hold(2, 3, &[2])),
        ("I", case_i()),
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
    assert_eq!(truth.bases.len(), 3);
    assert!(truth.bases.values().all(|b| b.side == Side::Allied));
    assert_eq!(truth.bases[&base(S)].supply_t, n(500));
    assert_eq!(truth.bases[&base(S)].fuel_t, n(0));
    assert_eq!(truth.land_units.len(), 2);
    let near = &truth.land_units[&unit(1)];
    assert_eq!(
        (near.side, near.men_ashore, near.hex),
        (Side::Allied, 1000, Some(t_hex()))
    );
    let far = &truth.land_units[&unit(2)];
    assert_eq!(
        (far.side, far.men_ashore, far.hex),
        (Side::Allied, 400, Some(u_hex()))
    );
    assert_eq!(truth.ships.len(), 2);
    for raw in [1, 2] {
        let s = &truth.ships[&ship(raw)];
        assert_eq!(s.side, Side::Allied);
        assert!(matches!(
            s.location,
            sim_core::state::ShipLocation::Port(b) if b == base(S)
        ));
    }
    assert_eq!(truth.ships[&ship(1)].class.raw(), 3);
    assert_eq!(truth.ships[&ship(1)].fuel_t, n(1000));
    assert_eq!(truth.ships[&ship(2)].class.raw(), 12);
    assert_eq!(truth.ships[&ship(2)].fuel_t, n(500));
    assert!(truth.task_forces.is_empty(), "no ship is in a task force");
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
    // S to T is 200 nm by sea, 5 hexes; S to U is 560 nm, 14 hexes.
    let (s, tt, u) = (hex(0, 0), t_hex(), u_hex());
    assert_eq!(truth.bases[&base(S)].hex, s);
    assert_eq!(truth.bases[&base(T)].hex, tt);
    assert_eq!(truth.bases[&base(U)].hex, u);
    assert_eq!(scenario.world.map().sea_distance(s, tt), Some(5));
    assert_eq!(s.air_distance_nm(tt), 200);
    assert_eq!(scenario.world.map().sea_distance(s, u), Some(14));
    assert_eq!(s.air_distance_nm(u), 560);
}

/// Statement 2: case A, the right ships, and the four runs.
#[test]
fn each_ship_to_her_own_mouth() {
    let g = game(&case_a());
    assert_eq!(
        plan_runs(&g, 2),
        vec![(1, n(12), 1, 2, 3, 7), (7, n(12), 7, 8, 9, 13),]
    );
    assert_eq!(
        plan_runs(&g, 1),
        vec![(3, n(25), 3, 4, 4, 7), (8, n(25), 8, 9, 9, 12),]
    );
    assert_eq!(g.runs().len(), 4, "no run on any other day");
    for r in g.runs() {
        assert_eq!(r.ships, vec![if r.plan == 1 { 1 } else { 2 }]);
    }
    assert_eq!(
        plan_commits(&g, 2),
        vec![(1, vec![2], 1, 6), (7, vec![2], 7, 12)]
    );
    assert_eq!(
        plan_commits(&g, 1),
        vec![(3, vec![1], 3, 6), (8, vec![1], 8, 11)]
    );
    assert_eq!(g.commits().len(), 4);
}

/// Statement 3: case A, the stocks of each night and every man fed.
#[test]
fn every_man_fed_every_night() {
    let g = game(&case_a());
    assert_stocks(
        &g,
        &[20, 15, 10, 30, 25, 20, 15, 10],
        &[4, 2, 12, 10, 8, 6, 4, 2],
        &[488, 488, 463, 463, 463, 463, 451, 426],
    );
    assert_eq!(
        g.fed(t_hex()),
        (1..=DAYS).map(|d| (d, 1000, 0)).collect::<Vec<_>>()
    );
    assert_eq!(
        g.fed(u_hex()),
        (1..=DAYS).map(|d| (d, 400, 0)).collect::<Vec<_>>()
    );
}

/// Statement 4: case A, "cannot meet" on day 6 alone, and the fuel.
#[test]
fn the_fast_ship_cannot_keep_the_floor_and_the_mission_is_safe() {
    let g = game(&case_a());
    assert_eq!(g.cannot_meet_days(), vec![6]);
    assert_eq!(ship_fuel(&g, 1), n(960));
    assert_eq!(ship_supply(&g, 1), n(25));
    assert_eq!(ship_fuel(&g, 2), n(312));
    assert_eq!(ship_supply(&g, 2), n(12));
    assert_all_fed(&g);
}

/// Statement 5: case B, 1 and 5, the lowest right answer of rung 1.
#[test]
fn the_lowest_right_answer_of_rung_one() {
    let g = game(&right((1, 5), (1, 5)));
    assert_eq!(
        g.runs()
            .iter()
            .map(|r| (r.plan, r.day, r.load))
            .collect::<Vec<_>>(),
        vec![(2, 1, n(10)), (1, 4, n(25)), (2, 7, n(12))]
    );
    assert_stocks(
        &g,
        &[20, 15, 10, 5, 25, 20, 15, 10],
        &[4, 2, 10, 8, 6, 4, 2, 0],
        &[490, 490, 490, 465, 465, 465, 453, 453],
    );
    assert_all_fed(&g);
    assert_eq!(g.stock(DAYS, U), n(0));
}

/// Statement 6: case C, 8 and 2, the largest loads.
#[test]
fn the_largest_loads() {
    let g = game(&right((8, 2), (8, 2)));
    assert_eq!(
        plan_loads(&g, 1)
            .iter()
            .map(|(d, _, l)| (*d, *l))
            .collect::<Vec<_>>(),
        vec![(1, n(30)), (5, n(20))]
    );
    assert_eq!(
        plan_loads(&g, 2)
            .iter()
            .map(|(d, _, l)| (*d, *l))
            .collect::<Vec<_>>(),
        vec![(1, n(18)), (7, n(12))]
    );
    assert_eq!(g.runs().len(), 4);
    assert_stocks(
        &g,
        &[20, 45, 40, 35, 30, 45, 40, 35],
        &[4, 2, 18, 16, 14, 12, 10, 8],
        &[452, 452, 452, 452, 432, 432, 420, 420],
    );
    assert_all_fed(&g);
}

/// Statement 7: case D, the ships exchanged, and the far garrison unfed on
/// the night of day 4.
#[test]
fn the_ships_exchanged() {
    let g = game(&exchanged((2, 5), (2, 5)));
    assert_eq!(plan_runs(&g, 2), vec![(1, n(14), 1, 2, 4, 9)]);
    assert_eq!(plan_commits(&g, 2), vec![(1, vec![1], 1, 8)]);
    assert_eq!(
        plan_loads(&g, 1)
            .iter()
            .map(|(d, s, l)| (*d, s.clone(), *l))
            .collect::<Vec<_>>(),
        vec![(3, vec![2], n(25)), (8, vec![2], n(25))]
    );
    assert_eq!(
        plan_commits(&g, 1),
        vec![(3, vec![2], 3, 6), (8, vec![2], 8, 11)]
    );
    assert_only_u_day_4(&g);
    assert_stocks(
        &g,
        &[20, 15, 10, 30, 25, 20, 15, 10],
        &[4, 2, 0, 14, 12, 10, 8, 6],
        &[486, 486, 461, 461, 461, 461, 461, 436],
    );
    assert_eq!(g.cannot_meet_days(), vec![7, 8]);
    assert_eq!(ship_fuel(&g, 1), n(888));
    assert_eq!(ship_fuel(&g, 2), n(450));
}

/// Statement 8: case E1, both ships named to both orders, and both sent to
/// the far place.
#[test]
fn both_ships_to_both_orders() {
    let g = game(&two_orders((2, 5, &[1, 2]), (2, 5, &[1, 2])));
    assert_eq!(
        plan_loads(&g, 2),
        vec![(1, vec![1], n(14)), (2, vec![2], n(14))]
    );
    assert_eq!(
        plan_commits(&g, 2),
        vec![(1, vec![1], 1, 8), (2, vec![2], 2, 7)]
    );
    assert_eq!(plan_loads(&g, 1).len(), 1);
    assert_eq!(plan_loads(&g, 1)[0].0, 8);
    assert_eq!(plan_loads(&g, 1)[0].2, n(30));
    assert_eq!(g.runs().len(), 3);
    assert_eq!(g.unfed_days(u_hex(), 400), vec![4]);
    assert_eq!(g.unfed_days(t_hex(), 1000), vec![6, 7, 8]);
    assert_stocks(
        &g,
        &[20, 15, 10, 5, 0, 0, 0, 0],
        &[4, 2, 0, 28, 26, 24, 22, 20],
        &[486, 472, 472, 472, 472, 472, 472, 442],
    );
}

/// Statement 9: cases E2 and E3, the same two orders met or failed by which
/// was given first.
#[test]
fn both_ships_and_the_order_of_the_orders() {
    // E2: T's order first.
    let g = game(&two_orders((4, 5, &[1, 2]), (4, 5, &[1, 2])));
    assert_eq!(
        plan_loads(&g, 1),
        vec![(1, vec![1], n(25)), (7, vec![2], n(30))]
    );
    assert_eq!(
        plan_loads(&g, 2),
        vec![(1, vec![2], n(16)), (5, vec![1], n(10))]
    );
    assert_eq!(g.runs().len(), 4);
    assert_all_fed(&g);
    assert_stocks(
        &g,
        &[20, 40, 35, 30, 25, 20, 15, 40],
        &[4, 2, 16, 14, 12, 10, 8, 16],
        &[459, 459, 459, 459, 449, 449, 419, 419],
    );

    // E3: U's order first.
    let g = game(&(leaves() + &supply(1, 1, 3, 4, 5, &[1, 2]) + &supply(1, 2, 2, 4, 5, &[1, 2])));
    assert_eq!(plan_loads(&g, 1), vec![(1, vec![1], n(18))]);
    assert_eq!(
        plan_loads(&g, 2),
        vec![(1, vec![2], n(25)), (6, vec![2], n(25))]
    );
    assert_eq!(g.runs().len(), 3);
    assert_only_u_day_4(&g);
    assert_stocks(
        &g,
        &[20, 40, 35, 30, 25, 20, 40, 35],
        &[4, 2, 0, 18, 16, 14, 12, 10],
        &[457, 457, 457, 457, 457, 432, 432, 432],
    );
}

/// Statement 10: case F, every right answer of rung 1 on each order, with
/// the right ships and with the ships exchanged.
#[test]
fn the_ships_alone_decide() {
    let band = [
        (1, 5),
        (2, 5),
        (3, 5),
        (4, 5),
        (5, 4),
        (6, 3),
        (6, 4),
        (8, 1),
        (8, 2),
    ];
    let mut games = 0;
    for t in band {
        for u in band {
            let g = game(&right(t, u));
            assert_all_fed(&g);
            assert_eq!(g.lost(), 0, "right ships, T {t:?}, U {u:?}");
            let g = game(&exchanged(t, u));
            assert_eq!(
                g.unfed_days(u_hex(), 400),
                vec![4],
                "exchanged, T {t:?}, U {u:?}"
            );
            assert!(
                g.unfed_days(t_hex(), 1000).is_empty(),
                "exchanged, T {t:?}, U {u:?}"
            );
            games += 2;
        }
    }
    assert_eq!(games, 162);
}

/// Statement 11: cases G and H, no command, and two holds.
#[test]
fn no_command_and_two_holds() {
    // G: the file as given.
    let g = game("");
    assert!(g.runs().is_empty());
    assert_eq!(g.unfed_days(u_hex(), 400), vec![4, 5, 6, 7, 8]);
    assert_eq!(g.unfed_days(t_hex(), 1000), vec![6, 7, 8]);
    assert_eq!(g.stock(DAYS, S), n(500));

    // H: two holds, with the right ships.
    let g = game(&(leaves() + &hold(1, 2, &[1]) + &hold(2, 3, &[2])));
    assert_eq!(
        g.formed(),
        vec![
            (
                1,
                IntentId::new(1),
                IntentKind::KeepSupplied {
                    base: base(T),
                    floor_days: 5,
                    interval_days: None
                }
            ),
            (
                1,
                IntentId::new(2),
                IntentKind::KeepSupplied {
                    base: base(U),
                    floor_days: 5,
                    interval_days: None
                }
            ),
        ]
    );
    let runs: Vec<_> = g.runs().iter().map(|r| (r.plan, r.day, r.load)).collect();
    assert_eq!(runs, vec![(1, 2, n(85)), (2, 2, n(30))]);
    assert_only_u_day_4(&g);
    assert_stocks(
        &g,
        &[20, 15, 90, 90, 85, 80, 75, 70],
        &[4, 2, 0, 30, 28, 26, 24, 22],
        &[500, 385, 385, 385, 385, 385, 385, 385],
    );
}

/// Statement 12: nobody is lost, and the game is the same played twice.
#[test]
fn nobody_lost_and_the_same_twice() {
    for (name, orders) in every_case() {
        let g = game(&orders);
        assert_eq!(g.lost(), 0, "case {name}");
        assert_eq!(g.truth[DAYS as usize].land_units[&unit(1)].men_ashore, 1000);
        assert_eq!(g.truth[DAYS as usize].land_units[&unit(2)].men_ashore, 400);
        let again = game(&orders);
        assert_eq!(
            g.scenario.world.day_hashes().len(),
            DAYS as usize,
            "case {name}"
        );
        let last = *g.scenario.world.day_hashes().last().expect("a day hash");
        let pin = format!("scenario_139_{}", name.to_lowercase());
        assert_eq!(common::pinned_hash(&pin, last), last, "case {name}");
        assert_eq!(
            g.scenario.world.day_hashes(),
            again.scenario.world.day_hashes(),
            "case {name}"
        );
    }
}

/// Statement 13: case I, the right ships a day late, and the far garrison
/// unfed as with the wrong ships.
#[test]
fn the_right_ships_a_day_late() {
    let g = game(&case_i());
    assert_eq!(
        plan_runs(&g, 2),
        vec![(2, n(14), 2, 3, 4, 8), (8, n(10), 8, 9, 10, 14),]
    );
    assert_eq!(plan_commits(&g, 2)[0], (2, vec![2], 2, 7));
    assert_eq!(
        plan_loads(&g, 1)
            .iter()
            .map(|(d, _, l)| (*d, *l))
            .collect::<Vec<_>>(),
        vec![(3, n(25)), (8, n(25))]
    );
    assert!(g.runs().iter().all(|r| r.day != 1), "no run on day 1");
    assert_eq!(g.runs().len(), 4);
    assert_only_u_day_4(&g);
    assert_stocks(
        &g,
        &[20, 15, 10, 30, 25, 20, 15, 10],
        &[4, 2, 0, 14, 12, 10, 8, 6],
        &[500, 486, 461, 461, 461, 461, 461, 426],
    );
    assert!(g.cannot_meet_days().is_empty());
}
