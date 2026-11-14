//! Scenario 34: cover, men who dig in, and a place under bombardment.
//! Troops dig fieldworks for themselves and lose them when they leave them;
//! deeper defences cost more work for each point; cover saves men from
//! shells and in the fight; men under fire do not rest and tire, up to a
//! limit; and fire wears cover down.
//!
//! Proves decision 0106 against a hand calculation. The data is placeholder
//! data: see `data/scenarios/34_cover.toml`. The frozen step 1 figures make
//! the rule do nothing, so this test gives its figures the values below, as
//! the test of ships' experience sets its figure. `engagement_ratio` and
//! `failed_attack_fatigue` (decision 0105) are left at their frozen values.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Where the dice decide (the
//! hits of a bombardment and where each lands, the hits of a fight), the
//! calculation gives the rule exactly and the tests read the hits from the
//! run. Everything else is asserted exactly. If the code disagrees, the
//! code or the rule is wrong: do not change a number here to make a test
//! pass. A mismatch goes back to the architect.
//!
//! ## Figures
//!
//! `cover_from_fire` 0.9, `cover_in_fight` 0.5, `fieldworks_per_day` 10,
//! `fieldworks_limit` 30, `prepared_from` 30, `fortified_from` 60,
//! `prepared_work_factor` 3, `fortified_work_factor` 10, `strain_per_hit`
//! 1, `strain_per_day` 10, `strain_limit` 60, `cover_from_strain` 0.5,
//! `cover_wear_per_hit` 0.2.
//!
//! From the earlier rules: `work_per_defence_point` 200, `work_per_man_day`
//! 1, `defences_strength` 0.5, `men_lost_per_base_hit` 10, `men_per_shot`
//! 100, `hit_chance` 0.3, `defender_hit_chance` 0.6, `men_per_hit` 20,
//! `fatigue_per_attack_day` 20, `rest_per_day` 5.
//!
//! A unit's cover is the higher of its base's defences and its own
//! fieldworks; a unit that marches or is on attack has none. Hits land in
//! the pulses; fieldworks are dug, work is made and the day's fatigue is
//! settled in the ground combat phase at the end of the day. So a hit meets
//! the cover the day began with, less what earlier hits of the day wore
//! away.
//!
//! ## Inputs
//!
//! A placeholder map with five coast hexes far apart and no land link: A,
//! B, C, D and F. The game is 6 days.
//!
//! | Base | Side     | Hex | Supply | Defences at the start |
//! | ---- | -------- | --- | ------ | --------------------- |
//! | 1    | Allied   | A   | 5000 t | 0                     |
//! | 2    | Allied   | B   | 5000 t | 30                    |
//! | 3    | Allied   | C   | 5000 t | 59                    |
//! | 4    | Allied   | D   | 5000 t | 80                    |
//! | 5    | Japanese | D   | 5000 t | 0                     |
//! | 6    | Allied   | F   | 5000 t | 50                    |
//!
//! | Unit | Side     | Kind     | Men  | Hex | Posture | Fatigue at the start |
//! | ---- | -------- | -------- | ---- | --- | ------- | -------------------- |
//! | 1    | Allied   | fighting | 1000 | A   | defend  | 0                    |
//! | 2    | Allied   | working  | 1800 | B   | defend  | 0                    |
//! | 3    | Allied   | working  | 1500 | C   | defend  | 0                    |
//! | 4    | Allied   | fighting | 1000 | D   | defend  | 0                    |
//! | 5    | Japanese | fighting | 1000 | D   | attack  | 0                    |
//! | 6    | Allied   | fighting | 1000 | F   | defend  | 58                   |
//! | 7    | Allied   | fighting | 500  | F   | defend  | 70                   |
//!
//! Two Japanese task forces with the bombardment mission, built as the
//! bombarding force of scenario 24 is: force 1 shells base 6 in the night
//! pulse of day 2, and force 2 shells base 1 in the night pulse of day 3.
//! Neither shells again.
//!
//! Orders: on day 1, units 2 and 3 work at the defences. On day 4, unit 1
//! is set to attack. On day 5, unit 1 is set to defend.
//!
//! ## At A: fieldworks, and shells on men who have dug
//!
//! | Day | Unit 1's posture | Fieldworks through the day | Fieldworks after the day |
//! | --- | ---------------- | -------------------------- | ------------------------ |
//! | 1   | defend           | 0                          | 10                       |
//! | 2   | defend           | 10                         | 20                       |
//! | 3   | defend           | 20, less the day's wear    | see below                |
//! | 4   | attack           | 0                          | 0                        |
//! | 5   | defend           | 0                          | 10                       |
//! | 6   | defend           | 10                         | 20                       |
//!
//! The order to attack on day 4 loses the fieldworks at intake, though no
//! enemy is at A and nothing is attacked. Set back to defend on day 5, the
//! unit digs again from nothing.
//!
//! Day 3, the shelling. Let h be the hits that land on men at A, read from
//! the run, in the order they land.
//!
//! - The k-th hit meets fieldworks of 20 - 0.2 x (k - 1), and base 1 has no
//!   defences, so that is unit 1's cover. The hit would take 10 men; it
//!   takes 10 x (1 - cover/100 x 0.9), cut to a whole man. The first hit
//!   takes 10 x 0.82 = 8.2: 8 men.
//! - Each hit wears the fieldworks by 0.2. After the day's digging they are
//!   20 - 0.2 x h + 10, and no more than 30.
//! - If h is 1 or more, unit 1 does not rest on day 3. It gains h, and no
//!   more than 10, times 1 less its cover over 100 times 0.5, with the
//!   cover it has when the day is settled, before that day's digging: 20 -
//!   0.2 x h. If h is 0 it rests.
//! - On every other day unit 1 rests, and its fatigue does not go below 0.
//!
//! ## At B and C: each grade costs more
//!
//! Unit 2 makes 1800 of work a day. Base 2's defences are 30, the bottom of
//! the prepared grade, where a point costs 200 x 3 = 600: 3 points a day.
//!
//! | After day | 1  | 2  | 3  | 4  | 5  | 6  |
//! | --------- | -- | -- | -- | -- | -- | -- |
//! | Base 2    | 33 | 36 | 39 | 42 | 45 | 48 |
//!
//! Unit 3 makes 1500 of work a day. Base 3's defences are 59. On day 1 the
//! one point to 60 costs 600; the 900 left are spent in the fortified
//! grade, where a point costs 200 x 10 = 2000: 0.45. After that each day
//! adds 1500 / 2000 = 0.75.
//!
//! | After day | 1     | 2    | 3     | 4    | 5     | 6    |
//! | --------- | ----- | ---- | ----- | ---- | ----- | ---- |
//! | Base 3    | 60.45 | 61.2 | 61.95 | 62.7 | 63.45 | 64.2 |
//!
//! ## At D: cover in the fight
//!
//! Day 1. The Japanese side attacks.
//!
//! - Unit 5: 1000 firing men, 10 shots at 0.3. It is on attack and has no
//!   cover.
//! - Unit 4: 1000 firing men, 10 shots at 0.6 x (1 + 0.5 x 80/100) = 0.84.
//!   Its cover is 80, the defences of base 4.
//!
//! Let a be unit 5's hits and d unit 4's. Unit 4 would lose 20 x a men; its
//! cover saves 80/100 x 0.5 of them: it loses 20 x a x 0.6 = 12 x a. Unit 5
//! loses 20 x d, or all it has.
//!
//! Unit 4 digs as it stands: its fieldworks are 10 after day 1 and 30 from
//! day 3. Its cover stays 80, the higher of the two. On each later day of
//! the fight the tests work the same rule from the men and fatigue the run
//! has before it.
//!
//! ## At F: strain, its limit, and wear on a base's defences
//!
//! Day 2, the shelling. Let g be the hits that land on men at F, in the
//! order they land.
//!
//! - The k-th hit meets defences of 50 - 0.2 x (k - 1), which is the cover
//!   of units 6 and 7 (their fieldworks are 10). The 10 men it would take
//!   are shared between the two units in proportion to their men, by the
//!   rule that serves every loss, and each unit's share is cut by its
//!   cover: times 1 less cover/100 x 0.9, cut to a whole man.
//! - Base 6's defences are 50 - 0.2 x g after the day. Nobody works on
//!   them: they stay there to the end. The fieldworks of units 6 and 7 are
//!   10 - 0.2 x g + 10 after the day's digging.
//! - If g is 1 or more, neither unit rests on day 2. The strain is g, and
//!   no more than 10, times 1 less the cover over 100 times 0.5, with the
//!   cover 50 - 0.2 x g.
//!   - Unit 6 began the day at 58 - 5 = 53. It gains the strain, and strain
//!     raises it no higher than 60.
//!   - Unit 7 began the day at 70 - 5 = 65, above 60: strain adds nothing,
//!     and it does not rest. It stays at 65.
//! - If g is 0 both rest: 48 and 60.
//! - On every other day both rest 5.
//!
//! # Properties
//!
//! - Cover never raises a loss: a unit never loses more men to a hit or in
//!   a fight than it would with no cover.
//! - Fieldworks are never below 0 or above the limit, and a base's defences
//!   never above 100.
//! - With the frozen figures every scenario gives what it gave before,
//!   apart from the hash version; scenario 30's defences are as its
//!   calculation has them.
//! - Crossing from the improvised grade to the prepared one is worked as
//!   crossing to the fortified one is at C: 800 of work at defences of 29
//!   gives 31.
//! - The enemy's view is the same in two worlds that differ only in the
//!   other side's fieldworks and defences, where the hits and the men lost
//!   are the same.
