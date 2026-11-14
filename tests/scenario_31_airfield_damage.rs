//! Scenario 31: an airfield damaged and mended. A damaged field flies a
//! share of its aircraft; working units mend it and nothing mends itself; a
//! badly damaged field takes no aircraft, and an arrival due at it waits;
//! bombs can land on an airfield and cannot land on one that is not there.
//!
//! Proves decision 0103 (an airfield damaged and mended) against a hand
//! calculation. The data is placeholder data: see
//! `data/scenarios/31_airfield_damage.toml`. The frozen step 1 figures set
//! `[base_hit] airfield` to 0, so this test gives it the value below, as
//! the test of ships' experience sets its figure. The three ground figures
//! are in the frozen figures at the values below.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Where the dice decide (the
//! hits of a raid and where each lands), the calculation gives the rule
//! exactly and the tests read the hits from the run. Everything else is
//! asserted exactly. If the code disagrees, the code or the rule is wrong:
//! do not change a number here to make a test pass. A mismatch goes back to
//! the architect.
//!
//! ## Figures
//!
//! `[base_hit] airfield` 0.2, `airfield_damage_per_hit` 2,
//! `work_per_airfield_damage_point` 50, `airfield_landing_refused_at` 50.
//! From decision 0099: `work_per_man_day` 1, `fighting_unit_work_share`
//! 0.5. From the earlier rules: `daily_supply_t_per_man` 0.005.
//!
//! The scenario names no wear, fatigue or air supply figures: nothing
//! wears, tires or costs supply but the men.
//!
//! Of a group at a base, the aircraft that may fly in a pulse are its ready
//! aircraft times 100 less the field's damage, over 100, cut to a whole
//! aircraft. The damage is read at the start of the pulse. Work is made in
//! the ground combat phase, at the end of the day, so a day's mending shows
//! in the next day's flying.
//!
//! ## Inputs
//!
//! A placeholder map with four coast hexes on one axis, far enough apart
//! that no group reaches another side's base but as stated: A at (0,0), B
//! at (5,0), C at (10,0), D at (15,0). The game is 6 days.
//!
//! | Base | Side     | Hex | Supply | Airfield | Damage at the start |
//! | ---- | -------- | --- | ------ | -------- | ------------------- |
//! | 1    | Allied   | A   | 5000 t | yes      | 30                  |
//! | 2    | Allied   | B   | 5000 t | yes      | 60                  |
//! | 3    | Japanese | C   | 5000 t | yes      | 0                   |
//! | 4    | Japanese | D   | 5000 t | none     | none                |
//!
//! | Unit | Side   | Kind     | Men  | Hex |
//! | ---- | ------ | -------- | ---- | --- |
//! | 1    | Allied | working  | 500  | A   |
//! | 2    | Allied | fighting | 1000 | B   |
//!
//! Air groups, all Allied:
//!
//! - Group 1: placeholder flying boat, at base 1, 12 ready, on search.
//! - Group 2: placeholder fighter (transfer range 500 nm), at base 1, 12
//!   ready, no mission. Base 2 is 200 nm away.
//! - Group 3: placeholder fighter, at base 2, 4 ready, no mission.
//! - Group 4: the placeholder bomber of scenario 24, at base 2, on a strike
//!   at base 3, which is within its strike radius.
//! - Group 5: the same bomber, at base 2, on a strike at base 4, which is
//!   within its strike radius.
//!
//! An arrival: 6 aircraft and 6 pilots for group 3, due on day 2.
//!
//! Orders:
//!
//! - Day 1: rebase group 2 to base 2. Unit 1 is given no order.
//! - Day 2: unit 2 works at the airfield.
//! - Day 3: rebase group 2 to base 2.
//! - Day 4: rebase group 2 to base 2.
//!
//! ## At A: a share flies, and a working unit mends
//!
//! Unit 1 has no order. The field of base 1 is built and damaged, so its
//! job is the airfield: 500 of work a day, 500 / 50 = 10 points mended.
//!
//! | Day | Damage through the day | Group 1 may fly | Damage after the day's work | Unit 1's job that day |
//! | --- | ---------------------- | --------------- | --------------------------- | --------------------- |
//! | 1   | 30                     | 12 x 70/100 = 8.4: 8 | 20                     | airfield              |
//! | 2   | 20                     | 9.6: 9          | 10                          | airfield              |
//! | 3   | 10                     | 10.8: 10        | 0                           | airfield              |
//! | 4   | 0                      | 12              | 0                           | port                  |
//! | 5   | 0                      | 12              | 0                           | port                  |
//! | 6   | 0                      | 12              | 0                           | port                  |
//!
//! From day 4 the field is whole and unit 1, still with no order, works the
//! port. Group 2 is at base 1 until day 4 and may fly 8, 9 and 10 of its 12
//! on days 1 to 3.
//!
//! Base 1 feeds 500 men, 2.5 t a night, six times: 4985 t at the end.
//!
//! ## At B: nothing mends itself, a level that refuses, an arrival that waits
//!
//! Unit 2 is a fighting unit. From day 2 it works at the airfield: 1000 x
//! 0.5 = 500 of work a day, 10 points mended. On day 1 nobody works on the
//! field and its damage stays at 60.
//!
//! | Day | Damage through the day | Night: the arrival | Group 3, ready | Group 3 may fly | Damage after the day's work |
//! | --- | ---------------------- | ------------------ | -------------- | --------------- | --------------------------- |
//! | 1   | 60                     | not due            | 4              | 4 x 40/100 = 1.6: 1 | 60                      |
//! | 2   | 60                     | due, waits: 60 is at or above 50 | 4 | 1               | 50                          |
//! | 3   | 50                     | waits: 50 is at or above 50 | 4     | 4 x 50/100 = 2  | 40                          |
//! | 4   | 40                     | delivered: 6 aircraft | 10          | 10 x 60/100 = 6 | 30                          |
//! | 5   | 30                     |                    | 10             | 7               | 20                          |
//! | 6   | 20                     |                    | 10             | 8               | 10                          |
//!
//! The rebase of group 2 to base 2:
//!
//! - Day 1, at intake: the damage is 60. Refused, as the field takes no
//!   aircraft.
//! - Day 3, at intake: the damage is 50. Refused: at the level is refused.
//! - Day 4, at intake: the damage is 40. Accepted. Group 2 is at base 2
//!   with 12 ready and flies nothing that day (decision 0091). On days 5
//!   and 6 it may fly 12 x 70/100 = 8.4: 8, and 12 x 80/100 = 9.6: 9.
//!
//! The bombers at base 2 are held by the same share. Of each group's ready
//! aircraft r at the start of a daylight pulse, r times 100 less the damage,
//! over 100, cut to a whole aircraft, fly the strike. The tests read r from
//! the run and assert the aircraft that flew.
//!
//! Base 2 feeds 1000 men, 5 t a night, six times: 4970 t at the end.
//!
//! ## At C and D: where bombs land
//!
//! Each hit of a raid on base 3 makes one draw for where it lands. With the
//! figure at 0.2 the airfield's share comes after those of aircraft, supply
//! and men in the table, and a hit on the airfield adds 2 to the damage of
//! base 3, up to 100. The tests read each raid's hits on the airfield from
//! the run and assert that base 3's damage after the raid is the damage
//! before it plus twice those hits. Nobody works on base 3's field: its
//! damage never falls.
//!
//! Base 4 has no airfield. The share counts for nothing there: no hit of
//! any raid on base 4 lands on an airfield, base 4 has no damage at the
//! end, and the raid's breakdown does not name the share.
//!
//! # Properties
//!
//! - Damage is never below 0 or above 100, and with no unit working on a
//!   field it never falls.
//! - The aircraft that may fly never rise when damage rises, and are never
//!   more than the ready aircraft.
//! - An arrival that waits is delivered once and whole, and none is lost.
//! - With the share at 0 and no damage stated, every scenario gives what it
//!   gave before, apart from the hash version.
//! - The enemy's view is the same in two worlds that differ only in the
//!   other side's airfield damage and the work done to mend it, where the
//!   hits are the same; and a raid's claim says nothing of the field.
