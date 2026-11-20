//! Scenario 113: men who replace the lost. A base with a stream of
//! replacement men brings the units ashore there back toward their
//! establishment, and their experience down toward a recruit's.
//!
//! Proves decision 0197. The arithmetic is the architect's, written
//! before the code. A mismatch goes back to the architect, and nobody else
//! changes a statement or a number here.
//!
//! # The rule of experience
//!
//! Experience is a whole level, 0 to 4; 2 is the standard and the level
//! of replacements. A unit above 2 loses one level on the night the men
//! who have joined it since its level last changed reach half its
//! establishment, and the count begins again.
//!
//! # The setting
//!
//! A placeholder scenario of eight days made for the test. Side J has a
//! base R with ample supply and a stream of 100 men a day from day 2. At
//! R, ashore, on defend, with no enemy in the hex: unit 1, 400 men, an
//! establishment of 600 by the scenario's key, experience 4; unit 2, 500
//! men with no key, so an establishment of 500, experience 2; unit 3,
//! 250 men, an establishment of 300, experience 3. A day's night comes
//! first; the fight ashore is at the day's end.
//!
//! # Statements
//!
//! 1. **Nothing on day 1.** At the end of day 1 the units have 400, 500
//!    and 250 men.
//! 2. **The lowest number first, up to its establishment.** Night of day
//!    2: unit 1 takes 100 and has 500. Night of day 3: unit 1 takes 100
//!    and has 600. Night of day 4: unit 1 is full and unit 2 is full;
//!    unit 3 takes 50 and has 300, and the other 50 are lost. Night of
//!    day 5: no unit is below its establishment; nobody joins and no
//!    event is recorded.
//! 3. **Fewer than half: no change.** Unit 1 has had 200 join by the
//!    night of day 3, fewer than 300, half of 600: experience 4. Unit 3
//!    has had 50 join, fewer than 150: experience 3.
//! 3a. **Half its establishment: one level.** With unit 1 beginning at
//!    200 men: 100 join on each of the nights of days 2, 3 and 4. On the
//!    night of day 4 the count is 300, half of 600: its experience falls
//!    from 4 to 3 and the count begins again. On the night of day 5 it
//!    takes 100 and has 600, with a count of 100: experience 3.
//! 3b. **Never below the standard, and never raised.** A unit at
//!    experience 2 that takes 400 men against an establishment of 500
//!    stays at 2. A unit at 1 that takes the same stays at 1.
//! 4. **The event.** `MenJoined` for unit 1 on day 2 gives 100 men, 500
//!    after, experience 4 before and after, and the count, 100. On the
//!    night of statement 3a's fall it gives 4 before and 3 after. J sees
//!    it whole; the other side is given nothing.
//! 5. **Not to a unit in a fight.** With an enemy unit in R's hex and
//!    unit 1 on attack on the night of day 2: it takes none that night,
//!    and unit 3, the next below its establishment, takes 50 and has
//!    300; 50 are lost. With the enemy attacking on day 1 and unit 1 on
//!    defend: unit 1 fought on day 1 and takes none on the night of day
//!    2; with no fight on day 2 it takes 100 on the night of day 3.
//! 5a. **Bombs are not a fight.** Unit 1 bombed on day 1 and not fought:
//!    it takes 100 on the night of day 2.
//! 6. **Not to a unit elsewhere, aboard, on a march or with no men.**
//!    Unit 1 in the next hex, or wholly aboard a ship at R, or marching,
//!    or destroyed: it takes none.
//! 6a. **Men aboard count toward its establishment.** Unit 1 with 400
//!    ashore at R and 150 aboard a ship there: it is 50 short and takes
//!    50, not 200.
//! 7. **Not when the base is not its side's.** With R taken at the end of
//!    day 2: men join on the night of day 2 and none from the night of
//!    day 3.
//! 8. **A unit divided.** Unit 1 with 400 men ashore at R and 200 aboard
//!    a ship, an establishment of 600: the 200 land in another hex and
//!    become a new unit. The new unit's establishment is 200 and unit
//!    1's is 400.
//! 9. **They are fed.** On the night of day 2 unit 1 draws for 500 men:
//!    500 x 0.005 = 2.5 t.
//! 10. **Refused scenarios:** an establishment below a unit's men; a
//!     stream of no men; a stream to a base the scenario has not.
//! 11. **No stream, no change.** The 39 days' first week gives the events
//!     it gave before.
//!
//! # Properties
//!
//! - A unit's men never pass its establishment by this rule.
//! - A unit's experience is never raised by replacements, never lowered
//!   below 2 by them, and never lowered by more than one level in a
//!   night.
//! - The men who join in a night are never more than the stream gives.
//!
//! The tests are written by the build under these statements.
