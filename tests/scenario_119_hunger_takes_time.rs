//! Scenario 119: hunger takes time. Men land with rations, a unit counts
//! its nights of hunger, and men are lost only from a unit that has been
//! hungry for many nights.
//!
//! Proves decision 0205. The statements are the architect's, written
//! before the code. A mismatch goes back to the architect, and nobody else
//! changes a statement or a number here.
//!
//! # The setting
//!
//! A placeholder scenario made for the test, on a placeholder map, with
//! no real unit. Its ground figures are in a file of its own: the daily
//! supply for a man is 0.005 t, `rations_landed_nights` is 3,
//! `hungry_below_share` is 0.5, `hunger_nights_before_loss` is 14 and
//! `unfed_loss_share_per_day` is 0.05. Every land unit has the standard
//! weight. Nothing fights and nothing is bombed.
//!
//! Side A has a base Q, with no aircraft, and a land unit V of 1,000 men
//! ashore there at the start. The men's charge at Q is 5 t a night. Q's
//! stock is what each statement gives it, and nothing is landed there
//! unless a statement says so. Side B has nothing ashore.
//!
//! # Statements: nights of hunger
//!
//! 1. **Hungry, and nobody lost.** With no stock at Q, V's fed share is 0
//!    every night. Its count is 1 after the first night and 14 after the
//!    fourteenth, and through those fourteen nights V loses no man and
//!    has 1,000.
//! 2. **The fifteenth night.** That night the count is 15, above 14. V's
//!    unfed are 1,000 and it loses 1,000 times 0.05, 50 men; 950 remain.
//!    On the sixteenth it loses 950 times 0.05, 47.5 cut to 47; 903
//!    remain.
//! 3. **By today's rule,** with the same scenario on a ground file
//!    without the three figures, V loses 50 men on the first night.
//! 4. **Short rations are not hunger.** With 3 t at Q, the first night
//!    feeds 600 of the 1,000: a fed share of 0.6, not below a half. The
//!    count stays at nothing and nobody is lost. With 2.5 t the share is
//!    exactly 0.5 and the count stays. With 2 t the share is 0.4 and the
//!    count is 1.
//! 5. **The count falls only on a full night.** With no stock for five
//!    nights the count is 5. Supply for V then reaches Q on day 6: 3.5 t,
//!    then 5 t on each of days 7 and 8. On the sixth night 700 are fed, a
//!    share of 0.7, and the count stays 5. On the seventh and the eighth
//!    all are fed, and the count is 4 and then 3.
//! 6. **A count given at the start.** With V given a count of 14 at the
//!    start and no stock at Q, V loses 50 men on the first night.
//!
//! # Statements: who is lost
//!
//! 7. **Only the units past the count.** V, 1,000 men with a count of 14
//!    at the start, and a second unit W, 500 men with none, are at Q with
//!    no stock. On the first night V's count is 15 and W's is 1. The loss
//!    is worked on V's men alone: 1,000 unfed, 50 lost, all from V. W
//!    loses nobody. By today's rule the hex would lose 1,500 times 0.05,
//!    75: 50 from V and 25 from W.
//! 8. **One man at least.** With V of 10 men and a count of 14 at the
//!    start and no stock, the first night's loss is 10 times 0.05, 0.5,
//!    cut to nothing, and one man at least: V loses 1 and has 9.
//! 9. **A unit aboard keeps its count, and a unit divided takes it.**
//!    With V's count at 5, V put aboard ship keeps 5 while it has no man
//!    ashore; a unit made by dividing V starts with 5.
//!
//! # Statements: rations landed
//!
//! Side A has a force T of two transports with a land unit U of 3,000
//! men aboard, and a hex P where A has no base.
//!
//! 10. **The landing brings three nights.** When T lands its first man at
//!     P a base is formed there, and 3,000 times 3 times 0.005 t, 45 t of
//!     supply, are in its stock at that moment, whatever number of men
//!     are ashore by then. The record of it names 3,000 men, 3 nights and
//!     45 t, and cites the landing.
//! 11. **A base that stands gets none.** A second force with 1,000 men
//!     landing at P on a later day adds nothing to P's stock but what it
//!     unloads. A force landing men at Q, a base at the start, brings
//!     none.
//! 12. **With the rations, the first nights are fed.** With all of U
//!     ashore at P before the first night, the charge is 15 t a night:
//!     the three nights after the landing are fed in full, the stock is
//!     then nothing, and U's count is still nothing after them.
//! 13. **Without the figures, a landing brings nothing,** and the base is
//!     formed with no supply, as today.
//!
//! # Properties
//!
//! - **No game on file moves.** Every scenario and recorded game on file
//!   gives the events it gave before, and the daily hashes it gave before
//!   but for the state's layout version.
//! - **Nothing is drawn.** The same game played twice gives the same
//!   events.
//! - **Each side sees its own.** B's view gives none of A's rations, A's
//!   counts or A's losses to hunger.
//! - **The table's copies are true copies.** Each scenario the table
//!   plays differs from the file it was made from in the line that names
//!   its ground figures and in nothing else.

// The architect's statement lists are list items in the doc comment above,
// which is left as written.
#![allow(clippy::doc_lazy_continuation)]
