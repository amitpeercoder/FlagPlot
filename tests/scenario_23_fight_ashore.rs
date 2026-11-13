//! Scenario 23: three places ashore. An attack on a fed defender that fails
//! and stops by fatigue; an unfed defender beaten on the first day, whose
//! base changes side; and an empty base that falls without a fight.
//!
//! Proves decision 0090 (the fight ashore, and a base that changes side)
//! against a hand calculation. The data is placeholder data: see
//! `data/scenarios/23_fight_ashore.toml`. The test runs on the frozen step 1
//! figures (`data/reference/step1`), to which the nine new ground figures
//! are added with their first values.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Where the dice decide, the
//! calculation gives the firing men, the shots and the chance exactly, and
//! what must follow from the hits the run rolls; the tests read the hits
//! from the run and assert the rest. Everything else is asserted exactly. If
//! the code disagrees, the code or the rule is wrong: do not change a number
//! here to make a test pass. A mismatch goes back to the architect.
//!
//! ## Figures
//!
//! `ground.toml`: `daily_supply_t_per_man` 0.005, `unfed_loss_share_per_day`
//! 0.05, `men_per_shot` 100, `hit_chance` 0.3, `defender_hit_chance` 0.6,
//! `men_per_hit` 20, `fatigue_per_attack_day` 20, `rest_per_day` 5,
//! `attack_fatigue_limit` 60, `beaten_share` 0.2, `captured_stock_share` 0.5.
//!
//! No air figures are loaded, so no aircraft flies and no base pays for
//! aircraft. The placeholder fighter has a strike range of 250 nm, so its
//! transfer range is 500 nm.
//!
//! ## Inputs
//!
//! A placeholder map with three coast hexes on one row: X at (0,0), Y at
//! (6,0) and Z at (10,0). The game is 6 days. Each night the men's supply is
//! charged before anything else in this scenario; the fight is after the
//! third pulse of the day.
//!
//! | Base | Side     | Hex | Supply | Fuel |
//! | ---- | -------- | --- | ------ | ---- |
//! | 1    | Allied   | X   | 1000 t | 0    |
//! | 2    | Japanese | X   | 100 t  | 0    |
//! | 3    | Japanese | Y   | 0 t    | 80 t |
//! | 4    | Allied   | Y   | 100 t  | 0    |
//! | 5    | Japanese | Z   | 60 t   | 30 t |
//!
//! | Land unit | Side     | Men  | Ashore at | Posture at the start |
//! | --------- | -------- | ---- | --------- | -------------------- |
//! | 1         | Allied   | 1000 | X         | defend               |
//! | 2         | Japanese | 600  | X         | attack               |
//! | 3         | Japanese | 300  | Y         | defend               |
//! | 4         | Allied   | 2000 | Y         | attack               |
//! | 5         | Allied   | 50   | Z         | attack               |
//!
//! Japanese air groups, all of the placeholder fighter, with no mission:
//! group 1 at base 3 with 9 ready and 2 damaged; group 2 at base 5 with 5
//! ready and 1 damaged; group 3 at base 5 with 8 ready.
//!
//! A Japanese placeholder destroyer, ship 1, lies in port at base 3, in no
//! task force. Japanese task force 1, of one placeholder destroyer, ship 2,
//! is stopped at sea at (3,1) with base 3 as its home port.
//!
//! Orders: on day 1 the Japanese side orders air group 3 to rebase to base 2.
//! Z to X is 10 hexes, 400 nm, within 500. There is no other order.
//!
//! Objectives. Allied: `base_held` base 3 for 10 points, `base_held` base 5
//! for 5, `ships_sunk` with 2 points for a destroyer, `men_lost` at 1 point
//! for each 100 men. Japanese: `base_held` base 1 for 10 points, `men_lost`
//! at 1 point for each 100 men.
//!
//! ## Z: an empty base
//!
//! Day 1 at intake: group 3 moves to base 2. It has no damaged aircraft, so
//! none is written off.
//!
//! Night of day 1: the Allied side has no base at Z. Unit 5's 50 men are
//! unfed: 50 x 0.05 = 2.5, cut to 2 lost. 48 are left.
//!
//! Day 1, ground combat: the Japanese side has no man at Z, so the hex is
//! not contested and there is no fight. Unit 5 is on attack and the Japanese
//! side has a base there and no man: it is beaten. Base 5 becomes Allied,
//! with 60 x 0.5 = 30 t of supply and 30 x 0.5 = 15 t of fuel. Air group 2 is
//! lost with 5 + 1 = 6 aircraft. Air group 3 is not there and is not
//! touched: at the end it is at base 2 with 8 ready. Unit 5 did not fire, so
//! it gains no fatigue; it stays on attack with fatigue 0 to the end.
//!
//! From the night of day 2 unit 5 is fed by base 5: 48 x 0.005 = 0.24 t a
//! night, all 48 fed.
//!
//! | Night of | Base 5 supply after |
//! | -------- | ------------------- |
//! | Day 2    | 29.76 t             |
//! | Day 3    | 29.52 t             |
//! | Day 4    | 29.28 t             |
//! | Day 5    | 29.04 t             |
//! | Day 6    | 28.8 t              |
//!
//! At the end: unit 5 has 48 men, base 5 is Allied with 28.8 t of supply and
//! 15 t of fuel.
//!
//! ## Y: an unfed defender
//!
//! Night of day 1. Base 3 has no supply: unit 3's 300 men are unfed, nothing
//! is paid, 300 x 0.05 = 15 are lost and 285 are left. Its fed share is 0.
//! Base 4 pays 2000 x 0.005 = 10 t for unit 4 and holds 90 t. Unit 4's fed
//! share is 1.
//!
//! Day 1, the fight. The Allied side attacks and the Japanese side defends.
//!
//! - Unit 4: firing men 2000 x 1 x 100/100 = 2000. 20 shots at 0.3.
//! - Unit 3: firing men 285 x 0 = 0. No shot.
//!
//! Let b1 be unit 4's hits, from 0 to 20. Unit 3 loses 20 x b1 men, or all
//! 285 if b1 is 15 or more. Unit 4 loses none. Unit 4's fatigue is 20; unit
//! 3 rests and stays at 0.
//!
//! Beaten: the Allied side attacked, and the Japanese side has at most 285
//! men against 0.2 x 2000 = 400. It is beaten whatever b1 is. Base 3 becomes
//! Allied: supply 0, fuel 80 x 0.5 = 40 t. Air group 1 is lost with 9 + 2 =
//! 11 aircraft. Ship 1 is sunk. Task force 1 has no home port.
//!
//! After day 1 the Japanese side has no base at Y. Each night what is left
//! of unit 3 is unfed, loses its men times 0.05, cut, and one man at least,
//! and its fed share is 0, so it never fires. The Allied side has two bases
//! at Y. Each night its 2000 men ask 10 t: base 3 has no supply and is asked
//! nothing, and base 4 pays 10 t. All are fed.
//!
//! While unit 3 has a man ashore the hex is contested and unit 4 fires:
//!
//! | Day | Unit 4's fatigue before | Firing men          | Shots at 0.3 | Fatigue after |
//! | --- | ----------------------- | ------------------- | ------------ | ------------- |
//! | 1   | 0                       | 2000                | 20           | 20            |
//! | 2   | 20                      | 2000 x 80/100 = 1600 | 16          | 40            |
//! | 3   | 40                      | 2000 x 60/100 = 1200 | 12          | 60            |
//! | 4   | 60                      | 2000 x 40/100 = 800  | 8           | 80, and it goes to defend |
//!
//! On a day when unit 3 has no man left there is no fight: unit 4 does not
//! fire, gains nothing and loses 5, and stays on attack. Unit 3 is destroyed
//! when its last man is lost, to a hit or unfed. The test reads each day's
//! hits and unit 3's men, and asserts each row above for the days there was
//! a fight, and the rest for the days there was none.
//!
//! Whatever the dice: unit 4 has 2000 men at the end; base 4's supply is 90,
//! 80, 70, 60, 50 and 40 t after the nights of days 1 to 6; base 3 is Allied
//! from day 1 with 0 t of supply and 40 t of fuel; the Allied side lost no
//! man at Y.
//!
//! ## X: an attack that fails
//!
//! Every night both sides are fed in full, so every fed share is 1. Base 1
//! pays 0.005 t for each Allied man there, and base 2 for each Japanese man.
//!
//! The Japanese side attacks and the Allied side defends. Let A and J be the
//! men of units 1 and 2 before a day's fight, and a and j the hits that
//! units 1 and 2 roll that day.
//!
//! | Day | Unit 2's fatigue before | Unit 2's firing men | Its chance | Unit 1's firing men | Its chance | Unit 2's fatigue after |
//! | --- | ----------------------- | ------------------- | ---------- | ------------------- | ---------- | ---------------------- |
//! | 1   | 0                       | J = 600             | 0.3        | A = 1000            | 0.6        | 20                     |
//! | 2   | 20                      | J x 80/100, cut     | 0.3        | A                   | 0.6        | 40                     |
//! | 3   | 40                      | J x 60/100, cut     | 0.3        | A                   | 0.6        | 60                     |
//! | 4   | 60                      | J x 40/100, cut     | 0.3        | A                   | 0.6        | 80, and it goes to defend |
//!
//! A unit's shots are its firing men over 100; a part of a shot is rolled at
//! that part of the chance. On day 1 unit 2 has 6 shots and unit 1 has 10.
//! After each fight unit 1 has A less 20 x j, and unit 2 has J less 20 x a,
//! or none. Both fire on the men they had before the fight.
//!
//! Unit 1 rests and stays at fatigue 0. On days 5 and 6 no unit at X is on
//! attack and there is no fight; unit 2's fatigue is 75 and then 70.
//!
//! Nobody is beaten at X. The Allied side never attacks, so the Japanese
//! side is not tested. Unit 2 has at most 6 shots a day, so unit 1 loses at
//! most 120 men a day and has at least 520 after four days, against 0.2
//! times at most 600. Base 1 is Allied and base 2 is Japanese at the end.
//!
//! If the dice destroy unit 2 before day 4, the rows after its last day do
//! not happen; the test then says so and the architect picks another seed.
//!
//! Supply at X after each night: base 1 has paid 0.005 t for each man unit 1
//! had that night, and base 2 for each man unit 2 had.
//!
//! ## The result
//!
//! - Allied `base_held`: base 3 is Allied, 10 points; base 5 is Allied, 5.
//! - Allied `ships_sunk`: ship 1, 2 points.
//! - Allied `men_lost`: the Japanese men lost over 100. They are the 300 of
//!   unit 3 less its men at the end, and the 600 of unit 2 less its men at
//!   the end.
//! - Japanese `base_held`: base 1 is not Japanese, 0 points.
//! - Japanese `men_lost`: the Allied men lost over 100: the 2 of unit 5 and
//!   the 1000 of unit 1 less its men at the end.
//!
//! Aircraft: the Japanese side lost 17, the 11 of group 1 and the 6 of group
//! 2.
//!
//! # Properties
//!
//! Beside these, the tests check what must always hold (decision 0090, "The
//! test"): a fight never creates men; more attackers never do worse over a
//! batch; men are conserved; the order of firing does not matter; a base's
//! side and the invariants agree at the end of every day; a scenario with no
//! posture order is unchanged from stage 1; and the enemy's view of a fight
//! does not show the other side's fatigue, fed share or number of units.
