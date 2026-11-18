//! Scenario 80: a take that waits. The commander asks for an airfield the
//! enemy is believed to hold with 600 men. The staff wants two men for
//! each of them and has one unit of 600 to hand; a second may be used
//! from day 4. It does not sail short: it waits for day 4 and goes with
//! both. Where no day in reach brings enough, it says so once and waits
//! for the commander.
//!
//! Proves decision 0159, rule B (items 4 to 7), step 2 of the opening of
//! stage 4, against a hand calculation. The data is placeholder data: see
//! `data/scenarios/80_a_take_that_waits.toml`. It runs on the frozen step
//! 1 figures with one ground figure of its own, `preparation_per_day`
//! 100, as scenario 69 does. The enemy is believed to be at B and is not:
//! no Japanese man is ashore anywhere, so no die is rolled.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Figures
//!
//! As scenarios 67 and 69. A force with its roster in port gains 100 of
//! loading in a day and a unit ready on it 100 of rehearsal. A man weighs
//! 0.25 t to load or land and eats 0.005 t a day, paid at the start of
//! the night pulse by his side's bases in his hex in base order. Supply
//! landed goes into the side's lowest-numbered base in the hex. A size 5
//! port works at 150 t an hour; a size 0 port by lighter at 20 t an hour
//! for each ship. Day d begins at hour 24 x (d - 1): a night pulse of 12
//! hours and two daylight pulses of 6. In a pulse ships move before cargo
//! is worked. A base that changes side does so at the end of the day.
//!
//! The staff's figures: for each man of the estimate it wants 3 at
//! preserve, 2 at balanced and 1 at press; a take with no supply of its
//! own lands 15 days of supply; it looks 7 days ahead for a day that
//! brings enough, where the commander gave no latest day.
//!
//! ## Inputs
//!
//! The placeholder map of scenario 27: A at (0,0), port size 5, and B at
//! (4,0), port size 0, 160 nm apart: 16 hours at 10 kn. The game is 12
//! days.
//!
//! | Base | Side | Hex | Supply | Fuel |
//! | ---- | ---- | --- | ------ | ---- |
//! | 1, "Placeholder base A" | Allied | A | 1,000 t | 0 |
//! | 2, "Placeholder airfield B" | Japanese | B | 0 | 0 |
//!
//! Base 2 has no gun, no aircraft and no man, and is on the Allied map.
//! The Allied side begins with an estimate, told, of 600 enemy men at B.
//! It has no air group, so nothing can look.
//!
//! Task force 1, Allied, on the transport mission: ships 1 and 2,
//! placeholder troop transports (600 men, 3,000 t of cargo, 10 kn, 0.1 t
//! of fuel a mile, 1,000 t of fuel), stopped at base 1, full of fuel,
//! nothing aboard.
//!
//! Ashore at A, fighting units on the defence, experience 2, every skill
//! 2, fed in full:
//!
//! | Unit | Men | May be used |
//! | ---- | --- | ----------- |
//! | 1, "Placeholder First" | 600 | from day 1 |
//! | 2, "Placeholder Second" | 600 | from day 4 |
//!
//! The Allied side's command of day 1: intent 1, take the place at base
//! 2, with a forces limit of task force 1, units 1 and 2 and base 1 as
//! the source; no risk, so balanced; no latest day. Leave to prepare and
//! leave to launch are both given.
//!
//! ## The wait
//!
//! The staff wants 600 x 2 = 1,200 men. On days 1, 2 and 3 it may use
//! unit 1 alone: 600. Unit 2 may be used from day 4, which is within 7
//! days of each of those mornings, and 600 + 600 = 1,200. So on each of
//! days 1, 2 and 3 the take forms nothing and gives no order, and its
//! reason says that it wants 1,200 men, has 600 to hand, that unit 2 may
//! be used from day 4, and that it waits for day 4 (rule B, item 6). It
//! hands in no notice. On day 4 it does what a take does.
//!
//! ## Day by day
//!
//! | Day | Hour | The staff's commands and orders | What happens |
//! | --- | --- | --- | --- |
//! | 1 to 3 | | none; the reason above | |
//! | 4 | 72 | Approves plan 1 to preparing. Forms intent 2, ready a landing by task force 1 at B, with a forces limit of units 1 and 2. Orders `prepare_landing` at B, roster 1 and 2 | Loading 100; each unit's rehearsal 100 |
//! | 5 | 96 | Notice: intent 2 is ready. Approves plan 1 to released. Orders a load of supply to a weight of 90 t | Ship 1 loads 90 t |
//! | 6 | 120 | Unit 1 set to attack. Load unit 1 | Ship 1 takes 600 men |
//! | 7 | 144 | Unit 2 set to attack. Load unit 2 | Ship 2 takes 600 men |
//! | 8 | 168 | A route to B and back to A, acting at B, and an unload order | Sails at hour 168, arrives at hour 184. Each ship lands 160 men in the morning and 440 in the afternoon; ship 1 then lands 10 t. Base 3, "Landing at (4,0)", is formed. Base 2 changes side at the end of the day |
//! | 9 | 192 | Marks plan 1 complete. Notice: intent 1 is met | Ship 1 lands 80 t in 4 hours: complete at hour 196. The force steams for A at hour 204 |
//! | 10 | 216 | none | The force arrives at A at hour 220 |
//! | 11, 12 | | none | |
//!
//! Eight orders in all, none before day 4. No staff order or command is
//! refused. The supply is 1,200 x 0.005 x 15 = 90 t.
//!
//! ## Supply, fuel and men
//!
//! - **Base 1** feeds 1,200 men, 6 t, on the nights of days 1 to 6, and
//!   600 men, 3 t, on the night of day 7: 39 t. It loads out 90 t on day
//!   5. It holds 880 t at the end of day 5 and 871 t at the end.
//! - **Base 3** takes 10 t on the afternoon of day 8. On the night of
//!   day 9 base 2, now Allied and the lower numbered, holds nothing, and
//!   base 3 pays 6 t: 4 t, which it keeps to the end.
//! - **Base 2** takes the 80 t landed on the night of day 9, after that
//!   night's charge. It pays 6 t on the nights of days 10, 11 and 12:
//!   62 t at the end. It is Allied from the end of day 8.
//! - **Each ship** steams 320 nm and burns 32 t: 968 t at the end.
//! - Both units land with a rehearsal of 100 and gain no fatigue from the
//!   landing. Each has 600 men at the end.
//!
//! # Properties
//!
//! - **Enough to hand.** With the estimate 300 men: the staff wants 600
//!   and has them. On day 1 it orders `prepare_landing` at B with roster
//!   1 alone, as before this decision, and no reason speaks of waiting.
//!   Only day 1 is asserted.
//! - **At press it goes.** With the risk of intent 1 set to press: the
//!   staff wants 600 x 1 = 600. On day 1 it orders `prepare_landing` at B
//!   with roster 1 alone. Only day 1 is asserted.
//! - **A latest day that waiting can meet.** With intent 1 given a latest
//!   day of 8: a take begun on day 4 lands on day 8. The game is the main
//!   case, order for order, and no notice that it is late is handed in.
//! - **A latest day that waiting cannot meet.** With a latest day of 7:
//!   no day in reach brings 1,200 men. On day 1 the staff hands in one
//!   notice, that it cannot bring enough: 1,200 wanted, 600 to hand, unit
//!   2 from day 4, which is too late for day 7. It gives no order on days
//!   1 and 2 and hands in no second notice on day 2. With the commander's
//!   command of day 3, risk press on intent 1: on day 3 the staff orders
//!   `prepare_landing` at B with roster 1 alone. Days 1 to 3 are
//!   asserted.
//! - **Beyond the horizon.** With unit 2 usable from day 9 and no latest
//!   day: on day 1, day 9 is more than 7 days off, and the staff hands in
//!   the notice that it cannot bring enough and gives no order. On day 2
//!   day 9 is within 7 days: its reason says that it waits for day 9, and
//!   it hands in no notice. Days 1 and 2 are asserted.
//! - **Nothing coming.** With unit 2 left out of the scenario: the notice
//!   on day 1, once, and no order to task force 1 or unit 1 on any day of
//!   the game.
//! - **The other side.** The Japanese side, given nothing, is given no
//!   order by its staff on any day.
//! - The same start gives the same orders; a save on day 3 and a resume
//!   give the same game.
