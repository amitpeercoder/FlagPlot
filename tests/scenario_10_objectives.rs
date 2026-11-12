//! Objectives and results (decision 0054), on scenario 5, Eastern Solomons.
//!
//! # Hand calculation
//!
//! Written by the architect before the test code. The tests below assert
//! these numbers. If the code disagrees, the code or the rule is wrong: do
//! not change a number here to make a test pass.
//!
//! The scenario is `data/scenarios/05_eastern_solomons.toml`, seed 1942, as
//! it runs today. Nothing of the battle changes: the objectives only read
//! what happened. What happened, from the event log of that run:
//!
//! | Fact | Count |
//! | ---- | ----- |
//! | Japanese ships sunk | 1: ship 33, Ryujo, a carrier |
//! | Japanese ships afloat with propulsion or flight deck out | 0 |
//! | Allied ships sunk | 0 |
//! | Allied ships afloat with propulsion or flight deck out | 1: ship 1, Enterprise, a carrier, flight deck out |
//! | Japanese aircraft lost | group 11: 3; group 12: 10 and 5; group 14: 1; group 15: 8 and 4; lost with Ryujo, group 18: 23 and group 19: 6. 3 + 15 + 1 + 12 + 29 = 60 |
//! | Allied aircraft lost | group 2: 1; group 6: 1; group 1: 2 and 1. 5 |
//!
//! Both sides hold the same three objectives on the placeholder scale:
//! ships sunk (carrier 10, battleship 8, cruiser 4, destroyer 1, transport
//! 2, auxiliary 1), ships crippled (half of each), and aircraft destroyed
//! at 0.05 each. Levels: minor 3, major 8, decisive 16.
//!
//! | Objective | Allied count | Allied points | Japanese count | Japanese points |
//! | --------- | ------------ | ------------- | -------------- | --------------- |
//! | Ships sunk | 1 carrier | 10 | 0 | 0 |
//! | Ships crippled | 0 | 0 | 1 carrier | 5 |
//! | Aircraft destroyed | 60 | 60 x 0.05 = 3 | 5 | 5 x 0.05 = 0.25 |
//! | Total | | 13 | | 5.25 |
//!
//! Margin for the Allied side: 13 - 5.25 = 7.75. It is at least 3 and less
//! than 8: a minor Allied victory, and so a minor Japanese defeat, margin
//! -7.75.
//!
//! What each side knows of its progress at the end is not fixed here: it
//! depends on which events decision 0047 lets a side see, and the developer
//! reports it. Two things are fixed. A side's count of its own losses is not
//! part of its progress (they are the enemy's points). And a side's known
//! count for an objective is never above the true count.
//!
//! # Properties
//!
//! - The result is the same run twice, and the same from a saved game
//!   replayed.
//! - The two sides' margins sum to zero, and their levels mirror.
//! - The state hash of every day is what it was before objectives existed
//!   (the golden files under `crates/sim-cli/tests/run_baseline` do not
//!   change).
//! - Sinking one more enemy ship, or losing one fewer of one's own, never
//!   lowers a side's margin (checked on the scoring function with made-up
//!   counts).
//! - Over seeds 1 to 200 the known count of each objective is never above
//!   the true count, for both sides.
