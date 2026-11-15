//! Scenario 42: interrupts, a run of days in one call, the command report,
//! and a save in the middle. A threshold already crossed when it is set, a
//! quiet week, a shortage that lasts (42a); two reasons on one day (42b);
//! an intent that cannot be met, as one matter with a history (42c); and a
//! game saved with a held ship, an amended plan and an acknowledged
//! interrupt, resumed, matching play that was not interrupted (42d).
//!
//! Proves step 4 of decision 0107 (step 3, stage 1: interrupts, `run`, the
//! command report, resuming; rulings 31 to 40 there) against a hand
//! calculation. The data is placeholder data: see
//! `data/scenarios/42a_a_shortage_stops_the_clock.toml`,
//! `42b_two_reasons_one_day.toml`, `42c_displaced_under_run.toml` and
//! `42d_saved_in_the_middle.toml`. It runs on the frozen step 1 figures.
//! There is no enemy, no aircraft and no random draw.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## The rules
//!
//! - **An interrupt is tested each morning,** when the staff is asked: on
//!   the reading of that day and on what the staff hands in that morning.
//! - **It fires on a change.** An interrupt is armed when it is set. When
//!   its condition holds and it is armed, it fires and is no longer armed.
//!   On a morning its condition does not hold, it is armed again.
//!   Acknowledging it does not arm it.
//! - **A threshold** is set for a base and a number of days: its condition
//!   holds when the hex's days of supply are below the number. With no
//!   figure it does not hold.
//! - **Three interrupts stand without being set:** the staff cannot meet an
//!   intent; a plan waits for the commander's leave; a ship of the side is
//!   lost. Each is armed apart for each plan, or each ship. The staff
//!   cannot meet an intent on a day it needs a run and can have none, or
//!   the run it can have leaves the reading it checks below the floor.
//! - **A run of days** resolves day after day, up to its number, and stops
//!   after a day on which any interrupt of the side fired, or at the end of
//!   the game. Everything that fired that day is listed together.
//! - **A matter** in the command report is a plan, a run where there is
//!   one, and a kind. It is open on each day the staff hands it in, and
//!   one matter however many days it lasts.
//!
//! # 42a: a threshold crossed when set, a quiet week, a shortage that lasts
//!
//! ## Inputs
//!
//! Scenario 36 whole: 18 days, both leaves given. One more command on day
//! 1: interrupt 1, base 2 below 35 days of supply.
//!
//! ## What happens
//!
//! The readings at T are scenario 36's. In days: 10 on day 1, 9, 16, then
//! 43 on day 4 falling by one a day to 30 on day 17, and 37 on day 18.
//!
//! | Call | Days resolved | Why it stops |
//! | ---- | ------------- | ------------ |
//! | `run` of 30 days | Day 1 | Interrupt 1 fired on day 1: 10 days, below 35. It was already true when it was set |
//! | `run` of 30 days | Days 2 to 13, twelve days | Interrupt 1 fired on day 13: 34 days |
//! | `acknowledge` interrupt 1, then `run` of 30 days | Days 14 to 18, five days | The game is over |
//!
//! - Days 2 and 3 read 9 and 16 days, below 35: the interrupt is not armed
//!   and does not fire. On day 4 the reading is 43 days: it is armed again.
//! - Days 5 to 12 read 42 down to 35 days. 35 is not below 35. The clock
//!   does not stop: a quiet week and more in one call.
//! - Day 13 reads 34 days: it fires. Days 14 to 17 read 33 down to 30: it
//!   does not fire again, acknowledged or not. Day 18 reads 37: armed.
//! - Interrupt 1 fired twice in the game, on days 1 and 13. No standing
//!   interrupt fired: the staff met the intent and waited for no leave.
//!
//! Every order, reading, stock and hour is scenario 36's, and the end
//! state is scenario 36's.
//!
//! The first call's answer lists the staff's orders for day 1 before the
//! day is resolved: the force formed and the load of 360 t.
//!
//! ## Properties of 42a
//!
//! - Played by `end_day` eighteen times in place of the three calls, the
//!   game has the same hash at the end of every day.
//! - With the acknowledgement left out, the third call resolves the same
//!   five days.
//! - With the threshold at 30 days in place of 35, it fires on day 1 only:
//!   the reading is never again below 30 after day 4.
//! - A `run` of 5 days from the start of day 2 resolves days 2 to 6 and
//!   stops for its number, having fired nothing.
//!
//! # 42b: two reasons on one day
//!
//! ## Inputs
//!
//! Scenario 36 with no leave set (leave to prepare is given by default,
//! leave to launch is not), 18 days. Commands of day 1: the intent, the
//! forces limit and the reserve of scenario 36, and interrupt 1, base 2
//! below 10 days of supply. On day 3 the commander approves the plan.
//!
//! ## What happens
//!
//! | Call | Days resolved | Why it stops |
//! | ---- | ------------- | ------------ |
//! | `run` of 30 days | Days 1 and 2 | Two fired on day 2, listed together: interrupt 1 (9 days, below 10), and the plan waits for leave to launch |
//! | `approve` the plan, then `run` of 30 days | Days 3 to 18, sixteen days | The game is over |
//!
//! - Day 1 reads 10 days, not below 10, and the staff makes the draft
//!   ready by the leave it has: nothing fires.
//! - Day 2 reads 9 days: interrupt 1 fires. Run 1 is loaded and the plan
//!   is not released: the standing interrupt for a plan that waits fires.
//!   The clock stops once.
//! - Day 3 reads 8 days and day 4 reads 15: interrupt 1 does not fire on
//!   day 3 and is armed again on day 4. The plan is released on day 3 and
//!   waits no more.
//! - The readings at T in tons: 100, 90, 80, 150, then 420 on day 5 falling
//!   by 10 a day to 310 on day 16 and 300 on day 17, and 370 on day 18. A
//!   second run of 150 t is ordered on day 16 and sails on day 17.
//!
//! At the end of day 18: base 2 holds 430 t; base 1 holds 4490 t of supply
//! and 1960 t of fuel; ship 1 is at sea, empty, 80 nm from S, with 968 t of
//! fuel; ship 2 has not moved.
//!
//! The command report asked after day 2 has one open matter, plan 1 waits
//! for leave to launch, opened on day 2, and interrupt 1 as fired and not
//! acknowledged. Asked at the end it has that matter closed, open on day 2
//! only.
//!
//! ## Property of 42b
//!
//! - **A convoy that waits a week is one matter.** With no approval on day
//!   3, the first call stops after day 2 and a second `run` of 7 days
//!   resolves days 3 to 9 and stops for its number, having fired nothing:
//!   the plan still waits and is not a new change. The report then has one
//!   open matter, plan 1 waits for leave to launch, opened on day 2, with a
//!   history of days 2 to 9, each leading to that day's reason.
//!
//! # 42c: an intent that cannot be met, under a run
//!
//! ## Inputs
//!
//! Scenario 40a whole, 16 days, with no command added. The standing
//! interrupts are on.
//!
//! ## What happens
//!
//! | Call | Days resolved | Why it stops |
//! | ---- | ------------- | ------------ |
//! | `run` of 30 days | Day 1 | The staff cannot meet intent 2: displaced by ship 1's run for intent 1 |
//! | `run` of 30 days | Days 2 to 10, nine days | The staff cannot meet intent 1: displaced by ship 1's run for intent 2 |
//! | `run` of 30 days | Days 11 to 16, six days | The game is over |
//!
//! - Intent 2 cannot be met on days 1 to 4: it fires on day 1 and not on
//!   days 2 to 4. On day 5 it is met and is armed again; it does not fire
//!   after.
//! - Intent 1 cannot be met on days 10 to 13: it fires on day 10 only.
//! - Every order, reading and stock is scenario 40a's.
//!
//! The command report at the end has two matters, both closed: intent 2's
//! plan could not be met, days 1 to 4; intent 1's plan could not be met,
//! days 10 to 13. Each day of each leads to that day's reason, which names
//! the commitment that displaced it.
//!
//! ## Property of 42c
//!
//! - On scenario 40b a `run` of 30 days stops after day 1 (the staff
//!   cannot meet intent 2: its load is cut to 14 t by the source, which
//!   leaves the reading of day 3 at 22 t, below the floor), and a second
//!   resolves days 2 to 5 to the end of the game, having fired nothing:
//!   intent 2 is short on every day. The report has one open matter with a
//!   history of days 1 to 5.
//! - A load cut that still leaves the reading it checks at or above the
//!   floor is not a matter: in 40a intent 1's first load is cut from 36 t
//!   to the ship's 30 t, the reading of day 3 is 38 t, and nothing fires
//!   for intent 1 on day 1.
//!
//! # 42d: saved in the middle
//!
//! ## Inputs
//!
//! Scenario 41a whole, 27 days, with two more commands: on day 1,
//! interrupt 1, base 2 below 38 days of supply; on day 11, interrupt 1 is
//! acknowledged.
//!
//! ## What happens
//!
//! The readings at T are scenario 41a's. In days of supply: 10, 9, 8 and
//! 15 on days 1 to 4; 42 on day 5 falling by one a day to 37 on day 10; 36
//! on day 11; 43 on day 12; 53 on day 13 falling by one a day to 40 on day
//! 26; 47 on day 27.
//!
//! - Interrupt 1 fires on day 1 (10 days). It is armed again on day 5 (42
//!   days), fires on day 10 (37 days), and is armed again on day 12. It
//!   does not fire after: the lowest later reading is 40 days.
//! - The plan waits for leave to launch on day 2: that fires on day 2.
//! - The game is saved at the end of day 12: ship 1 is held, the plan has
//!   one amendment, and interrupt 1 has fired twice and is acknowledged.
//! - Resumed from the save and played to day 27, the game has the same
//!   hash at the end of every day from 13 to 27 as the game played through
//!   without a save, and scenario 41a's end state.
//! - The command report asked on the resumed game after day 12 is, word
//!   for word, the report of the game that was not saved.
//!
//! ## Properties of 42d
//!
//! - A check of the saved game against the staff passes.
//! - Resuming the save for more play under another doctrine version is
//!   refused, and says which version the game was begun under. Replaying it
//!   is not refused.
