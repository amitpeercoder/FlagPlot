//! Scenario 13b: a carrier closes an enemy airfield and strikes it in the
//! afternoon of the first day and in the morning of the second.
//!
//! Stage 1 of decision 0063, decision 7. It proves that a base strike
//! launches in the first daylight pulse of the day in which it can, and
//! still once a day. Nobody opposes the strike.
//!
//! The data is placeholder data: see
//! `data/scenarios/13b_afternoon_base_strike.toml`. The rule is
//! `docs/decisions/0063-strike-target-priority.md`, which settles item 3 of
//! `docs/decisions/0034-raids-on-a-base.md`.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. The tests below assert these
//! numbers. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass.
//!
//! ## The setting
//!
//! The frozen step 1 figures, unchanged. The map is
//! `placeholder_two_airfields.toml`. Two days.
//!
//! Japanese base 1, an airfield, is at (0,0), with 1,000 t of supply, no
//! anti-aircraft fire and no aircraft.
//!
//! The Allied carrier force, task force 1, starts at (12,0): carrier A,
//! ship 1 (class 4), and a destroyer, ship 2. It is ordered on day 1 to
//! move to (3,0), and steams at 16 knots, the cruising speed of both
//! classes. Aboard carrier A is air group 1, 18 dive bombers (type 3:
//! radius 250 nm), ordered on day 1 to strike base 1.
//!
//! ## Where the carrier is
//!
//! The way is clear along the row, so each leg is charged 40 miles. A force
//! is in the hex whose centre is nearest: the hex ahead from 20 miles along
//! a leg. Movement comes after air combat in a daylight pulse, so a
//! decision reads the position the pulse began with.
//!
//! | Moment              | Miles steamed | Legs done and miles into the next | Hex    | To base 1 |
//! | ------------------- | ------------- | --------------------------------- | ------ | --------- |
//! | Day 1, 06:00        | 12 x 16 = 192 | 4 and 32                          | (7,0)  | 280 nm    |
//! | Day 1, 12:00        | 288           | 7 and 8                           | (5,0)  | 200 nm    |
//! | Day 1, 16:30        | 360           | 9: arrived                        | (3,0)  | 120 nm    |
//! | Day 2, every pulse  | 360           | Stopped                           | (3,0)  | 120 nm    |
//!
//! ## The launches
//!
//! | Pulse             | Carrier at | In reach of 250? | Launch                                           |
//! | ----------------- | ---------- | ---------------- | ------------------------------------------------ |
//! | Day 1, morning    | (7,0)      | No, 280          | None. Nothing recorded                           |
//! | Day 1, afternoon  | (5,0)      | Yes, 200         | 18 at base 1, 200 nm                             |
//! | Day 2, morning    | (3,0)      | Yes, 120         | 18 at base 1, 120 nm                             |
//! | Day 2, afternoon  | (3,0)      | Yes, 120         | None: the group has flown at its base that day   |
//!
//! Two `BaseStrikeLaunched` events in every run, and no more. Before this
//! rule there was one: day 2's.
//!
//! ## The attacks
//!
//! Each raid finds its base. 18 shots at a bomb's chance of 0.2: 3.6
//! expected hits, from 0 to 18. A hit lands on aircraft with chance 0.3,
//! and there are none; on supply with chance 0.3, destroying 50 t; on
//! nothing with chance 0.4. So a raid destroys 3.6 x 0.3 x 50 = 54 t on
//! average, and the base's stock at the end is 1,000 less 50 times the
//! supply hits of both raids.
//!
//! No aircraft is lost: the group has 18 at the end of every run.
//!
//! Over seeds 1 to 1000, each raid's hits in all are within four standard
//! deviations of 3,600. One raid's hits have a standard deviation of the
//! square root of (18 x 0.2 x 0.8) = 1.697056, so 1,000 raids have 53.67,
//! and the allowed range is 3,386 to 3,814.
//!
//! ## Variants
//!
//! | Change                                      | Launches                                        |
//! | ------------------------------------------- | ----------------------------------------------- |
//! | The force starts stopped at (5,0), no move order | Morning of day 1 and of day 2, at 200 nm. None in an afternoon: the rule as it was |
//! | The force starts stopped at (7,0), no move order | None on either day: 280 nm                |
//!
//! # Properties
//!
//! - A base strike group launches at most once in a game day.
//! - It never launches at night, and never at a base beyond its reach from
//!   where its ship is when the pulse begins.
//! - If it can launch in the morning it does, and then not in the
//!   afternoon.
//! - A replay of any seed reproduces the daily hash.

mod common;
