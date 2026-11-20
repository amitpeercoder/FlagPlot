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
//!    gives day 5 (100 / 20).
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
