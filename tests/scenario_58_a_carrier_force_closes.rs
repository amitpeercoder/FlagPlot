//! Scenario 58: a carrier force closes the range. A carrier at her hex
//! holds a contact of a carrier beyond her bombers' reach. At press she
//! steams at full speed in the night to the nearest hex from which they
//! reach it, strikes in the morning, and goes back when it is gone.
//!
//! Proves decision 0130 against a hand calculation. It stands on scenario
//! 55's for the orders to the groups. The data is placeholder data: see
//! `data/scenarios/58_a_carrier_force_closes.toml`. It runs on the frozen
//! step 1 figures with scenario 53's table of aircraft for each ship. The
//! upkeep, wear, fatigue and supply figures are not loaded. There is no
//! enemy unit: the strike finds nothing and no die is rolled.
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
//! A hex is 40 nm. A day is a night pulse of 12 hours and two daylight
//! pulses of 6. The placeholder carrier (class 4): 16 kn at cruise, 30 kn
//! at most. The placeholder dive bomber (type 3): a strike radius of 250
//! nm. The placeholder carrier scout (type 2) searches to 300 nm.
//! `strike_contact_max_age_pulses` 0; a contact gains a pulse of age at
//! the end of each pulse. From a hex, the hexes on its own row to the east
//! lie on 090. A carrier has no figure in the table of aircraft for each
//! ship: she needs everything.
//!
//! ## Inputs
//!
//! A placeholder map of open sea, with S at (0,5) a coast hex with a port:
//! Allied base 1, with 5000 t of fuel. The game is 3 days. There is no
//! Japanese unit and no Japanese base.
//!
//! Task force 1, Allied: one placeholder carrier, ship 1, stopped at sea
//! at (0,0), full of fuel, on the carrier mission, at normal readiness,
//! at cruising speed, with the stance engage and base 1 as its home port.
//! Aboard her, scenario 55's groups: group 1, 12 fighters; group 2, 6
//! carrier scouts; group 3, 10 dive bombers; all resting.
//!
//! The Allied side starts holding one contact: 1 ship, a carrier, at
//! (12,0), age 0. It is 480 nm from the force.
//!
//! The Allied side's commands of day 1: intent 1, operate task force 1
//! from (0,0); its risk, press; leave to prepare and leave to launch both
//! given.
//!
//! ## Day 1
//!
//! The contact is fresh, a carrier, and 480 nm away against a reach of
//! 250. A night at full speed is 360 nm: 480 is within 360 and 250. She
//! closes. Along the row, (5,0) is 280 nm from the contact and (6,0) is
//! 240: the first hex within reach is (6,0), 240 nm from her. At cruise
//! that is 15 hours, more than the night; at 30 kn it is 8.
//!
//! The staff's orders, all serving intent 1 and decided at the level of
//! task force 1, except the sizing order, whose reason says the group
//! decided:
//!
//! - **The force,** four: battle readiness; rules of engagement with the
//!   stance avoid and a break-off level of 0.75; full speed; a route to
//!   (6,0).
//! - **Group 1,** two: combat air patrol; a patrol share of 0.25.
//! - **Group 2,** two: the search mission; the arc. The one threat is the
//!   contact, on 090 from (0,0): the narrowest arc is 080 to 090, widened
//!   to 050 to 120, at 300 nm.
//! - **Group 3,** four, as scenario 55 at press: the naval strike mission;
//!   the oldest contact, 2; extended reach; strike sizing on.
//!
//! Twelve orders.
//!
//! She sails at the start of the night and reaches (6,0) at hour 8, where
//! she stops. In the morning the contact is 1 pulse old and 240 nm away:
//! group 3 launches its 10 at it. The raid finds nothing and the contact
//! is dropped. Nothing is launched in the afternoon.
//!
//! ## Day 2
//!
//! The side holds no contact: there is nothing in reach and nothing to
//! close on, and she goes back to her hex. (0,0) is 240 nm: full speed again, which she has. Two
//! orders: the force's route to (0,0); and group 2's arc, the whole
//! circle, there being no threat. She arrives at hour 32 and stops.
//!
//! ## Day 3
//!
//! She is at her hex. No order.
//!
//! ## At the end
//!
//! One strike of 10. Fourteen staff orders: 12, 2 and 0. The force is at
//! (0,0), stopped, having steamed 480 nm at 30 kn. Her fuel is not
//! asserted as a number: the test asserts that she burnt what a force of
//! the same ship ordered over the same two passages at full speed burns.
//!
//! # Properties
//!
//! - **Balanced.** A carrier is what balanced closes on: she closes as
//!   above, with a break-off level of 0.5, a patrol share of 0.5 and two
//!   orders to group 3: ten orders on day 1. She launches nothing: the
//!   contact is a pulse old against the figure of 0. On the morning of
//!   day 2 the contact is still held, at age 3, and is in reach from
//!   (6,0), 240 nm: she stays, and the staff gives no order. The contact
//!   is dropped at the end of the morning of day 2, at 5 pulses. On day 3
//!   she holds none and goes back: two orders, the route to (0,0) and
//!   group 2's arc, the whole circle. She arrives at hour 56.
//! - **Preserve the force.** She does not close and is given no route or
//!   speed, and the reason says preserve the force closes on nothing.
//! - **A kind it does not close on.** With the contact a cruiser: at
//!   balanced she stays and the reason says so; at press she closes.
//! - **Already in reach.** With the contact at (5,0), 200 nm: no route.
//! - **Too far.** With the contact at (30,0), 1200 nm, against 610: she
//!   stays, and the reason gives the two distances.
//! - **Not the fuel.** With ship 1 holding 50 t: she does not close, and
//!   the reason gives the fuel aboard against the fuel needed.
//! - **Speed alone.** With no contact and the commander stating "operate
//!   task force 1 from (6,0)": at balanced she is ordered to full speed
//!   and arrives at hour 8; at preserve the force she is given no speed
//!   order and arrives at hour 15, in the morning. With the hex (2,0), 80
//!   nm, she goes at cruise at every risk and arrives at hour 5.
//! - A held force is left alone; the same start gives the same orders; two
//!   worlds that differ only in what the Allied side cannot see give the
//!   same orders and reasons.
//! - Every staffed scenario built before this one gives the orders it
//!   gave, scenario 55 among them: there the three contacts are in reach,
//!   or of a kind press closes on and in reach.
