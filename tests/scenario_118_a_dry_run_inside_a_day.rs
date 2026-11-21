//! Scenario 118: a dry run asked inside a day. A trial asked at 0600 or
//! at 1200 is answered, from the state as it stands at that hour taken as
//! if it were that evening.
//!
//! Proves decision 0204. The statements are the architect's, written
//! before the code. A mismatch goes back to the architect, and nobody else
//! changes a statement or a number here.
//!
//! # The setting
//!
//! A placeholder scenario made for the test, on a placeholder map of open
//! sea, with no real ship. It lets a seat stop the clock inside a day
//! (decision 0156) and gives the figures of a trial. Hexes are 40 nm.
//!
//! Side A's seat has a carrier force C, one carrier and two destroyers,
//! stopped at sea at (10,0) with every ship at 1,000 t of fuel. C cruises
//! at 15 kn; each ship's class has a capacity of 1,000 t and an endurance
//! of 4,000 nm, so it burns 0.25 t a mile at cruise. A has a base H at
//! (10,2). Side B has a base E at (-10,0) on A's map and nothing at sea.
//! Neither side has men ashore unless a statement says so.
//!
//! At the dusk before day 1 A's commander orders C himself to (0,0) at
//! cruise, and states order 1: "deny the waters within 2 hexes of (0,0)"
//! with its forces limit naming C. He gives no leave to launch. He then
//! lets the night pass and stops at 0600 of day 1.
//!
//! In the night C steams 180 nm along r = 0: it enters (9,0) after 20 nm,
//! (8,0) after 60, (7,0) after 100, (6,0) after 140 and (5,0) after 180,
//! at the very end of the night. At 0600 it is in (5,0), half-way along
//! the step into it, and each ship has burned 45 t and holds 955 t.
//!
//! # Statements: asked at 0600
//!
//! 1. **It is answered.** A trial of order 1, of the small size, asked at
//!    0600 of day 1, is answered and not refused.
//! 2. **How it was asked.** What was asked says `inside_day` is day 1 and
//!    the clock "0600". The first day played is day 2, and the day the
//!    course is handed in for is day 2.
//! 3. **From the state as it stands.** In every world of that trial, at
//!    its start, C is in (5,0) with 20 nm of its step made, on its route
//!    to (0,0), and each of its ships holds 955 t.
//! 4. **The rest of the day is not played.** The answer's words say that
//!    it was asked at 0600 of day 1 and that the rest of that day was not
//!    played. With no man of A's ashore they say nothing of what is
//!    ashore.
//! 5. **Ashore, a day behind.** With a land unit of A's ashore at H, the
//!    words also say that what is ashore is a day behind.
//! 6. **Described.** A trial of order 1 described at 0600 gives the first
//!    day as day 2 and the same `inside_day`. It does not give day 1.
//! 7. **At later dates.** A trial of order 1 at later dates, asked at
//!    0600, has day 2 as its first date.
//!
//! # Statements: asked at 1200
//!
//! In the morning C steams 90 nm more, 270 nm in all: it enters (4,0)
//! after 220 nm and (3,0) after 260. At 1200 it is in (3,0), 30 nm along
//! the step into it, and each ship holds 932.5 t.
//!
//! 8. **It is answered,** with `inside_day` day 1 and the clock "1200",
//!    the first day played day 2, and in every world C in (3,0) with 30
//!    nm of its step made and 932.5 t in each ship.
//!
//! # Statements: orders to units
//!
//! 9. **They are counted and not tried.** With one order to a unit given
//!    at 0600, for the morning, the answer counts one order to units not
//!    tried and its words say that orders given for the day's pulses, and
//!    the holds they make, are not in the trial. With none given the
//!    count is 0 and the words say nothing of it.
//!
//! # Statements: the same answer, and the list
//!
//! 10. **Asked twice at one hour.** The same trial asked twice at 0600
//!     gives the same answer but for its number.
//! 11. **Asked at dusk, as today.** The same trial asked at the dusk
//!     before day 1 says `inside_day` is nothing and gives the first day
//!     as day 1, as it does today.
//! 12. **The list of trials.** At 0600 the list says of statement 1's
//!     trial that it was asked at 0600 of day 1. At 1200 it counts that
//!     trial as one asked earlier, as it counts one asked on an earlier
//!     evening today.
//!
//! # Statements: contacts
//!
//! 13. **Ages in hours, as they hold.** A contact one pulse old at 0600
//!     was reported in the afternoon and has aged through the night: it
//!     is 12 hours old. The trial reads it as 6 hours old, the age of a
//!     contact one pulse old at dusk. A contact reported in the night
//!     just resolved is read as fresh at both.
//!
//! # Properties
//!
//! - **The game is as it was.** A game in which trials are asked at 0600
//!   and at 1200 of day 1 gives, when the day is resolved, the events and
//!   the hash of the same game with no trial asked.
//! - **No game on file moves.** No event and no hash of any game changes;
//!   the recorded transcripts change in the interface's version alone.
//! - **A state never built from before.** On the 39 days of the full
//!   slice, a trial asked at 0600 and at 1200 of a day on which men are
//!   ashore and a bombardment's hits wait is answered and not refused.
//! - **The staff reads its own side.** Nothing in the answer comes from
//!   the enemy's truth.

// The architect's statement lists are list items in the doc comment above,
// which is left as written.
#![allow(clippy::doc_lazy_continuation)]
