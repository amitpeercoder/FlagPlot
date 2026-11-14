//! Scenario 29: what a land unit is good at. Mountain troops march faster in
//! the mountains and no faster out of them; troops raised to land from the
//! sea rehearse a landing faster; a heavy unit fires more and eats more; and
//! troops fitted to hold ground hit more often when they defend.
//!
//! Proves decision 0098 (what a land unit is good at) against a hand
//! calculation. The data is placeholder data: see
//! `data/scenarios/29_skills.toml`. The frozen step 1 figures set every
//! skill's strength to 0, so this test gives the four strengths the values
//! below, as the test of ships' experience does. Experience is left at its
//! frozen strength of 0 and every unit has experience 2.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Where the dice decide (the hits
//! of a fight), the calculation gives the shots and the chance exactly; the
//! tests read the hits from the run. Everything else is asserted exactly.
//! If the code disagrees, the code or the rule is wrong: do not change a
//! number here to make a test pass. A mismatch goes back to the architect.
//!
//! ## Figures
//!
//! `assault_skill_strength` 0.25, `movement_skill_strength` 0.25,
//! `holding_skill_strength` 0.15, `weight_strength` 0.25. A skill's factor
//! is 1 + strength x (skill - 2). At 0.25 it is 0.5 for a skill of 0, 1 for
//! 2 and 1.5 for 4; at 0.15 it is 1.3 for a skill of 4.
//!
//! From the earlier rules: `march_days_per_hex` 3, `fatigue_per_march_day`
//! 10, `march_supply_days` 5, `rest_per_day` 5, `preparation_per_day` 5,
//! `daily_supply_t_per_man` 0.005, `men_per_shot` 100, `hit_chance` 0.3,
//! `defender_hit_chance` 0.6.
//!
//! ## Inputs
//!
//! A placeholder map. Coast hexes A at (0,0), L at (4,0), F at (6,0) and G
//! at (8,0), all clear. Land hexes M1 at (0,3) and M2 at (1,3), both
//! mountain, and P at (2,3), clear. Land links: M1 to M2, and M2 to P. The
//! game is 8 days.
//!
//! | Base | Side     | Hex | Supply |
//! | ---- | -------- | --- | ------ |
//! | 1    | Allied   | A   | 5000 t |
//! | 2    | Allied   | M1  | 1000 t |
//! | 3    | Allied   | M2  | 1000 t |
//! | 4    | Allied   | F   | 5000 t |
//! | 5    | Japanese | F   | 5000 t |
//! | 6    | Allied   | G   | 5000 t |
//! | 7    | Japanese | G   | 5000 t |
//!
//! Every skill not named is 2.
//!
//! | Unit | Side     | Men  | Where                              | Skills            | Posture |
//! | ---- | -------- | ---- | ---------------------------------- | ----------------- | ------- |
//! | 1    | Allied   | 200  | M1                                 | Mountain 4        | defend  |
//! | 2    | Allied   | 200  | M1                                 | none              | defend  |
//! | 3    | Allied   | 200  | M1                                 | Mountain 0        | defend  |
//! | 4    | Allied   | 200  | M2                                 | Mountain 4        | defend  |
//! | 5    | Allied   | 600  | Aboard force 1, a troop transport at base 1 | Assault 4 | defend  |
//! | 6    | Allied   | 600  | Aboard force 2, the same           | none              | defend  |
//! | 7    | Allied   | 600  | Aboard force 3, the same           | Assault 0         | defend  |
//! | 8    | Allied   | 1000 | F                                  | Weight 4          | defend  |
//! | 9    | Japanese | 1000 | F                                  | Weight 0          | attack  |
//! | 10   | Allied   | 500  | G                                  | Holding 4         | defend  |
//! | 11   | Japanese | 500  | G                                  | none              | attack  |
//!
//! Orders, all on day 1: units 1, 2 and 3 march to M2; unit 4 marches to P;
//! forces 1, 2 and 3 each prepare a landing at L with the unit aboard as
//! roster. No force sails.
//!
//! ## Movement
//!
//! A day's march counts as the movement factor of a day, for the lower of
//! the unit's skills for the terrain of the hex it leaves and of the hex it
//! enters. A unit arrives when its days marched reach 3.
//!
//! | Unit | Hexes               | Skill that counts | A day counts as | Arrives, end of day | Fatigue on arrival |
//! | ---- | ------------------- | ----------------- | --------------- | ------------------- | ------------------ |
//! | 1    | Mountain to mountain | 4                | 1.5             | 2                   | 20                 |
//! | 2    | Mountain to mountain | 2                | 1               | 3                   | 30                 |
//! | 3    | Mountain to mountain | 0                | 0.5             | 6                   | 60                 |
//! | 4    | Mountain to clear   | 2, for the clear hex | 1            | 3                   | 30                 |
//!
//! Each gains 10 fatigue for each day it marches and rests 5 a day after.
//! At the end of day 8: unit 1 has 0, unit 2 has 5, unit 3 has 50 and unit 4
//! has 5.
//!
//! Each carries 5 x 200 x 0.005 = 5 t on arrival. Unit 4 finds no base at P
//! and forms base 8 there.
//!
//! Supply, night by night (each unit of 200 costs 1 t a night):
//!
//! - Base 2 at M1: nights 1 and 2, three units, 3 t each: 994 t. Day 2,
//!   unit 1 takes 5: 989. Night 3, two units: 987. Day 3, unit 2 takes 5:
//!   982. Nights 4, 5 and 6, one unit: 979. Day 6, unit 3 takes 5: 974. It
//!   holds 974 t at the end.
//! - Base 3 at M2: nights 1 and 2, unit 4: 998 t. Day 2, unit 1 brings 5:
//!   1003. Night 3, units 1 and 4: 1001. Day 3, unit 2 brings 5 and unit 4
//!   takes 5: 1001. Nights 4, 5 and 6, units 1 and 2: 995. Day 6, unit 3
//!   brings 5: 1000. Nights 7 and 8, three units: 994 t at the end.
//! - Base 8 at P: 5 t on day 3. Unit 4 costs 1 t on each of the nights of
//!   days 4 to 8: 0 t at the end, and it was fed every night.
//!
//! ## Rehearsal
//!
//! Each force lies at base 1 with its order for all 8 days. Each gains 5 of
//! loading a day: 40 at the end. A unit gains 5 of rehearsal a day times its
//! assault factor.
//!
//! | Unit | Assault skill | Rehearsal gained a day | After day 8 | Days to reach 100 |
//! | ---- | ------------- | ---------------------- | ----------- | ----------------- |
//! | 5    | 4             | 7.5                    | 60          | 14                |
//! | 6    | 2             | 5                      | 40          | 20                |
//! | 7    | 0             | 2.5                    | 20          | 40                |
//!
//! The men aboard do not eat. Base 1 holds 5000 t at the end.
//!
//! ## Weight, at F
//!
//! Weight multiplies a unit's shots and the supply each of its men costs.
//!
//! Night of day 1: unit 8 costs 1000 x 0.005 x 1.5 = 7.5 t, and base 4
//! holds 4992.5 t. Unit 9 costs 1000 x 0.005 x 0.5 = 2.5 t, and base 5
//! holds 4997.5 t. Both are fed in full.
//!
//! Day 1, the fight. The Japanese side attacks.
//!
//! - Unit 9, weight 0: 1000 firing men, 10 x 0.5 = 5 shots at 0.3.
//! - Unit 8, weight 4: 1000 firing men, 10 x 1.5 = 15 shots at 0.6.
//!
//! On each later night each unit costs its men times 0.005 times its weight
//! factor.
//!
//! ## Holding ground, at G
//!
//! Day 1. The Japanese side attacks and the Allied side defends.
//!
//! - Unit 11: 500 firing men, 5 shots at 0.3.
//! - Unit 10, holding 4: 500 firing men, 5 shots at 0.6 x 1.3 = 0.78.
//!
//! The holding skill acts only while the unit's side defends. The breakdown
//! of each unit's fire names the skill and its factor wherever one acts.
//!
//! # Properties
//!
//! - A unit with every skill at 2 gives what it gave before the rule, and
//!   with the frozen strengths of 0 every scenario with land units gives
//!   what it gave before, apart from the hash version.
//! - A higher movement skill never arrives later, and a higher assault skill
//!   never rehearses slower.
//! - The enemy's view is the same in two worlds that differ only in the
//!   other side's skills, where the hits are the same.
