//! Scenario 56: an escorted run. Scenario 36's convoy, with an escort
//! force named in the forces limit and a contact near the destination on
//! the first days: the staff escorts the first run and not the second.
//!
//! Proves decision 0126 against a hand calculation, with decision 0121's
//! rule beneath it. It stands on scenario 36's hand calculation for every
//! reading, load, hour and stock of the supply plan, and repeats none of
//! it. The data is placeholder data: see
//! `data/scenarios/56_an_escorted_run.toml`. It runs on the frozen step 1
//! figures. There is no enemy unit and no random draw.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything stated is
//! asserted exactly. If the code disagrees, the code or the rule is
//! wrong: do not change a number here to make a test pass. A mismatch
//! goes back to the architect.
//!
//! ## Inputs
//!
//! Scenario 36's, unchanged: S at (0,0) and T at (5,0), bases 1 and 2,
//! land unit 1, ships 1 and 2 in port at base 1, the intent to keep base 2
//! at 30 days with an interval of 15, the reserve, both leaves, 18 days.
//!
//! Added: task force 1, Allied, two placeholder destroyers, ships 3 and 4,
//! stopped at base 1, full of fuel, on the escort mission, with no charge.
//! The forces limit of the intent names it beside ships 1 and 2 and the
//! source.
//!
//! Added: the Allied side starts holding one contact: 2 ships, a
//! destroyer, at (8,0), age 0. It is 3 hexes from T. A contact gains a
//! pulse of age at the end of each pulse and is dropped at 5: it is held,
//! at age 3, on the morning of day 2, and is gone from the end of the
//! morning of day 2.
//!
//! No risk is set: it is balanced. A run is escorted on a threat, where an
//! escort is free.
//!
//! ## The first run
//!
//! The supply plan is scenario 36's: on day 1 the staff forms a force of
//! ship 1 and loads 360 t; on day 2 it routes it to T and back.
//!
//! On the morning of day 2 the contact is held, 3 hexes from T: a threat.
//! Force 1 is free: it is stopped at base 1, has no charge and is full.
//! So day 2 has one more order than in scenario 36: force 1 is given the
//! run's force as its charge.
//!
//! Force 1 keeps station (decision 0121, 3). It is in the run's hex at the
//! end of every pulse from day 2 to day 4:
//!
//! | When | The run's force | Force 1 |
//! | ---- | --------------- | ------- |
//! | Hour 24, the start of day 2 | Sails from S | Sails with it |
//! | Hour 44 | Arrives at T and unloads | Arrives and stops with it |
//! | Hour 66, the start of the afternoon of day 3 | Steams for S | Steams with it |
//! | Hour 86, in the afternoon of day 4 | Arrives at S | Arrives at S; its ships are filled in that pulse |
//!
//! On the morning of day 5 the run's force is back at the source: force 1
//! has no charge from that morning, by the staff's order to clear it or,
//! if the run's force has ceased to be, by decision 0121's rule 7. The
//! test asserts that it has none, and that the staff gives it no other
//! order.
//!
//! ## The second run
//!
//! On day 16 the staff loads 150 t and on day 17 routes the run, as in
//! scenario 36. On the morning of day 17 the side holds no contact: no
//! threat. Force 1 is given no order, and the reason of the routing says
//! no contact of ships is held within 15 hexes of T. The run sails alone.
//! Force 1 does not move after day 4.
//!
//! ## At the end
//!
//! Every reading at T, every load, every stock and ship 1's fuel are
//! scenario 36's. Force 1 is stopped at base 1 with no charge and full of
//! fuel. Base 1's fuel is scenario 36's less what force 1 took on day 4,
//! which is what a force of the same two destroyers ordered to T and back
//! at 10 kn burns: the test runs that force beside it and compares. No
//! staff order is refused.
//!
//! # Properties
//!
//! - **Press.** No run is escorted. Force 1 is given no order on any day.
//! - **Preserve the force.** Both runs are escorted: force 1 is given the
//!   run's force as its charge on day 2 and again on day 17.
//! - **Preserve, a threat and no escort.** With force 1 not in the forces
//!   limit: on day 2 the run is not routed, the staff hands in that it
//!   cannot meet the intent, and its reason gives the contact. On day 3
//!   the contact is gone and the run is routed, a day late.
//! - **Balanced, a threat and no escort.** With force 1 not in the forces
//!   limit: the run sails alone on day 2 as in scenario 36, and its reason
//!   says a threat was believed and no escort was free.
//! - **An escort short of fuel is not free.** With ships 3 and 4 holding
//!   10 t each: the run sails alone on day 2, and the reason gives the
//!   fuel aboard against the fuel needed.
//! - **A held escort is left alone.**
//! - **A covering force.** With task force 2, two placeholder cruisers on
//!   the surface combat mission stopped at base 1, also named: a stand
//!   task is formed on day 1 with (5,0) as its station. T is 200 nm from
//!   S: 12.5 hours at cruise, more than the night, and 6.25 hours at 32
//!   kn. On days 1 and 2, with the contact held 3 hexes from the station,
//!   force 2 is ordered to full speed and routed to (5,0) and back,
//!   standing at (5,0); from day 3 it stays in port.
//! - **None named, the game as it was.** With no task force in the forces
//!   limit, every order and event is scenario 36's.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see give the same orders and reasons.
