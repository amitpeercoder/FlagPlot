//! Scenario 36: one supply intent, one pool, no enemy. The staff keeps a
//! base at 30 days of supply: it forms a convoy, loads a stated weight,
//! sails, unloads, comes home, and sends the next run when the projection
//! calls for it. With no orders the base runs out.
//!
//! Proves the doctrine of decision 0107 (step 3, stage 1; decisions 37 to
//! 44, with the rulings the calculation needed, recorded there) against a
//! hand calculation. The data is placeholder data: see
//! `data/scenarios/36_one_convoy.toml`. It runs on the frozen step 1
//! figures. There is no enemy, no aircraft and no random draw.
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
//! `daily_supply_t_per_man` 0.005. A size 1 port has one berth, at 50 t an
//! hour; a size 0 port works by lighter at 20 t an hour. The placeholder
//! cargo ship: 3000 t of cargo, 10 kn at cruise, 0.1 t of fuel a mile, 1000
//! t of fuel. A day is 24 hours: a night pulse of 12 and two daylight
//! pulses of 6. Day d begins at hour 24 x (d - 1). In a pulse ships move
//! before cargo is worked. The men's supply is paid at the start of the
//! night pulse. Ships in port refuel at the start of a day.
//!
//! ## Inputs
//!
//! A placeholder map: S at (0,0), a coast hex with port size 1, and T at
//! (5,0), a coast hex with port size 0. S to T is 5 hexes on one axis: 200
//! nm, 20 hours at cruise. The game is 18 days.
//!
//! | Base | Side   | Hex | Supply | Fuel   |
//! | ---- | ------ | --- | ------ | ------ |
//! | 1    | Allied | S   | 5000 t | 2000 t |
//! | 2    | Allied | T   | 100 t  | 0      |
//!
//! Land unit 1, Allied, 2000 men, at T on defend. It costs 10 t a day.
//!
//! Ships 1 and 2, placeholder cargo ships, lie in port at base 1 in no
//! task force, full of fuel.
//!
//! The Allied side's commands, in force from day 1:
//!
//! - An intent: keep base 2 at a floor of 30 days of supply, with an
//!   interval between shipments of 15 days. At 10 t a day the floor is a
//!   stock of 300 t.
//! - The forces limit of that intent: ships 1 and 2, with base 1 as the
//!   source.
//! - A reserve at base 1 of 1000 t of supply.
//! - Leave to prepare and leave to launch both given.
//!
//! There is no other order and no Japanese unit.
//!
//! ## The rules the staff works by
//!
//! - **When a run is needed.** A run ordered on day d is loaded that day,
//!   sails on day d + 1 and arrives that day. It is needed on day d if,
//!   without it, the figure read on the day after it would arrive, day d +
//!   2, would be below the intent's floor.
//! - **Its ships** are the pool's free ships in number order until their
//!   room covers the load. A ship is free when it is back at the source and
//!   has refuelled there.
//! - **Its load** is the floor's stock, less the stock projected for the
//!   reading on the day it arrives and not below 0, plus the planned use of
//!   one interval between shipments. The intent's interval is 15 days: 150
//!   t.
//! - **One run of the plan is loading at a time.**
//!
//! ## Day by day
//!
//! Readings are at the start of the day, before its orders. The game is 18
//! days.
//!
//! | Day | Read at T: stock, days | The staff's orders                    | What happens |
//! | --- | ---------------------- | ------------------------------------- | ------------ |
//! | 1   | 100 t, 10              | Form a force of ship 1. Load 360 t of supply at base 1 | Loaded in 360 / 50 = 7.2 hours. Base 1 holds 4640 t. T pays 10 t |
//! | 2   | 90 t, 9                | Route to T and back, acting at T, with an unload order | Sails at hour 24, arrives at hour 44. T pays 10 t. 4 hours of the afternoon at 20 t an hour: 80 t landed |
//! | 3   | 160 t, 16              | none                                  | T pays 10 t. The last 280 t take 14 hours: the 12 of the night and 2 of the morning. Unloaded at hour 62. The force steams for S |
//! | 4   | 430 t, 43              | none                                  | T pays 10 t. The force arrives at S at hour 82 |
//! | 5   | 420 t, 42              | none                                  | Ship 1 refuels 40 t: base 1 holds 1960 t of fuel. T pays 10 t |
//! | 6 to 15 | 410 t falling to 320 t, 41 to 32 | none                    | T pays 10 t a day |
//! | 16  | 310 t, 31              | Load 150 t of supply at base 1        | Loaded in 3 hours. Base 1 holds 4490 t of supply. T pays 10 t |
//! | 17  | 300 t, 30              | Route to T and back, acting at T, with an unload order | Sails at hour 384, arrives at hour 404. T pays 10 t. 80 t landed |
//! | 18  | 370 t, 37              | none                                  | T pays 10 t. The last 70 t take 3.5 hours of the night: unloaded at hour 411.5. The force steams for S and arrives at hour 431.5 |
//!
//! Why each run, and its size:
//!
//! - **Day 1.** A run ordered today arrives on day 2. Without it the
//!   reading of day 3 is 80 t, below 300: a run is needed. The stock
//!   projected for the reading of day 2 is 90 t. The load is 300 - 90 +
//!   150 = 360 t. Ship 1 is the first free ship and has the room.
//! - **Days 2 to 15.** With the run ashore the reading falls by 10 t a day
//!   from 430 t on day 4: 310 t on day 16, 300 t on day 17 and 290 t on day
//!   18. On day 15 a run ordered then would arrive on day 16, and the
//!   reading of day 17 is 300 t, not below: no run. Ship 2 is free
//!   throughout and is not sent.
//! - **Day 16.** A run ordered today arrives on day 17, and without it the
//!   reading of day 18 is 290 t, below 300: a run is needed. The stock
//!   projected for the reading of day 17 is 300 t. The load is 0 + 150 =
//!   150 t. Ship 1 has been back and refuelled since day 5, and is the
//!   first free ship in number order.
//!
//! At the end of day 18: base 2 holds 430 t; base 1 holds 4490 t of supply
//! and 1960 t of fuel; ship 1 is at S, empty, with 960 t of fuel, and
//! refuels the next morning; ship 2 has not moved. The plan has made two
//! runs. No staff order was refused.
//!
//! Each order the staff gives, and each day it gives none, has a reason
//! with the numbers of this table in its "worked" part. On the days it
//! gives none, the reason says which reading the next run waits for.
//!
//! ## With no orders
//!
//! The same scenario with no command in force and no order, over 12 days.
//! Base 2 pays 10 t on each of the nights of days 1 to 10 and holds 0 t
//! after the night of day 10. On the night of day 11 its 2000 men are
//! unfed.
//!
//! ## What it shows
//!
//! From day 4, when the first run is ashore, the reading at T is never
//! below 30 days, and it never passes 45: the floor and one interval. A
//! run sails every 15 days and carries 15 days of use. The interval's
//! first value rests on one row, fuel oil shipped every 15 days
//! (`land_unit_facts` 1099), and on the doctrine of the period that a
//! shipment brings the use of one interval between shipments (1057, 1065).
//! It is a starting value, to be looked at again in stage 2, when a convoy
//! can be lost.
//!
//! # Properties
//!
//! - The staff never loads the source below its reserve: with base 1's
//!   supply set to 1100 t at the start, the first run loads 100 t and the
//!   reason names the reserve as what binds.
//! - With ship 1 held, the same runs are made by ship 2.
//! - The same start gives the same orders on every run.
//! - With the intent withdrawn on day 5 the staff gives no order from then,
//!   and a force at sea completes its run and comes home.
