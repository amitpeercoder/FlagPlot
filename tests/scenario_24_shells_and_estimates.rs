//! Scenario 24: a force bombards a base on two nights, and an attack is
//! made on an opening estimate that is far too low.
//!
//! Proves decision 0092 (shells and bombs on a base and its men, and what a
//! side knows of the enemy ashore) against a hand calculation. The data is
//! placeholder data: see `data/scenarios/24_shells_and_estimates.toml`. The
//! test runs on the frozen step 1 figures (`data/reference/step1`), to which
//! the new figures are added with their first values.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Where the dice decide, the
//! calculation gives the shots and the chance exactly and what must follow
//! from what the run rolls; the tests read the rolls from the run's events
//! and assert the rest. Everything else is asserted exactly. If the code
//! disagrees, the code or the rule is wrong: do not change a number here to
//! make a test pass. A mismatch goes back to the architect.
//!
//! ## Figures
//!
//! - `surface_combat.toml`: `bombardment_minutes` 30, `bombardment_hit_chance`
//!   0.05, `bombardment_reference_gun_damage` 20,
//!   `bombardment_false_claim_chance` 0.01.
//! - `air_combat.toml`, frozen: `[base_hit]` aircraft 0.3, supply 0.3, and
//!   the new men 0.2, so 0.2 of hits land on nothing;
//!   `base_supply_destroyed_per_hit_t` 50.
//! - `ground.toml`: `men_lost_per_base_hit` 10, `report_error_low` 0.5,
//!   `report_error_high` 1.5, and the figures of stages 1 and 2
//!   (`daily_supply_t_per_man` 0.005, `men_per_shot` 100, `hit_chance` 0.3,
//!   `defender_hit_chance` 0.6, `men_per_hit` 20, `fatigue_per_attack_day`
//!   20, `attack_fatigue_limit` 60, `beaten_share` 0.2).
//! - Classes, frozen: the placeholder cruiser has 8 guns at 3 rounds a
//!   minute, 100 rounds a gun and a gun damage of 20; it cruises at 16 kn and
//!   burns 2000 t over 8000 nm, 0.25 t a mile. The placeholder destroyer has
//!   4 guns at 6 rounds a minute, 150 rounds a gun and a gun damage of 5; it
//!   cruises at 16 kn and burns 0.125 t a mile.
//!
//! ## Inputs
//!
//! A placeholder map with two coast hexes on one row, X at (0,0) and Y at
//! (8,0), and sea between. The game is 4 days.
//!
//! | Base | Side     | Hex | Supply | On the enemy's map |
//! | ---- | -------- | --- | ------ | ------------------ |
//! | 1    | Allied   | X   | 2000 t | yes                |
//! | 2    | Allied   | Y   | 1000 t | yes                |
//! | 3    | Japanese | Y   | 100 t  | no                 |
//!
//! - Allied air group 1 at base 1: placeholder fighters, 20 ready and 4
//!   damaged, no mission. Base 1 repairs no aircraft and has no guns.
//! - Allied land unit 1: 1000 men ashore at X, on defend.
//! - Allied land unit 2: 3000 men ashore at Y, on defend.
//! - Japanese land unit 3: 800 men ashore at Y, on attack from the start.
//! - Japanese task force 1, mission bombardment, at sea at (2,0): ship 1, a
//!   placeholder cruiser with 800 rounds, and ship 2, a placeholder
//!   destroyer with 600 rounds. Both are full of fuel: 2000 t and 500 t.
//! - The Japanese side's opening estimate: 300 Allied men at Y.
//!
//! Orders, Japanese: on day 1, task force 1 to (0,0) at cruise; on day 3,
//! task force 1 to (2,0) at cruise. There is no other order.
//!
//! The air supply figures are not loaded, so base 1 pays nothing for its
//! aircraft. Each night base 1 pays 0.005 t for each man of unit 1.
//!
//! ## X: the bombardment
//!
//! Day 1, night. The men's supply is charged first: base 1 pays 1000 x 0.005
//! = 5 t and holds 1995 t. Then the force steams 2 hexes, 80 nm, at 16 kn,
//! and arrives at (0,0) 5 hours into the pulse. The cruiser burns 20 t and
//! has 1980 t; the destroyer burns 10 t and has 490 t. Its position after
//! movement holds base 1, an enemy base on its map: it bombards.
//!
//! | Night | Ship      | Shells: rate x guns x 30 | Rounds aboard | Shells fired | Weighed shots        | Rounds left |
//! | ----- | --------- | ------------------------ | ------------- | ------------ | -------------------- | ----------- |
//! | 1     | Cruiser   | 3 x 8 x 30 = 720         | 800           | 720          | 720 x 20/20 = 720    | 80          |
//! | 1     | Destroyer | 6 x 4 x 30 = 720         | 600           | 600          | 600 x 5/20 = 150     | 0           |
//! | 2     | Cruiser   | 720                      | 80            | 80           | 80                   | 0           |
//! | 2     | Destroyer | 720                      | 0             | 0: it does not fire | none          | 0           |
//!
//! Every weighed shot hits at 0.05. For each hit one draw puts it on
//! aircraft (0.3), supply (0.3), men (0.2) or nothing (0.2).
//!
//! For each night, with a, s and m the hits of the whole force on aircraft,
//! supply and men, read from the run:
//!
//! - Aircraft destroyed are a, or all that are left at the base if fewer.
//!   Damaged aircraft are destroyed before ready ones. There are 24 before
//!   the first night.
//! - Supply destroyed is 50 x s t, or all that is left if less.
//! - Men lost are 10 x m, or all of unit 1 that are left if fewer.
//! - On night 1 the hits are at most 870 and on night 2 at most 80.
//!
//! The Japanese side gets one claim for each night the force fired. Its
//! aircraft claimed on the ground are the aircraft really destroyed that
//! night plus false ones, rolled from the weighed shots (870 on night 1, 80
//! on night 2) less the aircraft really destroyed, at 0.01. It is never
//! below the aircraft really destroyed. The Japanese side is told its
//! shells fired, 1320 on night 1 and 80 on night 2, and no hit.
//!
//! The Allied side gets a sighting report of the force on each night it
//! fired, and so holds a contact at (0,0) or a hex beside it. It sees the
//! shots against base 1 and where each hit landed, and is not told which
//! force or ship fired.
//!
//! Day 2. No order: the force stays at (0,0). On the night of day 2 base 1
//! pays 0.005 t for each man unit 1 has left, and the force bombards again
//! by the table's second pair of rows.
//!
//! Day 3, night. The force steams back to (2,0): 80 nm, 20 t and 10 t more,
//! so the cruiser ends with 1960 t and the destroyer with 480 t. It is not
//! in a hex with an enemy base after movement and does not bombard. It
//! would not in any case: no ship has a round left. On the nights of days 3
//! and 4 nothing is fired.
//!
//! At the end: ship 1 has 0 rounds and ship 2 has 0. Base 1's supply is 2000
//! less the four nightly charges for unit 1's men less the supply destroyed
//! on the two nights.
//!
//! ## Y: an attack on a wrong estimate
//!
//! Before day 1 the Japanese side holds an estimate of 300 Allied men at Y,
//! dated day 0. The Allied side holds no estimate, and base 3 is not on its
//! map.
//!
//! Every night both sides at Y are fed in full: base 2 pays 0.005 t for each
//! man of unit 2 and base 3 for each man of unit 3. Every fed share is 1.
//!
//! Day 1, the fight. The Japanese side attacks and the Allied side defends.
//!
//! - Unit 3: 800 firing men, 8 shots at 0.3.
//! - Unit 2: 3000 firing men, 30 shots at 0.6.
//!
//! With j the hits of unit 3 and a the hits of unit 2: unit 2 has 3000 less
//! 20 x j, which is at least 2840, and unit 3 has 800 less 20 x a, or none.
//! Unit 3's fatigue is 20.
//!
//! Beaten: only the Allied side is tested, and it has at least 2840 men
//! against 0.2 times at most 800. It is not beaten on any day. Base 2 is
//! Allied at the end and base 3 is Japanese while unit 3 has a man ashore.
//!
//! The ground reports of day 1, made after the fight if both sides still
//! have men ashore:
//!
//! - The Japanese side's estimate of Allied men at Y is replaced. With A the
//!   men unit 2 has after the fight, the new estimate is between 0.5 x A
//!   and 1.5 x A, cut to a whole man, so it is at least 1420: nearly five
//!   times the 300 it attacked on. It is dated day 1.
//! - The Allied side gets its first estimate of Japanese men at Y: between
//!   0.5 and 1.5 times the men unit 3 has after the fight, cut. Base 3 is
//!   put on the Allied map, with an event the Allied side sees.
//!
//! Each later day that both sides have men ashore at Y, unit 3 attacks again
//! with its men times 100 less its fatigue over 100, cut, as firing men
//! (fatigue 20, then 40, then 60 before the fights of days 2, 3 and 4), and
//! each side's estimate is replaced by a new report within the same bounds
//! of that day's truth. On a day unit 3 has no man left there is no fight
//! and no report, and each side keeps the estimate it had, with its day.
//!
//! If unit 3 is destroyed on day 1, no report is made at Y that day: the
//! Japanese side keeps its estimate of 300, the Allied side has none and
//! base 3 stays off its map. The test then says so and the architect picks
//! another seed.
//!
//! # Properties
//!
//! Beside these, the tests check what must always hold (decision 0092, "The
//! test"): a side's view is the same in two worlds that differ only in enemy
//! men it has no report of; a ground report always lies between the low and
//! high share of the truth; a bombardment never destroys more than is
//! there; a raid from the air on a base with no men ashore gives the hits it
//! gave before the table gained its share for men; rounds are conserved;
//! and a scenario with no bombardment mission, no men and no estimate is
//! unchanged apart from the hash version.
