//! Scenario 109: men ashore with no more coming. Men landed to take a
//! place, too few for what the risk asks and with nothing more to send,
//! attack, or wait and say so, by the risk.
//!
//! Proves decision 0193. The arithmetic is the architect's, written
//! before the code. A mismatch goes back to the architect, and nobody else
//! changes a statement or a number here.
//!
//! # The sums
//!
//! fit men of a unit = men ashore x (100 - fatigue) / 100, cut to a whole
//! man. Men wanted = the estimate x 3, 2 or 1 by the risk while more is
//! coming; x 3, 1 or 0 with nothing more coming.
//!
//! # The setting
//!
//! A placeholder scenario made for the test, on the placeholder map, with
//! no real unit. Side J has taken a transport force to a coast hex P that
//! side A holds, under "take the place P", and landed two units: U, 3,000
//! men at fatigue 10, and V, 2,386 men at fatigue 0. Both can attack.
//! J's estimate of the enemy's men at P, made by its men in contact, is
//! 3,444. The transports lie empty. No unit of the take is left behind
//! at the source.
//!
//! Fit men: U, 3,000 x 90 / 100 = 2,700; V, 2,386. Together 5,086.
//!
//! # Statements
//!
//! 1. **Today, at balanced:** men wanted 3,444 x 2 = 6,888; 5,086 is
//!    fewer; the staff waits. (Before this rule it waited every
//!    morning.)
//! 2. **Nothing more is coming** on that morning, and the reason says
//!    what was looked for: no unit left behind, no force of the take
//!    with fighting men aboard, no unit waiting for its men.
//! 3. **Balanced, with nothing more coming:** men wanted 3,444 x 1 =
//!    3,444; 5,086 is at least that; U and V are set to attack. The
//!    reason gives the estimate and its day, 3,444 wanted, 2,700 and
//!    2,386, 5,086, and that nothing more is coming.
//! 4. **Balanced, too few even so.** The estimate is 5,200: 5,086 is
//!    fewer. The staff waits and hands in the notice `cannot_take`, with
//!    the estimate, the 5,086, the 5,200 wanted and that nothing more is
//!    coming. It hands it in again the next morning.
//! 5. **Press, with nothing more coming:** U and V are set to attack with
//!    an estimate of 5,200, and with one of 20,000.
//! 6. **Preserve the force waits:** men wanted 3,444 x 3 = 10,332; it
//!    waits and hands in `cannot_take`.
//! 7. **While more is coming, as today.** A third unit, W, 2,000 men, is
//!    ashore at the source and the transports lie there empty: a second
//!    lift is to be sent. At balanced the men wanted are 6,888 and the
//!    staff waits, without the notice.
//! 8. **A lift at sea is more coming.** W is aboard the transports under
//!    way to P: the men wanted are 6,888 and the staff waits.
//! 9. **A lift turned back is more coming** (decision 0191): W aboard,
//!    its route ending at the source: as statement 8.
//! 10. **A unit that cannot attack is not counted.** U's fatigue is above
//!     the balanced mark. The fit men that can attack are V's 2,386,
//!     fewer than 3,444: the staff waits and hands in the notice.
//! 11. **No estimate, as today:** the units attack and the reason says it
//!     attacks with no count of the enemy.
//! 12. **The commander is told.** With the standing interrupt for the
//!     notice on, a seat's day stops on the morning of statement 4.
//!     Setting press on the intent has the units attack the next morning.
//! 13. **Truth and belief.** With the defenders doubled in truth and J's
//!     estimate left at 3,444, statement 3's orders are the same.
//!
//! # Properties
//!
//! - While more is coming, every order and reason of the judgement ashore
//!   is what it was before this rule.
//! - At preserve the force no unit is ever set to attack by this rule
//!   that would not have been before it.
//! - The men wanted with nothing more coming are never more than while
//!   more is coming.
//!
//! The tests are written by the build under these statements.
