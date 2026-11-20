//! Scenario 113: men who replace the lost. A base with a stream of
//! replacement men brings the units ashore there back toward their
//! establishment, and their experience down toward a recruit's.
//!
//! Proves decision 0197. The arithmetic is the architect's, written
//! before the code. A mismatch goes back to the architect, and nobody else
//! changes a statement or a number here.
//!
//! # The sums
//!
//! experience after = (experience before x men before + recruit's
//! experience x men joined) / men after, an exact fraction. The recruit's
//! experience is 1.
//!
//! # The setting
//!
//! A placeholder scenario of eight days made for the test. Side J has a
//! base R with ample supply and a stream of 100 men a day from day 2. At
//! R, ashore, on defend, with no enemy in the hex: unit 1, 400 men, an
//! establishment of 600 by the scenario's key, experience 4; unit 2, 500
//! men with no key, so an establishment of 500; unit 3, 250 men, an
//! establishment of 300, experience 3.
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
//! 3. **Experience falls.** Unit 1 after day 2: (4 x 400 + 1 x 100) /
//!    500 = 1,700 / 500 = 3.4. After day 3: (3.4 x 500 + 1 x 100) / 600 =
//!    1,800 / 600 = 3. Unit 3 after day 4: (3 x 250 + 1 x 50) / 300 =
//!    800 / 300 = 2.666..., held as the fraction 8/3. The rules read
//!    each cut to a whole level: 3, 3 and 2.
//! 4. **The event.** `MenJoined` for unit 1 on day 2 gives 100 men, 500
//!    after, experience 4 before and 3.4 after. J sees it whole; the
//!    other side is given nothing.
//! 5. **Not to a unit in a fight.** With unit 1 on attack on day 2: it
//!    takes none that night, and unit 3, the next below its
//!    establishment, takes 50 and has 300; 50 are lost. With unit 1
//!    attacked on day 2 and on defend: it takes none that night.
//! 6. **Not to a unit elsewhere, aboard, or with no men.** Unit 1 in the
//!    next hex, or aboard a ship at R, or destroyed: it takes none.
//! 7. **Not when the base is not its side's.** With R taken on day 3: no
//!    man joins from that night.
//! 8. **A unit divided.** Unit 1, 600 men with an establishment of 600,
//!    is divided so that a new unit takes 200: the new unit's
//!    establishment is 200 and unit 1's is 400.
//! 9. **They are fed.** On the night of day 2 unit 1 draws for 500 men:
//!    500 x 0.005 = 2.5 t.
//! 10. **Refused scenarios:** an establishment below a unit's men; a
//!     stream of no men; a stream to a base the scenario has not.
//! 11. **No stream, no change.** The 39 days' first week gives the events
//!     it gave before, and every unit's experience reads as the whole
//!     level it was.
//!
//! # Properties
//!
//! - A unit's men never pass its establishment by this rule.
//! - A unit's experience after replacements lies between the recruit's
//!   and its experience before.
//! - The men who join in a night are never more than the stream gives.
//!
//! The tests are written by the build under these statements.
