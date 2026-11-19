//! Scenario 85: a night's load spread over its ships. Three fast
//! transports keep a garrison fed at a place an enemy base overlooks.
//! They arrive with two hours of the night left. Each carries what it
//! can land in two hours, and the run lands three ships' worth where it
//! landed one.
//!
//! Proves decision 0162, the first part of the first mend of step 3's
//! stage 4, against a hand calculation. The data is placeholder data:
//! see `data/scenarios/85_a_nights_load_spread.toml`. It reads the same
//! figures as scenario 70. Nothing is sighted and nothing fights, so no
//! die changes anything.
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
//! As scenario 70's. A man eats 0.005 t a day, paid at the start of the
//! night pulse. A size 5 port works at 150 t an hour; a size 0 port by
//! lighter at 20 t an hour for each ship. Day d begins at hour
//! 24 x (d - 1): a night pulse of 12 hours and two daylight pulses of 6.
//! In a pulse ships move before cargo is worked. A force that has landed
//! its load steams on at the start of the next pulse. A ship burns its
//! cruising rate a mile at or below cruising speed, and that times the
//! square of the speed over the cruising speed above it.
//!
//! The supply doctrine: a run ordered on a morning loads that day, sails
//! the next and, with a passage of a day or less, arrives that same day;
//! it is ordered when the reading of the day after it would arrive is
//! below the floor's stock; its load is the floor's stock on the day it
//! arrives less that day's reading, plus the use of 15 days, cut to the
//! ships' room.
//!
//! ## Inputs
//!
//! The placeholder map of scenario 27: A at (0,0), port size 5; B at
//! (4,0), port size 0; C at (0,6), port size 0. A to B is 4 hexes,
//! 160 nm. B to C is 6 hexes. The game is 14 days.
//!
//! | Base | Side | Hex | Supply | Fuel |
//! | ---- | ---- | --- | ------ | ---- |
//! | 1, "Placeholder base A" | Allied | A | 1,000 t | 0 |
//! | 2, "Placeholder garrison B" | Allied | B | 100 t | 0 |
//! | 3, "Placeholder enemy base C" | Japanese | C | 0 | 0 |
//!
//! Base 3 is on the Allied map and has no ship, aircraft or man. Unit 1,
//! Allied, "Placeholder Garrison", a fighting unit of 2,000 men, is
//! ashore at B on the defence.
//!
//! A new placeholder class, 13, "Placeholder fast transport with 50 t of
//! cargo": class 12 with 50 t of cargo in place of 30 (16 kn at cruise,
//! 36 kn at full, 500 t of fuel over 4,000 nm, which is 0.125 t a mile at
//! cruise; 200 men). It is in a file of its own.
//!
//! Task force 1, Allied, on the transport mission: ships 1, 2 and 3 of
//! class 13, stopped at base 1 at cruising speed, each with 150 t of
//! fuel and nothing aboard. Base 1 holds no fuel, so no ship is fuelled.
//!
//! The Allied side's commands of day 1: intent 1, keep base 2 supplied,
//! with a floor of 5 days and no interval, and a forces limit of ships
//! 1, 2 and 3 and base 1 as the source. No risk is set, so balanced.
//! Leave to prepare and leave to launch are both given.
//!
//! ## By night
//!
//! Base 3 is 6 hexes from B, within 15, and not in B's hex: the run goes
//! by night. At 16 kn the passage is 160 / 16 = 10 hours: it arrives in
//! the night with 2 hours of it left. A ship burns 160 x 0.125 = 20 t
//! each way, 40 t out and home, and holds 150 t: it has the fuel. At
//! 36 kn the passage is 160 / 36 = 4 hours 26 minutes 40 seconds, and a
//! ship would burn 0.125 x (36 / 16) squared = 0.6328125 t a mile,
//! 202.5 t out and home: it has 150 t, so full speed is passed over, and
//! the reason gives 150 against 202.5. Cruising speed is the speed; the
//! force is at it already, so no speed order is given.
//!
//! A ship's limit is 20 x 2 = 40 t. The level is 40 t.
//!
//! ## The supply doctrine's sums
//!
//! The use at B is 2,000 x 0.005 = 10 t a day; the floor's stock is 50 t.
//!
//! | Morning | The stock at B | The day checked | Its reading | The staff |
//! | --- | --- | --- | --- | --- |
//! | 1 | 100 t | 3 | 80 t | No run |
//! | 2 | 90 t | 4 | 70 t | No run |
//! | 3 | 80 t | 5 | 60 t | No run |
//! | 4 | 70 t | 6 | 50 t | Not below 50: no run |
//! | 5 | 60 t | 7 | 40 t | A run |
//! | 7 | 160 t | 9 | 140 t | No run |
//! | 8 to 14 | 150 t, falling by 10 t a day to 90 t | | 130 t, falling to 70 t | No run |
//!
//! The run's load: the floor's stock on the day it arrives, 50 t, less
//! that day's reading, 50 t, which is 0; plus 15 days of use, 150 t; the
//! ships' room is 150 t, so 150 t. The night's limit: each of the three
//! ships takes the least of its room, 50 t, and the level, 40 t: 40 t
//! each, 120 t in all. The load of 150 t is cut to 120 t. With the run
//! landed the reading of the day checked is 40 + 120 = 160 t, above the
//! floor: no notice.
//!
//! As the rule stood before decision 0162 the load was cut to 40 t, all
//! in ship 1.
//!
//! ## Day by day
//!
//! | Day | Hour | The staff's commands and orders | What happens |
//! | --- | --- | --- | --- |
//! | 1 | 0 | Approves plan 1 to preparing and to released | |
//! | 5 | 96 | `add_run`, `commit_ships`. Orders a load of supply to a weight of 120 t, no ship to hold more than 40 t | Ships 1, 2 and 3 load 40 t each |
//! | 6 | 120 | A route to B and back to A, acting at B, and an unload order | Sails at hour 120. Arrives at hour 130. Each ship lands 40 t by hour 132. The force steams for A at hour 132 and arrives at hour 142 |
//!
//! Three orders in all. On every other morning the staff gives one
//! reason for giving no order. No staff order or command is refused.
//!
//! The force is at B from hour 130 to hour 132, and at no hour of
//! daylight.
//!
//! ## Supply and fuel
//!
//! - **Base 2** pays 10 t at the start of each night: 90, 80, 70, 60 and
//!   50 t after the nights of days 1 to 5. On day 6 it pays 10 t and then
//!   takes 120 t: 160 t. Then it falls by 10 t a day: 80 t after day 14.
//!   No man goes unfed.
//! - **Base 1** loads out 120 t and ends with 880 t of supply.
//! - **Fuel.** Each ship burns 40 t and ends with 110 t. None is fuelled.
//!
//! ## The order alone
//!
//! A second game on the same map, with no staff: task force 1 of ships
//! 1, 2 and 3 of class 13 stopped at base 1, with 10 t, nothing and 45 t
//! of supply aboard. The Allied side orders a load of supply to a weight
//! of 100 t, no ship to hold more than 40 t. The force has 55 t aboard
//! and is to have 100 t: 45 t to take. Ship 1 takes the least of its
//! room, 40 t, the 45 t left and 40 - 10 = 30 t: 30 t. Ship 2 takes the
//! least of 50 t, the 15 t left and 40 t: 15 t. Ship 3 holds 45 t, above
//! the level: it takes nothing and gives nothing up. The ships end with
//! 40 t, 15 t and 45 t; base 1 ends with 955 t.
//!
//! # Properties
//!
//! - **The weight is not reached.** In the second game with nothing
//!   aboard any ship, a weight of 150 t and a level of 40 t: each ship
//!   loads 40 t, the force has 120 t and loads no more on any later day.
//! - **Refused.** A load with a level and no weight; with a level and a
//!   unit; with a level of 0. Each is refused by the engine, by the
//!   scenario's loader and by the agent's decoder.
//! - **No level.** A load to a weight of 100 t with no level, from
//!   nothing aboard: ships 1 and 2 load 50 t each and ship 3 none, as
//!   before.
//! - **Fuel as cargo** takes a level as supply does.
//! - **Press.** At press the run of the main case goes as before
//!   decision 0145: the load is 150 t with no level, 50 t in each ship;
//!   it sails at hour 120 and arrives at hour 130.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see give the same orders and reasons; a
//!   save on day 8 and a resume give the same game.
