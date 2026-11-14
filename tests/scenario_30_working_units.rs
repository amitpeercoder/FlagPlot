//! Scenario 30: units that work. Working units build an airfield and an air
//! group can then be flown to it; work on a port speeds the next day's
//! unloading, up to twice; work on aircraft repairs more of them the next
//! night; work on defences raises a defender's chance to hit; a fighting
//! unit set to work makes half as much and neither trains nor rehearses;
//! hungry men make less; working men do not fire and do not hold a base.
//!
//! Proves decision 0099 (units that work, and what they build) against a
//! hand calculation. The data is placeholder data: see
//! `data/scenarios/30_working_units.toml`. The frozen step 1 figures hold
//! the seven figures of this rule at the values below. The strengths of
//! experience and of the skills are 0 there, and this test leaves them so.
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
//! `work_per_man_day` 1, `airfield_work` 20000, `work_to_double_unloading`
//! 1000, `work_per_aircraft_repair` 200, `work_per_defence_point` 200,
//! `defences_strength` 0.5, `fighting_unit_work_share` 0.5.
//!
//! From the earlier rules: `daily_supply_t_per_man` 0.005,
//! `unfed_loss_share_per_day` 0.05, `men_per_shot` 100, `hit_chance` 0.3,
//! `defender_hit_chance` 0.6, `men_per_hit` 20, `beaten_share` 0.2,
//! `captured_stock_share` 0.5, `preparation_per_day` 5,
//! `training_days_per_level` 30. A size 0 port works cargo by lighter at 20
//! t an hour. The night pulse is 12 hours and each daylight pulse 6.
//!
//! The scenario names the wear and repair figures and not the air supply
//! figures: aircraft are repaired and cost no supply. No group has a
//! mission, so nothing flies and nothing wears.
//!
//! A unit's work in a day is its men ashore, times its fed share, times 1,
//! times its works skill over 2, and for a fighting unit times 0.5. It is
//! made in the ground combat phase, after the fights, in a hex that holds a
//! base of its side, on a day the unit did not march and its hex was not
//! contested.
//!
//! ## Inputs
//!
//! A placeholder map with five coast hexes on one axis, all clear, port
//! size 0, no land links: A at (0,0), B at (5,0), C at (10,0), D at (15,0)
//! and E at (20,0). The game is 6 days. Day d begins at hour 24 x (d - 1).
//!
//! | Base | Side     | Hex | Supply  | Airfield                 | Defences | Repairs a day |
//! | ---- | -------- | --- | ------- | ------------------------ | -------- | ------------- |
//! | 1    | Allied   | A   | 5000 t  | none, 17000 of work done | 0        | 0             |
//! | 2    | Allied   | B   | 5000 t  | yes                      | 0        | 0             |
//! | 3    | Allied   | C   | 13.5 t  | yes                      | 0        | 1             |
//! | 4    | Allied   | D   | 5000 t  | yes                      | 40       | 0             |
//! | 5    | Japanese | D   | 5000 t  | yes                      | 0        | 0             |
//! | 6    | Allied   | E   | 1000 t  | none, 5000 of work done  | 30       | 0             |
//! | 7    | Japanese | E   | 100 t   | yes                      | 0        | 0             |
//!
//! Every unit has experience 2 but unit 4, and every skill 2 but as stated.
//!
//! | Unit | Side     | Kind     | Men  | Hex | Stated                | Posture |
//! | ---- | -------- | -------- | ---- | --- | --------------------- | ------- |
//! | 1    | Allied   | working  | 1000 | A   |                       | defend  |
//! | 2    | Allied   | working  | 400  | A   | Works 4               | defend  |
//! | 3    | Allied   | working  | 500  | B   |                       | defend  |
//! | 4    | Allied   | fighting | 3000 | B   | Experience 1          | defend  |
//! | 5    | Allied   | working  | 600  | C   |                       | defend  |
//! | 6    | Allied   | fighting | 1000 | D   |                       | defend  |
//! | 7    | Allied   | working  | 2000 | D   |                       | defend  |
//! | 8    | Japanese | fighting | 1000 | D   |                       | attack  |
//! | 9    | Allied   | working  | 500  | E   |                       | defend  |
//! | 10   | Japanese | fighting | 100  | E   |                       | attack  |
//! | 11   | Allied   | fighting | 900  | A   |                       | defend  |
//! | 12   | Allied   | fighting | 600  | B   |                       | defend  |
//!
//! Forces, all Allied, all at base 2 and stopped there throughout:
//!
//! | Force | Ship                                         |
//! | ----- | -------------------------------------------- |
//! | 1     | 1, placeholder cargo ship, 1500 t of supply  |
//! | 2     | 2, placeholder cargo ship, 1000 t of supply  |
//! | 3     | 3, placeholder troop transport, empty        |
//! | 4     | 4, placeholder troop transport, empty        |
//!
//! Air groups, both Allied, neither with a mission:
//!
//! - Group 1: placeholder fighter (transfer range 500 nm), at base 2, 12
//!   ready.
//! - Group 2: placeholder flying boat, at base 3, 2 ready and 20 damaged.
//!
//! Orders:
//!
//! - Day 1: unit 3 works at the port. Unit 4 trains. Unit 5 works at the
//!   aircraft. Unit 11 works at the defences. Force 1 unloads. Force 3
//!   prepares a landing at E with unit 12 as its roster. Units 1, 2, 7 and
//!   9 are given no order.
//! - Day 2: rebase group 1 to base 1.
//! - Day 3: rebase group 1 to base 1. Unit 4 works at the port. Unit 12
//!   works at the defences.
//! - Day 4: force 2 unloads.
//! - Day 5: unit 12 stands ready. Force 4 prepares a landing at E with unit
//!   12 as its roster.
//!
//! ## At A: an airfield built, and defences dug
//!
//! Units 1 and 2 have no order. Base 1 has no airfield, so they build it.
//!
//! - Unit 1: 1000 x 1 x 2/2 = 1000 a day.
//! - Unit 2: 400 x 1 x 4/2 = 800 a day.
//!
//! | After day | Work done on the airfield | Airfield                    |
//! | --------- | ------------------------- | --------------------------- |
//! | 1         | 18800                     | none                        |
//! | 2         | 20600                     | built, in the phase of day 2 |
//! | 6         | 20600                     | built                       |
//!
//! The day's work is added whole on the day the airfield is built, and
//! nothing is added after. From day 3 units 1 and 2, still with no order,
//! work the port of base 1: 1800 a day. No ship is there and it changes
//! nothing.
//!
//! The rebase of group 1 from base 2, 5 hexes and 200 nm away, within 500:
//!
//! - Day 2, at intake: refused, as base 1 has no airfield.
//! - Day 3, at intake: accepted. Group 1 is at base 1 with 12 ready.
//!
//! Unit 11, a fighting unit at the defences: 900 x 1 x 2/2 x 0.5 = 450 a
//! day, 450 / 200 = 2.25 points a day. Defences are not cut to a whole
//! number. Base 1's defences are 2.25 after day 1 and 13.5 after day 6.
//!
//! Supply. Base 1 feeds 2300 men, 11.5 t a night, six times: 4931 t at the
//! end.
//!
//! ## At B: the port, and one activity a day
//!
//! Work on the port of base 2, and the rate of the day after:
//!
//! | Day | Unit 3 | Unit 4                        | Work on the port | Rate the next day                 |
//! | --- | ------ | ----------------------------- | ---------------- | --------------------------------- |
//! | 1   | 500    | trains                        | 500              | 20 x 1.5 = 30 t an hour           |
//! | 2   | 500    | trains                        | 500              | 30 t an hour                      |
//! | 3   | 500    | 3000 x 0.5 = 1500             | 2000             | 20 x 3, held at twice: 40 t an hour |
//! | 4   | 500    | 1500                          | 2000             | 40 t an hour                      |
//! | 5   | 500    | 1500                          | 2000             | 40 t an hour                      |
//! | 6   | 500    | 1500                          | 2000             | the game is over                  |
//!
//! Day 1 has no work from the day before: 20 t an hour.
//!
//! Force 1, 1500 t, unloading from the night pulse of day 1:
//!
//! | Day | Rate        | Night, 12 h | Morning, 6 h | Afternoon, 6 h | Landed so far |
//! | --- | ----------- | ----------- | ------------ | -------------- | ------------- |
//! | 1   | 20 t an hour | 240 t      | 120 t        | 120 t          | 480 t         |
//! | 2   | 30 t an hour | 360 t      | 180 t        | 180 t          | 1200 t        |
//! | 3   | 30 t an hour | 300 t      |              |                | 1500 t        |
//!
//! It is complete 300 / 30 = 10 hours into the night pulse of day 3, at
//! hour 58.
//!
//! Force 2, 1000 t, unloading from the night pulse of day 4 at 40 t an
//! hour: 480 t, 240 t and 240 t on day 4, 960 t. The last 40 t take 1 hour
//! of the night pulse of day 5: complete at hour 97.
//!
//! Unit 4, a fighting unit of experience 1:
//!
//! | After day | Its day was | Days of training counted | Experience |
//! | --------- | ----------- | ------------------------ | ---------- |
//! | 1         | training    | 1                        | 1          |
//! | 2         | training    | 2                        | 1          |
//! | 3 to 6    | work        | 2                        | 1          |
//!
//! Unit 12 and the two forces. Unit 12 is ashore at B, in the hex of both
//! forces.
//!
//! | After day | Unit 12's day was            | Its rehearsal, for E | Force 3's loading | Force 4's loading | Base 2's defences |
//! | --------- | ---------------------------- | -------------------- | ----------------- | ----------------- | ----------------- |
//! | 1         | rehearsal, with force 3      | 5                    | 5                 | 0                 | 0                 |
//! | 2         | rehearsal, with force 3      | 10                   | 10                | 0                 | 0                 |
//! | 3         | work: 600 x 0.5 = 300        | 10                   | 15                | 0                 | 1.5               |
//! | 4         | work                         | 10                   | 20                | 0                 | 3                 |
//! | 5         | rehearsal, with force 4      | 15                   | 20                | 5                 | 3                 |
//! | 6         | rehearsal, with force 4      | 20                   | 20                | 10                | 3                 |
//!
//! - Days 3 and 4: unit 12 works. It stays on force 3's roster, the force
//!   goes on loading and nothing is cut.
//! - Day 5: force 4's order takes unit 12 from force 3's roster, which
//!   costs force 3 nothing. Force 3's roster is empty: its order stands,
//!   it keeps its 20 and gains no more. Force 4 begins at 0. Unit 12 keeps
//!   its rehearsal of 10, which is for E, the hex of the new order.
//!
//! Supply. Base 2 feeds units 3, 4 and 12, 4100 men, 20.5 t a night, six
//! times: 123 t. It is given 1500 t and 1000 t. It holds 5000 - 123 + 2500
//! = 7377 t at the end.
//!
//! ## At C: aircraft repaired, and hungry men
//!
//! Unit 5 costs 600 x 0.005 = 3 t a night. Repair comes before the men's
//! supply in the night pulse. A night's repairs at base 3 are its own 1 and
//! one more for each whole 200 of the work on the aircraft of the day
//! before.
//!
//! | Day | Night: repairs | Group 2, ready and damaged | Night: base 3 pays | Stock | Men fed | Fed share | Men lost | Unit 5's men | Work that day |
//! | --- | -------------- | -------------------------- | ------------------ | ----- | ------- | --------- | -------- | ------------ | ------------- |
//! | 1   | 1              | 3 and 19                   | 3 t                | 10.5 t | 600    | 1         | 0        | 600          | 600           |
//! | 2   | 1 + 3 = 4      | 7 and 15                   | 3 t                | 7.5 t | 600     | 1         | 0        | 600          | 600           |
//! | 3   | 4              | 11 and 11                  | 3 t                | 4.5 t | 600     | 1         | 0        | 600          | 600           |
//! | 4   | 4              | 15 and 7                   | 3 t                | 1.5 t | 600     | 1         | 0        | 600          | 600           |
//! | 5   | 4              | 19 and 3                   | 1.5 t              | 0 t   | 300     | 0.5       | 15       | 585          | 292.5         |
//! | 6   | 1 + 1 = 2      | 21 and 1                   | nothing            | 0 t   | 0       | 0         | 29       | 556          | 0             |
//!
//! - Night of day 5: 1.5 t feeds 300 men. 300 are unfed and 300 x 0.05 =
//!   15 are lost. The day's work is 585 x 0.5 = 292.5.
//! - Night of day 6: 292.5 / 200 gives 1 more repair. Nobody is fed: 585 x
//!   0.05 = 29.25, 29 men lost.
//!
//! ## At D: defences in a fight, and working men under fire
//!
//! Night of day 1: base 4 pays 3000 x 0.005 = 15 t and holds 4985 t; base 5
//! pays 5 t and holds 4995 t. All are fed.
//!
//! Day 1. The hex is contested: both sides have fighting men ashore and
//! unit 8 is on attack.
//!
//! - Unit 8: 1000 firing men, 10 shots at 0.3.
//! - Unit 6, defending behind defences of 40: 1000 firing men, 10 shots at
//!   0.6 x (1 + 0.5 x 40/100) = 0.6 x 1.2 = 0.72.
//! - Unit 7 fires nothing.
//!
//! The men the Allied side loses, 20 for each hit, are taken from units 6
//! and 7 in proportion to their men, by the rule that serves every loss.
//! For one hit: 20 x 1000/3000 is 6 from unit 6 and 20 x 2000/3000 is 13
//! from unit 7, and the man still to take comes from unit 6: 7 and 13.
//!
//! The test of a side beaten counts unit 6's men and not unit 7's. Ten hits
//! would leave unit 6 with at least 933 men against 0.2 x 1000 = 200: the
//! Allied side is not beaten on day 1 whatever the dice give.
//!
//! Unit 7 makes no work on a day the hex is contested. On a day it is not
//! (unit 8 back on defend), unit 7, with no order at a base that has an
//! airfield, works the port of base 4: its men ashore times its fed share.
//! The tests assert that from the run, day by day, and that base 4's
//! defences are 40 at the end.
//!
//! ## At E: working men do not hold a base
//!
//! Night of day 1: base 6 pays 500 x 0.005 = 2.5 t and holds 997.5 t. Base
//! 7 pays 100 x 0.005 = 0.5 t and holds 99.5 t.
//!
//! Day 1. The Allied side has no fighting man ashore at E, so the hex is
//! not contested and there is no fight. Unit 10 is on attack and the Allied
//! side has a base there: the Allied side is beaten without a fight, and
//! base 6 changes side.
//!
//! - Base 6 keeps 997.5 x 0.5 = 498.75 t, and 498.75 t are lost.
//! - It keeps its defences of 30 and its 5000 of work done on the airfield.
//!   It still has no airfield.
//! - Unit 9 makes no work on day 1: when the work is made its side has no
//!   base in the hex. It makes none after.
//! - Unit 10 is not tired: it attacked nothing.
//!
//! From the night of day 2 the Allied side has no base at E, and unit 9 is
//! unfed, with a fed share of 0:
//!
//! | Night of day | Unfed | Lost             | Unit 9's men |
//! | ------------ | ----- | ---------------- | ------------ |
//! | 2            | 500   | 25               | 475          |
//! | 3            | 475   | 23.75, 23        | 452          |
//! | 4            | 452   | 22.6, 22         | 430          |
//! | 5            | 430   | 21.5, 21         | 409          |
//! | 6            | 409   | 20.45, 20        | 389          |
//!
//! From the night of day 2 unit 10 is fed by base 6, the lower-numbered of
//! its side's two bases there: 0.5 t a night, five times. Base 6 holds
//! 498.75 - 2.5 = 496.25 t at the end and base 7 holds 99.5 t.
//!
//! # Properties
//!
//! - No unit's day is counted twice: on no day does a unit gain two of
//!   work, training and rehearsal.
//! - Work never falls when men, fed share or works skill rise, and a day's
//!   port rate is never above twice the port's own.
//! - With no working unit, no activity order, every stated base with an
//!   airfield and no defences, a scenario gives what it gave before, apart
//!   from the hash version.
//! - The enemy's view is the same in two worlds that differ only in the
//!   other side's activities, jobs, work done and defences, where the hits
//!   are the same.
