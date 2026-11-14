//! Scenario 37: men who are falling back are fired on only when the side
//! that beat them is on attack. A fight another unit of their own side
//! starts costs them nothing and does not count them; a victor who goes on
//! attacking fires on them and counts them, as before.
//!
//! Proves decision 0109, correction 1, against a hand calculation. The data
//! is placeholder data: see `data/scenarios/37_men_falling_back.toml`. It
//! runs on the frozen step 1 figures and sets `engagement_ratio` to 3, as
//! the test of scenario 33 does; `failed_attack_fatigue` is left at 0.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Where the dice decide (the
//! hits of a fight), the calculation gives the firing men, the shots after
//! the limit and the chance exactly, and the rule for what follows; the
//! tests read the hits from the run. Everything else is asserted exactly.
//! If the code disagrees, the code or the rule is wrong: do not change a
//! number here to make a test pass. A mismatch goes back to the architect.
//!
//! ## Figures
//!
//! `engagement_ratio` 3. From the earlier rules: `men_per_shot` 100,
//! `hit_chance` 0.3, `defender_hit_chance` 0.6, `men_per_hit` 20,
//! `fatigue_per_attack_day` 20, `fatigue_per_march_day` 10, `rest_per_day`
//! 5, `attack_fatigue_limit` 60, `beaten_share` 0.2, `beaten_fatigue` 30,
//! `march_days_per_hex` 3, `march_supply_days` 5, `daily_supply_t_per_man`
//! 0.005, `unfed_loss_share_per_day` 0.05, `captured_stock_share` 0.5.
//!
//! In the ground combat phase the marches come first, then the fight, the
//! day's fatigue, the test of a side beaten and the bases that change side.
//! So a unit that arrives by march fights that day with the fatigue of the
//! days before, and the fatigue of the day's march is added after the
//! fight.
//!
//! ## Inputs
//!
//! A placeholder map. Every hex is a coast hex and clear. The game is 6
//! days.
//!
//! - P at (0,0), with a land link to R at (1,0).
//! - T at (10,0), with a land link to U at (11,0).
//!
//! | Base | Side     | Hex | Supply |
//! | ---- | -------- | --- | ------ |
//! | 1    | Japanese | P   | 1000 t |
//! | 2    | Japanese | R   | 100 t  |
//! | 3    | Allied   | P   | 5000 t |
//! | 4    | Japanese | T   | 1000 t |
//! | 5    | Japanese | U   | 100 t  |
//! | 6    | Allied   | T   | 5000 t |
//!
//! | Unit | Side     | Men  | Hex | Posture |
//! | ---- | -------- | ---- | --- | ------- |
//! | 1    | Japanese | 1000 | P   | defend  |
//! | 2    | Allied   | 6000 | P   | attack  |
//! | 3    | Japanese | 300  | R   | attack  |
//! | 4    | Japanese | 1000 | T   | defend  |
//! | 5    | Allied   | 6000 | T   | attack  |
//!
//! All are fighting units. Orders: on day 1, unit 3 marches to P. On day
//! 2, unit 2 is set to defend. Unit 5 is given no order and stays on
//! attack.
//!
//! ## At P: a fight their own side starts
//!
//! Day 1. The Allied side attacks. Unit 3 is at R and is not in the fight.
//!
//! - Unit 2: 6000 firing men, limited to 3 x 1000 = 3000: 30 shots at 0.3.
//! - Unit 1: 1000 firing men, 10 shots at 0.6.
//!
//! Let b be unit 2's hits and a unit 1's. Unit 1 has 1000 - 20 x b men,
//! at least 400; unit 2 has 6000 - 20 x a. The Japanese side is beaten
//! whatever the dice give: at most 1000 men against 0.2 times at least
//! 5800. Base 1 becomes Allied. Unit 1 falls back: R is the one linked hex
//! and holds base 2. Its fatigue is 0 after the day's rest, and the defeat
//! adds 30.
//!
//! Day 2. Unit 2 is set to defend at intake. No unit at P is on attack:
//! there is no fight. Unit 1 is unfed that night, since its side has no
//! base at P, and loses its men times 0.05, cut to a whole man: its fed
//! share is 0. It marches, day 1 of 3. Unit 2 rests: its fatigue goes from
//! 20 to 15.
//!
//! Day 3. Unit 1 is unfed again and marches, day 2 of 3. Unit 3 arrives at
//! P on its third day of march and puts down the 300 x 0.005 x 5 = 7.5 t it
//! carried: a Japanese base is formed at P. It is on attack, so the hex is
//! contested.
//!
//! - Unit 3: fed in full at R the night before; its fatigue at the fight is
//!   20, from two days of march. Firing men 300 x 80/100 = 240: 2.4 shots
//!   at 0.3.
//! - Unit 1 is falling back, and the Allied side is not on attack in the
//!   hex. So it is not counted among the Japanese fighting men for the
//!   limit, and it takes no share of the fight's losses. Its fed share is
//!   0, so it fires nothing in any case.
//! - Unit 2: its fatigue is 15, so its firing men are its men times 85/100,
//!   cut, far above the limit of 3 x 300 = 900: 9 shots at 0.6.
//!
//! Let c be unit 2's hits on day 3. Unit 3 loses 20 x c men, at most 180.
//! Unit 1 loses none in the fight: the only men it loses on days 2 to 4
//! are the unfed. Without the correction it would have shared the 20 x c
//! in proportion to its men, and the limit would have been three times the
//! men of both units.
//!
//! Unit 3's fatigue after day 3 is 20 + 10 for the march + 20 for the
//! attack: 50. That is not above 60, and it stays on attack.
//!
//! Day 4. Unit 1 marches its third day and arrives at R, where base 2
//! feeds it from the night of day 5. If unit 3 has a man left the fight
//! goes on between it and unit 2, with the limit at three times unit 3's
//! men; the tests work each later day by the rule from the men and fatigue
//! the run has.
//!
//! Supply at R: base 2 pays 1.5 t for unit 3 on each of the nights of days
//! 1, 2 and 3, and gives it 7.5 t on the day it arrives at P: 88 t after
//! day 3.
//!
//! ## At T: a victor who goes on attacking
//!
//! Day 1 is as at P: unit 5 is limited to 30 shots at 0.3, unit 4 fires 10
//! at 0.6, the Japanese side is beaten, base 4 becomes Allied and unit 4
//! falls back toward U with fatigue 30. Unit 5's fatigue is 20.
//!
//! Days 2 and 3. Unit 5 is still on attack and the hex is contested. Unit
//! 4 is falling back and the side that beat it is on attack: it is counted
//! and it is fired on, as before the correction.
//!
//! - Let m be unit 4's men before the day's fight, after that night's
//!   unfed loss. Its fed share is 0: it fires nothing.
//! - Unit 5's firing men are its men times 80/100 on day 2 and 60/100 on
//!   day 3, cut, far above the limit of 3 x m: its shots are 3 x m / 100,
//!   at 0.3.
//! - Unit 4 loses 20 for each hit, or all it has. It is already marching:
//!   beaten again, it is left as it is.
//!
//! Unit 5's fatigue is 40 after day 2 and 60 after day 3. 60 is not above
//! the limit: it stays on attack.
//!
//! Day 4. The marches come first: unit 4, if it has a man left, arrives at
//! U before the fight. No Japanese man is at T: there is no fight, and
//! unit 5 rests to 55.
//!
//! # Properties
//!
//! - A unit that is not falling back shares in every fight in its hex, as
//!   before: with unit 1 never beaten (unit 2 on defend from day 1) and
//!   unit 3 attacking on day 3, units 1 and 3 share the losses in
//!   proportion and both count for the limit.
//! - With the frozen `engagement_ratio`, and no unit falling back in a hex
//!   where its own side attacks, every scenario gives what it gave before,
//!   apart from the hash version. A scenario that does differ is listed
//!   for the architect before its baseline is remade.
//! - The enemy's view is the same in two worlds that differ only in
//!   whether a unit it cannot see is falling back or standing, where the
//!   hits are the same.
