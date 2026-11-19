//! Scenario 99: who made a claim, and an air group's tally of its claims.
//!
//! Proves decision 0180 against a hand calculation. It runs on the frozen
//! step 1 figures with the false claim chances altered, as scenario 11's
//! test does, on `06a_base_raid.toml` and `05b_strike.toml`, which are
//! placeholder data.
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
//! Scenario 11's hand calculation (see it). The base raid, three
//! mornings: Japanese group 1 (bombers) and group 2 (escorts) raid base
//! 2; Allied group 3 patrols over it and the base's guns fire.
//!
//! | Day | Firing | By | Shots | Hits | Missed |
//! | --- | ------ | -- | ----- | ---- | ------ |
//! | 1 | escorts at the patrol | group 2 | 6 | 3 | 3 |
//! | 1 | patrol at the escorts | group 3 | 6 | 1 | 5 |
//! | 1 | patrol at the bombers | group 3 | 6 | 2 | 4 |
//! | 1 | base guns | base 2 | 20 | 0 | 20 |
//! | 1 | bombs | group 1 | 16 | none on aircraft | 16 |
//! | 2 | escorts at the patrol | group 2 | 6 | 2 | 4 |
//! | 2 | patrol at the escorts | group 3 | 6 | 1 | 5 |
//! | 2 | patrol at the bombers | group 3 | 3 | 1 | 2 |
//! | 2 | base guns | base 2 | 20 | 0 | 20 |
//! | 2 | bombs | group 1 | 15 | 2 aircraft destroyed | 13 |
//! | 3 | escorts at the patrol | group 2 | 5 | 0 | 5 |
//! | 3 | patrol at the escorts | group 3 | 5 | 1 | 4 |
//! | 3 | patrol at the bombers | group 3 | 2 | 1 | 1 |
//! | 3 | base guns | base 2 | 20 | 3 | 17 |
//! | 3 | bombs | group 1 | 11 | none on aircraft | 11 |
//!
//! ## Who claims what, each day
//!
//! With every false claim chance at 0 a unit claims what it really did;
//! at 1 it claims that and every whole shot or bomb that missed. A unit
//! that claims nothing is not in the list.
//!
//! The Japanese claim:
//!
//! | Day | Chances 0 | Chances 1 |
//! | --- | --------- | --------- |
//! | 1 | group 2: 3 shot down | group 2: 6 shot down; group 1: 16 on the ground |
//! | 2 | group 2: 2 shot down; group 1: 2 on the ground | group 2: 6 shot down; group 1: 15 on the ground |
//! | 3 | no unit | group 2: 5 shot down; group 1: 11 on the ground |
//!
//! The Allied claim:
//!
//! | Day | Chances 0 | Chances 1 |
//! | --- | --------- | --------- |
//! | 1 | group 3: 3 shot down | group 3: 12 shot down; base 2: 20 shot down |
//! | 2 | group 3: 2 shot down | group 3: 9 shot down; base 2: 20 shot down |
//! | 3 | group 3: 2 shot down; base 2: 3 shot down | group 3: 7 shot down; base 2: 20 shot down |
//!
//! Day 1, chances 1, group 3: 1 + 2 really, and 5 + 4 missed: 12. The
//! sums are scenario 11's: 32, 29 and 27 for the Allied side at chances
//! 1, and every claim's own numbers are as scenario 11 has them.
//!
//! Each list is in the order given: the Japanese side's escorts fire
//! before its bombs fall, and the Allied patrol fires before the base's
//! guns.
//!
//! ## The tallies after the three days
//!
//! | Group | Chances 0 | Chances 1 |
//! | ----- | --------- | --------- |
//! | 1 | 2 on the ground | 42 on the ground |
//! | 2 | 5 shot down | 17 shot down |
//! | 3 | 7 shot down | 28 shot down |
//! | 4 and 5 | nothing | nothing |
//!
//! Group 1 at chances 1: 16 + 15 + 11. Group 3: 12 + 9 + 7. A tally's
//! other numbers are 0. Base 2 has no tally.
//!
//! ## The unopposed strike
//!
//! `05b_strike.toml`: group 2 drops 18 bombs on ship 3 for 5 hits; no
//! fighter or gun fires. The Allied claim's list: group 2, 5 hits at
//! chances 0 and 18 at chances 1. Group 2's tally of hits is the same.
//! The Japanese side has no claim.
//!
//! ## A firing of two groups, parted shot by shot
//!
//! The rule's own function: the aircraft each group has in the firing,
//! the whole shots of the firing, which shots hit (in the order drawn),
//! and what the firing really shot down. It gives each group's shots,
//! its real aircraft and its shots that missed; at false chance 1 a
//! group claims its real aircraft and every shot of its own that missed.
//!
//! | In the firing | Shots | Shots that hit | Really shot down | Shots, by group | Real, by group | Claimed at chance 1 |
//! | --- | --- | --- | --- | --- | --- | --- |
//! | group 3: 8, group 7: 4 | 6 | the 1st, 5th and 6th | 3 | 4 and 2 | 1 and 2 | 4 and 2 |
//! | group 3: 8, group 7: 4 | 12 | the 2nd and 9th | 2 | 8 and 4 | 1 and 1 | 8 and 4 |
//! | group 3: 1, group 7: 1 | 1 | none | 0 | 1 and 0 (a tie goes to the lower number) | 0 and 0 | 1 and 0 |
//! | group 3: 5, group 7: 5 | 3 | the 3rd | 1 | 2 and 1 (1.5 each: 1 and 1, the one left to group 3) | 0 and 1 | 2 and 1 |
//! | group 3: 8, group 7: 4 | 6 | the 1st, 5th and 6th | 2 | 4 and 2 | 1 and 1 (the cut comes off the last shot) | 4 and 2 |
//! | group 3: 12 | 6 | the 1st | 1 | 6 | 1 | 6 |
//!
//! Row 1: 6 x 8 / 12 = 4 and 6 x 4 / 12 = 2; group 3 has shots 1 to 4,
//! of which the 1st hit, and 3 missed; group 7 has shots 5 and 6, both
//! hits. In every row the shots, the real aircraft and the claims add to
//! the firing's own.
//!
//! In a game: the base raid's first morning with a second Allied group
//! on patrol over base 2 is not worked here by hand, the fight being
//! another fight; the build reports its lists, and the test asserts for
//! it only that the two patrol groups' lines add to the Allied claim
//! less the guns', and that neither is credited with more shots than the
//! aircraft it had on patrol.
//!
//! ## Bombs, group by group
//!
//! Two groups bomb one base in one raid: group 1 drops 10 bombs that
//! destroy 2 aircraft, group 2 drops 5 that destroy none. At false
//! chance 0 the lines are group 1: 2 on the ground, and group 2 is not
//! in the list. At 1 they are group 1: 10 and group 2: 5. This is the
//! rule's function, given the bombs and what they destroyed.
//!
//! # Properties
//!
//! - **Nothing else moves.** At every setting, every claim's numbers,
//!   every fight and every event but the claims' own are those of
//!   scenario 11's variants as they run today.
//! - **The sums hold.** Over 200 seeds of the base raid at the
//!   placeholder chances: in every claim the list's aircraft shot down,
//!   aircraft on the ground and hits each add to the claim's own; no
//!   line is all 0; and each unit's line is no less than what that unit
//!   really did, read from the firing events.
//! - **A tally is the sum of the group's lines** in every claim of the
//!   game, over those seeds.
//! - **A tally is kept:** a group that rebases, and a group that has lost
//!   every aircraft, has the tally it had.
//! - **A surface fight's claim and a bombardment's name no unit:**
//!   scenario 24's bombardment claim has an empty list.
//! - **The other side is told nothing:** neither side's events hold the
//!   other's claim, and no answer of a seat's tools holds a tally of an
//!   enemy group.
//! - **With command levels on,** a superior's knowledge of a group's
//!   tally is that of its latest state report to arrive (decision 0136).
//! - **A picture keeps the list and the tallies** (decisions 0170 and
//!   0171): a world rebuilt from the picture after day 2 has them.
//! - **The hash names the fields:** `hash::name_every_field` lists them,
//!   and two worlds that differ only in one group's tally hash
//!   differently.

// The tests are written by the build, to the statements above.
