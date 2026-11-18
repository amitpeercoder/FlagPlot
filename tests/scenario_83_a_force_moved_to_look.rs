//! Scenario 83: a force moved to look. The commander asks for an airfield
//! 400 nm away and names a carrier force with its scouts. Nothing the
//! staff may use can see the airfield from where it lies. The staff sends
//! the carrier forward at once, to the hex she would cover the landing
//! from, looks from there, learns the airfield is empty, takes one unit
//! in place of two, and goes two days later than it would have gone
//! blind.
//!
//! Proves decision 0159, rule D, item 14, step 5 of the opening of stage
//! 4, against a hand calculation. It stands on scenario 73's for the
//! carrier force and the landing and on scenario 72's for the looking.
//! The data is placeholder data: see
//! `data/scenarios/83_a_force_moved_to_look.toml`. It is scenario 73's
//! start with air group 2 named in the forces limit, and it runs on
//! scenario 73's figures with scenario 72's ground figures in place of
//! 73's, so that a reconnaissance flight is flown and reports the true
//! count. Base 2 has no gun, no aircraft and no man, so no aircraft is
//! lost and no die changes anything asserted.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly, except what each bomb hits, how many strikes a group launches
//! in a day, and the supply and fatigue a flight costs, which are the air
//! rules'. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## The rule, as it is built
//!
//! A "take a place" has a carrier task for each carrier force its forces
//! limit names: "operate the carrier force from a hex", the first hex on
//! her way from which her bombers reach the place (decision 0148). Until
//! now it was formed when the plan was released. Now:
//!
//! 1. It is formed while the plan is preparing, on a morning when the
//!    need to know is not met, the forces limit names an air group that
//!    can search and is aboard a ship of that force, and no group the
//!    limit names reaches the place from where it is.
//! 2. Its hex is the covering hex, where a group aboard that the limit
//!    names and that can search reaches the place from it; else the first
//!    hex on her way from which one does. So the force moves once.
//! 3. Before the plan is released the task gives every order it gives
//!    today but the strike on the place's base.
//! 4. A group that is on reconnaissance of the place is given no order by
//!    the carrier task, and when the need is met the looking leaves a
//!    group aboard such a force to the carrier task and does not rest it.
//! 5. At balanced the staff does this without asking. At preserve the
//!    force it does it only with leave to launch given; without it the
//!    reason says that the force could look from that hex and waits for
//!    leave. At press nothing need be known and the task is formed at
//!    release, as before.
//! 6. The force is sent home when the take is met, as before. It is not
//!    brought back when the need is met: it stays to cover.
//!
//! This differs from the decision's first wording in two things, ruled
//! by the architect on Nov 18, 2026: the hex is the covering hex and not
//! the nearest from which a scout reaches (here (4,0) and not (3,0)), so
//! that the force is not moved twice; and it stays there.
//!
//! ## Figures and inputs
//!
//! As scenario 73's: a hex is 40 nm; the placeholder dive bomber reaches
//! 250 nm; the placeholder carrier scout searches to 300 nm; the carrier
//! makes 16 kn and the transports 10 kn; A at (0,0) and B at (10,0),
//! 400 nm apart; base 1, Allied, at A; base 2, Japanese, at B, with no
//! gun, aircraft, supply or man; task force 1, the carrier, with group 1,
//! 12 fighters, group 2, 6 scouts, and group 3, 10 dive bombers; task
//! force 2, ships 2 and 3, troop transports; units 1 and 2, 600 men each,
//! at A. No contact and no estimate is held. A reconnaissance flight
//! reports the true count. The game is 9 days.
//!
//! The Allied side's commands of day 1: intent 1, take the place at base
//! 2, with a forces limit of task forces 1 and 2, units 1 and 2, air
//! group 2 and base 1 as the source. No risk is set, so balanced. Leave
//! to prepare and leave to launch are both given.
//!
//! ## The hex
//!
//! From (0,0) the scouts are 400 nm from B, beyond their 300. The
//! covering hex is (4,0), 240 nm from B: within the bombers' 250, and
//! within the scouts' 300. It is 160 nm from A, 10 hours at 16 kn.
//!
//! ## Day by day
//!
//! | Day | Hour | The landing | The carrier and the looking | What happens |
//! | --- | --- | --- | --- | --- |
//! | 1 | 0 | Approves plan 1 to preparing. Forms intent 2, ready a landing by task force 2 at B, with a forces limit of units 1 and 2. Orders `prepare_landing` at B, roster 1 and 2 | Forms intent 3, operate task force 1 from (4,0), under intent 1. The force: rules of engagement, avoid, break-off level 0.5; a route to (4,0). Group 1: combat air patrol; a patrol share of 0.5. Group 2: the search mission; the arc 050 to 120 at 300 nm. Group 3: the naval strike mission; strike sizing on | The carrier sails at hour 0 and arrives at hour 10. Loading 100; each unit's rehearsal 100 |
//! | 2 | 24 | Notice: intent 2 is ready. The plan is not released: a group is looking | The force: battle readiness. Group 2: reconnaissance of B | In the morning one scout flies 240 nm and reports 0 men, seen from the air |
//! | 3 | 48 | Sets the forces limit of intent 2 to unit 1. Orders `prepare_landing` at B, roster 1. The landing was ordered again and is not ready this morning | Group 2: the search mission | |
//! | 4 | 72 | Approves plan 1 to released. Orders a load of supply to a weight of 45 t | Group 3: base strike at base 2 | Group 3 strikes base 2 |
//! | 5 | 96 | Unit 1 set to attack. Load unit 1 | none | Group 3 strikes base 2 |
//! | 6 | 120 | A route to B and back to A, acting at B, and an unload order | none | The transports sail at hour 120. Group 3 strikes base 2 |
//! | 7 | 144 | none | none | The transports arrive at hour 160. Unit 1's ship lands 160 men in the morning and 440 in the afternoon, and then 10 t. Group 3 strikes base 2. Base 2 changes side at the end of the day |
//! | 8 | 168 | Marks plan 1 complete. Notice: intent 1 is met | The force: a route to A | The carrier sails at hour 168 and arrives at hour 178. The last 35 t is landed in 1.75 hours |
//! | 9 | 192 | none | none | |
//!
//! - **Day 1.** With no estimate the staff takes every unit the ships can
//!   carry. The need is not met; group 2 can search and cannot reach B
//!   from A; no other group is named. The carrier task is formed that
//!   morning with the orders scenario 73 gives on its day 2. The arc is
//!   worked from (0,0), where base 2 is on 090.
//! - **Day 2.** The force is at its hex. Group 2 reaches B, 240 nm, and
//!   is set to reconnaissance. The carrier task gives it no order and
//!   gives no strike on the base: the plan is not released.
//! - **Day 3.** The count is 0, seen from the air, of day 2: the need is
//!   met, at any age at balanced. The men wanted are 0 x 2 = 0 and the
//!   staff takes one unit, unit 1. Group 2 is no longer looking and its
//!   mission is not the search: the carrier task sets it. From (4,0)
//!   base 2 is still on 090: no arc order.
//! - **Day 4.** The landing is ready and the need is met: released. The
//!   supply is 600 x 0.005 x 15 = 45 t. The plan is released, B is within
//!   the bombers' reach and no contact is held: group 3 is given the base
//!   strike.
//! - **Day 6.** 400 nm at 10 kn is 40 hours.
//! - **The end of day 7.** Base 2 is Allied, and group 3's mission is
//!   cleared to rest by the rule of a base that changes side.
//!
//! Seven orders to the landing and thirteen to the carrier and her
//! groups, twenty in all. No staff order or command is refused. Group 2
//! flies the reconnaissance once, on day 2. Beside scenario 73, which
//! goes blind with two units and is ashore on day 6, the staff knows the
//! place is empty, sends one unit, and is ashore on day 7; the carrier is
//! forward from day 1 and not from day 2.
//!
//! # Properties
//!
//! - **The scouts not named.** With air group 2 left out of the forces
//!   limit: the game of scenario 73. No carrier task is formed on day 1;
//!   the plan is released on day 2 on no count, and the carrier task is
//!   formed that morning. Days 1 and 2 are asserted.
//! - **At press.** With the risk press: no carrier task on day 1 and no
//!   group set to reconnaissance on any day; the plan is released on day
//!   2 and the carrier task is formed that morning. Days 1 and 2 are
//!   asserted.
//! - **At preserve the force, with leave.** With the risk preserve: on
//!   day 1 intent 3, operate task force 1 from (4,0), is formed under
//!   intent 1 and the force is given a route to (4,0). Only that is
//!   asserted of day 1; the other orders are those of a carrier task at
//!   preserve the force.
//! - **At preserve the force, without leave.** The same with leave to
//!   launch withheld: no carrier task is formed on days 1 and 2, the
//!   force is given no order, and the reason of each morning says that
//!   task force 1 could look from (4,0) and that the staff waits for
//!   leave to launch.
//! - **Already in reach.** With task force 1 at sea, stopped, at (4,0)
//!   from the start: on day 1 group 2 is set to reconnaissance of B and
//!   no carrier task is formed that morning. Day 1 is asserted.
//! - **The enemy sees nothing of the flight.**
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see and has not been told give the same
//!   orders and reasons; a save on day 3 and a resume give the same game;
//!   scenarios 72 and 73 give the orders they gave.
