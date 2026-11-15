//! Scenario 41: a plan made ready that waits for leave to launch, a hold, a
//! changed intent, a week, a release (41a); an order of the commander's
//! that overrides the staff (41b); and a plan cancelled with its ship at
//! sea (41c).
//!
//! Proves step 3 of decision 0107 (step 3, stage 1: holds and release,
//! overrides, previews and amendments, the two leaves, cancelling; rulings
//! 17 to 30 there) against a hand calculation. The data is placeholder
//! data: see `data/scenarios/41a_hold_change_release.toml`,
//! `41b_an_override.toml` and `41c_cancelled_at_sea.toml`. It runs on the
//! frozen step 1 figures. There is no enemy, no aircraft and no random
//! draw.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Figures and the rules the staff works by
//!
//! Those of scenario 36: the placeholder cargo ship (3000 t of cargo, 10
//! kn at cruise, 0.1 t of fuel a mile, 1000 t of fuel), a berth at 50 t an
//! hour at S and lighters at 20 t an hour at T, 10 t a day for the 2000 men
//! at T, a day of a night pulse of 12 hours and two daylight pulses of 6,
//! and a force that steams on at the start of the first pulse after its
//! work is done. And:
//!
//! - **A plan that is preparing forms and loads its first run and does not
//!   sail it.** It sails on the first day the plan is released.
//! - **A held ship is not free,** and no ship of a force that has a held
//!   ship in it, or is itself held, is free. The staff gives no order to
//!   what is held.
//! - **A run not yet sailed whose ship is held is ended.** What is aboard
//!   stays aboard.
//! - **An order of the commander's to a force in an intent's forces limit
//!   holds that force,** before the staff is asked that morning.
//! - **A released ship is newly free.** The staff takes the free ships in
//!   number order, as it always does.
//!
//! # 41a: ready and not sailed, a hold, a changed intent, a release
//!
//! ## Inputs
//!
//! Scenario 36's map, bases, land unit and ships: S at (0,0) with base 1
//! (5000 t of supply, 2000 t of fuel), T at (5,0) with base 2 (100 t),
//! ships 1 and 2 in port at base 1 in no force. The game is 27 days.
//!
//! The Allied side's commands:
//!
//! - Day 1: intent 1, keep base 2 at a floor of 30 days, interval 15 days;
//!   its forces limit, ships 1 and 2 with base 1 as the source; a reserve
//!   at base 1 of 1000 t of supply. No leave is set: by default leave to
//!   prepare is given and leave to launch is not.
//! - Day 3: approve the plan (it is preparing: this releases it).
//! - Day 8: hold ship 1.
//! - Day 10: intent 1 changed to a floor of 40 days, interval 15 days.
//! - Day 17: release ship 1.
//!
//! ## Day by day
//!
//! | Day | Read at T | The staff | What happens |
//! | --- | --------- | --------- | ------------ |
//! | 1   | 100 t, 10 days | Makes the draft ready (preparing). Forms a force of ship 1; load 360 t | Loaded in 7.2 hours. Base 1 holds 4640 t. T pays 10 t, as on every night |
//! | 2   | 90 t      | No order: run 1 is loaded and waits for leave to launch | |
//! | 3   | 80 t      | The plan is released. Run 1 is timed again: sails day 3, due day 3, free day 6. Route to T and back, acting at T, with an unload order | Sails at hour 48, arrives at hour 68, 80 t landed |
//! | 4   | 150 t     | none      | The last 280 t take 14 hours: unloaded at hour 86. Steams for S at hour 90 |
//! | 5   | 420 t     | none      | Arrives at S at hour 110 |
//! | 6   | 410 t     | none      | Ship 1 refuels 40 t: 1960 t of fuel at base 1. Ship 1 is free |
//! | 7   | 400 t     | none      | |
//! | 8   | 390 t     | none      | Ship 1 is held |
//! | 9   | 380 t     | none      | |
//! | 10  | 370 t, 37 days | The floor is 40 days: amendment 1. Forms a force of ship 2; load 190 t | Loaded in 3.8 hours. Base 1 holds 4450 t |
//! | 11  | 360 t     | Route to T and back, acting at T, with an unload order | Sails at hour 240, arrives at hour 260, 80 t landed |
//! | 12  | 430 t     | none      | The last 110 t take 5.5 hours: unloaded at hour 269.5. Steams for S at hour 276 |
//! | 13  | 530 t     | none      | Arrives at S at hour 296 |
//! | 14  | 520 t     | none      | Ship 2 refuels 40 t: 1920 t of fuel at base 1 |
//! | 15 to 24 | 510 t falling to 420 t | none | Ship 1 is released on day 17 |
//! | 25  | 410 t     | Load 150 t on ship 1's force | Loaded in 3 hours. Base 1 holds 4300 t |
//! | 26  | 400 t     | Route to T and back, acting at T, with an unload order | Sails at hour 600, arrives at hour 620, 80 t landed |
//! | 27  | 470 t     | none      | The last 70 t take 3.5 hours: unloaded at hour 627.5. Steams for S at hour 636 and is at sea at the end of the day |
//!
//! Why, day by day:
//!
//! - **Day 1.** As scenario 36: the load is 300 - 90 + 150 = 360 t. Leave
//!   to prepare is given, so the staff makes the draft ready and loads its
//!   first run. The run's days as planned are load 1, sail 2, due 2, free 5.
//! - **Day 2.** The load is complete and the plan is not released. The
//!   staff gives no order; its reason says the run is loaded and waits for
//!   leave to launch, and that leave given, or the commander's approval,
//!   would change it.
//! - **Day 3.** The commander's approval releases the plan. The load is not
//!   sized again. The run sails a day later than planned, so the reading of
//!   day 5 is 420 t and not 430.
//! - **Day 8.** With ship 1 held the staff still needs no run: the reading
//!   of day 10 is 370 t. Its reason says the next run waits for the reading
//!   of day 18 (290 t) and is expected to be ordered on day 16.
//! - **Day 10.** The floor is now a stock of 400 t. A run ordered today
//!   arrives on day 11, and the reading of day 12 without it is 350 t,
//!   below 400: a run is needed. The stock projected for day 11 is 360 t:
//!   the load is 400 - 360 + 150 = 190 t. Ship 1 is held, so the run is
//!   ship 2's. The reason says it serves the plan's amendment 1, that ship
//!   1 is held, and that without the hold ship 1 would have been sent.
//! - **Day 17.** Ship 1 is released and is free. No run is needed: the
//!   readings from day 13 are 530 t less 10 t a day, 400 t on day 26 and
//!   390 t on day 27, so the next run is expected to be ordered on day 25.
//! - **Day 25.** A run ordered today arrives on day 26; the reading of day
//!   27 without it is 390 t: needed. The stock projected for day 26 is 400
//!   t: the load is 0 + 150 = 150 t. Ship 1 is the first free ship in
//!   number order, and lies in the force formed on day 1.
//!
//! At the end of day 27: base 2 holds 530 t (100 + 360 + 190 + 150 - 270).
//! Base 1 holds 4300 t of supply and 1920 t of fuel. Ship 2 is stopped at
//! base 1, empty, with 1000 t of fuel. Ship 1 is at sea, empty, 80 nm from
//! S, with 968 t of fuel. The plan has made three runs and has one
//! amendment. No staff order was refused.
//!
//! ## The preview
//!
//! Asked before day 10 is ended, with ship 1 held: a preview of intent 1
//! at a floor of 40 days says that as things stand the next run is 150 t,
//! expected to be ordered on day 16, and that with the change a run of 190
//! t is ordered on day 10 by ship 2; that ship 1 is held; that nothing made
//! ready is lost; and that no other plan gives anything up. It changes
//! nothing: the command state, the queue and the hash are as they were.
//! Approving the preview queues the changed intent for day 10, and the day
//! then runs as the table has it.
//!
//! If the commander queues a release of ship 1 after the preview, the
//! preview is void: approving it is refused and says why. Asked again, the
//! preview gives the run of 190 t to ship 1.
//!
//! ## Properties of 41a
//!
//! - **With both leaves given on day 1** and none of the later commands the
//!   game is scenario 36's, day for day.
//! - **With no approval on day 3** the run never sails: base 2 holds 0 t
//!   after the night of day 10 and its men are unfed on the night of day
//!   11, and on each day from day 2 the staff's reason says the run waits
//!   for leave to launch.
//! - **A run set aside.** With ship 1 held on day 2 in place of day 8, and
//!   released on day 8: on day 2 the staff ends run 1, with 360 t still
//!   aboard ship 1, and, the plan still preparing, loads ship 2 in its
//!   place: a run ordered on day 2 arrives on day 3, the stock projected
//!   for day 3 is 80 t, and the load is 300 - 80 + 150 = 370 t. It sails on
//!   day 3, when the plan is released, and the readings at T are those of
//!   41b. Run 1 is not taken up again when ship 1 is released. The next run
//!   is needed on day 17 (the reading of day 19 is 290 t) and is ship 1's:
//!   its load is 150 t by the sum, and it carries the 360 t it has aboard,
//!   with no load order; the reason says so. It sails on day 18.
//! - **A held force.** With the force formed on day 1 held on day 8 in
//!   place of ship 1, the days are the same.
//! - **A held ship in a force of two.** With ships 1 and 2 in one force
//!   from the start and ship 1 held on day 8, the staff has no free ship on
//!   day 10 and says the hold is what binds; a third ship in the pool,
//!   lying in no force, is sent.
//! - **A held setting.** A hold on the search arc of an air group is
//!   recorded, shown by `command_state`, and changes no order the supply
//!   staff gives.
//! - **The guard.** A staff that hands in an order to a held ship's force
//!   has it refused, with the hold as the reason.
//! - The staff never gives an order to what is held, on any day of any of
//!   these games.
//!
//! # 41b: an override
//!
//! ## Inputs
//!
//! Scenario 36 whole, both leaves given. The game is 5 days. On day 2 the
//! commander orders the force formed on day 1 to unload.
//!
//! ## Day by day
//!
//! - **Day 1.** As scenario 36: a force of ship 1 is formed and loads 360
//!   t. Base 1 holds 4640 t.
//! - **Day 2.** The commander's order is to a force in intent 1's forces
//!   limit: the force is held from that morning, as an override, before the
//!   staff is asked. The staff ends run 1 and frees ship 1's commitment. It
//!   needs a run: ordered today it arrives on day 3, the reading of day 4
//!   without it is 70 t, and the stock projected for day 3 is 80 t. The
//!   load is 300 - 80 + 150 = 370 t, on ship 2, in a new force. Its reason
//!   says ship 1's force is held by the commander's order.
//! - In the night of day 2 the first force has the berth and puts its 360
//!   t ashore in 7.2 hours; the second loads 240 t by lighter. In the
//!   morning the second has the berth and loads the other 130 t in 2.6
//!   hours. Base 1 holds 4630 t.
//! - **Day 3.** Ship 2's force sails at hour 48 and arrives at hour 68: 80
//!   t landed.
//! - **Day 4.** The last 290 t take 14.5 hours: unloaded at hour 86.5. It
//!   steams for S at hour 90.
//! - **Day 5.** It arrives at S at hour 110.
//!
//! Readings at T: 100, 90, 80, 150 and 430 t on days 1 to 5. At the end of
//! day 5: base 2 holds 420 t; base 1 holds 4630 t of supply and 2000 t of
//! fuel; ship 1 is at base 1, empty, with 1000 t of fuel, its force held;
//! ship 2 is at base 1, empty, with 960 t of fuel.
//!
//! ## Property of 41b
//!
//! - The override is recorded as an event that cites the commander's
//!   order, and `command_state` shows the hold and that an order made it.
//!
//! # 41c: cancelled with a ship at sea
//!
//! ## Inputs
//!
//! Scenario 36 with T at (9,0): 360 nm from S, 36 hours at cruise. Both
//! leaves given. The game is 5 days. On day 3 the commander cancels the
//! plan.
//!
//! ## Day by day
//!
//! - **Day 1.** The passage out is 2 days, so a run ordered today sails on
//!   day 2 and arrives on day 3. Without it the reading of day 4 is 70 t:
//!   needed. The stock projected for day 3 is 80 t: the load is 300 - 80 +
//!   150 = 370 t, on ship 1. Loaded in 7.4 hours. Base 1 holds 4630 t.
//! - **Day 2.** The force sails at hour 24.
//! - **Day 3.** At hour 48 it is 240 nm out, in hex (6,0). The plan is
//!   cancelled. The staff frees ship 1's commitment and gives the force a
//!   route home, to S, with a reason that names the cancelling. The force
//!   turns and is back at S at hour 72, the end of the day, with 370 t
//!   aboard and 952 t of fuel.
//! - **Day 4.** Ship 1 refuels 48 t: base 1 holds 1952 t of fuel. Its
//!   unload order stands, and it puts the 370 t ashore at S in 7.4 hours:
//!   base 1 holds 5000 t. The staff gives no order; its reason says the
//!   plan is cancelled.
//! - **Day 5.** No order.
//!
//! At the end of day 5: base 1 holds 5000 t of supply and 1952 t of fuel;
//! base 2 holds 50 t; ship 1 is stopped at base 1, empty, with 1000 t of
//! fuel; the plan is cancelled and intent 1 is still in force with no plan
//! that can serve it.
//!
//! ## Property of 41c
//!
//! - Cancelled on day 4 in place of day 3, when the force is at T and has
//!   begun to unload (it arrived at hour 60 and landed 240 t in the 12
//!   hours of daylight left), the force is given no new order: it lands the
//!   other 130 t and comes home by the route it has.
