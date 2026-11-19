//! Scenario 90: a take that does not go to starve. The commander says
//! "control the island". The side's men are at sea in transports, fewer
//! than the staff wants against the enemy it has been told is there, with
//! no more in reach and not the supply aboard to last until a run could
//! reach them. At balanced the staff does not lift them: it hands in
//! "cannot bring enough" and waits.
//!
//! Proves decision 0169, the second mend of step 3's stage 4, against a
//! hand calculation. It is scenario 86 with two changes: the Allied side
//! is told of 400 enemy men at B, and ship 1 has 10 t of supply aboard
//! where it had 30 t. The data is placeholder data: see
//! `data/scenarios/90_a_take_that_does_not_go_to_starve.toml`. It runs on
//! the frozen step 1 figures. No enemy man is ashore anywhere, so no die
//! is rolled.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Figures
//!
//! As scenario 86's. A man eats 0.005 t a day. A hex is 40 nm and the
//! transports make 10 kn. At balanced the staff wants 2 men for each of
//! the enemy's it holds an estimate of; the margin of the check that a
//! place can be held is 5 days; a supply intent with no interval uses 15
//! days; with no latest day the staff looks 7 days ahead.
//!
//! ## Inputs
//!
//! Scenario 86's map, bases, task force, ships, units, island and
//! commands, with:
//!
//! - an estimate held by the Allied side from the start: 400 enemy men
//!   at B, (4,0);
//! - ship 1 with 10 t of supply aboard. Ship 2 has none.
//!
//! So: base 1 at A, (0,0), Allied, 1,000 t of supply; base 2 at B,
//! Japanese, with nothing at it; base 4 at C, (0,6), Allied, with
//! nothing. Task force 1, Allied, on the transport mission, ships 1 and
//! 2, at sea at (2,0), stopped, its home port base 4; unit 1, fighting,
//! 600 men, in ship 1; unit 2, working, 400 men, in ship 2. Intent 1,
//! control island 1, on day 1; no risk set, so balanced; leave to
//! prepare and to launch both given. The game is 8 days.
//!
//! ## The count, on the morning of day 1
//!
//! - **The men wanted:** 400 x 2 = 800.
//! - **The lift:** task force 1, a lift as it stands. Its fighting men
//!   are unit 1's 600. No man of the operation is ashore at B.
//!   600 < 800: the lift is short, by 200.
//! - **The men in reach:** no latest day is set, so the staff reckons to
//!   day 1 + 7 = day 8. No other transport force has a fighting unit
//!   aboard. The men in reach are the 600: too few.
//!
//! ## Whether they can be fed
//!
//! The source is base 1, named on day 1 as in scenario 86, and the hold
//! of base 2 has it.
//!
//! - **The check that the place can be held** (decision 0144), for the
//!   1,000 men the lift lands: the daily need is 1,000 x 0.005 = 5 t.
//!   The days until supply follows are 1 to land the men, 1 back to base
//!   1 (160 nm at 10 kn = 16 hours), 1 to load and 1 out: 4 days. The
//!   supply that lands with the men is 5 x (4 + 5) = 45 t and the first
//!   run 5 x 15 = 75 t: 120 t. Base 1 holds 1,000 t above its reserve
//!   and task force 1 has 6,000 t of room. Nothing is short: it can be
//!   held.
//! - **What is aboard against the days until supply follows:** 5 t x 4
//!   days = 20 t. The force lies away from its source and loads nothing:
//!   it has 10 t. 10 < 20, short by 10 t. The men cannot be fed.
//!
//! ## Day by day
//!
//! | Day | The staff | What happens |
//! | --- | --- | --- |
//! | 1 | Forms no landing under the take and gives task force 1 no order. Hands in the notice "cannot bring enough" for the take's plan | The standing interrupt "cannot bring enough" fires. Task force 1 lies at (2,0) |
//! | 2 to 8 | Each morning the same reckoning, with the day reckoned to moved on by one (day 9 on day 2, and so to day 15 on day 8). No landing, no order, no second notice | Nothing |
//!
//! The reason of each morning under the take gives: 800 men wanted, 600
//! in the lift, 600 in reach, the day reckoned to, and 10 t aboard
//! against the 20 t of the 4 days until supply follows.
//!
//! - **No staff order is given in the whole game,** to task force 1 or
//!   to a unit. None is refused.
//! - **One notice "cannot bring enough"** is handed in, on day 1, and
//!   none after.
//! - **At the end:** task force 1 is at (2,0) with unit 1's 600 men and
//!   unit 2's 400 aboard, none ashore. Base 2 is Japanese. No base has
//!   been formed at B. Base 1 holds 1,000 t and base 4 nothing.
//!
//! As the rule stood before decision 0169 the lift sailed on day 1, the
//! staff saying "at balanced the staff goes with what it has", and 1,000
//! men were ashore at B with 10 t, two days' food, and the first run due
//! on day 5.
//!
//! # Properties
//!
//! Each is the main case with the change named.
//!
//! - **Fed: it goes.** With ship 1's 30 t aboard, as in scenario 86:
//!   30 t is not less than 20 t, the place can be held, and the men can
//!   be fed. The lift is short by the same 200 men and goes: on day 1
//!   task force 1 is given its landing order with unit 1, its route to
//!   B and on to base 1, and its unload order, as in scenario 86. No
//!   notice "cannot bring enough" is handed in. The reason of the
//!   landing says it goes short and that the men can be fed.
//! - **The source too poor.** With 30 t aboard and base 1 holding
//!   100 t: base 1 is still the source, and the check finds 100 t
//!   against the 120 t wanted, short by 20 t. The place cannot be held,
//!   so the men cannot be fed: no landing, no order, one notice on day
//!   1, as in the main case.
//! - **No count.** With no estimate: nothing is wanted, nothing is
//!   short, and the lift goes on day 1 with its 10 t, as it did before
//!   the rule.
//! - **Not short.** With the estimate 300: 600 men are wanted and 600
//!   are in the lift. The lift goes on day 1 with its 10 t, and no
//!   notice "cannot bring enough" is handed in.
//! - **At press it goes.** With the risk of intent 1 set to press on day
//!   1: 400 men are wanted, the lift is not short, and it goes on day 1.
//!   With the estimate 800 as well: 800 are wanted, the lift is short
//!   by 200 and cannot be fed, and at press it goes on day 1 all the
//!   same, with no notice "cannot bring enough".
//! - **At preserve it is as it was.** With the risk of intent 1 set to
//!   preserve on day 1: 1,200 men are wanted. The take forms no landing
//!   and gives no order, as before the rule, and hands in no notice
//!   "cannot bring enough": what it hands in is what it handed in
//!   before the rule.
//! - **The commander says go.** With the risk of intent 1 set to press
//!   on day 3, and nothing else changed: on days 1 and 2 the main case;
//!   on the morning of day 3 task force 1 is given its landing order
//!   with unit 1.
//! - **More men in reach.** With a second transport force, task force 2,
//!   of one placeholder troop transport at (2,0) with a fighting unit of
//!   200 men aboard and no supply, which may be used from day 5: on day
//!   1 the men in reach are 600 + 200 = 800, the number wanted, by day
//!   5, inside day 8. The lift of task force 1 goes on day 1 with its
//!   10 t, and no notice "cannot bring enough" is handed in. Only day 1
//!   is asserted.
//! - **Out of reach.** As "More men in reach", with task force 2 usable
//!   from day 10: on days 1 and 2 the men in reach are 600 (day 10 is
//!   after day 8 and after day 9), and the take does not lift, with the
//!   one notice on day 1. On the morning of day 3 the staff reckons to
//!   day 10: the men in reach are 800, and task force 1 is given its
//!   landing order. Only days 1 to 3 are asserted.
//! - **Scenario 86 is as it was.** Scenario 86's own test passes
//!   unchanged: it holds no estimate.
//! - **The same start gives the same game,** and a game saved and
//!   replayed gives the same.

// The tests are written by the build, to the statements above.
