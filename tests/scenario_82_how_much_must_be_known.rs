//! Scenario 82: how much must be known. The commander asks for an
//! airfield at balanced risk and says that the count of the enemy there
//! must be no more than a day old when the landing goes. The staff looks,
//! readies one unit on what it sees, finds its count a day too old on the
//! morning the landing is ready, looks again, and goes the day after. The
//! plan says each morning what must be known, what is held and who is
//! looking.
//!
//! Proves decision 0159, rule D, items 12 and 13, step 4 of the opening
//! of stage 4, against a hand calculation. The data is placeholder data:
//! see `data/scenarios/82_how_much_must_be_known.toml`. It is scenario
//! 72's start, and runs on scenario 72's figures: the frozen step 1
//! figures with `preparation_per_day` 100 and the four error figures at
//! 1, so a report gives the true count and no die changes anything. The
//! game ends before the landing.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly, except the supply and the fatigue the flights cost, which
//! are the search rule's. If the code disagrees, the code or the rule is
//! wrong: do not change a number here to make a test pass. A mismatch
//! goes back to the architect.
//!
//! ## The limit
//!
//! "Knowledge" is a limit on a "take a place" or a "control an island"
//! of the commander's, read up the tree as the other limits are, and
//! refused on any other kind. It has three settings:
//!
//! - **none:** the landing may go on any count or on none;
//! - **a count:** on a count seen from the air or by men in contact, of
//!   any day;
//! - **a recent count:** on such a count no more than a number of days
//!   old on the morning it goes, 3 where the commander gives no number.
//!
//! Where it is not set, the risk sets it as before this decision: press
//! none, balanced a count, preserve a recent count of 3 days. Where it is
//! set, the risk no longer says how much must be known, and still says
//! how many men are wanted for each of the enemy's.
//!
//! ## The need, as data
//!
//! With each plan of a take the staff hands in what must be known: the
//! place; what is asked (the setting and its days); what is held (the
//! count, where it came from, its day and its age in days, or nothing);
//! what the count held could be in truth, from its grade's range in the
//! figures, the lowest cut and the highest raised to a whole man (none
//! for a count that was told); which group is looking and since which
//! day, or none; and whether the need is met.
//!
//! ## Inputs
//!
//! As scenario 72's: the map of scenario 27, A at (0,0) and B at (4,0);
//! base 1, Allied, at A, with an airfield and 1,000 t; base 2, Japanese,
//! at B, with 100 t; air group 1, Allied, at base 1, 6 ready aircraft
//! that can search 4 hexes or more, at rest; task force 1, two
//! placeholder troop transports of 600 berths, stopped at base 1; units 1
//! and 2, Allied, 600 men each at A; unit 10, Japanese, 150 men at B. No
//! estimate is held. The game is 5 days.
//!
//! The Allied side's commands of day 1: intent 1, take the place at base
//! 2, with a forces limit of task force 1, units 1 and 2, air group 1
//! and base 1 as the source; no risk, so balanced; and the limit
//! "knowledge: a recent count, 1 day" on intent 1. Leave to prepare and
//! leave to launch are both given.
//!
//! ## Day by day
//!
//! | Day | The need that morning | The staff's commands and orders | What happens |
//! | --- | --- | --- | --- |
//! | 1 | Asked: a count no more than 1 day old. Held: nothing. Looking: air group 1, since day 1. Not met | Approves plan 1 to preparing. Forms intent 2, ready a landing by task force 1 at B, with a forces limit of units 1 and 2. Orders `prepare_landing` at B, roster 1 and 2. Sets air group 1 to reconnaissance of B | In the morning one aircraft reports 150 men, seen from the air. Loading 100; each unit's rehearsal 100 |
//! | 2 | Held: 150, seen from the air, of day 1, 1 day old; it could be 150 to 150. Looking: none. Met | Sets the forces limit of intent 2 to unit 1. Orders `prepare_landing` at B, roster 1. Sets air group 1 to rest | The landing was ordered again this morning and is not ready this morning |
//! | 3 | Held: the same, 2 days old. Looking: air group 1, since day 3. Not met | Notice: intent 2 is ready. The plan is not released: the reason says the count is 2 days old and one no more than 1 day old is asked. Sets air group 1 to reconnaissance of B | In the morning one aircraft reports 150 men |
//! | 4 | Held: 150, seen from the air, of day 3, 1 day old. Looking: none. Met | Approves plan 1 to released. Orders a load of supply to a weight of 45 t. Sets air group 1 to rest | |
//! | 5 | The same, 2 days old: the plan is released, and the need is no longer asked | Unit 1 set to attack. Load unit 1 | |
//!
//! - **Day 1.** With no estimate the staff takes every unit the ships can
//!   carry and says it goes with no count.
//! - **Day 2.** At balanced the men wanted are 150 x 2 = 300. Unit 1 has
//!   600 and is enough; unit 2 is left out.
//! - **Day 3.** The second count is the same, so the choice of units is
//!   not made again.
//! - **Day 4.** The supply is 600 x 0.005 x 15 = 45 t.
//!
//! Nine orders in all: two on each of days 1, 2, 4 and 5 and one on day
//! 3. Air group 1 flies twice, on days 1 and 3. No staff order or command
//! is refused. Beside scenario 72's balanced case, which goes on day 3
//! after one flight, the fresher count costs a day and a flight.
//!
//! # Properties
//!
//! - **Not set.** With no knowledge limit: the game of scenario 72's
//!   "Balanced" case. The plan is released on day 3 and air group 1 flies
//!   once. Days 1 to 3 are asserted.
//! - **A count, at preserve the force.** With the risk preserve and
//!   "knowledge: a count": the men wanted on day 2 are 150 x 3 = 450,
//!   unit 1 alone; the plan is released on day 3 on the count of day 1,
//!   2 days old, and it would be released on that count at any age. Days
//!   1 to 3 are asserted.
//! - **None, at preserve the force.** With the risk preserve and
//!   "knowledge: none": no group is sent on any day. The landing is ready
//!   on the morning of day 2 with units 1 and 2: the plan is released,
//!   the reason says it goes on no count, and the load is to 90 t. Days 1
//!   and 2 are asserted.
//! - **Told is not a count.** With an opening estimate of 100 men at B,
//!   told, and "knowledge: a count": on day 1 the need is not met, what
//!   is held is 100, told, with no range, and air group 1 is sent.
//! - **The range.** In the staff's own tests, on the decision's figures:
//!   a count of 3,000 seen from the air, with the range 0.25 to 2, could
//!   be 1,500 to 12,000; by men in contact, with 0.5 to 1.5, 2,000 to
//!   6,000; a count of 100 with the range 0.25 to 2 could be 50 to 400;
//!   a count of 7 with 0.5 to 1.5 could be 4 to 14.
//! - **Refused at intake.** The limit on a "hold" is refused. A recent
//!   count of 0 days is refused.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see and has not been told give the same
//!   orders, reasons and needs; a save on day 3 and a resume give the
//!   same game; with the limit not set, scenarios 67 to 81 give the
//!   orders they gave.
