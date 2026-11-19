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
//! (Added on review.) The build writes those two files, each with its
//! frozen copy: the trial's figures also hold `seed = 1942`, and the
//! file of sides has a row for every class and type scenario 86's
//! reference files give, the troop transport's among them.
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
//! The four worlds hold the same state at the end of every day: ships,
//! task forces, bases, land units and both sides' command states, field
//! for field. (Amended on review: this said they have the same hash. A
//! world's hash holds its trial's number and its own, so the hashes of
//! two worlds are never equal.)
//!
//! ## 2. The live game is not touched
//!
//! The live game's hash is taken before the trial of case 1 and after
//! it: they are equal. The live game is then resolved for 8 days with
//! both staffs: no order is given on any day, task force 1 is at (2,0)
//! at the end, base 2 is Japanese, and its eight daily hashes are those
//! of the same game resolved with no trial ever asked.
//!
//! (Added on review.) And on every evening: the live game is played its
//! 8 days with both staffs, and on each evening from the one before day
//! 1 to the evening of day 7 a trial of 2 worlds for 2 days, with the
//! course of case 1, is asked of the Allied side's view and played out.
//! The live game's eight daily hashes are those of the game never
//! asked.
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
//! In the pictures of worlds 1 to 8 the Japanese force is not at the
//! same hex in all of them. (Amended on review: this compared the
//! worlds' hashes, which differ by their numbers alone.)
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
//! - **A world that cannot be played is reported, and the others are
//!   played.** (Amended on review, which found no means named.) The
//!   trial of case 1 played from its pictures, with world 2's picture
//!   altered by the test so that its world is refused when built (its
//!   Allied counter of forces set below the side's first number): world
//!   2's record has its number, day 0 and the engine's words; worlds 1,
//!   3 and 4 are as in case 1. A day that fails in play is tested inside
//!   the crate, with a day's step made to fail by the test: that world's
//!   record has the day and the days before it, and the other worlds
//!   are played.
//! - **The enemy's intents are handed in once.** (Added on review.) With
//!   the Allied side believing that the Japanese side holds base 2: in
//!   each world of case 1, at the end of day 1 the Japanese side's
//!   command state has exactly one intent, to hold base 2, and no
//!   command of that side was refused.
//! - **Two courses, the same worlds.** (Added on review.) With case 4's
//!   contact: the pictures of worlds 1 to 4 are equal, world for world,
//!   whether the course is case 1's or empty.
//! - **The seed is the trial's.** (Added on review.) The live game
//!   loaded on another seed of its own gives the Allied side the same
//!   pictures and, played out, worlds with the same hashes.
//! - **A command on its day.** (Added on review.) The course of case 1
//!   with each command given day 3: in each world nothing is ordered on
//!   days 1 and 2, and the landing is ordered on day 3.
//! - **A refused command is reported.** (Added on review.) The course of
//!   case 1 with a fourth command, the approval of a plan that does not
//!   exist: each world's record has that command as refused, with the
//!   engine's words, and the world plays as in case 1.
//! - **No live game changes** by the build: no baseline, golden file or
//!   pinned hash moves.

// The tests are written by the build, to the statements above.
