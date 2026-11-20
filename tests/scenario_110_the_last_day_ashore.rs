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
//! 3,444 enemy men, nothing more coming, at balanced. Its first morning
//! with nothing coming is the morning of day 4, with 3 days resolved. The
//! game has N days, which the test sets: the days left that morning are
//! N - 3. The staff begins no attack with fewer than 3 days left.
//!
//! # Statements
//!
//! 1. **A side reads the game's days.** With N = 6, each side's view
//!    gives 6 as the game's days, and a seat's view gives 6. No daily
//!    hash of the game differs from the same game before this rule.
//! 2. **Three days left: they go in.** N = 6 (6 - 3 = 3): on the morning
//!    of day 4 the units are set to attack, as scenario 109's statement
//!    3.
//! 3. **Two days left: they do not.** N = 5 (5 - 3 = 2): on the morning
//!    of day 4 they stay on defend. The reason gives the estimate, the
//!    5,086 fit men, the 3,444 wanted, that nothing more is coming, that
//!    2 days are left, and that the staff begins no attack so late. No
//!    notice is handed in.
//! 4. **Press too, and with the men enough.** N = 5 at press with an
//!    estimate of 20,000: they stay on defend. N = 5 at balanced with an
//!    estimate of 2,000, so that 5,086 is more than twice it: they stay
//!    on defend, with the same words. With no estimate held: the same.
//! 5. **Men already in.** N = 7: sent in on day 4 (4 left). On the
//!    morning of day 6 (2 left), their attack of day 5 having failed, the
//!    staff sets them to defend and says so. In a variant where the
//!    attack of day 5 did not fail (the defenders cut to 100 men), they
//!    stay on attack.
//! 6. **With three days left or more, as before:** N = 9, a unit still to
//!    come: the staff waits, 6,888 wanted, as scenario 109's statement 7.
//! 7. **A unit that comes after the game is not coming.** N = 6, a third
//!    unit in the take's forces limit that may be used from day 8: on
//!    the morning of day 4 nothing more is coming, and the units go in.
//!    With N = 9 the same unit is coming and the staff waits.
//! 8. **A game of one day.** N = 1 with men ashore from the start and no
//!    estimate: on its only morning 1 day is left, and no attack is
//!    begun.
//! 9. **In a trial.** A dry run of the take asked on the evening before
//!    day 4 of the game of 5 days plays its staffs as statement 3: no
//!    world of it has the units sent in. A dry run that plays past day 5
//!    does not fail, and its staffs begin no attack after it.
//!
//! # Properties
//!
//! - On every morning with 3 days left or more, every order and reason of
//!   the judgement ashore is what decision 0193 gives.
//! - With fewer than 3 days left no unit on defend is set to attack.
//! - The days left are never below 0.
//!
//! The tests are written by the build under these statements.
