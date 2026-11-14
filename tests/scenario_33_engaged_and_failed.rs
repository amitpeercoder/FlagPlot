//! Scenario 33: who is engaged in a fight ashore, and an attack that fails.
//! A large side fires with no more men than three times the enemy's; an
//! attack whose weight of fire is the weaker fails and wears its men out;
//! tired attackers hold or fall back as their order says, and with no
//! instruction hold where they can be fed and fall back where they cannot.
//!
//! Proves decision 0105 against a hand calculation. The data is placeholder
//! data: see `data/scenarios/33_engaged_and_failed.toml`. The frozen step 1
//! figures hold `engagement_ratio` at 1000000 and `failed_attack_fatigue`
//! at 0, so this test gives them the values below, as the test of ships'
//! experience sets its figure.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Where the dice decide (the
//! hits of a fight), the calculation gives the firing men, the shots, the
//! chance and the weight of fire exactly, and the rule for what follows;
//! the tests read the hits from the run. Everything else is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Figures
//!
//! `engagement_ratio` 3, `failed_attack_fatigue` 30. From the earlier
//! rules: `men_per_shot` 100, `hit_chance` 0.3, `defender_hit_chance` 0.6,
//! `men_per_hit` 20, `fatigue_per_attack_day` 20, `rest_per_day` 5,
//! `attack_fatigue_limit` 60, `beaten_share` 0.2, `beaten_fatigue` 30,
//! `march_days_per_hex` 3, `fatigue_per_march_day` 10,
//! `daily_supply_t_per_man` 0.005, `unfed_loss_share_per_day` 0.05. The
//! strengths of experience and of the skills are 0, and no base has
//! defences.
//!
//! A unit's firing men are its men ashore times its fed share, times 100
//! less its fatigue over 100, cut to a whole man. A side's firing men count
//! up to 3 times the other side's fighting men ashore in the hex at the
//! start of the fight; above that each of its units' shots are cut in
//! proportion. A side's weight of fire is the sum of its units' shots times
//! their chance, after the cut. A side that attacked and whose weight is
//! below the other's has failed that day.
//!
//! In the ground combat phase the order is: the marches, the fight, the
//! day's fatigue (with the fatigue of a failed day), the tired back to
//! defend, what a spent attacker does, the test of a side beaten.
//!
//! ## Inputs
//!
//! A placeholder map. Every hex is a coast hex and clear. The game is 6
//! days.
//!
//! - K at (0,0), with no land link.
//! - L at (10,0), with a land link to M at (10,1).
//! - N at (20,0), with a land link to O at (20,1).
//! - P at (30,0), with no land link.
//!
//! | Base | Side     | Hex | Supply |
//! | ---- | -------- | --- | ------ |
//! | 1    | Allied   | K   | 5000 t |
//! | 2    | Japanese | K   | 1000 t |
//! | 3    | Allied   | L   | 5000 t |
//! | 4    | Japanese | L   | 10 t   |
//! | 5    | Japanese | M   | 100 t  |
//! | 6    | Allied   | N   | 5000 t |
//! | 7    | Japanese | N   | 2.5 t  |
//! | 8    | Allied   | P   | 5000 t |
//! | 9    | Japanese | P   | 1000 t |
//!
//! | Unit | Side     | Men  | Hex | Posture | Fatigue at the start | Instruction when spent |
//! | ---- | -------- | ---- | --- | ------- | -------------------- | ---------------------- |
//! | 1    | Allied   | 6000 | K   | defend  | 0                    |                        |
//! | 2    | Japanese | 1000 | K   | attack  | 0                    | none                   |
//! | 3    | Allied   | 3000 | L   | defend  | 0                    |                        |
//! | 4    | Japanese | 500  | L   | attack  | 40                   | fall back              |
//! | 5    | Allied   | 3000 | N   | defend  | 0                    |                        |
//! | 6    | Japanese | 500  | N   | attack  | 40                   | none                   |
//! | 7    | Allied   | 4000 | P   | attack  | 0                    | none                   |
//! | 8    | Japanese | 1600 | P   | defend  | 0                    |                        |
//!
//! Units 2, 4, 6 and 7 are set to attack by an order of day 1, which
//! carries the instruction of the table. There is no other order. Every
//! unit is fed in full on the night of day 1.
//!
//! ## At K: the limit, and an attack that fails twice and holds
//!
//! Day 1. The Japanese side attacks.
//!
//! - Unit 2: 1000 firing men, 10 shots at 0.3. Weight 3.
//! - Unit 1: 6000 firing men. The limit is 3 x 1000 = 3000, so its 60 shots
//!   are cut to 60 x 3000/6000 = 30, at 0.6. Weight 18.
//!
//! The Japanese attack fails: 3 is below 18. Unit 2's fatigue is 0 + 20 +
//! 30 = 50. That is not above 60: it stays on attack.
//!
//! Let J be unit 2's men after day 1 and A unit 1's, read from the run. J
//! is at least 1000 - 30 x 20 = 400, and A at least 6000 - 10 x 20 = 5800.
//!
//! Day 2.
//!
//! - Unit 2: firing men J x 50/100, cut. Its shots are those over 100, at
//!   0.3.
//! - Unit 1: A firing men, limited to 3 x J, which is below A whatever the
//!   dice gave. Its shots are 3 x J over 100, at 0.6.
//!
//! The attack fails again whatever the dice gave: unit 1's weight is 1.8 x
//! J/100 and unit 2's at most 0.15 x J/100. Unit 2's fatigue is 50 + 20 +
//! 30 = 100. It is above 60: unit 2 goes back to defend. Its order carries
//! no instruction, and base 2 at K holds supply: it holds.
//!
//! Days 3 to 6: no unit at K is on attack and there is no fight. Unit 2's
//! fatigue is 95, 90, 85 and 80.
//!
//! The Allied side never attacks at K, so the Japanese side is not tested
//! for being beaten.
//!
//! ## At L: a failed attack, and an order to fall back
//!
//! Night of day 1: base 4 pays 500 x 0.005 = 2.5 t and holds 7.5 t.
//!
//! Day 1.
//!
//! - Unit 4: firing men 500 x 60/100 = 300. 3 shots at 0.3. Weight 0.9.
//! - Unit 3: 3000 firing men, limited to 3 x 500 = 1500: 30 x 1500/3000 =
//!   15 shots at 0.6. Weight 9.
//!
//! The attack fails. Unit 4's fatigue is 40 + 20 + 30 = 90, above 60: it
//! goes back to defend. Its order says fall back, so it does, though base 4
//! holds supply: it is set to march for M, the one linked hex, which holds
//! base 5. The falling back adds 30: its fatigue is 100, the most there is.
//!
//! Let m be unit 4's men after the fight of day 1, read from the run. It is
//! at least 500 - 15 x 20 = 200.
//!
//! | Day | Night: fed                         | In the phase                | Fatigue after |
//! | --- | ---------------------------------- | --------------------------- | ------------- |
//! | 1   | by base 4, 2.5 t                   | fights, fails, falls back   | 100           |
//! | 2   | by base 4, m x 0.005 t             | marches, day 1 of 3         | 100           |
//! | 3   | by base 4, m x 0.005 t             | marches, day 2 of 3         | 100           |
//! | 4   | by base 4, m x 0.005 t             | marches, day 3: arrives at M | 100          |
//! | 5   | by base 5, m x 0.005 t             | rests                       | 95            |
//! | 6   | by base 5, m x 0.005 t             | rests                       | 90            |
//!
//! - Unit 3 is on defend and nobody attacks at L after day 1: there is no
//!   fight on days 2 to 4 while unit 4 is still there, and it loses no man
//!   after day 1.
//! - Base 4 holds 7.5 - 3 x m x 0.005 t at the end, which is 0 or more
//!   whatever the dice gave, and is still Japanese: nobody attacked it.
//! - Unit 4 carries nothing to M. Base 5 holds 100 - 2 x m x 0.005 t at
//!   the end.
//!
//! ## At N: a failed attack, no instruction, and nothing to eat
//!
//! Night of day 1: base 7 pays 2.5 t, all it has, and holds 0 t. Unit 6 is
//! fed in full.
//!
//! Day 1.
//!
//! - Unit 6: firing men 500 x 60/100 = 300. 3 shots at 0.3. Weight 0.9.
//! - Unit 5: limited to 1500: 15 shots at 0.6. Weight 9.
//!
//! The attack fails. Unit 6's fatigue is 90: it goes back to defend. Its
//! order carries no instruction. Its side's only base at N holds no supply,
//! so it cannot be fed where it stands: it falls back. O is the one linked
//! hex and holds no enemy: it marches for O, and its fatigue is 100.
//!
//! Let n be unit 6's men after the fight of day 1. On each of the nights of
//! days 2, 3 and 4 it is unfed at N and loses its men times 0.05, cut to a
//! whole man and one man at least. It arrives at O in the phase of day 4,
//! carrying nothing, and forms no base. On the nights of days 5 and 6 it is
//! unfed at O and loses the same share. The tests work each night's loss
//! from the men the run has before it. Its fatigue is 100 after days 1 to
//! 4, 95 after day 5 and 90 after day 6.
//!
//! ## At P: an attack that does not fail
//!
//! Day 1. The Allied side attacks.
//!
//! - Unit 7: 4000 firing men. The limit is 3 x 1600 = 4800, which does not
//!   cut. 40 shots at 0.3. Weight 12.
//! - Unit 8: 1600 firing men, 16 shots at 0.6. Weight 9.6.
//!
//! The attack does not fail: 12 is not below 9.6. Unit 7's fatigue is 20,
//! and nothing is added.
//!
//! The Japanese side is not beaten on day 1 whatever the dice give: unit 8
//! has at least 1600 - 40 x 20 = 800 men, and 0.2 times unit 7's men is at
//! most 800.
//!
//! On each later day the tests work both weights from the men, fed shares
//! and fatigue the run has before the fight, and assert: the limit where 3
//! times the enemy's men is below a side's firing men; that unit 7 gains 20
//! on a day it attacks and its weight is not the lower, and 50 on a day it
//! is; that it goes back to defend when its fatigue is above 60, and then
//! holds, since base 8 holds supply; and, if the Japanese side is beaten,
//! that its men surrender, P having no land link (decision 0104, 7).
//!
//! # Properties
//!
//! - A side's shots are never more than they were without the limit, and
//!   the smaller side's are never cut.
//! - With the frozen figures every scenario gives what it gave before,
//!   apart from the hash version.
//! - An instruction to hold keeps a spent attacker in the hex where the
//!   default would have it fall back: unit 6 told to hold stays at N.
//! - An instruction to fall back where no hex can be fallen back to leaves
//!   the unit in the hex, on the defence, and it does not surrender: unit 2
//!   told to fall back stays at K.
//! - The enemy's view is the same in two worlds that differ only in the
//!   instruction the other side's attack orders carry, until a unit has
//!   left the hex.
