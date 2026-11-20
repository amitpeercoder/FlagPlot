//! Scenario 111: forces that arrive. A task force with all aboard it, and
//! a land unit or an air group at a base, come onto the map on a day the
//! scenario names, and are not there before.
//!
//! Proves decision 0195. The statements are the architect's, written
//! before the code. A mismatch goes back to the architect, and nobody else
//! changes a statement or a number here.
//!
//! # The setting
//!
//! A placeholder scenario of six days made for the test, on the placeholder
//! map, with no real ship or unit. Side A has a base B with a land unit
//! and a search group, and a force already at sea. Side J has a base R.
//! J's force K, a carrier with a fighter group and a bomber group aboard
//! and two destroyers, has `enters = 3` at a sea hex E, 5 hexes from A's
//! force. J's transport force L, with a land unit U of 600 men and 40 t of
//! supply aboard, has `enters = 4` at R. J's land unit V, 300 men, and air
//! group G, 12 aircraft, have `enters = 3` at R.
//!
//! # Statements
//!
//! 1. **Not there before.** On days 1 and 2: K, its three ships and its
//!    two groups are in no hex. A's search of E's hex, flown at a chance
//!    of 1, sights nothing. Nothing of K is in the truth's forces, ships
//!    or groups that the day's rules walk. K's ships burn no fuel and its
//!    groups gain no fatigue: on entering, each has the fuel and the
//!    figures the scenario wrote.
//! 2. **It enters at the start of its day.** At the start of day 3's
//!    night pulse K is in E, stopped, with its ships and groups, and the
//!    event `ForceEntered` names day 3, E, the force, its three ships
//!    and its two groups. In that same pulse A's force and K may sight
//!    each other.
//! 3. **Who sees the event.** J sees it whole. A is given nothing of it.
//! 4. **A side knows what is coming.** On day 1, J's view lists K (day 3,
//!    E, a carrier and two destroyers, two groups), L (day 4, R, one
//!    unit, 600 men), V and G (day 3, R). A's view lists nothing of
//!    them. J's briefing has each under what is due.
//! 5. **Orders before the day are refused,** to K, to a ship of K's, to a
//!    group aboard, to U and to V, each with the words that it enters on
//!    day N. On the day it enters an order is taken.
//! 6. **An intent may name it.** J's "operate a carrier force" for K,
//!    given on day 1, is accepted. The staff gives K no order on days 1
//!    and 2 and says K enters on day 3; on the morning of day 3 it orders
//!    K as it would any carrier force.
//! 7. **A unit and a group at a base.** On day 3 V is ashore at R with
//!    300 men and G is at R with 12 aircraft; `UnitEntered` is recorded
//!    for each. On days 1 and 2 R feeds no man of V.
//! 8. **It waits for its base.** With R taken by A on day 2: V and G do
//!    not enter on day 3; `EntryWaiting` is recorded each night, to J;
//!    they enter on the night R is J's again.
//! 9. **A force enters whoever is there.** With A's force lying in E on
//!    day 3, K enters E, and the two forces are in one hex as two forces
//!    that met.
//! 10. **Men and cargo aboard come with it.** On day 4 L is at R with U's
//!     600 men and 40 t of supply aboard.
//! 11. **Its numbers are its own.** A task force formed in play on day 1
//!     and a land unit made by division on day 2 take no number of K's,
//!     L's, U's or V's.
//! 12. **With "may be used from".** K with `enters = 3` and
//!     `available_from = 5`: it is on the map from day 3 and refuses
//!     orders until day 5.
//! 13. **Refused scenarios.** `enters = 1`; `enters = 7` in a game of
//!     six days; `enters` on a ship, or on a group aboard a ship, or on a
//!     unit aboard one; a force entering on a land hex with no base of
//!     its side. Each with its reason.
//! 14. **Never entered, never counted.** A game of two days ends with
//!     K's carrier in no count of ships sunk, out of action or afloat.
//! 15. **Saved and loaded** on day 2, the game goes on the same, and the
//!     daily hashes are the same.
//! 16. **No `enters`, no change.** Scenario 5 and the 39 days' first week
//!     give the events they gave before.
//!
//! # Properties
//!
//! - A unit that has not entered appears in no event but its own entry.
//! - The same seed gives the same daily hashes twice.
//!
//! The tests are written by the build under these statements.
