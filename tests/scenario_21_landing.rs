//! Scenario 21: a convoy lands two land units and a little supply on an empty
//! shore, a third unit wastes on a shore with no base, and part of it is
//! carried across and lands as a new unit.
//!
//! Proves decision 0066 (land units, carried by sea and fed from a base)
//! against a hand calculation: men worked as cargo at a fixed weight, troop
//! capacity, a landing that forms a base, a unit divided, the nightly charge
//! for men, the unfed, and men lost taken from units in proportion.
//!
//! The data is placeholder data: see `data/scenarios/21_landing.toml`. The
//! test runs on the frozen step 1 figures (`data/reference/step1`), to which
//! `ground.toml` and the placeholder transport classes are added with their
//! first values.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. The tests below assert these
//! numbers exactly. If the code disagrees, the code or the rule is wrong:
//! do not change a number here to make a test pass. A mismatch goes back to
//! the architect.
//!
//! ## Inputs
//!
//! Figures, `ground.toml`:
//!
//! - `handling_t_per_man` 0.25, `daily_supply_t_per_man` 0.005,
//!   `unfed_loss_share_per_day` 0.05.
//!
//! Map, `placeholder_landing.toml`: the strip of scenario 2 (deep sea, q from
//! -1 to 10, r from -1 to 1), with three coast hexes: (0,0) with port size 5,
//! (8,0) with port size 0 and (10,0) with port size 0. A size 0 port has no
//! berth, so every ship at (8,0) and (10,0) works by lighter at 20 t an hour.
//!
//! Classes, `ship_classes_placeholder_transports.csv`:
//!
//! - Class 10, Placeholder troop transport: the placeholder cargo ship
//!   (cruise 10 kn, 1000 t of fuel for 10000 nm, so 0.1 t a mile; cargo
//!   capacity 3000 t) with a troop capacity of 600 men.
//! - Class 11, Placeholder fast transport: the placeholder destroyer (cruise
//!   16 kn, 500 t of fuel for 4000 nm, so 0.125 t a mile; no cargo capacity,
//!   no guns, no torpedoes) with a troop capacity of 200 men.
//!
//! The Allied side:
//!
//! - Base 1 at (0,0), supply 1000 t. It is the only base at the start.
//! - Task force 1 at base 1: ships 1 and 2, class 10, 1000 t of fuel each.
//!   Ship 1 starts with 14 t of supply as cargo.
//! - Task force 2 at (10,0), stopped: ship 3, class 11, 500 t of fuel.
//! - Land unit 1, 600 men, aboard task force 1. Land unit 2, 400 men, aboard
//!   task force 1. Units are put aboard in unit order, each over the force's
//!   ships in ship order, and a ship carries one unit: unit 1 fills ship 1
//!   (600), and unit 2 goes into ship 2 (400).
//! - Land unit 3, 500 men, ashore at (10,0), where the side has no base.
//!
//! There is no Japanese unit, ship or base. The game is 7 days.
//!
//! Orders:
//!
//! - Day 1, task force 1: route to (8,0), speed cruise, unload.
//! - Day 1, task force 2: load, cargo none, unit 3.
//! - Day 3, task force 2: route to (8,0), speed cruise, unload.
//!
//! Clock: hour 0 is 18:00 on day 1. The night pulse is 12 hours and each
//! daylight pulse 6. Day d begins at hour 24 x (d - 1). In the night pulse
//! the men's supply is charged before movement, and loading is the last
//! phase of every pulse.
//!
//! In all: 1500 men at the start and 14 t of supply aboard.
//!
//! ## Task force 1: the passage
//!
//! 8 hexes, 320 nm, at 10 kn.
//!
//! | Day | Pulse     | Hours | Distance | Total  |
//! | --- | --------- | ----- | -------- | ------ |
//! | 1   | Night     | 12    | 120 nm   | 120 nm |
//! | 1   | Morning   | 6     | 60 nm    | 180 nm |
//! | 1   | Afternoon | 6     | 60 nm    | 240 nm |
//! | 2   | Night     | 8     | 80 nm    | 320 nm |
//!
//! It arrives at (8,0) at hour 32, 02:00 on day 2. Each ship burns 320 x 0.1
//! = 32 t and has 968 t left.
//!
//! ## Task force 1: the landing
//!
//! Day 2, night: 4 hours are left of the pulse. Each ship's work is 4 x 20 =
//! 80 t, which is 80 / 0.25 = 320 men.
//!
//! - Ship 1 lands 320 men of unit 1. The first of them forms base 2 at (8,0):
//!   Allied, the next free base number, no stock. 280 men stay aboard. No
//!   work is left, so no supply moves.
//! - Ship 2 lands 320 men of unit 2. 80 stay aboard.
//!
//! Day 2, morning: 6 hours. Each ship's work is 120 t.
//!
//! - Ship 1 lands its 280 men, which take 70 t. 50 t of work are left: the
//!   14 t of supply are landed. Base 2 holds 14 t.
//! - Ship 2 lands its 80 men, which take 20 t.
//!
//! Unloading is complete. The slowest ship is ship 1: 70 + 14 = 84 t of work
//! at 20 t an hour is 4.2 hours, so the hour is 36 + 4.2 = 40.2, 10:12 on day
//! 2. Ashore at (8,0): unit 1 with 600 men and unit 2 with 400.
//!
//! ## Task force 2: taking men off
//!
//! Day 1, night. The men's supply is charged first: unit 3 loses 25 men (see
//! below) and has 475 ashore. Then loading: the load order with a unit is
//! accepted with no base at (10,0) and loads men only. The force is stopped
//! for all 12 hours: 240 t of work, which is 960 men. Ship 3's troop capacity
//! is 200, so 200 men of unit 3 go aboard and 275 stay ashore. Loading is
//! complete: 200 x 0.25 = 50 t at 20 t an hour is 2.5 hours, hour 2.5, 20:30
//! on day 1.
//!
//! Day 2: the force stays, full. Nothing moves and nothing is recorded.
//!
//! Day 3, night: 2 hexes, 80 nm, at 16 kn is 5 hours. It arrives at (8,0) at
//! hour 48 + 5 = 53, 23:00 on day 3. Ship 3 burns 80 x 0.125 = 10 t and has
//! 490 t left. 7 hours are left of the pulse: 140 t of work, 560 men.
//!
//! Unit 3 has men ashore in another hex, 249 at (10,0), so the 200 men aboard
//! become land unit 4, the next free number, before any lands. Unit 4 lands
//! its 200 men, which take 50 t. Base 2 is there already, so no base is
//! formed. Unloading is complete at hour 53 + 2.5 = 55.5, 01:30 on day 3.
//!
//! ## Unit 3 at (10,0): no base
//!
//! Every man ashore is unfed every night. The loss is the unfed men times
//! 0.05, cut to a whole man. Nothing is paid and no payment is recorded.
//!
//! | Night of | Ashore before | Times 0.05 | Lost | Ashore after      |
//! | -------- | ------------- | ---------- | ---- | ----------------- |
//! | Day 1    | 500           | 25         | 25   | 475, then 275 (200 embark) |
//! | Day 2    | 275           | 13.75      | 13   | 262               |
//! | Day 3    | 262           | 13.1       | 13   | 249               |
//! | Day 4    | 249           | 12.45      | 12   | 237               |
//! | Day 5    | 237           | 11.85      | 11   | 226               |
//! | Day 6    | 226           | 11.3       | 11   | 215               |
//! | Day 7    | 215           | 10.75      | 10   | 205               |
//!
//! Unit 3 loses 95 men unfed. The 200 men aboard ship 3 on the nights of days
//! 2 and 3 do not eat and are not lost.
//!
//! ## The men at (8,0): base 2
//!
//! On the night of day 2 no man is ashore there when the charge is made (the
//! convoy arrives later in that pulse), so nothing happens. Unit 4 lands
//! after the charge of day 3, so it first eats on the night of day 4.
//!
//! | Night of | Men  | Asked | Stock before | Paid | Stock after | Fed  | Unfed | Lost |
//! | -------- | ---- | ----- | ------------ | ---- | ----------- | ---- | ----- | ---- |
//! | Day 3    | 1000 | 5 t   | 14 t         | 5 t  | 9 t         | 1000 | 0     | 0    |
//! | Day 4    | 1200 | 6 t   | 9 t          | 6 t  | 3 t         | 1200 | 0     | 0    |
//! | Day 5    | 1200 | 6 t   | 3 t          | 3 t  | 0 t         | 600  | 600   | 30   |
//! | Day 6    | 1170 | none  | 0 t          | none | 0 t         | 0    | 1170  | 58   |
//! | Day 7    | 1112 | none  | 0 t          | none | 0 t         | 0    | 1112  | 55   |
//!
//! Day 5: 3 / 0.005 = 600 fed. 600 x 0.05 = 30. Day 6: the base has no stock,
//! so nothing is asked, paid or recorded as paid; 1170 x 0.05 = 58.5, cut to
//! 58. Day 7: 1112 x 0.05 = 55.6, cut to 55.
//!
//! Men lost are taken from the units in the hex in proportion: unit i gives
//! the loss times its men over all the men, in whole numbers, cut; the men
//! still to take are taken one from each unit in unit order.
//!
//! | Night of | Loss | Unit 1          | Unit 2           | Unit 4          | Left over | After: 1, 2, 4 |
//! | -------- | ---- | --------------- | ---------------- | --------------- | --------- | -------------- |
//! | Day 5    | 30   | 30x600/1200 = 15 | 30x400/1200 = 10 | 30x200/1200 = 5 | 0         | 585, 390, 195  |
//! | Day 6    | 58   | 58x585/1170 = 29 | 58x390/1170 = 19.33, 19 | 58x195/1170 = 9.67, 9 | 1, from unit 1 | 555, 371, 186 |
//! | Day 7    | 55   | 55x555/1112 = 27.45, 27 | 55x371/1112 = 18.35, 18 | 55x186/1112 = 9.2, 9 | 1, from unit 1 | 527, 353, 177 |
//!
//! ## The end, after day 7
//!
//! | Unit | Ashore | Where  | Aboard | Lost unfed | Began with            |
//! | ---- | ------ | ------ | ------ | ---------- | --------------------- |
//! | 1    | 527    | (8,0)  | 0      | 73         | 600                   |
//! | 2    | 353    | (8,0)  | 0      | 47         | 400                   |
//! | 3    | 205    | (10,0) | 0      | 95         | 500, of which 200 became unit 4 |
//! | 4    | 177    | (8,0)  | 0      | 23         | 200, from unit 3      |
//!
//! 527 + 353 + 205 + 177 = 1262 men ashore, and 73 + 47 + 95 + 23 = 238 lost:
//! 1500 in all. Base 1 holds 1000 t and base 2 holds 0 t; the 14 t landed
//! were all eaten. Ships 1 and 2 hold 968 t of fuel and ship 3 holds 490 t.
//! No ship has men or cargo aboard.
//!
//! Men ashore at the end of each day, units 1, 2, 3, 4:
//!
//! - Day 1: 0, 0, 275, none. Day 2: 600, 400, 262, none.
//! - Day 3: 600, 400, 249, 200. Day 4: 600, 400, 237, 200.
//! - Day 5: 585, 390, 226, 195. Day 6: 555, 371, 215, 186.
//! - Day 7: 527, 353, 205, 177.
//!
//! # Properties
//!
//! Beside the exact numbers, the tests check what must always hold (decision
//! 0066, "The test"): men are conserved; more supply never loses more men;
//! men aboard never exceed a ship's troop capacity and a ship never holds men
//! of two units; a scenario with no land unit is unchanged; the enemy's view
//! is the same in two worlds that differ only in a land unit, its landing and
//! the base it formed.
