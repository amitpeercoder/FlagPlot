//! Scenario 28: a raw unit trains to the standard and no further; a unit one
//! day of fighting short of a step gains it; experience moves the chance a
//! shot hits; and a unit that is not fed does not train.
//!
//! Proves decision 0097 (the quality of a land unit, and training) against a
//! hand calculation. The data is placeholder data: see
//! `data/scenarios/28_experience_and_training.toml`. The frozen step 1
//! figures set `experience_fire_strength` to 0, so this test gives the four
//! figures the values below, as the test of ships' experience does.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Where the dice decide (the
//! hits of a fight), the calculation gives the firing men, the shots and
//! the chance exactly; the tests read the hits from the run. Everything
//! else is asserted exactly. If the code disagrees, the code or the rule is
//! wrong: do not change a number here to make a test pass. A mismatch goes
//! back to the architect.
//!
//! ## Figures
//!
//! `experience_fire_strength` 0.25, `training_days_per_level` 30,
//! `training_ceiling` 2, `fight_days_per_level` 10. From the earlier stages:
//! `daily_supply_t_per_man` 0.005, `unfed_loss_share_per_day` 0.05,
//! `men_per_shot` 100, `hit_chance` 0.3, `defender_hit_chance` 0.6,
//! `men_per_hit` 20, `fatigue_per_attack_day` 20.
//!
//! The factor of experience on a unit's chance to hit is 1 + 0.25 x
//! (experience - 2): 0.5 at 0, 0.75 at 1, 1 at 2, 1.25 at 3 and 1.5 at 4.
//!
//! ## Inputs
//!
//! A placeholder map with four coast hexes far apart and no land links: A,
//! F, G and H. The game is 62 days.
//!
//! | Base | Side     | Hex | Supply |
//! | ---- | -------- | --- | ------ |
//! | 1    | Allied   | A   | 5000 t |
//! | 2    | Allied   | F   | 5000 t |
//! | 3    | Japanese | F   | 5000 t |
//! | 4    | Allied   | G   | 5000 t |
//! | 5    | Japanese | G   | 5000 t |
//! | 6    | Allied   | H   | 1 t    |
//!
//! | Unit | Side     | Men  | At | Experience | Days counted at the start | Posture |
//! | ---- | -------- | ---- | -- | ---------- | ------------------------- | ------- |
//! | 1    | Allied   | 1000 | A  | 0          | none                      | defend  |
//! | 2    | Allied   | 1000 | F  | 2          | 9 days of fighting        | defend  |
//! | 3    | Japanese | 600  | F  | 3          | none                      | attack  |
//! | 4    | Allied   | 500  | G  | 0          | none                      | attack  |
//! | 5    | Japanese | 500  | G  | 4          | none                      | defend  |
//! | 6    | Allied   | 100  | H  | 1          | none                      | defend  |
//!
//! Orders: on day 1, units 1 and 6 are set to train.
//!
//! A unit's day is settled in the ground combat phase, after the fights. A
//! day of training counts if all its men are ashore at a base of its side,
//! it was fed in full that night, it did not march and its hex was not
//! contested.
//!
//! ## Unit 1: training to the standard
//!
//! It is fed in full every night (5 t a night from base 1) and nothing
//! disturbs it.
//!
//! | After day | Days of training counted | Experience |
//! | --------- | ------------------------ | ---------- |
//! | 29        | 29                       | 0          |
//! | 30        | 0, the count starts again | 1         |
//! | 59        | 29                       | 1          |
//! | 60        | 0                        | 2          |
//! | 62        | 0                        | 2          |
//!
//! At 2 it is at the ceiling of training. Days 61 and 62 are spent training
//! and count nothing. Base 1 has paid 62 x 5 = 310 t and holds 4690 t.
//!
//! ## Unit 6: not fed, not trained
//!
//! Base 6 has 1 t. Nights of days 1 and 2: 100 x 0.005 = 0.5 t each, fed in
//! full; days 1 and 2 count. From the night of day 3 the base has no stock:
//! the unit is unfed, loses men by the unfed rule, and no day counts. At the
//! end it has 2 days of training counted and its experience is 1.
//!
//! Its men: 100 x 0.05 = 5 lost on the night of day 3, leaving 95; then its
//! men times 0.05, cut, and one man at least, each night after.
//!
//! ## F: a step from fighting, and the chance
//!
//! Both sides are fed in full every night. The Japanese side attacks and
//! the Allied side defends.
//!
//! Day 1.
//!
//! - Unit 3, experience 3: 600 firing men, 6 shots at 0.3 x 1.25 = 0.375.
//! - Unit 2, experience 2: 1000 firing men, 10 shots at 0.6 x 1 = 0.6.
//!
//! Both fired. Unit 2 had 9 days of fighting counted: this is its tenth, so
//! its experience rises to 3 and its count starts again at 0. Unit 3 has 1
//! day counted and stays at 3.
//!
//! Day 2, if both still have men: unit 2 now fires at 0.6 x 1.25 = 0.75.
//! Unit 3 fires at 0.375 with its men times 80/100, cut, since its fatigue
//! is 20.
//!
//! Each later day of fighting adds one to the count of each unit that
//! fired. Unit 3 attacks on four days and then goes back to defend; there
//! is no fight after that. A unit's experience at the end is what follows
//! from the days it fired: unit 2 is 3 unless it fired on ten more days,
//! which it cannot here.
//!
//! ## G: raw against the best
//!
//! Day 1. The Allied side attacks and the Japanese side defends.
//!
//! - Unit 4, experience 0: 500 firing men, 5 shots at 0.3 x 0.5 = 0.15.
//! - Unit 5, experience 4: 500 firing men, 5 shots at 0.6 x 1.5 = 0.9.
//!
//! The breakdown of each unit's fire names its experience and the factor.
//!
//! # Properties
//!
//! - Experience is never below 0 nor above 4, training never takes a unit
//!   above the ceiling, and a day counts for one thing only.
//! - With the frozen strength of 0, every scenario with land units gives
//!   what it gave before, apart from the hash version.
//! - The enemy's view of a fight is the same in two worlds that differ only
//!   in the other side's experience, where the hits are the same.
