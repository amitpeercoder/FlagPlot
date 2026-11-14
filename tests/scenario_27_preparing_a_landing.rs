//! Scenario 27: five forces at one port. One prepares its landing for ten
//! days, one does not prepare at all, one prepares for one beach and lands
//! at another, one changes its landing place, and one takes on a second
//! unit.
//!
//! Proves decision 0095 (preparing a landing) against a hand calculation.
//! The data is placeholder data: see
//! `data/scenarios/27_preparing_a_landing.toml`. The frozen step 1 figures
//! switch this rule off (`unprepared_landing_share` 1,
//! `unprepared_landing_fatigue` 0), so this test gives the four figures the
//! values below, in the way the experience test gives a strength above 0.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Nothing here is decided by
//! dice. The tests assert these numbers exactly. If the code disagrees, the
//! code or the rule is wrong: do not change a number here to make a test
//! pass. A mismatch goes back to the architect.
//!
//! ## Figures
//!
//! `preparation_per_day` 5, `unprepared_landing_share` 0.5,
//! `preparation_kept_on_change` 0.5, `unprepared_landing_fatigue` 40. From
//! the earlier stages: `handling_t_per_man` 0.25, `daily_supply_t_per_man`
//! 0.005, `unfed_loss_share_per_day` 0.05, `rest_per_day` 5. A port of size
//! 0 works cargo by lighter at 20 t an hour.
//!
//! ## Inputs
//!
//! A placeholder map with three coast hexes: A at (0,0) with port size 5, B
//! at (4,0) with port size 0 and C at (4,2) with port size 0. The game is 12
//! days. Day d begins at hour 24 x (d - 1): day 11 at hour 240 and day 12 at
//! hour 264. The night pulse is 12 hours and each daylight pulse 6.
//!
//! All is Allied. Base 1 at A holds 1000 t of supply. Base 2 at B holds 100
//! t. There is no base at C.
//!
//! Classes: the placeholder troop transport (600 men, 10 kn at cruise, 0.1 t
//! of fuel a mile, 1000 t of fuel) and the placeholder fast transport (200
//! men, 16 kn at cruise, 0.125 t a mile, 500 t of fuel).
//!
//! | Force | Ship                | Unit aboard at the start       |
//! | ----- | ------------------- | ------------------------------ |
//! | 1     | 1, troop transport, with 60 t of supply | Unit 1, 600 men |
//! | 2     | 2, troop transport  | Unit 2, 600 men                |
//! | 3     | 3, fast transport   | Unit 3, 200 men                |
//! | 4     | 4, troop transport  | Unit 4, 600 men                |
//! | 5     | 5, troop transport  | Unit 5, 300 men                |
//!
//! Unit 6, 300 men, is ashore at A. Every force starts at base 1, full of
//! fuel. Every unit starts with fatigue 0.
//!
//! Orders:
//!
//! - Day 1: forces 1, 3, 4 and 5 are each ordered to prepare a landing at B,
//!   with the unit aboard as the roster. Force 2 is given no such order.
//! - Day 3: force 1 is given the same order again.
//! - Day 6: force 4 is ordered to prepare a landing at C, same roster. Force
//!   5 is ordered to prepare a landing at B with units 5 and 6 as its
//!   roster.
//! - Day 11: forces 1 and 2 are sent to B and force 3 to C, each at cruise,
//!   each with an unload order.
//!
//! Loading and rehearsal grow after the day's orders are taken, for a force
//! stopped at a base of its side. A force ordered to sail that day is under
//! way and gains nothing.
//!
//! ## What is prepared, day by day
//!
//! | After the gain of day | Force 1 and unit 1 | Force 3 and unit 3 | Force 4 and unit 4 | Force 5 | Unit 5 | Unit 6 |
//! | --------------------- | ------------------ | ------------------ | ------------------ | ------- | ------ | ------ |
//! | 5                     | 25                 | 25                 | 25, for B          | 25      | 25     | 0      |
//! | 6                     | 30                 | 30                 | 5, for C           | 17.5    | 30     | 5      |
//! | 10                    | 50                 | 50                 | 25                 | 37.5    | 50     | 25     |
//! | 12                    | 50, until it lands | 50, until it lands | 35                 | 47.5    | 60     | 35     |
//!
//! - Force 1's second order, on day 3, is the same order: nothing changes.
//! - Force 4, day 6: a new place loses everything. Its loading and unit 4's
//!   rehearsal go from 25 to 0 at intake and gain 5 that day. Unit 4's
//!   rehearsal is for C from then on.
//! - Force 5, day 6: a unit added keeps half the loading, 25 x 0.5 = 12.5,
//!   and it gains 5 that day. Unit 5 keeps its 25. Unit 6 starts at 0. Unit
//!   6 is ashore at A, in the force's hex, and gains with the rest.
//! - Force 2 and unit 2 have 0 throughout.
//!
//! ## The landings
//!
//! Forces 1 and 2: A to B is 4 hexes, 160 nm, at 10 kn: 16 hours. They
//! arrive at hour 256, 2 hours before the end of the morning pulse of day
//! 11. Each ship burns 16 t of fuel and has 984 t.
//!
//! Force 1 has loading 50 for B: its rate is 20 x (0.5 + 0.5 x 50/100) = 15
//! t an hour. Force 2 has none: 20 x 0.5 = 10 t an hour.
//!
//! | Pulse             | Hours | Force 1 at 15 t an hour            | Force 2 at 10 t an hour  |
//! | ----------------- | ----- | ---------------------------------- | ------------------------ |
//! | Day 11, morning   | 2     | 30 t: 120 men                      | 20 t: 80 men             |
//! | Day 11, afternoon | 6     | 90 t: 360 men                      | 60 t: 240 men            |
//! | Day 12, night     | 12    | 120 men take 30 t; then the 60 t of supply | 280 men take 70 t |
//!
//! - Unit 1's first men land with a rehearsal of 50 for B: it gains 40 x
//!   50/100 = 20 fatigue, and its rehearsal is 0.
//! - Unit 2's first men land with none: it gains 40.
//! - Force 1's supply is landed at 15 t an hour after its last man is
//!   ashore: the loading holds for the whole unloading. It is complete 90 /
//!   15 = 6 hours into the night pulse of day 12, at hour 270. Its loading
//!   is then 0 and its order is done.
//! - Force 2 is complete 70 / 10 = 7 hours into that pulse, at hour 271.
//!
//! Force 3: A to C is 6 hexes, 240 nm, at 16 kn: 15 hours. It arrives at
//! hour 255, 3 hours before the end of the morning pulse of day 11. Ship 3
//! burns 30 t and has 470 t. Its loading of 50 is for B and it lands at C:
//! it works at 10 t an hour.
//!
//! - Day 11, morning: 3 hours, 30 t, 120 men. The first man forms base 3 at
//!   C. Unit 3's rehearsal of 50 is for B and counts as none at C: it gains
//!   40 fatigue, and its rehearsal is 0.
//! - Day 11, afternoon: the last 80 men take 20 t. Complete 2 hours into the
//!   pulse, at hour 260.
//! - Force 3 has not landed at B: its loading is still 50 and its order
//!   stands.
//!
//! ## Ashore
//!
//! Fatigue. A unit ashore that neither attacks nor marches rests 5 a day in
//! the ground combat phase.
//!
//! | Unit | On landing, day 11 | After day 11 | After day 12 |
//! | ---- | ------------------ | ------------ | ------------ |
//! | 1    | 20                 | 15           | 10           |
//! | 2    | 40                 | 35           | 30           |
//! | 3    | 40                 | 35           | 30           |
//!
//! Supply. Base 1 feeds unit 6 every night: 300 x 0.005 = 1.5 t, twelve
//! times, and holds 982 t at the end.
//!
//! Base 2, night of day 12, charged before that pulse's loading: 480 men of
//! unit 1 and 320 of unit 2 are ashore, 800 x 0.005 = 4 t. It holds 96 t,
//! and then 156 t when force 1's 60 t are landed.
//!
//! Base 3 at C has no stock. On the night of day 12 unit 3's 200 men are
//! unfed: 200 x 0.05 = 10 are lost and 190 are left.
//!
//! At the end: unit 1 has 600 men at B, unit 2 has 600 at B, unit 3 has 190
//! at C. Units 4 and 5 are aboard at A, and unit 6 is ashore there.
//!
//! # Properties
//!
//! - More days of preparation never land a force's men later.
//! - Loading and rehearsal are never above 100 nor below 0, and rehearsal is
//!   never copied: the rehearsal of all units is no more after a force
//!   splits or joins than before.
//! - With the frozen values of the two switches, every scenario with land
//!   units gives what it gave before, apart from the hash version.
//! - The enemy's view is the same in two worlds that differ only in a
//!   prepare order.
