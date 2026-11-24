//! Stage 4's proof, case W2: a carrier force believed absent that is
//! there (decision 0161, condition 8). The Japanese side has a carrier
//! and two cruisers lying at (9,0) and believes no enemy carrier is in
//! these waters. One is, 200 nm west, searching the arc they lie in and
//! striking what it finds, every day for a week. A trial of the Japanese
//! side holds no enemy in any world.
//!
//! The data is placeholder data: see
//! `data/scenarios/stage-4-proof/case_w2_a_carrier_believed_absent.toml`,
//! which is scenario 5b, the unopposed strike, with 7 days in place of 1,
//! a file of sides, the trial's figures and one belief. It runs on the
//! frozen step 1 figures.
//!
//! **Written before the case is built.** Every test below is empty and
//! marked `#[ignore = "statement written, not built"]`. It is a case of a
//! proof. Its trial is worked by reasoning and may be played by its test.
//! Its 200 games are the judged run's and are played once, by the
//! harness: no test below plays a game of seeds 1 to 200.
//!
//! # The working
//!
//! Written by the architect before the code. If the code disagrees, the
//! code or the rule is wrong: do not change a number here to make a test
//! pass. A mismatch goes back to the architect.
//!
//! It is a statement of direction. The trial's side is certain. The
//! game's side rests on scenario 5b's hand calculation, which is of one
//! day, and is carried over seven days as a rough sum and no more; the
//! mark is set far below it. The architect played the file once, on its
//! own seed, 1942, to see that it loads and runs for seven days. Its
//! last lines showed an Allied claim of hits and a sinking on day 6;
//! nothing else of it was read, and the mark below was set from the
//! working. Scenario 5b's own test plays seeds 1 to
//! 1,000 of its one day, as it always has; nobody has read them from the
//! Japanese side.
//!
//! ## The rules it reads, and their figures
//!
//! Scenario 5b's, as its hand calculation gives them (decisions 0026,
//! 0028 and 0033): eight scouts on an arc of 90 degrees to 300 nm sight a
//! force at 200 nm with a chance of 0.509295 in each daylight pulse; a
//! report names the true hex with a chance of 0.7; the eighteen dive
//! bombers launch at a contact of that pulse or the one before and attack
//! if the force is in the hex they flew to; each bomb hits with a chance
//! of 0.2; a hit does 10 points, on flotation with a chance of 0.75; ten
//! hits on flotation sink the carrier. And of the trial (decision 0174):
//! a force the side believes absent is in no world, and the list of what
//! was assumed says the side ruled it out.
//!
//! ## Inputs
//!
//! Scenario 5b's forces, air groups and orders, unchanged. Task force 1,
//! Allied: carrier A and a destroyer at (4,0), with eight scouts on the
//! search and eighteen dive bombers on the naval strike. Task force 2,
//! Japanese: carrier B with no aircraft and two cruisers at (9,0).
//! Nothing moves. 7 days.
//!
//! **What the Japanese side believes:** a believed force, "Allied carrier
//! force believed elsewhere", a carrier, two ships, believed absent. It
//! holds no contact.
//!
//! **The trial:** of the Japanese side, from the evening before day 1,
//! with an empty course, 8 worlds for 7 days, the small size, which is
//! the length of the game.
//!
//! ## In every world
//!
//! The side's own half is task force 2. The enemy's half is nothing: no
//! contact makes a force, and the one believed force is believed absent
//! and is made in no world. No ship or aircraft of an enemy exists, so
//! nothing can hit a ship of the side's on any day. The side's ships
//! sunk, ships out of action and aircraft lost are 0 in each of the 8
//! worlds, and so 0 at the lowest, the middle and the highest. The list
//! of what was assumed names the believed force as ruled out and in no
//! world.
//!
//! ## In the game
//!
//! By scenario 5b's working, on a day the bombers attack in the morning
//! with a chance of 0.509295 x 0.7 = 0.356506, and launch in the
//! afternoon with a chance of 0.684234. An attack is 18 bombs at 0.2,
//! 3.6 hits, of which 2.7 fall on flotation. Taking the morning's attack
//! alone, a day gives 0.356506 x 2.7 = 0.96 hits on flotation and seven
//! days 6.7; with the afternoons, which attack about as often again, the
//! week gives about twice that, against the ten that sink the carrier.
//! So in most games carrier B is sunk within the week. The sum is rough:
//! it does not follow the contact from day to day, nor the bombers'
//! turn to the cruisers once she is gone.
//!
//! ## What the condition reads
//!
//! - **The direction.** The estimate's highest for the side's ships sunk
//!   is 0. Over the 200 games, seeds 1 to 200, each played 7 days, the
//!   side has a ship sunk in at least 100. The mark is the 100. The
//!   count, and the games in which a ship of the side's has any damage,
//!   are reported.
//! - **What was assumed.** The estimate's list names the Allied carrier
//!   force as believed absent, ruled out and in no world.
//!
//! If the trial cannot be made at all, because a picture with no enemy in
//! it is refused, that is reported as the case's own fault and comes to
//! the architect before the judged run.
#![allow(clippy::doc_lazy_continuation, clippy::doc_markdown)]

/// The file loads, plays 7 days, and the Japanese side's belief is one
/// carrier force of two ships, believed absent.
#[test]
#[ignore = "statement written, not built"]
fn the_case_loads_with_its_belief() {}

/// In each of the 8 worlds no enemy ship, aircraft or base exists, and at
/// the end of day 7 no ship of the side's has any damage.
#[test]
#[ignore = "statement written, not built"]
fn in_every_world_there_is_no_enemy() {}

/// The estimate's ships sunk, ships out of action and aircraft lost are 0
/// at the lowest, the middle and the highest.
#[test]
#[ignore = "statement written, not built"]
fn the_estimate_costs_nothing() {}

/// What was assumed names the believed force as ruled out and in no world.
#[test]
#[ignore = "statement written, not built"]
fn what_was_assumed_names_the_carrier() {}
