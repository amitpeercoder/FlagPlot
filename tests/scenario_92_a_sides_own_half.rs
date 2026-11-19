//! Scenario 92: a side's own half of a picture. A side's view gives, on an
//! evening, everything of its own that a picture of a world needs, with no
//! number of the live game in it; two halves make a picture.
//!
//! Proves decision 0171, the first part of step 4 of step 3's stage 4,
//! against a hand calculation. It has no scenario file of its own: it reads
//! live games of scenarios 1, 68 and 86 and of the full slice. It runs on
//! the frozen step 1 figures, but for the full slice, which runs as its own
//! tests run it.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## 1. By hand: the Allied half of scenario 1 on the evening of day 1
//!
//! Scenario 1 is played for one day (see its hand calculation: the force
//! steams 384 nm on day 1 and is 10 nm short of (8,3), in its hex). The
//! Allied side's view gives this half:
//!
//! - **The day:** 1. **The side:** Allied.
//! - **Bases:** base 1, Noumea, at (0,0) and base 2, Espiritu Santo, at
//!   (8,3), as the game holds them.
//! - **Task force 1:** at (8,3), under way, with its route of one
//!   waypoint, (8,3), and the speed cruise. Its route cites the order at
//!   place 1 and its speed the order at place 2. It cites no event: it
//!   has not arrived, is not alerted, halted or on a yard trip.
//! - **Ships:** ship 1 with 2,000 - 96 = 1,904 t of fuel and ship 2 with
//!   500 - 48 = 452 t, both in task force 1.
//! - **No air group, land unit, formation, replacement or arrival.**
//! - **The orders in force,** two: at place 1 the route for task force 1
//!   to (8,3), and at place 2 the speed cruise for task force 1; each
//!   Allied, issued on day 1, by the commander.
//! - **The belief:** no contact, no claim, no estimate.
//! - **The command state:** empty, the next plan 1.
//! - **The counters:** 1,000,001 for a force and 1,000,001 for an air
//!   group.
//!
//! The Japanese side's half of the same evening holds the day, 1, the
//! side, nothing else, an empty command state, and the counters 2,000,001
//! and 2,000,001.
//!
//! The two joined are a picture of day 1 with orders 1 and 2 as above. A
//! world built from it on the game's own seed with no domain (the
//! umpire's rebuild of decision 0170) and resolved for days 2 and 3 ends
//! each with ship 1 at 1,901.5 t and ship 2 at 450.75 t, the force
//! arrived at hour 24.625, as scenario 1 does.
//!
//! ## 2. Two halves are the whole
//!
//! - **Scenario 86,** played with both staffs to the evening of day 3.
//!   The Allied half and the Japanese half, joined, are equal to the
//!   umpire's whole picture of that evening, field for field, with no
//!   objectives. (The Japanese side has given no order, so the orders
//!   are in the same places.) A world built from the joined halves on
//!   the game's seed with no domain and played on to day 8 with both
//!   staffs has, at the end of each of days 4 to 8, the live game's
//!   ships, task forces, bases and land units, and ends with base 1 at
//!   915 t, base 2 at 70 t and base 5 at 10 t.
//! - **The full slice,** `07_full_slice.toml`, played as its own test
//!   plays it to the evening of day 3. The two halves joined hold the
//!   units, the belief, the command states and the counters of the
//!   umpire's whole picture, field for field; their orders are the same
//!   orders, each unit citing the same order, with the Allied side's
//!   first and then the Japanese side's where the whole picture has them
//!   in the order they were given. A world built from the joined halves
//!   on the game's seed with no domain, given the scenario's orders of
//!   days 4 and 5 as the scenario gives them, has at the end of each of
//!   those days the live game's ships, task forces, bases and air groups,
//!   field for field but for the numbers of the orders and events they
//!   cite. Day 4 is the day of the carrier battle: the fights of the
//!   built world are the live game's, hit for hit.
//!
//!   This case is also the test decision 0170 owes of a world with days
//!   behind it (its rulings, 7). If the built world and the live game
//!   part, the first difference is traced to the rule that read the
//!   history, and goes to the architect: the statement is not changed.
//!
//! ## 3. No peek
//!
//! Scenario 68's pair of worlds, as its property "what the side cannot
//! see changes nothing" makes them: the same game, and in one of them
//! Japanese base 2 holds 300 t of supply. Each is played to the evening
//! of day 5. The Allied side's two halves are equal, field for field.
//! The Japanese side's two halves differ, in base 2's supply and nowhere
//! else.
//!
//! In the full slice on the evening of day 3, the count of orders in the
//! Allied half is the count of orders its own units and command state
//! refer to, and no order in it is at a place above that count, whatever
//! the Japanese side has ordered.
//!
//! ## 4. What it refuses
//!
//! | Asked | Refused because |
//! | --- | --- |
//! | A half while a day is open (after the night pulse of day 2 of scenario 1) | a picture is of an evening |
//! | A half at a view cut to a seat (scenario 63 or another with command levels, at a task force's seat) | only a whole side has a half |
//! | Two Allied halves joined | two halves of one side |
//! | An Allied half of day 1 and a Japanese half of day 2 joined | halves of different days |
//! | Two halves that each hold a ship 1 | two units of one number |
//!
//! # Properties
//!
//! - **A half taken twice is the same half.**
//! - **Nothing of the live game is in it.** Every order a unit of the
//!   half cites is at a place from 1 to the count of its orders, and
//!   every event a unit cites is the first.
//! - **A half made by hand joins as a copied one does.** A Japanese half
//!   of day 1 made in the test, of one base and one force of one ship
//!   with a route citing the order at its place 1, joined to scenario
//!   1's Allied half of day 1, gives a picture whose Japanese force
//!   cites order 3, and a world is built from it.
//! - **Nothing that plays can take or join a half** outside what its
//!   own crate is allowed: the test that reads the agent's source
//!   passes, with the new words in it.
//! - **No live game changes:** no baseline, golden file or pinned hash
//!   moves.

// The tests are written by the build, to the statements above.
