//! Scenario 91: a world built from a picture. A picture holds everything a
//! world needs on an evening; `World::from_picture` builds a world from
//! it, at that day, with the belief, the orders, the command state and the
//! counters the picture gives.
//!
//! Proves decision 0170, step 3 of step 3's stage 4, against a hand
//! calculation. It has no scenario file of its own: its pictures are made
//! in the test, from the data of scenario 1 and from live games of
//! scenarios 1 and 86. It runs on the frozen step 1 figures.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## 1. By hand: scenario 1's force, from a picture of day 4
//!
//! The picture, made in the test from scenario 1's map, reference data,
//! bases and ships, with nothing taken from a game that was played:
//!
//! - **The day:** 4.
//! - **The state:** base 1, Noumea, at (0,0) and base 2, Espiritu Santo,
//!   at (8,3), both Allied. Task force 1, Allied, on the transport
//!   mission, at base 1, with a route of one waypoint, (8,3), and the
//!   speed cruise; its route refers to order 1 and its speed to order 2.
//!   Ship 1, class 1, 2,000 t of fuel, and ship 2, class 2, 500 t, both
//!   in task force 1. No air group and no land unit.
//! - **The orders in force:** 1, Allied, a route for task force 1 to
//!   (8,3); 2, Allied, the speed cruise for task force 1. Both issued on
//!   day 1 by the commander.
//! - **Belief:** the Allied side holds one contact, 3 ships taken for
//!   cruisers at (4,2), 2 pulses old; one claim, at (4,2) on day 3 in the
//!   afternoon, of 1 hit on a cruiser from the air and nothing else; and
//!   one estimate, 200 men at (8,0), of day 2, seen from the air. The
//!   Japanese side holds nothing.
//! - **Command state:** empty for both sides, the next plan 1.
//! - **The counters:** Allied 1,000,003 for a force and 1,000,001 for an
//!   air group; Japanese 2,000,001 for each.
//! - **The stream:** seed 1942, trial 1, world 1.
//!
//! The world as built, before any day is resolved:
//!
//! - Its day is 4. It has no day in hand and no daily hash.
//! - Its log holds one event, event 1, on day 4 with no pulse: the world
//!   was built from a picture. Each side sees it.
//! - Its order book holds orders 1 and 2, as the picture gave them, both
//!   as issued on day 1.
//! - Task force 1 is at (0,0) with its route and speed, citing orders 1
//!   and 2. Ship 1 has 2,000 t and ship 2 has 500 t.
//! - The Allied side's belief: contact 1, 3 cruisers at (4,2), 2 pulses
//!   old, its last report event 1; the claim, as given, received by
//!   event 1; the estimate, 200 men at (8,0), of day 2, seen from the
//!   air, reported by event 1. The Japanese side's belief is empty.
//! - The next force the Allied side makes would be 1,000,003.
//!
//! No order is given. Days 5, 6 and 7 are resolved. The steaming is
//! scenario 1's (see its hand calculation: 16 kn, 394 nm to (8,3), the
//! cruiser burning 0.25 t a mile and the destroyer 0.125 t), with day 5
//! in the place of its day 1. Day d begins at hour 24 x (d - 1).
//!
//! | Day | Pulse     | Miles steamed | Position |
//! | --- | --------- | ------------- | -------- |
//! | 5   | Night     | 192           | (5,0)    |
//! | 5   | Morning   | 288           | (8,0)    |
//! | 5   | Afternoon | 384           | (8,3)    |
//! | 6   | Night     | 394, arrived  | (8,3)    |
//!
//! - **The end of day 5:** the world's day is 5. Ship 1 has 2,000 - 96 =
//!   1,904 t and ship 2 has 500 - 48 = 452 t.
//! - **Day 6:** the force arrives 0.625 hours into the night pulse, at
//!   hour 24 x 5 + 0.625 = 120.625. Ship 1 has 1,904 - 2.5 = 1,901.5 t
//!   and ship 2 has 452 - 1.25 = 450.75 t.
//! - **Day 7:** nothing moves and no fuel is burned. Ship 1 has 1,901.5 t
//!   and ship 2 has 450.75 t at the end.
//! - **The arrival's causes** lead back to order 1, the route, as
//!   scenario 1's do.
//! - **The first event of day 5** is event 2.
//! - **Three daily hashes** are kept, for days 5, 6 and 7.
//! - **The contact** is 2 + 3 = 5 pulses old at the end of day 5, where
//!   scenario 1's search figures still hold a contact of that age; where
//!   they drop it sooner, it is dropped on the pulse they give, as a
//!   contact of that age in a game from day 0 is. (The build reads the
//!   figure and asserts the pulse; it is not a number of this rule.)
//!
//! ## 2. Nothing lost: a world rebuilt from its own whole picture
//!
//! - **Scenario 1.** The live game is played to the evening of day 1.
//!   The umpire takes its whole picture and builds a world from it on
//!   the game's own seed with no domain. Both are resolved for days 2
//!   and 3. At the end of each of those days the built world's ships,
//!   task forces, bases and air groups are the live game's, field for
//!   field, but for the numbers of orders and events they cite. The
//!   built world's first event of day 2 is event 2.
//! - **Scenario 86,** which has a staff, a command state, land units, a
//!   landing and a base formed in play. The live game is played to the
//!   evening of day 3, each side's staff on doctrine. The umpire takes
//!   its whole picture and builds a world from it on the game's own seed
//!   with no domain. Both are played on to day 8 with the same staffs.
//!   At the end of each of days 4 to 8 the built world's ships, task
//!   forces, bases and land units are the live game's, field for field,
//!   but for the numbers of orders and events they cite; each side's
//!   command state is the live game's, but for those numbers; and the
//!   staff gives the built world the orders it gives the live game, in
//!   kind, target and day. At the end the bases hold what scenario 86's
//!   hand calculation gives: base 1 915 t, base 2 70 t, base 5 10 t.
//! - **A picture taken twice is the same picture,** and two worlds built
//!   from one picture on one stream have the same hash on every day.
//!
//! ## 3. A stream of its own
//!
//! - **A live game's draws are what they were.** Every pinned hash and
//!   run baseline of the repository is unchanged by the build.
//! - **The domain moves a draw.** For one key (seed 1942, day 5, the
//!   night pulse, one subsystem, one entity), the first draw with no
//!   domain, with trial 1 and world 1, with trial 1 and world 2, and
//!   with trial 2 and world 1 are four different numbers; and each is
//!   the same asked twice.
//!
//! ## 4. What it refuses
//!
//! Each is the picture of case 1 with one change, and each is refused
//! with words that name the fault. No world is built.
//!
//! | The change | Refused because |
//! | --- | --- |
//! | Task force 1's route refers to order 3 | an order referred to is not in the picture |
//! | A task force 1,000,003, Allied, is in the state | a unit made in play has a number at or above its side's counter |
//! | The Allied counter of forces is 1,000,000 | a counter is below its side's first number |
//! | The contact has 0 ships | a contact has no ship |
//! | The contact is at a hex off the map | a contact is off the map |
//! | The claim is of day 5 | a claim is after the picture's day |
//! | The estimate is of day 5 | an estimate is after the picture's day |
//! | A second estimate at (8,0) | two estimates for one hex |
//! | Ship 2's class is one the reference data does not have | what `World::new` refuses of it |
//! | The Allied command state holds plan 1 and its next plan is 1 | the next plan is not above the plans held |
//!
//! # Properties
//!
//! - **`World::new` is as it was.** Every scenario of the repository
//!   loads and plays as before: no baseline, golden file or pinned hash
//!   changes.
//! - **The picture is not state.** A world built from a picture and
//!   resolved holds nothing that refers to the picture: dropping the
//!   picture changes nothing, and the world's hash does not read it.
//! - **Nothing that plays can rebuild.** The whole picture of a live
//!   world and a build with no domain both ask for the umpire's key.
//!   The test that reads the agent's source still passes.

// The tests are written by the build, to the statements above.
