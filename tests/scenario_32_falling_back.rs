//! Scenario 32: a beaten side falls back. Its units march out of the hex to
//! a hex with a base of their side, or else to one with no enemy fighting
//! men, tired by the defeat and carrying nothing; men with nowhere to go
//! surrender.
//!
//! Proves decision 0104 (a beaten side falls back) against a hand
//! calculation. The data is placeholder data: see
//! `data/scenarios/32_falling_back.toml`. It runs on the frozen step 1
//! figures, which hold `beaten_fatigue` at the value below.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Where the dice decide (the
//! hits of the one fight), the calculation gives the shots and the chance
//! exactly and the rule for what follows; the tests read the hits from the
//! run. Everything else is asserted exactly. If the code disagrees, the
//! code or the rule is wrong: do not change a number here to make a test
//! pass. A mismatch goes back to the architect.
//!
//! ## Figures
//!
//! `beaten_fatigue` 30. From the earlier rules: `march_days_per_hex` 3,
//! `fatigue_per_march_day` 10, `rest_per_day` 5, `daily_supply_t_per_man`
//! 0.005, `unfed_loss_share_per_day` 0.05, `captured_stock_share` 0.5,
//! `beaten_share` 0.2, `men_per_shot` 100, `hit_chance` 0.3,
//! `defender_hit_chance` 0.6, `men_per_hit` 20, `work_per_man_day` 1.
//!
//! In the ground combat phase the marches come first, then the fights, the
//! fatigue of the day, the test of a side beaten and the bases that change
//! side. A unit set to fall back on day d therefore marches first on day
//! d + 1 and arrives in the phase of day d + 3. The fatigue of defeat is
//! added when the unit is set to fall back, after the day's fatigue step.
//!
//! ## Inputs
//!
//! A placeholder map. Every hex is a coast hex and clear. The game is 6
//! days.
//!
//! - Group one: P at (0,0), with land links to Q at (0,1) and to R at
//!   (1,0). In hex order Q comes before R.
//! - Group two: T at (10,0), with land links to U at (10,1) and to V at
//!   (11,0). In hex order U comes before V.
//! - S at (20,0), with no land link.
//!
//! | Base | Side     | Hex | Supply |
//! | ---- | -------- | --- | ------ |
//! | 1    | Japanese | P   | 1000 t |
//! | 2    | Japanese | R   | 100 t  |
//! | 3    | Allied   | P   | 1000 t |
//! | 4    | Japanese | T   | 1000 t |
//! | 5    | Allied   | T   | 5000 t |
//! | 6    | Allied   | U   | 1000 t |
//! | 7    | Japanese | S   | 1000 t |
//! | 8    | Allied   | S   | 1000 t |
//!
//! Every base has an airfield.
//!
//! | Unit | Side     | Kind     | Men  | Hex | Posture |
//! | ---- | -------- | -------- | ---- | --- | ------- |
//! | 1    | Japanese | working  | 300  | P   | defend  |
//! | 2    | Allied   | fighting | 500  | P   | attack  |
//! | 3    | Japanese | fighting | 1000 | T   | defend  |
//! | 4    | Allied   | fighting | 6000 | T   | attack  |
//! | 5    | Allied   | fighting | 100  | U   | defend  |
//! | 6    | Japanese | working  | 200  | S   | defend  |
//! | 7    | Allied   | fighting | 100  | S   | attack  |
//!
//! The Allied side has a `men_lost` objective that counts Japanese men
//! lost.
//!
//! Orders: on day 2, units 2 and 4 are set to defend.
//!
//! ## At P: falling back to a base, with no fight
//!
//! Night of day 1: base 1 pays 300 x 0.005 = 1.5 t and holds 998.5 t. Base
//! 3 pays 2.5 t.
//!
//! Day 1. The Japanese side has no fighting man at P, so the hex is not
//! contested. Unit 2 is on attack and the Japanese side has a base there:
//! it is beaten without a fight. Base 1 changes side and keeps 998.5 x 0.5
//! = 499.25 t.
//!
//! Unit 1 falls back. Q comes first in hex order and holds nothing; R holds
//! base 2, a base of its side, and a base is preferred: it marches for R.
//! Its fatigue is 0 after the day's rest, and the defeat adds 30.
//!
//! | Day | Night: fed          | Men lost unfed    | Men  | In the phase            | Fatigue after |
//! | --- | ------------------- | ----------------- | ---- | ----------------------- | ------------- |
//! | 1   | by base 1, in full  | 0                 | 300  | beaten, set to fall back | 30           |
//! | 2   | unfed: no base at P | 300 x 0.05 = 15   | 285  | marches, day 1 of 3     | 40            |
//! | 3   | unfed               | 14.25: 14         | 271  | marches, day 2 of 3     | 50            |
//! | 4   | unfed               | 13.55: 13         | 258  | marches, day 3: arrives at R | 60       |
//! | 5   | by base 2, in full  | 0                 | 258  | works                   | 55            |
//! | 6   | by base 2, in full  | 0                 | 258  | works                   | 50            |
//!
//! - It carries nothing to R: its side has no base at P to draw from.
//! - Base 2 pays 258 x 0.005 = 1.29 t on each of the nights of days 5 and
//!   6, and holds 97.42 t at the end.
//! - At R on days 5 and 6 unit 1, with no order at a base that has an
//!   airfield, works the port: 258 of work each day.
//! - On days 2 to 4 it makes no work: it marches.
//! - Base 1, now Allied, and base 3 feed unit 2. Base 1 is the lower
//!   numbered and pays 2.5 t on each of the nights of days 2 to 6: it holds
//!   499.25 - 12.5 = 486.75 t at the end. Base 3 holds 997.5 t.
//!
//! ## At T: falling back after a fight, away from the enemy
//!
//! Day 1, the fight. The Allied side attacks.
//!
//! - Unit 4: 6000 firing men, 60 shots at 0.3.
//! - Unit 3: 1000 firing men, 10 shots at 0.6.
//!
//! The Japanese side is beaten whatever the dice give: unit 3 has at most
//! 1000 men, and unit 4 has at least 6000 - 10 x 20 = 5800, of which 0.2 is
//! 1160. Base 4 changes side.
//!
//! Let m be unit 3's men after the fight, read from the run.
//!
//! - If m is 0 the unit is destroyed and nothing falls back.
//! - If m is above 0, unit 3 falls back. U comes first in hex order and
//!   holds Allied fighting men: it is passed over. V holds no one: unit 3
//!   marches for V. Its fatigue is 0 after the day's rest and the defeat
//!   adds 30. It goes on the defence, as it was.
//! - On day 2 unit 4 is set to defend, so there is no fight on days 2 to 4
//!   while unit 3 is still at T.
//! - On each of the nights of days 2, 3 and 4 it is unfed at T and loses
//!   its men times 0.05, cut to a whole man and one man at least. On the
//!   nights of days 5 and 6 it is unfed at V, where it has no base and
//!   brought nothing, and loses the same share. The tests work each night's
//!   loss from the men the run has before it.
//! - Its fatigue is 40, 50 and 60 after days 2, 3 and 4. It arrives at V in
//!   the phase of day 4 and forms no base there. Its fatigue is 55 and 50
//!   after days 5 and 6.
//!
//! ## At S: nowhere to go
//!
//! Day 1. The Japanese side has no fighting man at S. Unit 7 is on attack:
//! the Japanese side is beaten without a fight and base 7 changes side.
//!
//! S has no land link. Unit 6 surrenders in the same phase: its 200 men are
//! lost and the unit is destroyed. Both sides are told, with the 200 men.
//! The men the Allied `men_lost` objective counts rise by 200.
//!
//! # Properties
//!
//! - Men are conserved: a unit's men before it falls back equal its men on
//!   arrival plus those lost on the way, each loss with its event.
//! - A unit that falls back never arrives in a hex that held enemy fighting
//!   men when it set out.
//! - A march order given to a unit on the day after it is set to fall back
//!   replaces the falling back, and an attack order to it is taken.
//! - A scenario in which no side is beaten in a hex with a land link, and
//!   no beaten side is left with men and nowhere to go, gives what it gave
//!   before, apart from the hash version. A scenario that does differ is
//!   listed for the architect before its baseline is remade.
//! - The enemy's view is the same in two worlds that differ only in where
//!   the beaten side's units fall back to.
