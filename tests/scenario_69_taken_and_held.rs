//! Scenario 69: taken and held. The commander asks for an airfield and
//! asks that it be held, from the base the men sail from. The staff works
//! out that the men can be fed, lands them with nine days of supply and
//! not fifteen, puts them on the defence when the airfield is theirs, and
//! sends the first run of supply after them when it is needed.
//!
//! Proves decision 0144, step 4 of stage 3, second part, against a hand
//! calculation. The data is placeholder data: see
//! `data/scenarios/69_taken_and_held.toml`. It runs on the frozen step 1
//! figures, with one ground figure of its own, `preparation_per_day` 100.
//! No enemy man is ashore anywhere, so no die is rolled.
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
//! A man weighs 0.25 t to load or land and eats 0.005 t a day, paid at
//! the start of the night pulse by the side's bases in the hex in base
//! order. Supply landed goes into the side's lowest-numbered base in the
//! hex. A size 5 port works at 150 t an hour; a size 0 port by lighter at
//! 20 t an hour for each ship. A force with its roster in port gains 100
//! of loading in a day and a unit ready on it 100 of rehearsal. Day d
//! begins at hour 24 x (d - 1): a night pulse of 12 hours and two
//! daylight pulses of 6. In a pulse ships move before cargo is worked. A
//! force that has acted steams on at the start of the next pulse.
//!
//! The staff's figures: at balanced the margin is 5 days; a supply intent
//! with no interval uses 15 days.
//!
//! ## Inputs
//!
//! The placeholder map of scenario 27: A at (0,0), a coast hex with port
//! size 5, and B at (4,0), a coast hex with port size 0, 160 nm apart: 16
//! hours at 10 kn, one day. The game is 14 days.
//!
//! | Base | Side | Hex | Supply | Fuel |
//! | ---- | ---- | --- | ------ | ---- |
//! | 1, "Placeholder base A" | Allied | A | 1,000 t | 0 |
//! | 2, "Placeholder airfield B" | Japanese | B | 0 | 0 |
//!
//! Base 2 has no gun, no aircraft and no man, and is on the Allied map.
//! No base has a reserve. The Allied side holds no estimate of the enemy.
//!
//! Task force 1, Allied, on the transport mission: ships 1 and 2,
//! placeholder troop transports (600 men, 3,000 t of cargo, 10 kn, 0.1 t
//! of fuel a mile, 1,000 t of fuel), stopped at base 1, full of fuel,
//! nothing aboard.
//!
//! Units 1 and 2, Allied, "Placeholder First" and "Placeholder Second":
//! fighting units of 600 men each, experience 2, every skill 2, ashore at
//! A on the defence, fed in full.
//!
//! The Allied side's commands of day 1: intent 1, take the place at base
//! 2, with a forces limit of task force 1, units 1 and 2 and base 1 as
//! the source; intent 2, hold base 2, with a forces limit of task force 1
//! and base 1 as the source. No risk is set on either, so balanced. Leave
//! to prepare and leave to launch are both given.
//!
//! ## The check, on the morning of day 2
//!
//! - The daily need: 1,200 men x 0.005 = 6 t.
//! - The days until supply follows: to land the men, 150 t in the fullest
//!   ship at 20 t an hour, 7.5 hours, 1 day; the passage back, 1; to
//!   load, 1; the passage out, 1. Four days.
//! - The margin: 5 days.
//! - The supply that lands with the men: 6 x (4 + 5) = 54 t.
//! - The first run: 6 x 15 = 90 t.
//! - Base 1 holds 994 t that morning, with no reserve: 994 is more than
//!   54 + 90 = 144. The force's room is 6,000 t. It can be held.
//!
//! ## Day by day
//!
//! | Day | Hour | The staff's commands and orders | What happens |
//! | --- | --- | --- | --- |
//! | 1 | 0 | For intent 1: approves plan 1 to preparing; forms intent 3, ready a landing by task force 1 at B, with a forces limit of units 1 and 2; orders `prepare_landing` at B, roster 1 and 2. For intent 2: approves its plan to preparing and to released; no order: base 2 is not the side's | Loading 100; each unit's rehearsal 100 |
//! | 2 | 24 | Notice: intent 3 is ready. Approves plan 1 to released. Orders a load of supply to a weight of 54 t | Ship 1 loads 54 t |
//! | 3 | 48 | Unit 1 set to attack. Load unit 1 | Ship 1 takes 600 men |
//! | 4 | 72 | Unit 2 set to attack. Load unit 2 | Ship 2 takes 600 men |
//! | 5 | 96 | A route to B and back to A, acting at B, and an unload order | Sails at hour 96, arrives at hour 112. Each ship lands 160 men in the morning and 440 in the afternoon; ship 1 then lands 10 t. Base 3, "Landing at (4,0)", is formed. Base 2 changes side at the end of the day |
//! | 6 | 120 | Marks plan 1 complete; notice: intent 1 is met. For intent 2: units 1 and 2 set to defend; forms intent 4, keep base 2 supplied, under intent 2, with a floor of 5 days | Ship 1 lands 44 t in 2.2 hours. The force steams for A at hour 132 |
//! | 7 | 144 | Intent 4: no run is needed | The force arrives at A at hour 148 |
//! | 8 | 168 | Intent 4: no run is needed | |
//! | 9 | 192 | Intent 4: a run. `add_run`, `commit_ships`; orders a load of supply to a weight of 90 t | Ship 1 loads 90 t |
//! | 10 | 216 | A route to B and back to A, acting at B, and an unload order | Sails at hour 216, arrives at hour 232. Ship 1 lands 40 t in the morning and 50 t in the afternoon |
//! | 11 | 240 | none | The force steams for A at hour 240 and arrives at hour 256 |
//! | 12 to 14 | | none | |
//!
//! Thirteen orders in all. No staff order or command is refused. On each
//! morning from day 2 to day 5 intent 2 has one reason for giving no
//! order: it waits for base 2 to be the side's. From day 6 the hold's
//! reason for no cover is that there is nothing at the base to cover it
//! with, and task force 1 is given no task of its own.
//!
//! The commands that prepare and release the plan of an intent the staff
//! formed are as the tree's rules give them, and are not counted here.
//!
//! ## The supply doctrine's sums for intent 4
//!
//! The use at B is the men ashore, 6 t a day. The floor's stock is
//! 5 x 6 = 30 t. A run ordered on a morning loads that day, sails the
//! next and arrives that same day; the reading checked is the day after.
//!
//! | Morning | The stock at B | The day checked | Its reading | The staff |
//! | --- | --- | --- | --- | --- |
//! | 7 | 48 t | 9 | 48 - 12 = 36 t | Not below 30: no run |
//! | 8 | 42 t | 10 | 42 - 12 = 30 t | Not below 30: no run |
//! | 9 | 36 t | 11 | 36 - 12 = 24 t | Below 30: a run |
//!
//! The run's load: the floor's stock on the day it arrives, day 10, 30 t,
//! less that day's reading, 30 t, which is 0; plus 15 days of use, 90 t.
//! Ship 1's room covers it; ship 2 goes with its force.
//!
//! ## Supply and fuel
//!
//! - **Base 1** feeds 1,200 men, 6 t, on the nights of days 1 to 3, and
//!   600 men, 3 t, on day 4: 21 t. It loads out 54 t and 90 t. It ends
//!   with 1,000 - 21 - 144 = 835 t. On the morning of day 2 it holds
//!   1,000 - 6 = 994 t.
//! - **Base 3** is given 10 t on the afternoon of day 5. On the night of
//!   day 6 base 2, now Allied and the lower numbered, holds nothing, and
//!   base 3 pays 6 t: 4 t, which it keeps to the end.
//! - **Base 2** takes the 44 t landed on the night of day 6, after that
//!   night's charge. It pays 6 t on each night from day 7: 38, 32, 26 and
//!   20 t after the nights of days 7 to 10. It takes 90 t on day 10:
//!   110 t. It pays 6 t on the nights of days 11 to 14: 86 t at the end.
//! - **The stock at B** on the mornings of days 7, 8, 9 and 10 is 48, 42,
//!   36 and 30 t.
//! - **Each ship** steams 320 nm twice and burns 64 t: 936 t at the end.
//!
//! # Properties
//!
//! - **No hold stated.** With intent 2 left out: the check is not made
//!   and the load of day 2 is to a weight of 6 x 15 = 90 t. Days 1 to 5
//!   are otherwise as in the main case. On day 6 the plan is marked
//!   complete and nothing else is ordered then or after: eight orders in
//!   all, and units 1 and 2 are on attack at the end.
//! - **A hold with no source.** With intent 2's forces limit left out:
//!   the load of day 2 is 90 t. On day 6 units 1 and 2 are set to defend,
//!   no supply intent is formed, and the hold's reason says the commander
//!   may name a source. Ten orders in all.
//! - **It cannot be held, at balanced.** With base 1 at 100 t of supply:
//!   on the morning of day 2 it holds 94 t against 144 t wanted, short by
//!   50 t. The staff says so, releases the plan and orders the load of
//!   54 t.
//! - **It cannot be held, at preserve the force.** With base 1 at 100 t
//!   and both intents at preserve the force: the margin is 10 days, the
//!   supply that lands with the men 6 x 14 = 84 t, and 84 + 90 = 174 t
//!   are wanted against 94 t, short by 80 t. The staff does not release
//!   the plan: it hands in the notice that it cannot meet intent 1 on the
//!   morning of day 2 and on each morning after, and nothing sails.
//! - **Press.** With both intents at press: the margin is 0, the load of
//!   day 2 is 6 x 4 = 24 t, and intent 4 is formed with a floor of 0
//!   days.
//! - **A hold of a base already the side's, with no man there,** plays as
//!   it did: every scenario built before this one gives the events it
//!   gave, scenarios 67 and 68 among them.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see give the same orders and reasons; a
//!   save on day 8 and a resume give the same game.
