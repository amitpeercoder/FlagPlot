//! Reach beyond the radius and running out of fuel (decision 0059, stage 1).
//!
//! # Hand calculation
//!
//! Written by the architect before the test code. The tests below assert
//! these numbers. If the code disagrees, the code or the rule is wrong: do
//! not change a number here to make a test pass.
//!
//! The scenario is `data/scenarios/12a_long_reach.toml`: scenario 5b, the
//! unopposed strike, with the enemy carrier force moved from (9,0) to
//! (11,0) and the bombing squadron given 18 pilots. Everything else is as in
//! 5b: the Allied carrier force lies in (4,0); group 1, 8 scouts, searches
//! the arc 040 to 130 out to 300 nm; group 2, 18 dive bombers, is on naval
//! strike. The dive bomber's strike radius is 250 nm.
//!
//! ## Distances and reach
//!
//! (4,0) to (11,0) is 7 hexes along an axis: 280 nm by air. That is 30 nm
//! beyond the radius. Extended reach is 250 x (1 + 0.25) = 312.5, cut to
//! 312 nm, so the target is within extended reach and outside normal reach.
//!
//! ## The search
//!
//! The scouts' chance to sight in a daylight pulse is 57.295779 x 8 x 20 /
//! (280 x 90): 9167.32464 / 25200 = 0.363782 (cut). It is 280 nm inside a
//! range of 300.
//!
//! ## With the bombing squadron on normal reach
//!
//! No strike is launched in any pulse, whatever is sighted. Nothing ditches.
//! The squadron ends with 18 aircraft and 18 pilots.
//!
//! ## With the bombing squadron on extended reach
//!
//! In the pulse the force is sighted and reported in its true hex, the
//! squadron launches all 18 at 280 nm. No fighter or gun fires (the enemy
//! has none), so 18 attack: 18 bombs at 0.2, 3.6 hits expected, as in 5b.
//!
//! All 18 come back from the target. The chance that one runs out of fuel
//! is 2 x (280 - 250) / 250: 30 / 250 = 0.12, times 2 is 0.24. Expected
//! ditched: 18 x 0.24 = 4.32. Each ditched aircraft's crew is recovered
//! with chance 0.7.
//!
//! The rolls cannot be fixed by hand, so three settings of the figures are:
//!
//! | `ditch_chance_per_radius_beyond` | `crew_recovered_short_of_home` | Chance | Ditched of 18 | Pilots lost | Squadron after |
//! | --- | --- | --- | --- | --- | --- |
//! | 0  | any | 0 | 0  | 0  | 18 aircraft, 18 pilots, less what wear writes off |
//! | 10 | 1   | 10 x 0.12 = 1.2, so 1 | 18 | 0  | 0 aircraft, 18 pilots |
//! | 10 | 0   | 1 | 18 | 18 | 0 aircraft, 0 pilots |
//!
//! With every aircraft ditched, none is rolled for wear: the wear event of
//! the strike sortie, if wear is in play, rolls 0 aircraft. The fight
//! itself (the sighting, the launch, the 18 bombs and their hits) is the
//! same at every setting.
//!
//! # Properties
//!
//! - On normal reach the log holds no launch and no ditching, at any
//!   setting.
//! - The aircraft ditched are never more than those that came back, and the
//!   pilots lost never more than the aircraft ditched.
//! - A group that flew within its radius never ditches: every scenario that
//!   existed before this rule gives the events it gave before, and only its
//!   hash lines change.
//! - Over seeds 1 to 1000 at the placeholder figures, on the runs where the
//!   strike is launched, the aircraft ditched sum to within 10 in 100 of
//!   0.24 times the aircraft that came back, and the crews recovered to
//!   within 10 in 100 of 0.7 times the aircraft ditched.
//! - The chance does not fall as the distance rises: at 290 and 300 nm
//!   (targets one and two hexes off the axis are not needed; use the
//!   function) it is 0.32 and 0.4.
