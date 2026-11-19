//! Scenario 88: what the attacked saw of a raid. A side whose ship or base
//! is raided from the air is told what the aircraft attacked with, and,
//! where its people recognised them, what type they were.
//!
//! Proves decision 0167 against a hand calculation. It has no scenario
//! file of its own: it plays scenario 19a (an air attack on a ship that is
//! certain to be made) and scenario 6a (a raid on a base), each as its own
//! test plays it, on the frozen step 1 figures with one figure altered by
//! `common::altered_reference`: `raid_recognised_chance`.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly, but for the one count over seeds, which has its allowed range.
//! If the code disagrees, the code or the rule is wrong: do not change a
//! number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## The figure
//!
//! `raid_recognised_chance` is the chance that the people of an attacked
//! ship or base recognise the aircraft. It is 0 in the frozen step 1
//! figures and 0.75 in the live figures. One draw is made for each air
//! attack on a ship and for each raid on a base, when it is resolved, from
//! a stream of its own; the true event records what it gave.
//!
//! ## Scenario 19a, the morning raid (b = 1, as its case 1)
//!
//! Japanese group 2, 12 Placeholder dive bombers (aircraft type 3, which
//! attacks with a bomb), strikes Allied task force 1 in the morning pulse
//! of day 1 and finds it. Task force 1 is one ship, ship 1, so the Allied
//! side has one event of an air attack against it in that pulse.
//!
//! | The figure | The weapon the Allied side is told | The type it is told |
//! | --- | --- | --- |
//! | 1 | bombs | aircraft type 3, "Placeholder dive bomber" |
//! | 0 | bombs | none: not recognised |
//!
//! The number of aircraft the event gives is what it gave before the
//! rule, on every seed.
//!
//! On the frozen figures as they stand, the figure 0, the event is that
//! of the second row.
//!
//! ## Scenario 6a, the raid of the morning of day 1
//!
//! Japanese group 1, Placeholder dive bombers (aircraft type 3), raids
//! Allied base 2 on the morning of day 1. The Allied side has one event of
//! a raid on its base in that pulse.
//!
//! | The figure | The weapon the Allied side is told | The type it is told |
//! | --- | --- | --- |
//! | 1 | bombs | aircraft type 3, "Placeholder dive bomber" |
//! | 0 | bombs | none: not recognised |
//!
//! Its shots, its hits and what they fell on are what the event gave
//! before the rule.
//!
//! ## The chance, over seeds
//!
//! Scenario 19a's morning raid with the figure at 0.75, seeds 1 to 1000.
//! The attack is made on every seed the raid reaches the ship with a
//! bomber left; a seed on which no attack is made counts as not
//! recognised.
//!
//! | Count of runs, over seeds 1 to 1000 | p    | Mean | Std dev | Allowed    |
//! | ----------------------------------- | ---- | ---- | ------- | ---------- |
//! | The type is recognised              | 0.75 | 750  | 13.69   | 696 to 804 |
//!
//! The standard deviation is the square root of 1000 x 0.75 x 0.25. The
//! allowed range is 3.94 of them either side, as scenario 5b's are.
//!
//! # Properties
//!
//! - **The attacker is told nothing new.** The Japanese side's events of
//!   the morning raid of scenario 19a, and of the first raid of scenario
//!   6a, are those it had before the rule, word for word, with the figure
//!   at 0 and at 1.
//! - **No fight is resolved differently.** On each of seeds 1 to 50, the
//!   ships, air groups and bases of scenario 19a and of scenario 6a end
//!   each day the same with the figure at 0, at 0.75 and at 1: the hits,
//!   the aircraft lost and the damage do not move.
//! - **It is the same twice.** The same seed gives the same recognition
//!   twice, and a game saved and replayed gives the same.
//! - **Nothing more is told.** Two worlds that differ only in what the
//!   Allied side cannot see and has not been told give it the same events
//!   of the raid.
//! - **The other raids of the scenarios.** In scenario 6a the raids of the
//!   mornings of days 2 and 3, where they fly, each have their own draw;
//!   with the figure at 1 each names type 3 and with it at 0 none does.

// The tests are written by the build, to the statements above.
