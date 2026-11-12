//! Claims from the air (decision 0055).
//!
//! # Hand calculation
//!
//! Written by the architect before the test code. The tests below assert
//! these numbers. If the code disagrees, the code or the rule is wrong: do
//! not change a number here to make a test pass.
//!
//! A claim is the real count plus a draw for each whole miss. Two settings
//! of the chances take the draws out: at 0 nothing false is claimed, and at
//! 1 every whole miss is. Both are fixed here from the event logs of the
//! scenarios as they run today with their own seeds; the fight itself is
//! the same at any setting (decision 10), which the tests also assert.
//!
//! ## The base raid, `06a_base_raid.toml`, three mornings
//!
//! Groups 1 (bombers) and 2 (escorts) raid base 2; group 3 patrols over it
//! and the base's guns fire. "Missed" is shots less hits.
//!
//! | Day | Firing | Shots | Hits | Missed | Really lost to it |
//! | --- | ------ | ----- | ---- | ------ | ----------------- |
//! | 1 | escorts at the patrol | 6 | 3 | 3 | 3 of group 3 |
//! | 1 | patrol at the escorts | 6 | 1 | 5 | 1 of group 2 |
//! | 1 | patrol at the bombers | 6 | 2 | 4 | 2 of group 1 |
//! | 1 | base guns | 20 | 0 | 20 | 0 |
//! | 1 | bombs | 16 | 1, none on aircraft | | 0 aircraft on the ground |
//! | 2 | escorts at the patrol | 6 | 2 | 4 | 2 of group 3 |
//! | 2 | patrol at the escorts | 6 | 1 | 5 | 1 of group 2 |
//! | 2 | patrol at the bombers | 3 | 1 | 2 | 1 of group 1 |
//! | 2 | base guns | 20 | 0 | 20 | 0 |
//! | 2 | bombs | 15 | 4, two on aircraft | | 2 of group 4 on the ground |
//! | 3 | escorts at the patrol | 5 | 0 | 5 | 0 |
//! | 3 | patrol at the escorts | 5 | 1 | 4 | 1 of group 2 |
//! | 3 | patrol at the bombers | 2 | 1 | 1 | 1 of group 1 |
//! | 3 | base guns | 20 | 3 | 17 | 3 of group 1 |
//! | 3 | bombs | 11 | 1, none on aircraft | | 0 |
//!
//! The raiding side's claim, each day. Shot down: what the escorts really
//! shot down, plus their misses at chance 1. On the ground: what the bombs
//! really destroyed, plus every other bomb at chance 1.
//!
//! | Day | Shot down, chances 0 | Shot down, chances 1 | On the ground, 0 | On the ground, 1 |
//! | --- | -------------------- | -------------------- | ---------------- | ---------------- |
//! | 1 | 3 | 3 + 3 = 6 | 0 | 0 + 16 = 16 |
//! | 2 | 2 | 2 + 4 = 6 | 2 | 2 + 13 = 15 |
//! | 3 | 0 | 0 + 5 = 5 | 0 | 0 + 11 = 11 |
//!
//! The defending side's claim, each day. Shot down: what the patrol and
//! the guns really shot down, plus all their misses at chance 1.
//!
//! | Day | Shot down, chances 0 | Shot down, chances 1 |
//! | --- | -------------------- | -------------------- |
//! | 1 | 1 + 2 + 0 = 3 | 3 + 5 + 4 + 20 = 32 |
//! | 2 | 1 + 1 + 0 = 2 | 2 + 5 + 2 + 20 = 29 |
//! | 3 | 1 + 1 + 3 = 5 | 5 + 4 + 1 + 17 = 27 |
//!
//! Neither side claims a ship hit or sunk: no ship was attacked. So there
//! are six claims in the three days, two a day, at any setting.
//!
//! At the placeholder chances (fighters 0.1, guns 0.05, bombs 0.1) the
//! defending side on day 1 expects 3 + 9 x 0.1 + 20 x 0.05 = 4.9 claimed
//! against 3 real. That is not asserted for one seed.
//!
//! ## The unopposed strike, `05b_strike.toml`
//!
//! Group 2 is launched at contact 1, taken for a carrier, and drops 18
//! bombs on ship 3 for 5 hits. Ship 3 ends with flotation 50 and fire 0; no
//! ship sinks and none burns. No fighter or gun fires.
//!
//! The Allied claim: hits on ships 5 at chances 0, and 5 + 13 = 18 at
//! chances 1; ships sunk 0 at both (nothing sank and no fire reaches 30);
//! the kind is carrier, what the contact was taken for at the launch,
//! although the report after the strike then reads it as a cruiser;
//! aircraft 0. The Japanese side fired nothing and gets no claim.
//!
//! # Properties
//!
//! - With claims in the game, every event that is not a claim is what it
//!   was before, and so is every ship, aircraft and base at the end: only
//!   claim events and the claims in belief are added. Checked on scenarios
//!   5, 6 and 7 against their event logs from before this change, with the
//!   claim events taken out.
//! - A claim is never below the real count it rests on, for every claim,
//!   over seeds 1 to 200 of scenario 5 and of the base raid.
//! - Over seeds 1 to 1000 of the base raid at the placeholder chances, the
//!   defending side's false claims (claimed less real, summed over all
//!   claims and seeds) are within 10 in 100 of the sum over the same runs of
//!   fighter misses x 0.1 + gun misses x 0.05.
//! - A side's claims are the same in two worlds that differ only in
//!   something that side cannot see and that did not change the firing: a
//!   claim holds no identity of an enemy unit.
//! - Run twice, and from a saved game, the claims are the same.
