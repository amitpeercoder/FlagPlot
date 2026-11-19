//! Scenario 89: a raid recognised by the experience of the ship attacked.
//! The chance that an attacked ship's people recognise the aircraft is
//! moved by the ship's own experience; a base keeps the plain chance.
//!
//! Proves decision 0168, an amendment to decision 0167, against a hand
//! calculation. It has no scenario file of its own. It plays scenario 19a
//! (an air attack on a ship that is certain to be made) as scenario 88
//! plays it, with ship 1 given an experience by the scenario's own key
//! (`experience` in its `[[ship]]` entry, added to the scenario's text by
//! the test), and scenario 6a (a raid on a base). Both are played on the
//! frozen step 1 figures with two figures altered by
//! `common::altered_reference`: `raid_recognised_chance` and
//! `raid_recognised_experience_strength`.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly, but for the counts over seeds, which have their allowed
//! ranges. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## The chance
//!
//! For an air attack on a ship:
//!
//! `chance = min(1, raid_recognised_chance x factor)`
//! `factor = 1 + raid_recognised_experience_strength x (experience - 2)`
//!
//! with the experience of the ship attacked, a whole number from 0 to 4.
//! With the strength at 0 the factor is 1 and the chance is the plain
//! figure. For a raid on a base the chance is the plain figure at any
//! strength. One draw is made against the chance, the draw of decision
//! 0167, from its own stream: the same seed gives the same draw whatever
//! the experience and whatever the two figures.
//!
//! Both figures are 0 in the frozen step 1 figures. In the live figures
//! the chance is 0.75 and the strength 0.25.
//!
//! ## The chance at each experience (the figure 0.75, the strength 0.25)
//!
//! | Experience of ship 1 | Factor              | Chance                  |
//! | -------------------- | ------------------- | ----------------------- |
//! | 0                    | 1 - 0.5  = 0.5      | 0.75 x 0.5  = 0.375     |
//! | 1                    | 1 - 0.25 = 0.75     | 0.75 x 0.75 = 0.5625    |
//! | 2                    | 1                   | 0.75                    |
//! | 3                    | 1 + 0.25 = 1.25     | 0.75 x 1.25 = 0.9375    |
//! | 4                    | 1 + 0.5  = 1.5      | 1.125, cut to 1         |
//!
//! The breakdown of the draw, with breakdowns on, gives the plain chance,
//! the experience and the factor of the row, and the chance of the row.
//!
//! ## Scenario 19a, the morning raid, over seeds 1 to 1000
//!
//! Japanese group 2, 12 Placeholder dive bombers (aircraft type 3),
//! strikes Allied task force 1, which is ship 1, in the morning pulse of
//! day 1. The count is of the seeds on which the Allied side's event of
//! the attack names type 3. A seed on which no attack is made counts as
//! not recognised, as in scenario 88.
//!
//! | Experience | p      | Mean  | Std dev | Allowed                        |
//! | ---------- | ------ | ----- | ------- | ------------------------------ |
//! | 0          | 0.375  | 375   | 15.31   | 314 to 436                     |
//! | 1          | 0.5625 | 562.5 | 15.69   | 500 to 625                     |
//! | 2          | 0.75   | 750   | 13.69   | 696 to 804, and see below      |
//! | 3          | 0.9375 | 937.5 | 7.65    | 907 to 968                     |
//! | 4          | 1      |       |         | every seed an attack is made   |
//!
//! The standard deviation is the square root of 1000 x p x (1 - p):
//! 234.375, 246.09375, 187.5 and 58.59375 under the root. The allowed
//! range is 3.94 of them either side, taken outward to whole seeds, as
//! scenario 88's is.
//!
//! Exactly, and not by a range:
//!
//! - **Experience 2 is decision 0167.** At experience 2 the seeds
//!   recognised are the very seeds recognised with the strength at 0, one
//!   for one.
//! - **Experience 4 is certain.** At experience 4 the type is named on
//!   every seed on which the attack is made, and the count is the count of
//!   those seeds.
//! - **More experience never sees less.** The draw is the same on a seed
//!   at every experience, so a seed recognised at an experience is
//!   recognised at every higher one: the seeds of 0 are among those of 1,
//!   those of 1 among those of 2, and so on to 4.
//!
//! ## The ends of the strength
//!
//! Scenario 19a's morning raid, seeds 1 to 50:
//!
//! | The figure | The strength | Experience | Factor | Chance | The type the Allied side is told |
//! | ---------- | ------------ | ---------- | ------ | ------ | -------------------------------- |
//! | 1          | 0.5          | 0          | 0      | 0      | none, on every seed              |
//! | 1          | 0.5          | 2          | 1      | 1      | type 3, on every seed            |
//! | 0.75       | 0            | 0          | 1      | 0.75   | as at experience 2, seed by seed |
//! | 0          | 0.5          | 4          | 2      | 0      | none, on every seed              |
//!
//! On every row the weapon is told: bombs.
//!
//! ## Scenario 6a, the raid of the morning of day 1: a base
//!
//! Japanese group 1, Placeholder dive bombers (aircraft type 3), raids
//! Allied base 2. A base has no experience and its chance is the plain
//! figure.
//!
//! | The figure | The strength | The type the Allied side is told |
//! | ---------- | ------------ | -------------------------------- |
//! | 1          | 0.5          | type 3, "Placeholder dive bomber" |
//! | 1          | 0            | type 3, "Placeholder dive bomber" |
//! | 0          | 0.5          | none: not recognised              |
//!
//! With the figure at 0.75, on each of seeds 1 to 50 the raid is
//! recognised with the strength at 0.5 if and only if it is with the
//! strength at 0.
//!
//! ## The figure's bounds
//!
//! A figures file with `raid_recognised_experience_strength` below 0, or
//! above 0.5, is refused when it is read. At 0 and at 0.5 it is read.
//!
//! # Properties
//!
//! - **No fight is resolved differently.** On each of seeds 1 to 50, the
//!   ships, air groups and bases of scenario 19a end each day the same at
//!   every experience of ship 1 from 0 to 4 with the strength at 0.25 as
//!   they do with it at 0: the hits, the aircraft lost and the damage do
//!   not move. (Scenario 19a is fought by day, and no other rule reads a
//!   ship's experience by day.)
//! - **The attacker is told nothing new.** The Japanese side's events of
//!   the morning raid of scenario 19a are the same, word for word, at
//!   every experience of ship 1 and at both strengths.
//! - **Experience is not written.** Ship 1 ends every game with the
//!   experience the scenario gave it.
//! - **It is the same twice.** The same seed gives the same recognition
//!   twice, and a game saved and replayed gives the same.
//! - **The frozen figures are as they were.** With both figures as the
//!   frozen step 1 files have them, every assertion of scenario 88 on
//!   those figures holds unchanged.

// The tests are written by the build, to the statements above.
