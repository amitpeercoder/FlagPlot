//! Scenario 110: the last day ashore. A staff with nothing more coming does
//! not send its men in on the last day of a game.
//!
//! Proves decision 0194. The statements are the architect's, written
//! before the code. A mismatch goes back to the architect, and nobody else
//! changes a statement or a number here.
//!
//! # The setting
//!
//! Scenario 109's, the placeholder scenario of decision 0193: two units
//! ashore on defend with 5,086 fit men that can attack, an estimate of
//! 3,444 enemy men, nothing more coming, at balanced. The game has N
//! days, which the test sets.
//!
//! # Statements
//!
//! 1. **A side reads the game's days.** With N = 6, each side's view
//!    gives 6 as the game's days, and a seat's view gives 6.
//! 2. **Two days left: they go in.** N = 6, the morning of day 5 (4 days
//!    resolved, 6 - 4 = 2 left): the units are set to attack, as
//!    scenario 109's statement 3.
//! 3. **One day left: they do not.** N = 6, the morning of day 6 (5
//!    resolved, 1 left), the units not having been sent in before: they
//!    stay on defend. The reason gives the estimate, the 5,086 fit men,
//!    the 3,444 wanted, that nothing more is coming, and that with one
//!    day left an attack cannot take the place. No notice is handed in.
//! 4. **Press too.** The same morning at press, with an estimate of
//!    20,000: the units stay on defend, with the same words.
//! 5. **Men already in stay in.** Sent in on day 5; on the morning of day
//!    6 the staff does not set them to defend, and says they are
//!    committed.
//! 6. **Not with more coming, and not with no estimate.** On the last
//!    morning with a unit still to come the judgement is today's (it
//!    waits, 6,888 wanted). With no estimate held the units attack as
//!    today, on the last day as on any.
//! 7. **A game of one day.** N = 1: on its only morning 1 day is left,
//!    and statement 3 holds.
//!
//! # Properties
//!
//! - On every morning with 2 days left or more, every order and reason of
//!   the judgement ashore is what decision 0193 gives.
//! - With fewer than 2 days left no unit is set to attack by decision
//!   0193's rule.
//!
//! The tests are written by the build under these statements.
