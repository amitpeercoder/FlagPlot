//! Scenario 81: told to wait. The commander asks for an airfield and
//! tells the staff not to go before day 6. The staff makes the landing
//! ready at once, holds it, and releases it on the morning of day 6.
//! Other cases have it wait for a force, or for another place to be
//! taken first.
//!
//! Proves decision 0159, rule C (items 9 to 11), step 3 of the opening of
//! stage 4, against a hand calculation. The data is placeholder data: see
//! `data/scenarios/81_told_to_wait.toml`. It runs on the frozen step 1
//! figures with one ground figure of its own, `preparation_per_day` 100,
//! as scenario 69 does. No Japanese man is ashore anywhere and no
//! estimate is held, so no die is rolled and the staff takes every unit
//! its ships can carry.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## The three limits
//!
//! Each is a limit on a "take a place" or a "control an island" of the
//! commander's, read up the tree as the other limits are, and refused on
//! any other kind of intent.
//!
//! - **Not before** a day: unmet on a morning before that day.
//! - **Wait for** a task force or a land unit of the side: unmet until
//!   the side may use it and it lies at the source the intent's forces
//!   limit names (a force stopped there, a unit ashore there).
//! - **After** another "take a place" or "control an island" of the
//!   commander's: unmet until, on a morning, that intent's place is the
//!   side's (for an island, every place of it).
//!
//! While a limit is unmet the staff readies the landing and does not
//! release its plan: the plan stays preparing, leave to launch is not
//! asked, and each morning's reason names what is waited for. On the
//! first morning every limit is met, the take goes on as it does today.
//!
//! ## Figures
//!
//! As scenario 80's.
//!
//! ## Inputs
//!
//! The placeholder map of scenario 27: A at (0,0), port size 5; B at
//! (4,0), port size 0, 160 nm from A, 16 hours at 10 kn; C at (0,6), port
//! size 0, 240 nm from A, 24 hours. The game is 13 days.
//!
//! | Base | Side | Hex | Supply |
//! | ---- | ---- | --- | ------ |
//! | 1, "Placeholder base A" | Allied | A | 1,000 t |
//! | 2, "Placeholder airfield B" | Japanese | B | 0 |
//!
//! Base 2 has no gun, no aircraft and no man, and is on the Allied map.
//! No estimate is held.
//!
//! Task force 1, Allied, on the transport mission: ships 1 and 2,
//! placeholder troop transports (600 men, 3,000 t of cargo, 10 kn, 0.1 t
//! of fuel a mile, 1,000 t of fuel), stopped at base 1, nothing aboard.
//!
//! Units 1 and 2, Allied, fighting units of 600 men each, experience 2,
//! every skill 2, ashore at A on the defence, usable from day 1.
//!
//! The Allied side's commands of day 1: intent 1, take the place at base
//! 2, with a forces limit of task force 1, units 1 and 2 and base 1 as
//! the source; no risk, so balanced; no latest day; and the limit "not
//! before day 6" on intent 1. Leave to prepare and leave to launch are
//! both given.
//!
//! ## Day by day
//!
//! | Day | Hour | The staff's commands and orders | What happens |
//! | --- | --- | --- | --- |
//! | 1 | 0 | Approves plan 1 to preparing. Forms intent 2, ready a landing by task force 1 at B, with a forces limit of units 1 and 2. Orders `prepare_landing` at B, roster 1 and 2 | Loading 100; each unit's rehearsal 100 |
//! | 2 | 24 | Notice: intent 2 is ready. The plan is not released: the reason says it is not to go before day 6 | |
//! | 3 to 5 | | none; the same reason; no notice | |
//! | 6 | 120 | Approves plan 1 to released. Orders a load of supply to a weight of 90 t | Ship 1 loads 90 t |
//! | 7 | 144 | Unit 1 set to attack. Load unit 1 | Ship 1 takes 600 men |
//! | 8 | 168 | Unit 2 set to attack. Load unit 2 | Ship 2 takes 600 men |
//! | 9 | 192 | A route to B and back to A, acting at B, and an unload order | Sails at hour 192, arrives at hour 208. Each ship lands 160 men in the morning and 440 in the afternoon; ship 1 then lands 10 t. Base 3, "Landing at (4,0)", is formed. Base 2 changes side at the end of the day |
//! | 10 | 216 | Marks plan 1 complete. Notice: intent 1 is met | Ship 1 lands 80 t in 4 hours: complete at hour 220. The force steams for A at hour 228 |
//! | 11 | 240 | none | The force arrives at A at hour 244 |
//! | 12, 13 | | none | |
//!
//! Eight orders in all, one on day 1 and none on days 2 to 5. No staff
//! order or command is refused. On days 2 to 5 the plan's state is
//! preparing and no notice that it waits for leave is handed in.
//!
//! ## Supply and fuel
//!
//! - **Base 1** feeds 1,200 men, 6 t, on the nights of days 1 to 7, and
//!   600 men, 3 t, on the night of day 8: 45 t. It loads out 90 t on day
//!   6. It holds 874 t at the end of day 6 and 865 t at the end.
//! - **Base 3** takes 10 t on the afternoon of day 9 and pays 6 t on the
//!   night of day 10: 4 t at the end.
//! - **Base 2** takes 80 t on the night of day 10, after that night's
//!   charge, and pays 6 t on the nights of days 11, 12 and 13: 62 t at
//!   the end. It is Allied from the end of day 9.
//! - **Each ship** steams 320 nm and burns 32 t: 968 t at the end.
//!
//! # Properties
//!
//! - **No limit.** With no "not before": day 1 as above; on day 2 the
//!   notice that intent 2 is ready, plan 1 approved to released, and a
//!   load of supply to a weight of 90 t. Days 1 and 2 are asserted.
//! - **The limit lifted.** As the main case, with the commander's command
//!   of day 3 that clears "not before" on intent 1: on day 3 plan 1 is
//!   approved to released and the load of 90 t is ordered. Days 1 to 3
//!   are asserted.
//! - **Leave withheld.** As the main case, with leave to launch withheld:
//!   on days 2 to 5 no notice that plan 1 waits for leave is handed in;
//!   on day 6 it is, and the plan is not released. Days 1 to 6 are
//!   asserted.
//! - **Wait for a force.** With task force 2, Allied, on the surface
//!   combat mission, one placeholder cruiser, ship 3, stopped at base 1
//!   and usable from day 5; no "not before"; and the limit "wait for task
//!   force 2" on intent 1. Task force 2 is not in intent 1's forces
//!   limit and is given no order. Day 1 as the main case. On days 2, 3
//!   and 4 the plan is not released and the reason names task force 2
//!   and day 5. On day 5 plan 1 is approved to released and the load of
//!   90 t is ordered. Days 1 to 5 are asserted.
//! - **After another place.** With base 4, Japanese, at C, on the Allied
//!   map, with nothing at it; task force 3, Allied, on the transport
//!   mission, ship 4, a placeholder troop transport, stopped at base 1
//!   with nothing aboard; unit 3, Allied, a fighting unit of 600 men
//!   ashore at A; intent 3, stated on day 1 after intent 1, take the
//!   place at base 4, with a forces limit of task force 3, unit 3 and
//!   base 1 as the source; no "not before"; and the limit "after intent
//!   3" on intent 1. Intent 3 goes as a take does: readied on day 1,
//!   released on day 2 with a load of 600 x 0.005 x 15 = 45 t, unit 3
//!   embarked on day 3, sailed at hour 72. It arrives at C at hour 96, as
//!   the afternoon of day 4 ends, with no hour left to work cargo. In the
//!   night of day 5 it lands 600 men in 7.5 hours and then 45 t in 2.25
//!   hours. Base 4 is Allied at the end of day 5. Intent 1: day 1 as the
//!   main case; on days 2 to 5 the plan is not released and the reason
//!   names intent 3; on day 6 plan 1 is approved to released and the load
//!   of 90 t is ordered. Days 1 to 6 are asserted for intent 1, and for
//!   intent 3 the hours and the day base 4 changes side.
//! - **A wait that can never end.** As "After another place", with the
//!   commander's command of day 2 that withdraws intent 3: on day 3 the
//!   staff hands in one notice that intent 1 cannot be met as it stands,
//!   naming the limit and the withdrawn intent; on day 4 none. Plan 1 is
//!   preparing on every day to day 6 and is not released. Days 1 to 6
//!   are asserted.
//! - **Refused at intake.** A "not before" on a "hold" is refused. An
//!   "after" that names its own intent, a "hold", or an intent of the
//!   staff's is refused. A "wait for" that names a task force of the
//!   other side is refused with the reason a forces limit naming it is
//!   refused with, so that the refusal tells nothing of the enemy.
//! - The same start gives the same orders; a save on day 3 and a resume
//!   give the same game; with no such limit set, scenarios 67 to 80 give
//!   the orders they gave.
