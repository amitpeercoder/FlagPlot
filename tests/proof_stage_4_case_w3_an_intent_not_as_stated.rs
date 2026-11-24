//! Stage 4's proof, case W3: an enemy intent not as stated (decision 0161,
//! condition 8). A landing is tried against a garrison the side has
//! counted rightly. The side knows of the enemy's second unit and its
//! transports at an anchorage of his own, and believes nothing of what he
//! means to do with them. In every world of the trial they stay where
//! they are. In the game he means to control the island, and when the
//! airfield is taken he lands the second unit on it.
//!
//! The data is placeholder data: see
//! `data/scenarios/stage-4-proof/case_w3_an_intent_not_as_stated.toml`.
//! It runs on the frozen step 1 figures with scenario 68's ground
//! figures, in which a whole shot always hits, and scenario 95's files of
//! the trial's figures and of sides.
//!
//! **Written before the case is built.** Every test below is empty and
//! marked `#[ignore = "statement written, not built"]`. It is a case of a
//! proof: the harness plays it once, for the judged run.
//!
//! # The working
//!
//! Written by the architect before the code. If the code disagrees, the
//! code or the rule is wrong: do not change a number here to make a test
//! pass. A mismatch goes back to the architect.
//!
//! It is a statement of direction, with the least the game must cost
//! worked by hand, and not a full hand calculation of the game: the
//! second fight has part shots, which hit by chance, and its later days
//! are not followed. One game was played by the architect on Nov 24,
//! 2026, on seed 1942, with the course written into a copy of the
//! scenario, to check the days and the least loss below; it agreed. No
//! trial of the case has been played.
//!
//! ## The rules it reads, and their figures
//!
//! Scenario 68's, as scenario 95's hand calculation gives them: a unit
//! fires one shot for each 100 firing men; a whole shot hits and takes 50
//! men; a part of a shot hits by chance; a side is beaten where the other
//! attacked and it has no man left, or fewer than 0.2 of the other's. A
//! size 0 port lands 80 men an hour for each ship. `preparation_per_day`
//! is 100, so a landing is ready the morning after it is ordered. And of
//! the staff: under "control an island" a take is formed for a named
//! place on the first morning the side holds no base there (decision
//! 0151); with no estimate short of his men it lifts the day after it is
//! ready (decisions 0142 and 0149).
//!
//! ## Inputs
//!
//! Scenario 95's, with the garrison at its estimate and the enemy's
//! second force added:
//!
//! | Base | Side | Hex | Supply |
//! | ---- | ---- | --- | ------ |
//! | 1 | Allied | A (0,0) | 1,000 t |
//! | 2 | Japanese | B (4,0) | 200 t |
//! | 3 | Japanese, on the Allied map | C (0,6) | 1,000 t |
//!
//! Unit 1, Allied, 600 men at A, with task force 1, two troop transports.
//! Unit 10, Japanese, 200 men at B. Unit 11, Japanese, 1,200 men at C,
//! with task force 2, two troop transports of 600 men each. C to B is
//! 212 nm, 21.2 hours at 10 kn. Island 1 is base 2.
//!
//! **What the Allied side holds:** an opening estimate of 200 men at B;
//! believed men, 1,200 at base 3, believed present; a believed force of
//! two transports at base 3, believed present; and no believed intent.
//!
//! **What the enemy means, in the game:** on day 1, control island 1,
//! with both leaves. It is in the scenario file. The trial is not told.
//!
//! **The course:** scenario 95's, the Allied side's four commands of day
//! 1: intent 1, take the place at (4,0); its forces limit, task force 1,
//! unit 1 and base 1; both leaves.
//!
//! **The trial:** of the Allied side, from the evening before day 1, 16
//! worlds for 14 days.
//!
//! ## In every world
//!
//! The enemy has base 2 with 200 men, base 3 with a unit of 1,200 men,
//! and a force of two transports at base 3. He is given no intent: he
//! holds what he holds, and nothing of his is ordered on any day. The
//! landing is scenario 95's worlds 1 and 3: the 600 land on day 4, fire
//! 6 hits and destroy the 200, who fire 2 hits; the side loses 100 men
//! and the place is its own at the end of day 4. Nothing lands at B
//! after. So the estimate says: met and kept in 16 of 16 worlds, on day
//! 4; lost again in none; the side's men lost to the enemy 100 at the
//! lowest, the middle and the highest; and, among what was assumed, that
//! the enemy was given no intent.
//!
//! ## In the game
//!
//! Days 1 to 4 are the worlds': the enemy's hold of base 2 leaves the 200
//! on the defence, and the side takes the place on day 4 with 100 men
//! lost and 500 left.
//!
//! | Day | The Japanese staff | What happens |
//! | --- | --- | --- |
//! | 5 | Base 2 is not his: forms a take of the place at B under the island. `prepare_landing` at B by task force 2, roster 11 | Loading and rehearsal 100 |
//! | 6 | Notice: the landing is ready. A load of supply | |
//! | 7 | Unit 11 set to attack. Load unit 11 | 600 men in each ship |
//! | 8 | A route to B and back to C, acting at B, and an unload order | Sails at hour 168 and arrives at hour 189.2, 3.2 hours into the afternoon. Each ship lands 2.8 x 80 = 224 men: 448 ashore. The fight |
//!
//! On day 8 the 448 fire 4 whole shots and a part of one: at least 4
//! hits, 200 men of unit 1. So by the end of day 8 the side has lost at
//! least 100 + 200 = 300 men, whatever the dice of the part shot and of
//! every later day.
//!
//! ## What the condition reads
//!
//! - **The direction.** The estimate's highest for the side's men lost
//!   to the enemy is 100, and it says the place is kept in every world.
//!   In the game the side's men lost by the end of day 8 are at least
//!   300. The estimate is too cheap, by at least 200 men, and it is so in
//!   every game: the mark is read on seeds 1 to 20, and holds in all 20.
//!   Whether the place is lost again by day 14 is reported and judged by
//!   nothing.
//! - **What was assumed.** The estimate's list names the men at base 3
//!   and the transports there as made in every world, and says that the
//!   enemy was given no intent.
#![allow(clippy::doc_lazy_continuation, clippy::doc_markdown)]

/// In every world of the trial: nothing of the enemy's is ordered, the
/// place is taken on day 4 and kept, and the side loses 100 men.
#[test]
#[ignore = "statement written, not built"]
fn in_every_world_the_second_unit_stays() {}

/// The estimate: met and kept in 16 of 16 worlds on day 4, lost again in
/// none, men lost 100 at the lowest, the middle and the highest.
#[test]
#[ignore = "statement written, not built"]
fn the_estimate_is_cheap() {}

/// What was assumed names the men and the transports at base 3 as made in
/// every world, and says the enemy was given no intent.
#[test]
#[ignore = "statement written, not built"]
fn what_was_assumed_names_the_intent() {}

/// The game, seeds 1 to 20: the take is formed on day 5, the landing is
/// ordered on days 5 to 8 as the table has it, 448 men are ashore on the
/// afternoon of day 8, and the side's men lost by the end of day 8 are at
/// least 300.
#[test]
#[ignore = "statement written, not built"]
fn in_the_game_he_comes_back() {}
