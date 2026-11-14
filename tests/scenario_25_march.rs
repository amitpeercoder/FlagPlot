//! Scenario 25: a unit marches to the next hex with supply, forms a dump
//! there and attacks on arrival; a march with no land link is refused; and a
//! unit that marches from a shore with no base carries nothing.
//!
//! Proves decision 0094 (marching overland) against a hand calculation. The
//! data is placeholder data: see `data/scenarios/25_march.toml`. The test
//! runs on the frozen step 1 figures (`data/reference/step1`), to which the
//! three new ground figures are added with their first values.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Where the dice decide, the
//! calculation gives the firing men, the shots and the chance exactly and
//! what must follow from the hits the run rolls; the tests read the hits
//! from the run and assert the rest. Everything else is asserted exactly. If
//! the code disagrees, the code or the rule is wrong: do not change a number
//! here to make a test pass. A mismatch goes back to the architect.
//!
//! ## Figures
//!
//! `ground.toml`: `march_days_per_hex` 3, `fatigue_per_march_day` 10,
//! `march_supply_days` 5, and from the earlier stages
//! `daily_supply_t_per_man` 0.005, `unfed_loss_share_per_day` 0.05,
//! `men_per_shot` 100, `hit_chance` 0.3, `defender_hit_chance` 0.6,
//! `men_per_hit` 20, `fatigue_per_attack_day` 20, `rest_per_day` 5,
//! `attack_fatigue_limit` 60, `beaten_share` 0.2.
//!
//! ## Inputs
//!
//! A placeholder map with four coast hexes on one row: P at (0,0), Q at
//! (1,0), R at (2,0) and S at (3,0). Its land links are P to Q and R to S.
//! Q and R are neighbours with no land link. The game is 5 days.
//!
//! | Base | Side     | Hex | Supply |
//! | ---- | -------- | --- | ------ |
//! | 1    | Allied   | P   | 100 t  |
//! | 2    | Japanese | Q   | 50 t   |
//!
//! | Land unit | Side     | Men  | Ashore at | Posture at the start |
//! | --------- | -------- | ---- | --------- | -------------------- |
//! | 1         | Allied   | 1000 | P         | attack               |
//! | 2         | Japanese | 400  | Q         | defend               |
//! | 3         | Japanese | 100  | R         | defend               |
//!
//! Orders: on day 1, unit 1 marches to Q and unit 3 marches to S. On day 4,
//! unit 1 is ordered to march to R.
//!
//! Each night the men's supply is charged first. In the ground combat phase
//! the marches come first, then the fights, then fatigue.
//!
//! ## Unit 1: the march from P to Q
//!
//! | Day | Night: base 1 pays, and holds | Ground combat phase | Unit 1 after the day |
//! | --- | ----------------------------- | ------------------- | -------------------- |
//! | 1   | 5 t, 95 t                     | One day marched. No enemy at P: no fight | At P, fatigue 10 |
//! | 2   | 5 t, 90 t                     | Two days marched    | At P, fatigue 20     |
//! | 3   | 5 t, 85 t                     | Three days marched: it arrives at Q | At Q; see below |
//!
//! On arrival, 5 days of the charge for its men, 5 x 1000 x 0.005 = 25 t,
//! are taken from base 1, which holds 60 t. The Allied side has no base at
//! Q, so base 3 is formed there, Allied, not on the Japanese map, and holds
//! the 25 t. The march order is spent. No Allied man is at P afterwards, so
//! base 1 pays nothing more and holds 60 t at the end.
//!
//! ## Q: the fight on arrival
//!
//! Unit 2 is fed by base 2 every night: 0.005 t for each man it has, so 2 t
//! on each of the first three nights (base 2 holds 48, 46 and 44 t). Unit 2
//! rests and stays at fatigue 0.
//!
//! Day 3. The Allied side attacks and the Japanese side defends.
//!
//! - Unit 1: its fatigue before the day's change is 20. Firing men 1000 x 1
//!   x 80/100 = 800. 8 shots at 0.3.
//! - Unit 2: firing men 400. 4 shots at 0.6.
//!
//! With a the hits of unit 1 and j the hits of unit 2: unit 2 has 400 less
//! 20 x a, at least 240, and unit 1 has 1000 less 20 x j, at least 920.
//!
//! Unit 1 marched and fired on attack that day: it gains 10 and 20, and its
//! fatigue is 50. It stays on attack, since 50 is not above 60.
//!
//! Beaten: the Japanese side is tested. It has at least 240 men against 0.2
//! times at most 1000. It is not beaten on day 3.
//!
//! The ground reports of day 3: each side gets an estimate of the other's
//! men at Q, between 0.5 and 1.5 times the truth after the fight, cut.
//! Base 3 is put on the Japanese map.
//!
//! Day 4, at intake: the order for unit 1 to march to R is refused, as
//! there is no land link from Q to R.
//!
//! Night of day 4: base 3 pays 0.005 t for each man unit 1 has and all are
//! fed; base 2 pays for each man unit 2 has.
//!
//! Day 4, the fight. Unit 1's fatigue before the day's change is 50: its
//! firing men are its men times 50/100, cut, at 0.3. Unit 2 fires all its
//! men at 0.6. Unit 1 fired on attack and did not march: it gains 20, to 70,
//! which is above 60, and it goes back to defend. The Japanese side is
//! tested again by the men left: it is beaten if its men are fewer than 0.2
//! times unit 1's men, and base 2 then becomes Allied.
//!
//! Day 5. No unit at Q is on attack: no fight. Unit 1 rests to 65. A ground
//! report is made for each side if both still have men at Q.
//!
//! At the end base 3 holds 25 t less 0.005 t for each man unit 1 had on the
//! nights of days 4 and 5.
//!
//! ## Unit 3: a march from a shore with no base
//!
//! The Japanese side has no base at R or at S. Unit 3 is unfed every night
//! and loses its men times 0.05, cut, and one man at least.
//!
//! | Day | Night: lost, left | Ground combat phase                  | Unit 3 after the day |
//! | --- | ----------------- | ------------------------------------ | -------------------- |
//! | 1   | 5, 95             | One day marched                      | At R, fatigue 10     |
//! | 2   | 4.75, cut to 4, 91 | Two days marched                    | At R, fatigue 20     |
//! | 3   | 4.55, cut to 4, 87 | Three days marched: it arrives at S | At S, fatigue 30     |
//! | 4   | 4.35, cut to 4, 83 | It rests                            | At S, fatigue 25     |
//! | 5   | 4.15, cut to 4, 79 | It rests                            | At S, fatigue 20     |
//!
//! On arrival there is no base of its side at R to take supply from: it
//! carries nothing, and no base is formed at S. At the end unit 3 has 79
//! men at S and has lost 21 unfed.
//!
//! # Properties
//!
//! - Supply is conserved by a march: what leaves the bases of the hex left
//!   is what enters the base of the hex reached.
//! - Men are conserved by a march.
//! - A unit never marches where the map has no land link, and a map with no
//!   land links allows no march.
//! - The enemy's view is the same in two worlds that differ only in a march
//!   to a hex where the enemy has no men.
//! - A scenario with no march order is unchanged, apart from the hash
//!   version.
