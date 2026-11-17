//! Scenario 65: a day from which a force may be used. A carrier force
//! that its side may use from day 2 is left alone on day 1 and played from
//! day 2 exactly as it would have been played from day 1.
//!
//! Proves decision 0140, step 1 of stage 3, against a hand calculation.
//! The data is placeholder data: `data/scenarios/65_a_day_from_which_a_force_may_be_used.toml`
//! is scenario 58's file with no contact, a game of 4 days, intent 1
//! "operate task force 1 from (6,0)" with no risk set, so balanced, and
//! task force 1 available from day 2. It runs on the frozen step 1
//! figures as scenario 58 does. There is no Japanese unit, so no die is
//! rolled.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## The game without the day
//!
//! "The same file without the day" is this scenario with task force 1's
//! day taken out. By scenario 58's property "speed alone", at balanced
//! she is ordered to full speed on day 1 and arrives at (6,0) at hour 8.
//! Call the orders the staff gives on its days 1, 2 and 3 the first,
//! second and third days' orders.
//!
//! ## The game with the day
//!
//! - **Day 1.** Task force 1 may not be used. The staff gives no order to
//!   her or to groups 1, 2 and 3, and hands in one reason: that intent 1
//!   is to be served by task force 1, which may be used from day 2. She
//!   lies stopped at (0,0) through day 1 and burns no fuel under way.
//! - **Day 2.** The staff gives the first day's orders. She arrives at
//!   (6,0) at hour 32: hour 8 and a day.
//! - **Day 3.** The second day's orders.
//! - **Day 4.** The third day's orders.
//!
//! Every order is the same order, to the same unit, with the same reason
//! but for the day it names; the count on each day is the count a day
//! earlier in the game without the day.
//!
//! # Properties
//!
//! - **An order before the day is refused.** A commander's order on day 1
//!   that routes task force 1 is refused with the day, 2, and changes
//!   nothing. The same order on day 2 is taken.
//! - **A group's own day.** With task force 1 available from day 1 and
//!   group 3, the dive bombers, from day 3: on days 1 and 2 the staff
//!   gives group 3 no order and every other order is as in the game
//!   without the day; on day 3 it gives group 3 the orders it gives it on
//!   day 1 in that game. A commander's order to group 3 on day 2 is
//!   refused with the day, 3.
//! - **The later of two days.** With task force 1 from day 2 and ship 1
//!   from day 3: nothing is ordered before day 3, and the reason names
//!   day 3. With task force 1 from day 3 and group 3 from day 2: group 3
//!   is given no order before day 3.
//! - **A land unit.** In a variant with a land unit ashore at base 1,
//!   available from day 3: a commander's `set_posture` on day 2 is refused
//!   with the day, and on day 3 is taken.
//! - **A ship in a pool.** In scenario 36, keeping a base supplied, with
//!   the second ship of the pool available from day 5: the staff commits
//!   it to no run and loads nothing on it before day 5, and where a reason
//!   lists the pool it names the ship with its day.
//! - **A side knows its own days and not the enemy's.** The Allied side's
//!   view gives task force 1's day. Two worlds that differ only in the day
//!   of a Japanese force give the Allied side the same view, the same
//!   orders and the same reasons.
//! - **No day, no change.** Every scenario built before this one gives the
//!   events it gave.
//! - **Formations.** In a variant with two land units in "Placeholder
//!   Battalion 1" and "Placeholder Battalion 2", both under "Placeholder
//!   Regiment", under "Placeholder Division": the side's view gives each
//!   unit's formation and each formation's parent, to the division, which
//!   has none. The loader refuses a parent that is not there and a
//!   formation that is its own ancestor. No order and no event differs
//!   from the same file without the formations.
//! - **The staff's setup of the 39 days** loads, runs its 39 days with no
//!   order from anyone and none refused, and gives the events that
//!   `data/scenarios/07_full_slice.toml` gives with its orders taken out.
//!   Every unit with a day in it has the day of its first scripted order
//!   in the scripted file, or the day the architect ruled.
//! - The same start gives the same game.
