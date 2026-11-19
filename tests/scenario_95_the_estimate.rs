//! Scenario 95: the estimate. A landing of 600 men is tried against a
//! place whose garrison the side puts at 200 and believes may be 600. In
//! four worlds it is taken in the two with 200 and not in the two with
//! 600, and the estimate says so in counts of worlds, with what each
//! cost.
//!
//! Proves decision 0176, step 6 of step 3's stage 4, against a hand
//! calculation. The data is placeholder data: see
//! `data/scenarios/95_the_estimate.toml`, which the build writes to the
//! inputs below. It runs on the frozen step 1 figures with scenario 68's
//! ground figures, in which every shot is a whole shot and hits and a
//! report gives the true count, so no die changes anything.
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
//! Scenario 68's (see its hand calculation). A unit fires one shot for
//! each 100 firing men; every shot hits and takes 50 men, shared between
//! the units hit in proportion to their men. A unit on attack that fired
//! gains 50 of fatigue; any other loses 25. A side is beaten where the
//! other attacked and it has no man left, or fewer than 0.2 of the
//! other's. A man eats 0.005 t a day. A size 0 port lands 20 t an hour
//! for each ship and a man weighs 0.25 t: 80 men an hour. At balanced
//! the staff wants 2 men for each man of its estimate, and a unit can
//! attack at a fatigue of 40 or less, or if it is on attack already; its
//! fit men are its men times 100 less its fatigue over 100.
//!
//! ## Inputs
//!
//! Scenario 68's map, bases, ships and figures, with these changes:
//!
//! - **One unit.** The Allied side has unit 1 alone, 600 fighting men at
//!   A. Units 2 and 3 are not in the scenario.
//! - **The opening estimate** of the Allied side is 200 enemy men at B.
//! - **Believed men:** the Allied side believes 400 more may be at B,
//!   standing unknown.
//! - **No commands.** Scenario 68's commands of day 1 are the course.
//! - **The trial's figures:** an enemy base is assumed to hold 200 t of
//!   supply and 0 t of fuel; the seed is 1942. **A file of sides** that
//!   gives every class to both sides.
//! - The Japanese garrison of the live game is as scenario 68 has it,
//!   1,200 men. The trial is not told.
//!
//! **The course:** for the Allied side on day 1: intent 1, take the place
//! at base 2, with a forces limit of task force 1, unit 1 and base 1 as
//! the source; leave to prepare; leave to launch.
//!
//! **The trial:** of the Allied side, from the evening before day 1,
//! trial 1, 4 worlds for 8 days.
//!
//! ## The worlds
//!
//! In every world the enemy has base 2 at B with 200 t of supply and
//! land unit 900,001, 200 fighting men on defend at B, from the
//! estimate. The 400 believed men are the first thing of unknown
//! standing: they are in worlds 2 and 4, as land unit 900,002, and not
//! in worlds 1 and 3. So the garrison is 200 in worlds 1 and 3 and 600
//! in worlds 2 and 4.
//!
//! ## The choice, and the days to the landing
//!
//! The estimate is 200: the men wanted are 400. Unit 1 takes ship 1:
//! 600 men, enough. The supply is 600 x 0.005 x 15 = 45 t.
//!
//! | Day | The Allied staff | What happens |
//! | --- | --- | --- |
//! | 1 | Approves plan 1 to preparing. Forms intent 2, ready a landing by task force 1 at B. Orders `prepare_landing` at B, roster 1 | Loading 100; unit 1's rehearsal 100 |
//! | 2 | Notice: intent 2 is ready. Approves plan 1 to released. Orders a load of supply to a weight of 45 t | Ship 1 loads 45 t |
//! | 3 | Unit 1 set to attack. Load unit 1 | Ship 1 takes 600 men |
//! | 4 | A route to B and back to A, acting at B, and an unload order | Sails at hour 72 and arrives at hour 88. Ship 1 lands 160 men in the morning's last 2 hours and 440 in the afternoon, and then 10 t. Base 3, "Landing at (4,0)", is formed. The fight |
//!
//! These days are the same in all four worlds: nothing of the enemy is
//! read before the fight but the estimate.
//!
//! ## The fight, day 4
//!
//! | | Worlds 1 and 3 | Worlds 2 and 4 |
//! | --- | --- | --- |
//! | Allied men ashore, on attack | 600 | 600 |
//! | Japanese men ashore, on defend | 200 | 200 and 400 |
//! | Allied shots, all hits | 6 | 6 |
//! | Japanese shots, all hits | 2 | 2 and 4 |
//! | Japanese men lost | 6 x 50 = 300, more than the 200: all 200 | 300: 100 and 200 |
//! | Allied men lost | 2 x 50 = 100 | 6 x 50 = 300 |
//! | Left | 500 against none | 300 against 100 and 200 |
//!
//! - **Worlds 1 and 3.** The Japanese side has no man at B and is
//!   beaten: base 2 is Allied at the end of day 4. The end is met on
//!   day 4. On the morning of day 5 the plan is marked complete and
//!   unit 1 is set to defend. No more men are lost on days 5 to 8.
//! - **Worlds 2 and 4.** The Japanese side has 300 men against
//!   0.2 x 300 = 60: it is not beaten. Unit 1 has 300 men at a fatigue
//!   of 50. That evening's report gives 300 enemy men. On the morning of
//!   day 5 the men wanted are 600 and unit 1's fit men are 150: the
//!   staff sets it to defend and waits. Its fit men are 225 on the
//!   morning of day 6 and 300 from day 7, against 600 wanted: it waits
//!   on every morning to day 8, and there is no unit to bring. No one
//!   attacks, so there is no fight after day 4 and no more men are
//!   lost. Base 2 is Japanese at the end of day 8: the end is not met.
//!
//! Every man of both sides is fed on every night: the Japanese from
//! base 2's 200 t, the Allied men ashore from base 3.
//!
//! ## The estimate of intent 1
//!
//! - **How it went.** 4 worlds played. Met and kept in 2, worlds 1 and
//!   3: the earliest day 4, the middle day 4, the latest day 4. Met and
//!   lost again in none. Not met by day 8 in 2, worlds 2 and 4. None
//!   could not be played.
//! - **What it cost the side,** lowest, middle, highest. Men lost to the
//!   enemy: the worlds give 100, 300, 100, 300; in order 100, 100, 300,
//!   300; the middle is the value at place 2: 100, 100, 300. Men lost
//!   unfed: 0, 0, 0. Ships sunk: 0, 0, 0. Ships out of action: 0, 0, 0.
//!   Aircraft: 0, 0, 0.
//! - **What the trial's enemy lost.** Men lost to the enemy: the worlds
//!   give 200, 300, 200, 300: 200, 200, 300. The rest: 0.
//! - **What it needs and has.** Men wanted 400, to hand 600. The day
//!   expected ashore: day 4.
//! - **What was assumed,** each with its count of worlds: base 2 with
//!   200 t of supply and 0 t of fuel, in 4; 200 men at B from the
//!   side's estimate, in 4; 400 men believed at B, standing unknown, in
//!   2 and not in 2; the enemy given no belief, in 4; the enemy given no
//!   intent, in 4.
//! - **Where the worlds part,** by the 400 men believed at B. With them,
//!   worlds 2 and 4: met in 0 of 2, not met in 2; the side's men lost to
//!   the enemy 300, 300, 300. Without them, worlds 1 and 3: met and kept
//!   in 2 of 2, on day 4; the side's men lost to the enemy 100, 100,
//!   100.
//! - **Its other plans:** the side has none, and no trial of things as
//!   they stand is given.
//!
//! Its words hold each of those figures, the words "worlds" and no
//! figure with a per cent sign or the word "chance" or "points".
//!
//! # Properties
//!
//! - **Things as they stand.** A second trial of the same four worlds
//!   with an empty course: nothing is ordered, no man is lost, and base
//!   2 is Japanese in all four. Given to the estimate of intent 1 to
//!   compare with, it changes nothing above: the side has no other plan.
//! - **A plan displaced.** With a second Allied unit of 600 men at A and
//!   a second intent already stated in the live game on day 1 and its
//!   leaves given, to take the place at base 2 with that unit and task
//!   force 1; and the course a third intent that takes task force 1 for
//!   unit 1 at once: reported by the build with its figures for the
//!   architect to state, and not asserted here. (The numbers of two
//!   takes that share a force are the staff's priority's, decision 0111,
//!   and are worked when the build shows what it gives.)
//! - **Believed present, and absent.** With the 400 men's standing
//!   present: the garrison is 600 in all four worlds, the end is met in
//!   none, and the estimate shows no parting. With it absent: 200 in all
//!   four, met in 4 on day 4, no parting.
//! - **Met and lost again.** Not reached by this scenario: the Japanese
//!   side is given no intent and never attacks. It is tested inside the
//!   crate on worlds' events made by the test: a place that changes side
//!   to the side on day 3 and away from it on day 6 is counted as met
//!   and lost again and not as met and kept.
//! - **An intent with no end.** The estimate of an intent to keep a base
//!   supplied gives the costs and says that it has no end an estimate
//!   can read.
//! - **A world that could not be played** is counted apart: with world
//!   2's picture made unsound by the test, the estimate has 3 worlds
//!   played and 1 that could not be, and its counts are of the 3.
//! - **Believed men are loaded as a believed force is,** and refused
//!   for the same faults: no man; an unknown standing; a base that is
//!   the believing side's own; a hex off the map; both a base and a
//!   hex; an unknown key.
//! - **The same twice,** and the live game untouched: its hash before
//!   and after the trials and the estimates is the same.
//! - **`sim-trial` reads a world only where it makes an estimate.** Its
//!   source test passes with the one allowance.
//! - **No live game changes** by the build: no baseline, golden file or
//!   pinned hash moves.

// The tests are written by the build, to the statements above.
