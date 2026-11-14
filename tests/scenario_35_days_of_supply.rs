//! Scenario 35: days of supply. The figure a side reads for a hex it holds:
//! its stock there over its planned use for the day. With no use there is
//! no figure; a unit on its way counts from the day it is due; a surge of
//! sorties shows the day after it is ordered; repairs count; and a base
//! taken gives a figure to the side that took it and none to the side that
//! lost it.
//!
//! Proves decisions 31 to 36 of decision 0107 (step 3, stage 1) against a
//! hand calculation. The data is placeholder data, in three scenario files
//! so that no case has dice: `data/scenarios/35a_days_of_supply.toml` (men,
//! no aircraft), `35b_days_of_supply_sorties.toml` (aircraft that fly, with
//! the air combat and air supply figures and no wear or fatigue figures)
//! and `35c_days_of_supply_repairs.toml` (aircraft that do not fly, with
//! the wear and repair and the air supply figures). They run on the frozen
//! step 1 figures; 35a's test sets `weight_strength` to 0.25, as the test
//! of scenario 29 does.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly: no case has a random draw. If the code disagrees, the code or
//! the rule is wrong: do not change a number here to make a test pass. A
//! mismatch goes back to the architect.
//!
//! ## The rule
//!
//! The figure is for a side in a hex where it holds a base. It is read at
//! the moment a staff is asked: at the start of a day, after ships in port
//! have refuelled and before the day's orders are taken in. So it reads the
//! stock as the day before left it, and the missions and postures as the
//! day before left them.
//!
//! It is the supply stock of all the side's bases in the hex, over the
//! side's planned use there for that day:
//!
//! - the men's supply for its men ashore in the hex, each unit at its
//!   weight, and for the men aboard its own forces that are ordered to land
//!   there and are due that day or were due earlier and have not landed;
//! - 0.2 t for each of its aircraft there, ready or damaged;
//! - for each of its bases there, the supply of the repairs it would make
//!   that night: the smaller of its repairs a day and the damaged aircraft
//!   at it, at 1 t a repair;
//! - for each of its groups there on search or on patrol, the sorties of
//!   the day's two daylight pulses with the aircraft able to fly. A patrol
//!   sortie costs 0.5 t an aircraft. A strike mission counts nothing.
//!
//! With no planned use there is no figure. A figure is cut at six decimals.
//!
//! ## Figures
//!
//! `daily_supply_t_per_man` 0.005, `weight_strength` 0.25 (a unit of weight
//! 4 costs 1.5 times as much a man), `daily_supply_t_per_aircraft` 0.2,
//! `repair_supply_t` 1, `sortie_supply_t` 0.5. From the earlier rules:
//! `unfed_loss_share_per_day` 0.05, `captured_stock_share` 0.5. The men's
//! supply and the aircraft's upkeep are paid at the start of each night
//! pulse, which is the first of a day; repairs are made, and paid for,
//! before them.
//!
//! ## 35a: men
//!
//! A placeholder map of coast hexes far apart with no land link: A, B, C,
//! D, G and H. C is 14 hexes, 560 nm, from D on one axis. The game is 6
//! days. Every unit has weight 2 but as stated.
//!
//! | Base | Side     | Hex | Supply |
//! | ---- | -------- | --- | ------ |
//! | 1    | Allied   | A   | 500 t  |
//! | 2    | Allied   | B   | 600 t  |
//! | 3    | Allied   | C   | 300 t  |
//! | 4    | Allied   | D   | 5000 t |
//! | 7    | Japanese | G   | 200 t  |
//! | 8    | Allied   | H   | 200 t  |
//! | 9    | Japanese | H   | 50 t   |
//!
//! | Unit | Side     | Kind     | Men  | Where                     | Stated   | Posture |
//! | ---- | -------- | -------- | ---- | ------------------------- | -------- | ------- |
//! | 1    | Allied   | fighting | 2000 | B                         |          | defend  |
//! | 2    | Allied   | fighting | 1000 | C                         |          | defend  |
//! | 3    | Allied   | fighting | 3000 | aboard force 1, at base 4 | Weight 4 | defend  |
//! | 4    | Allied   | fighting | 100  | G                         |          | attack  |
//! | 6    | Allied   | working  | 100  | H                         |          | defend  |
//! | 7    | Japanese | fighting | 100  | H                         |          | attack  |
//!
//! Force 1 is five placeholder troop transports (600 men each, 10 kn at
//! cruise). On day 1 it is ordered to C at cruise with an unload order. 560
//! nm at 10 kn is 56 hours: it arrives at hour 56, in the night pulse of
//! day 3, so it is due on day 3. Each ship has 600 men to land, 150 t of
//! work at 20 t an hour: all are ashore 7.5 hours later, on day 3.
//!
//! ### At A: no use
//!
//! Base 1 holds 500 t and the Allied side has no man and no aircraft there.
//! There is no figure on any day.
//!
//! ### At B: men
//!
//! Unit 1 costs 2000 x 0.005 = 10 t a day.
//!
//! | Read on day | 1   | 2   | 3   | 4   | 5   | 6   |
//! | ----------- | --- | --- | --- | --- | --- | --- |
//! | Stock       | 600 | 590 | 580 | 570 | 560 | 550 |
//! | Figure      | 60  | 59  | 58  | 57  | 56  | 55  |
//!
//! ### At C: a unit arriving
//!
//! Unit 2 costs 5 t a day. Unit 3 costs 3000 x 0.005 x 1.5 = 22.5 t a day,
//! and counts from day 3, the day it is due. It is ashore on day 3 after
//! that night's feeding, so base 3 first pays for it on the night of day 4.
//!
//! | Read on day | Stock   | Planned use | Figure    |
//! | ----------- | ------- | ----------- | --------- |
//! | 1           | 300     | 5           | 60        |
//! | 2           | 295     | 5           | 59        |
//! | 3           | 290     | 27.5        | 10.545454 |
//! | 4           | 285     | 27.5        | 10.363636 |
//! | 5           | 257.5   | 27.5        | 9.363636  |
//! | 6           | 230     | 27.5        | 8.363636  |
//!
//! On day 1 the figure is 60 days, and the regiment that will cut it to ten
//! is already at sea. That is what the projection of decision 35 is for:
//! worked on day 1 against an intent of 30 days, the first day it falls
//! below is day 3. The projection is the staff's and is asserted in the
//! tests of stage 1, step 1; this file asserts the figure.
//!
//! ### At G: a base taken
//!
//! Day 1. The Allied side holds no base at G: it has no figure. The
//! Japanese side holds base 7 and has no man there: no figure. Unit 4 is
//! unfed on the night of day 1 and loses 100 x 0.05 = 5 men: 95. In the
//! ground combat phase the Japanese side, with a base and no man, is beaten
//! without a fight, and base 7 becomes Allied with 200 x 0.5 = 100 t.
//!
//! Unit 4 costs 95 x 0.005 = 0.475 t a day from then.
//!
//! | Read on day | Allied stock | Allied figure | Japanese figure |
//! | ----------- | ------------ | ------------- | --------------- |
//! | 1           | none         | none          | none            |
//! | 2           | 100          | 210.526315    | none            |
//! | 3           | 99.525       | 209.526315    | none            |
//! | 4           | 99.05        | 208.526315    | none            |
//!
//! ### At H: a base lost
//!
//! Day 1. Allied: 200 / (100 x 0.005) = 400. Japanese: 50 / 0.5 = 100.
//!
//! Night of day 1: base 8 pays 0.5 t and holds 199.5 t; base 9 pays 0.5 t
//! and holds 49.5 t. In the ground combat phase the Allied side, with only
//! working men, is beaten without a fight. Base 8 becomes Japanese with
//! 199.5 x 0.5 = 99.75 t. H has no land link and unit 6 surrenders.
//!
//! From day 2 the Japanese side holds two bases at H and the figure counts
//! both. Base 8 is the lower numbered and pays the 0.5 t a night.
//!
//! | Read on day | Allied figure | Japanese stock          | Japanese figure |
//! | ----------- | ------------- | ----------------------- | --------------- |
//! | 1           | 400           | 50                      | 100             |
//! | 2           | none          | 99.75 + 49.5 = 149.25   | 298.5           |
//! | 3           | none          | 148.75                  | 297.5           |
//!
//! ## 35b: a surge of sorties
//!
//! One coast hex, E. Base 5, Allied, 400 t, with an airfield. Air group 1:
//! 20 placeholder fighters ready, no mission. No enemy. The game is 6 days.
//! On day 3 the group is ordered to patrol.
//!
//! With no mission its use is its upkeep, 20 x 0.2 = 4 t a day. On patrol
//! it is 4 + 20 x 0.5 x 2 = 24 t a day. The order of day 3 is taken in
//! after the figure of day 3 is read, so the surge shows on day 4.
//!
//! | Read on day | Stock | Planned use | Figure    | That day the base pays       |
//! | ----------- | ----- | ----------- | --------- | ---------------------------- |
//! | 1           | 400   | 4           | 100       | 4                            |
//! | 2           | 396   | 4           | 99        | 4                            |
//! | 3           | 392   | 4           | 98        | 4, and 10 in each daylight pulse |
//! | 4           | 368   | 24          | 15.333333 | 24                           |
//! | 5           | 344   | 24          | 14.333333 | 24                           |
//! | 6           | 320   | 24          | 13.333333 | 24                           |
//!
//! ## 35c: repairs
//!
//! One coast hex, F. Base 6, Allied, 100 t, with an airfield and 3 repairs
//! a day. Air group 2: 10 ready and 5 damaged, no mission. The game is 4
//! days. Nothing flies, so nothing wears.
//!
//! | Read on day | Damaged | Stock | Upkeep | Repairs that night | Planned use | Figure    |
//! | ----------- | ------- | ----- | ------ | ------------------ | ----------- | --------- |
//! | 1           | 5       | 100   | 3      | 3                  | 6           | 16.666666 |
//! | 2           | 2       | 94    | 3      | 2                  | 5           | 18.8      |
//! | 3           | 0       | 89    | 3      | 0                  | 3           | 29.666666 |
//! | 4           | 0       | 86    | 3      | 0                  | 3           | 28.666666 |
//!
//! # Properties
//!
//! - The figure is the same in two worlds that differ only in what the
//!   side cannot see: the enemy's men, bases and stocks in other hexes, and
//!   in this one.
//! - A side is never given a figure for a hex where it holds no base, and
//!   never the enemy's figure.
//! - Setting a group to rest lowers the planned use, and the figure says in
//!   its parts what it counted, so that the reason can be read.
//! - Where the planned use is 0 the tools say that there is no figure, and
//!   never show a number.
