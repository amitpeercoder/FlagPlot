//! Scenario 40: two supply intents and one pool. In 40a one ship serves two
//! bases: the higher intent has it, the lower's shortfall is stated with the
//! commitment that caused it, and a changed priority moves the ship when it
//! next comes free. In 40b two plans load from one stock on the same
//! morning and do not both count the same ton.
//!
//! Proves step 2 of decision 0107 (step 3, stage 1: priority, commitments
//! of stock, a second intent, displaced demand reported; rulings 10 to 16
//! there) against a hand calculation. The data is placeholder data: see
//! `data/scenarios/40a_one_pool_two_hexes.toml` and
//! `data/scenarios/40b_one_stock_two_plans.toml`. It runs on the frozen
//! step 1 figures. There is no enemy, no aircraft and no random draw.
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
//! hour, and lighters at 20 t an hour; a size 0 port works by lighter at 20
//! t an hour. The placeholder fast transport with cargo (class 12): 30 t of
//! cargo, 16 kn at cruise, 500 t of fuel over 4000 nm, which is 0.125 t a
//! mile. A day is 24 hours: a night pulse of 12 and two daylight pulses of
//! 6. Day d begins at hour 24 x (d - 1). In a pulse ships move before cargo
//! is worked. The men's supply is paid at the start of the night pulse.
//! Ships in port refuel at the start of a day. A force that acts at a place
//! and steams on does so at the start of the first pulse after its work is
//! done (decision 0096).
//!
//! ## The rules the staff works by
//!
//! Those of scenario 36, and:
//!
//! - **Intents are served in priority order,** rank 1 first; at equal rank
//!   the lower intent number.
//! - **What the staff gives a higher intent in the morning is committed
//!   when it turns to the next:** the ships of a run it has just added, and
//!   the tons that run has still to load.
//! - **A free ship goes to the first intent, in that order, that needs a
//!   run today.** The staff does not keep a free ship idle for an intent
//!   that will need one later, and does not recall a run under way.
//! - **A ship is free** on the day after its commitment ends. A run's
//!   commitment is counted in whole days: the day it loads, the day it
//!   sails and arrives, a day to unload and a day back, so a ship ordered
//!   on day d is free on day d + 4 here.
//! - **An intent that needs a run and cannot have one names what displaced
//!   it:** its shortfall in days at the reading it checked, and the
//!   commitment that holds the ship or the tons (the plan, the run and the
//!   days).
//!
//! # 40a: one ship, two bases
//!
//! ## Inputs
//!
//! A placeholder map: S at (0,0), a coast hex with port size 1; T at (5,0)
//! and U at (0,5), coast hexes with port size 0. S to T and S to U are each
//! 5 hexes on one axis: 200 nm, 12.5 hours at cruise, 25 t of fuel. The
//! game is 16 days. Everything is Allied.
//!
//! | Base | Hex | Supply | Fuel   |
//! | ---- | --- | ------ | ------ |
//! | 1    | S   | 1000 t | 1000 t |
//! | 2    | T   | 10 t   | 0      |
//! | 3    | U   | 10 t   | 0      |
//!
//! Land unit 1, 200 men, at T on defend, and land unit 2, 200 men, at U on
//! defend. Each costs 1 t a day.
//!
//! Ship 1, a placeholder fast transport with cargo, lies in port at base 1
//! in no task force, full of fuel.
//!
//! The Allied side's commands:
//!
//! - Day 1. Intent 1: keep base 2 at a floor of 30 days, interval 15 days.
//!   Intent 2: keep base 3 at a floor of 30 days, interval 15 days. The
//!   floor is a stock of 30 t at each, and an interval's use is 15 t. The
//!   forces limit of each intent: ship 1, with base 1 as the source.
//!   Intent 1 has priority 1 and intent 2 priority 2. Leave to prepare and
//!   leave to launch are both given. No reserve is set.
//! - Day 3. Intent 2 is given priority 1, then intent 1 priority 2.
//!
//! ## A run, hour by hour
//!
//! A run ordered on day d: the load is aboard within the first hour of the
//! night of day d (30 t at 50 t an hour is 0.6 hours). On day d + 1 the
//! force sails at hour 24 x d, steams the 12 hours of the night and arrives
//! half an hour into the morning. It lands its load by lighter in the
//! morning (30 t in 1.5 hours), steams for S at the start of the afternoon,
//! and arrives at S 6.5 hours into the night of day d + 2. Ship 1 has 450 t
//! of fuel then. It lies there on day d + 3, when it refuels 50 t at the
//! start of the day, and is free on day d + 4.
//!
//! ## Day by day
//!
//! Readings are at the start of the day, before its orders.
//!
//! | Day | Read at T | Read at U | The staff's orders | What happens |
//! | --- | --------- | --------- | ------------------ | ------------ |
//! | 1   | 10 t, 10 days | 10 t, 10 days | Intent 1: form a force of ship 1; load 30 t. Intent 2: none, displaced | Base 1 holds 970 t. T and U pay 1 t each, as on every night |
//! | 2   | 9 t       | 9 t       | Intent 1: route to T and back, acting at T, with an unload order. Intent 2: none, displaced | Sails at hour 24, arrives at hour 36.5, 30 t landed by hour 38, steams for S at hour 42 |
//! | 3   | 38 t      | 8 t       | none; intent 2 displaced | The priorities are changed. The force arrives at S at hour 54.5 |
//! | 4   | 37 t      | 7 t       | none; intent 2 displaced | Ship 1 refuels 50 t: base 1 holds 950 t of fuel |
//! | 5   | 36 t      | 6 t       | Intent 2: load 30 t. Intent 1: none, no run needed | Base 1 holds 940 t |
//! | 6   | 35 t      | 5 t       | Intent 2: route to U and back, acting at U, with an unload order | Sails at hour 120, arrives at hour 132.5, 30 t landed by hour 134, steams for S at hour 138 |
//! | 7   | 34 t      | 34 t      | none               | The force arrives at S at hour 150.5 |
//! | 8   | 33 t      | 33 t      | none               | Ship 1 refuels 50 t: 900 t of fuel at base 1 |
//! | 9   | 32 t      | 32 t      | none: ship 1 is free, and neither intent needs a run | |
//! | 10  | 31 t      | 31 t      | Intent 2: load 15 t. Intent 1: none, displaced | Base 1 holds 925 t |
//! | 11  | 30 t      | 30 t      | Intent 2: route to U and back, acting at U, with an unload order. Intent 1: none, displaced | Sails at hour 240, arrives at hour 252.5, 15 t landed by hour 253.25, steams for S at hour 258 |
//! | 12  | 29 t, 29 days | 44 t  | none; intent 1 displaced | The force arrives at S at hour 270.5 |
//! | 13  | 28 t      | 43 t      | none; intent 1 displaced | Ship 1 refuels 50 t: 850 t of fuel at base 1 |
//! | 14  | 27 t      | 42 t      | Intent 1: load 19 t. Intent 2: none, no run needed | Base 1 holds 906 t |
//! | 15  | 26 t, 26 days | 41 t  | Intent 1: route to T and back, acting at T, with an unload order | Sails at hour 336, arrives at hour 348.5, 19 t landed by hour 349.45, steams for S at hour 354 |
//! | 16  | 44 t      | 40 t      | none               | The force arrives at S at hour 366.5 |
//!
//! Why each run, its size, and what is displaced:
//!
//! - **Day 1.** Intent 1 is first. A run ordered today arrives on day 2;
//!   without it the reading of day 3 at T is 8 t, below 30: a run is
//!   needed. The stock projected for the reading of day 2 is 9 t. The load
//!   is 30 - 9 + 15 = 36 t, cut to the ship's room of 30 t; the reason
//!   names the room as what cut it. Intent 2 needs a run by the same sum,
//!   and its one ship has just been given to intent 1: no ship is free
//!   until day 5. Its reason gives the shortfall, 30 - 8 = 22 days at the
//!   reading of day 3, and what displaced it: ship 1 committed to run 1 of
//!   intent 1's plan from day 1 to day 4.
//! - **Days 2 to 4.** Intent 2 is displaced on each day by the same
//!   commitment. On day d it checks the reading of day d + 2, which is 9 -
//!   d tons, and its shortfall is 21 + d days: 23, 24 and 25. The change
//!   of priority on day 3 recalls nothing: the run is under way.
//! - **Day 5.** Ship 1 is free. Intent 2 is now first. A run ordered today
//!   arrives on day 6; the reading of day 7 without it is 4 t: needed. The
//!   stock projected for day 6 is 5 t: 30 - 5 + 15 = 40 t, cut to 30 t by
//!   the room. Intent 1 needs none: the reading of day 7 at T is 34 t.
//! - **Day 9.** Ship 1 is free. A run ordered today would arrive on day
//!   10, and the reading of day 11 is 30 t at both bases, not below the
//!   floor: no run for either.
//! - **Day 10.** Both need a run: the reading of day 12 is 29 t at both.
//!   Intent 2 is first and has ship 1. The stock projected for day 11 is
//!   30 t: the load is 0 + 15 = 15 t. Intent 1 is displaced: no ship free
//!   until day 14, a shortfall of 1 day at the reading of day 12, by ship 1
//!   committed to run 2 of intent 2's plan from day 10 to day 13.
//! - **Days 11 to 13.** Intent 1 is displaced by the same commitment, with
//!   a shortfall of 2, 3 and 4 days, at the readings of days 13, 14 and
//!   15.
//! - **Day 14.** Ship 1 is free. Intent 2 needs none (the reading of day
//!   16 at U is 40 t). Intent 1: a run ordered today arrives on day 15, the
//!   reading of day 16 without it is 25 t: needed. The stock projected for
//!   day 15 is 26 t: the load is 30 - 26 + 15 = 19 t.
//!
//! At the end of day 16: base 2 holds 43 t (10 + 30 + 19 - 16) and base 3
//! holds 39 t (10 + 30 + 15 - 16). Base 1 holds 906 t of supply and 850 t
//! of fuel. Ship 1 is stopped at base 1, empty, with 450 t of fuel. Intent
//! 1's plan has made two runs, of 30 t and 19 t, ordered on days 1 and 14;
//! intent 2's plan two, of 30 t and 15 t, ordered on days 5 and 10. No
//! staff order was refused. The reading at T was below its floor on days 12
//! to 15, lowest on day 15 at 26 days; the reading at U was never below its
//! floor after day 6.
//!
//! The proposal for intent 1 asked for on day 10, before the day is ended,
//! says what it is displaced by: intent 2's run, ship 1, days 10 to 13.
//!
//! ## Properties of 40a
//!
//! - **Without the commands of day 3** the days to 9 are the same, and from
//!   day 10 T and U change places: intent 1 has ship 1 on day 10 with 15 t,
//!   intent 2 is displaced on days 10 to 13 and loads 19 t on day 14. At
//!   the end base 2 holds 39 t and base 3 holds 43 t.
//! - **With ship 1 in intent 1's protected limit** intent 2 is never given
//!   it, on any day, free or not: each day's reason for intent 2 says that
//!   no ship of its pool may be taken. Base 3 holds 0 t after the night of
//!   day 10 and its 200 men are unfed on the night of day 11.
//! - **At equal priority** (both intents with priority 1, or neither with
//!   any) the days are those of the first property: the lower intent
//!   number is first.
//! - The same start gives the same orders on every run, and a check of the
//!   saved game against the staff passes.
//! - Two worlds that differ only in what the Allied side cannot see give
//!   the same orders, commands, reasons and proposals.
//!
//! # 40b: one stock, two plans
//!
//! ## Inputs
//!
//! The map, bases 2 and 3 and the land units of 40a. Base 1 holds 1040 t of
//! supply and 1000 t of fuel. Ships 1 and 2, placeholder fast transports
//! with cargo, lie in port at base 1 in no task force, full of fuel. The
//! game is 5 days.
//!
//! Commands of day 1: intents 1 and 2 as in 40a, with priorities 1 and 2;
//! the forces limit of each is ships 1 and 2 with base 1 as the source; a
//! reserve at base 1 of 1000 t of supply; both leaves given.
//!
//! ## Day by day
//!
//! - **Day 1.** Base 1 has 40 t above its reserve. Intent 1 is first: its
//!   load is 36 t, cut to the room of ship 1, the first free ship: 30 t.
//!   The staff forms a force of ship 1. Intent 2: ship 2 is free. Its load
//!   is 36 t; the source has 40 t above its reserve, of which 30 t are
//!   still to be taken by intent 1's run: 10 t. It forms a force of ship 2
//!   and orders a load of 10 t. Its reason names the source as what cut the
//!   load, with the three numbers: 1040 t of stock, the reserve of 1000 t,
//!   and 30 t still to be taken by a run loading. The two forces take the
//!   side's next two force numbers, ship 1's the lower.
//! - In the night ship 1's force has the berth and loads 30 t in 0.6
//!   hours; ship 2's force loads 10 t by lighter in half an hour. Base 1
//!   holds 1000 t, its reserve, and never less on any day.
//! - **Day 2.** Both forces are routed, sail at hour 24 and arrive at hour
//!   36.5. T has 30 t landed by hour 38 and U 10 t by hour 37. Both steam
//!   for S at hour 42 and arrive at hour 54.5.
//! - **Readings.** At T: 10, 9, 38, 37, 36 t on days 1 to 5. At U: 10, 9,
//!   18, 17, 16 t.
//! - **Days 3 and 4.** Intent 2 needs a run (the reading it checks, of day
//!   5 and of day 6, is 16 t and 15 t) and has no ship free until day 5:
//!   its own run holds ship 2 and intent 1's holds ship 1. Its reason
//!   names both commitments.
//! - **Day 5.** Both ships are free. Intent 1 needs no run. Intent 2 needs
//!   one, the reading of day 7 being 14 t, a shortfall of 16 days; the
//!   source has nothing above its reserve, and its reason says so: 1000 t
//!   of stock, a reserve of 1000 t, nothing still to be taken. It gives no
//!   order.
//!
//! At the end of day 5: base 1 holds 1000 t of supply and 900 t of fuel
//! (each ship refuelled 50 t on day 4); base 2 holds 35 t and base 3 holds
//! 15 t; both forces are stopped at base 1, empty.
//!
//! ## Property of 40b
//!
//! - With no reserve at base 1, intent 2's first load is 30 t, and with
//!   the reserve at 1010 t it is 0: intent 2 gives no order on day 1 and
//!   names the source.
