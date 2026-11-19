//! Scenario 94: a trial. One course is played in a number of worlds built
//! from a side's picture, for a number of days, with both sides on
//! doctrine; the live game is not touched.
//!
//! Proves decision 0175, step 5 of step 3's stage 4, against a hand
//! calculation. The data is placeholder data: see
//! `data/scenarios/94_a_trial.toml`, which the build writes to the inputs
//! below. It runs on the frozen step 1 figures.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Inputs
//!
//! Scenario 86 (see its hand calculation), with three changes:
//!
//! - its three commands of day 1 are taken out of the scenario: they are
//!   the course to be tried, and the live game is given none of them;
//! - it names a file of the trial's figures in which an enemy base is
//!   assumed to hold 0 t of supply and 0 t of fuel, and the three sizes
//!   are as decision 0175 has them;
//! - it names a file of sides that gives every class to both sides.
//!
//! So the live game is: base 1 at A, (0,0), Allied, 1,000 t of supply;
//! base 2 at B, (4,0), Japanese, with nothing; base 4 at C, (0,6),
//! Allied, with nothing; task force 1, Allied, two troop transports at
//! sea at (2,0), unit 1 (600 fighting men) and 30 t of supply in ship 1
//! and unit 2 (400 working men) in ship 2. The Allied side holds no
//! contact and no estimate and believes nothing. No commands. In the
//! live game nothing is ever ordered.
//!
//! **The course:** for the Allied side, intent 1, control island 1;
//! leave to prepare given; leave to launch given.
//!
//! **The trial:** of the Allied side, from the evening before day 1,
//! seed 1942, trial 1.
//!
//! ## 1. Every world is scenario 86's game
//!
//! The enemy's half of every picture is base 2 with 0 t of supply and
//! 0 t of fuel and nothing else, as scenario 86 has it. The Japanese
//! side is given no intent. No die is rolled in scenario 86. So each
//! world of the trial plays scenario 86's days as its hand calculation
//! gives them.
//!
//! A trial of 4 worlds and 8 days. In each of worlds 1 to 4:
//!
//! - **The orders the Allied staff gives** are scenario 86's eight, on
//!   scenario 86's days: on day 1 the landing at B with unit 1, the
//!   route to B and on to base 1, and the unload order; on day 2 unit 1
//!   set to attack; on day 3 unit 1 set to defend; on day 4 a load of
//!   supply to a weight of 85 t; on day 5 the route to B and back to
//!   base 1, and the unload order. None on days 6 to 8.
//! - **Base 2 is the Allied side's** from the end of day 2, and the take
//!   of B is met on the morning of day 3.
//! - **At the end of day 8:** base 1 holds 915 t, base 2 holds 70 t and
//!   base 5, the base the landing formed, 10 t. Each ship has 944 t of
//!   fuel. Unit 1 has 600 men and unit 2 has 400, all ashore at B.
//! - **The Japanese staff gives no order** on any day.
//!
//! The four worlds have the same hash at the end of every day.
//!
//! ## 2. The live game is not touched
//!
//! The live game's hash is taken before the trial of case 1 and after
//! it: they are equal. The live game is then resolved for 8 days with
//! both staffs: no order is given on any day, task force 1 is at (2,0)
//! at the end, base 2 is Japanese, and its eight daily hashes are those
//! of the same game resolved with no trial ever asked.
//!
//! ## 3. The sizes
//!
//! Small is 8 worlds for 7 days, 56 days of play; standard is 16 for 14,
//! 224; full is 64 for 21, 1,344. A trial asked at "small" plays worlds
//! 1 to 8 for days 1 to 7 and no more: each world's day at its end is
//! day 7.
//!
//! ## 4. A world depends on its own number alone
//!
//! With a contact added to the live game, 3 ships taken for a cruiser at
//! (3,2), one pulse old, so that worlds differ in where the force is,
//! and with an empty course:
//! world 3 of a trial of 4 worlds and world 3 of a trial of 8 have the
//! same hash at the end of each of 3 days, and their pictures are equal.
//! Worlds 1 to 8 of the trial of 8 do not all have the same hash at the
//! end of day 1.
//!
//! ## 5. Stopped, and begun again
//!
//! The trial of case 1 is worked to the end of day 2 of world 2 and
//! stopped. Nothing of it is kept: the live game's hash is what it was.
//! The trial is begun again and played out: each world's hash at the end
//! of each day is that of case 1.
//!
//! ## 6. An empty course
//!
//! The trial of case 1 with no command in its course: in each world
//! nothing is ordered by either staff on any of the 8 days, task force 1
//! is at (2,0) at the end, and base 2 is Japanese.
//!
//! # Properties
//!
//! - **The same twice.** The same view, beliefs, figures, seed, trial,
//!   size and course give worlds with the same hash on every day.
//! - **The course goes to the side tried and to no other.** In case 1
//!   the Japanese side's command state holds no intent on any day.
//! - **No seat, no trial inside.** `sim-trial`'s source test passes: it
//!   hands in commands to and resolves only a world it built, and names
//!   no umpire, no whole picture and no view of a world it was not
//!   given.
//! - **A failed world is reported, and the others are played.** With one
//!   world made to break an invariant on its day 2 by a means of the
//!   test, the trial reports that world and that day, and the other
//!   worlds are as in case 1.
//! - **No live game changes** by the build: no baseline, golden file or
//!   pinned hash moves.

// The tests are written by the build, to the statements above.
