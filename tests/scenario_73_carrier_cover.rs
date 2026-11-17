//! Scenario 73: carrier cover. The commander asks for an airfield and
//! names a carrier force beside the transports. The staff sends the
//! carrier to the first hex from which her bombers reach the airfield,
//! strikes it each day until the men are ashore and it is theirs, and
//! then sends her home.
//!
//! Proves decision 0148, step 6 of stage 3, second part, against a hand
//! calculation. It stands on scenario 55's for a carrier force's orders
//! and on scenario 69's for the landing. The data is placeholder data:
//! see `data/scenarios/73_carrier_cover.toml`. It runs on the frozen step
//! 1 figures with scenario 53's table of aircraft for each ship and one
//! ground figure of its own, `preparation_per_day` 100. The upkeep, wear,
//! fatigue and supply figures of the air are not loaded. Base 2 has no
//! gun, no aircraft and no man, so no aircraft is lost.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly, except what each bomb hits and how many strikes a group
//! launches in a day, which are the air rules' and are not this
//! scenario's to test. If the code disagrees, the code or the rule is
//! wrong: do not change a number here to make a test pass. A mismatch
//! goes back to the architect.
//!
//! ## Figures
//!
//! A hex is 40 nm. The placeholder dive bomber (type 3) has a strike
//! radius of 250 nm. The placeholder carrier scout (type 2) searches to
//! 300 nm. The placeholder carrier (class 4): 16 kn at cruise, 3,000 t of
//! fuel over 9,000 nm. The placeholder troop transport: 10 kn. At
//! balanced a patrol share is 0.5, the least a raid sends is 6, and a
//! raid goes without an escort. Day d begins at hour 24 x (d - 1).
//!
//! ## Inputs
//!
//! A placeholder map of open sea along the row r = 0, with A at (0,0), a
//! coast hex with port size 5, and B at (10,0), a coast hex with port
//! size 0: 400 nm apart. The game is 8 days.
//!
//! Base 1, Allied, at A: 1,000 t of supply and 5,000 t of fuel. Base 2,
//! Japanese, at B, on the Allied map: no gun, aircraft, supply or man.
//! The Allied side holds no contact and no estimate. There is no Japanese
//! unit.
//!
//! Task force 1, Allied, on the carrier mission: ship 1, a placeholder
//! carrier, stopped at base 1, its home port, at cruising speed, full of
//! fuel, at normal readiness, with the stance engage. Aboard her, as
//! scenario 55: group 1, 12 fighters; group 2, 6 carrier scouts; group 3,
//! 10 dive bombers; each resting with no setting.
//!
//! Task force 2, Allied, on the transport mission: ships 2 and 3,
//! placeholder troop transports, stopped at base 1, nothing aboard. Units
//! 1 and 2, Allied, fighting units of 600 men, experience 2, every skill
//! 2, ashore at A.
//!
//! The Allied side's commands of day 1: intent 1, take the place at base
//! 2, with a forces limit of task forces 1 and 2, units 1 and 2 and base
//! 1 as the source. No risk is set, so balanced. Leave to prepare and
//! leave to launch are both given.
//!
//! ## The landing
//!
//! As scenario 69 with no hold stated, on a longer passage: 400 nm at
//! 10 kn is 40 hours, two days. The plan is released on day 2; the days
//! to embark are 3; the force sails on day 5 and is due on day 6.
//!
//! | Day | The staff's commands and orders to the landing |
//! | --- | --- |
//! | 1 | Approves plan 1 to preparing. Forms intent 2, ready a landing by task force 2 at B, with a forces limit of units 1 and 2. Orders `prepare_landing` at B, roster 1 and 2 |
//! | 2 | Notice: intent 2 is ready. Approves plan 1 to released. Orders a load of supply to a weight of 90 t |
//! | 3 | Unit 1 set to attack. Load unit 1 |
//! | 4 | Unit 2 set to attack. Load unit 2 |
//! | 5 | A route to B and back to A, acting at B, and an unload order |
//! | 7 | Marks plan 1 complete. Notice: intent 1 is met |
//!
//! The force sails at hour 96 and arrives at hour 136. Each ship lands
//! 160 men on the morning of day 6 and 440 in the afternoon. Base 2
//! changes side at the end of day 6.
//!
//! ## The covering hex
//!
//! On the morning of day 2 the carrier is at (0,0). The reach is 250 nm.
//! Along the row, (3,0) is 280 nm from B and (4,0) is 240 nm: the
//! covering hex is (4,0), 160 nm from A, 10 hours at 16 kn, inside the
//! night.
//!
//! ## The carrier, day by day
//!
//! | Day | The staff's commands and orders to the carrier | What happens |
//! | --- | --- | --- |
//! | 2 | Forms intent 3, operate task force 1 from (4,0), under intent 1. The force: rules of engagement, avoid, break-off level 0.5; a route to (4,0). Group 1: combat air patrol; a patrol share of 0.5. Group 2: the search mission; the arc 060 to 120 at 300 nm. Group 3: the naval strike mission; strike sizing on | Sails at hour 24, arrives at hour 34 |
//! | 3 | The force: battle readiness. Group 3: base strike at base 2 | Group 3 strikes base 2 with its 10 |
//! | 4, 5, 6 | none | Group 3 strikes base 2 on each day |
//! | 7 | The force: a route to A | Sails at hour 144, arrives at hour 154 |
//! | 8 | none | |
//!
//! - **Day 2.** B is 400 nm from the force, beyond the reach: group 3 is
//!   given the orders of scenario 55 at balanced. The one threat of the
//!   search is base 2, on 090 from (0,0): the arc is 060 to 120. No
//!   speed order is given: the force is at cruise, and cruise brings it
//!   there in the night.
//! - **Day 3.** The force is at its hex. B is 240 nm away, within the
//!   reach, and no contact is held: group 3 can send 10, more than the 6
//!   the risk asks, and a raid goes without an escort at balanced. From
//!   (4,0) base 2 is still on 090: no arc order.
//! - **The end of day 6.** Base 2 is Allied, and group 3's mission is
//!   cleared to rest by the rule of a base that changes side.
//! - **Day 7.** The intent is met: the force is routed home at cruise,
//!   and its groups are given no order.
//!
//! The carrier is given 8, 2 and 1 orders, the transports 8: nineteen in
//! all, and six staff commands. Group 3 launches at base 2 on each of
//! days 3 to 6 and on no other day. No aircraft is lost. No staff order
//! is refused.
//!
//! The carrier burns a third of a ton a mile: 53.333333 t each way. On
//! the morning of day 3 her passage home needs 53.333333 x 1.2 = 64 t
//! against 2,946.666667 aboard.
//!
//! # Properties
//!
//! - **Preserve the force.** At preserve the force group 3 holds back a
//!   third and can send 6, fewer than the 12 the risk asks: no raid goes.
//!   Group 3 is given no base strike order on any day, and the reason of
//!   day 3 gives 6 against 12.
//! - **Press.** At press the orders of days 3 and 7 are the same; the
//!   break-off level ordered on day 2 is 0.75, the patrol share 0.25, and
//!   group 3's orders of day 2 are scenario 55's four at press.
//! - **Ships first.** With the Allied side given a contact of 2 cruisers
//!   at (6,0) of age 0 on the morning of day 3: group 3 keeps the naval
//!   strike mission that morning and is given no base strike order. On
//!   the morning of day 5, the contact long dropped, it is set to the
//!   base strike.
//! - **No carrier named.** With task force 1 left out of the forces
//!   limit: no carrier task is formed, the carrier is given no order, and
//!   the landing's orders are unchanged.
//! - **Already in reach.** With the carrier starting stopped at sea at
//!   (5,0), 200 nm from B: the task is formed at (5,0), no route is
//!   given, and group 3 is set to the base strike on day 2.
//! - **Held.** With task force 1 held from day 4: it is given no order
//!   from day 4, day 7's route included.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see give the same orders and reasons; a
//!   save on day 4 and a resume give the same game; every scenario built
//!   before this one gives the events it gave.
