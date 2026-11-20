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
//! supply and 12.5 t of fuel a day from day 3 to day 6. Side A has a base
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
//! 5. **Not to a base that has changed hands.** With R taken by A on day
//!    4: R receives on day 3 only. Its stock is not raised on days 4 to
//!    6 for either side, and with R taken back by J on day 6 it receives
//!    on day 6.
//! 6. **Two lines add.** A second line to R of 10 t of supply on days 5
//!    to 10: at the end of day 10, 220 + 6 x 10 = 280 t.
//! 7. **It feeds what draws on the base.** With 1,000 men at R drawing
//!    1,000 x 0.005 = 5 t a day, at the end of day 6: 100 - 6 x 5 + 4 x
//!    30 = 190 t.
//! 8. **Told in the briefing.** J's briefing of day 1 names the line, its
//!    tons and its days, under what is due. A's says nothing of it.
//! 9. **Refused scenarios:** a line to a base the scenario has not; one
//!    with neither supply nor fuel; a last day before its first; tons
//!    below 0.
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
