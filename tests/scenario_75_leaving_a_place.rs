//! Scenario 75: leaving a place. The commander gives up a base. The
//! staff flies its fighters out, sends two transports, takes the
//! builders off first and the infantry after, and brings them home.
//!
//! Proves decision 0150, step 8 of stage 3, first part, against a hand
//! calculation. The data is placeholder data: see
//! `data/scenarios/75_leaving_a_place.toml`. It runs on the frozen step 1
//! figures. There is no enemy unit, so no die is rolled.
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
//! A man weighs 0.25 t to load or land and eats 0.005 t a day, paid at
//! the start of the night pulse. A size 5 port works at 150 t an hour on
//! a berth; a size 0 port by lighter at 20 t an hour for each ship. Day d
//! begins at hour 24 x (d - 1): a night pulse of 12 hours and two
//! daylight pulses of 6. In a pulse ships move before cargo is worked. A
//! group's transfer range is twice the longer of its type's search range
//! and strike radius.
//!
//! ## Inputs
//!
//! The placeholder map of scenario 27: A at (0,0), port size 5, and B at
//! (4,0), port size 0, 160 nm apart: 16 hours at 10 kn. The game is 5
//! days.
//!
//! Base 1, Allied, at A, with an airfield and 1,000 t of supply. Base 2,
//! Allied, at B, with an airfield and 100 t of supply.
//!
//! Air group 1, Allied, at base 2: 8 ready fighters of a placeholder type
//! of the frozen figures whose transfer range is 160 nm or more, at rest.
//!
//! Task force 1, Allied, on the transport mission: ships 1 and 2,
//! placeholder troop transports of 600 berths, stopped at base 1,
//! nothing aboard.
//!
//! | Unit | Kind | Where | Men |
//! | ---- | ---- | ----- | --- |
//! | 1, "Placeholder Infantry" | fighting | B | 600 |
//! | 2, "Placeholder Builders" | working | B | 400 |
//!
//! The Allied side's commands of day 1: intent 1, leave base 2, with a
//! forces limit of task force 1 and base 1 as the source; no date. Leave
//! to prepare and leave to launch are both given.
//!
//! ## Day by day
//!
//! | Day | Hour | The staff's commands and orders | What happens |
//! | --- | --- | --- | --- |
//! | 1 | 0 | Approves plan 1 to preparing and to released. Air group 1: rebase to base 1. Task force 1: a route to B; load unit 2 | The group is at base 1. The force sails at hour 0 and arrives at hour 16. Ship 1 takes 160 men in the morning and 240 in the afternoon: unit 2 is aboard at hour 21 |
//! | 2 | 24 | Load unit 1 | Ship 2 takes 600 men, 150 t, in 7.5 hours: unit 1 is aboard at hour 31 and 30 minutes |
//! | 3 | 48 | A route to A and an unload order. Marks plan 1 complete. Notice: intent 1 is met | Sails at hour 48, arrives at hour 64. Ship 1 lands 100 t and ship 2 150 t, each on a berth, by hour 65 |
//! | 4, 5 | | none | |
//!
//! - **Day 1.** Unit 2, the working unit, goes first. It takes ship 1.
//! - **Day 2.** The berths not yet filled are ship 2's 600, which hold
//!   unit 1.
//! - **Day 3.** No man of the side is ashore at B and no group is based
//!   there: the intent is met. The force is routed home first.
//!
//! Six orders in all and three staff commands. No staff order is refused.
//!
//! Base 2 pays 5 t for 1,000 men on the night of day 1 and 3 t for 600
//! men on the night of day 2: 92 t are left there, and the reason of day
//! 3 says so. Base 1 pays 5 t on the nights of days 4 and 5 for the
//! 1,000 men landed there: 990 t at the end.
//!
//! # Properties
//!
//! - **Two trips.** With ship 2 left out, so that the force has 600
//!   berths: on day 1 the force is routed to B with a load order for unit
//!   2. On the morning of day 2 the free berths are 200 and unit 1 has
//!   600 men: it waits. The force is routed to A with an unload order,
//!   sails at hour 24, arrives at hour 40 and lands unit 2. On day 3 it
//!   is routed to B with a load order for unit 1, arrives at hour 64 and
//!   has it aboard at hour 71 and 30 minutes. On day 4 it is routed to A
//!   with an unload order and the plan is marked complete. Nine orders in
//!   all.
//! - **Without leave to launch.** With leave to launch withheld: the plan
//!   is approved to preparing, no order is given on any day, and the
//!   notice that plan 1 waits for leave to launch is handed in each
//!   morning.
//! - **A hold.** With unit 1 held from day 1: unit 2 is taken off as in
//!   the main case, the force is routed home on day 2, and on the morning
//!   of day 2 the intent is met without unit 1, which the reason names:
//!   five orders in all.
//! - **A group that cannot fly out.** With base 1 with no airfield: air
//!   group 1 is given no order on any day, each morning's reason says the
//!   source has no airfield, and the plan is marked complete on day 3
//!   with the group named as left.
//! - **Only what is named.** With unit 2 alone named in the forces limit:
//!   unit 1 and air group 1 are given no order, and the intent is met on
//!   the morning of day 2.
//! - **A date it cannot make.** With a date of day 1: on the morning of
//!   day 1 the staff says the last men will be off on day 2 and hands in
//!   the notice that it cannot meet intent 1, one day late.
//! - The same start gives the same orders; a save on day 2 and a resume
//!   give the same game; every scenario built before this one gives the
//!   events it gave.
