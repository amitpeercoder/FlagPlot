//! Scenario 55: a carrier force played from an end. The commander states
//! "deny the waters at (0,5)", at press, and names task force 1, a
//! carrier. The staff gives the force the task of operating from its hex,
//! and at the force's level flies its patrol, searches and sets its
//! bombers to strike.
//!
//! Proves decision 0125 against a hand calculation. It stands on scenario
//! 53's for the strike orders and the sizes of the strikes. The data is
//! placeholder data: see `data/scenarios/55_a_carrier_force_from_an_end.toml`.
//! It runs on the frozen step 1 figures with scenario 53's table of
//! aircraft for each ship. The upkeep, wear, fatigue and supply figures
//! are not loaded. There is no enemy unit: every strike finds nothing,
//! the search sights nothing, and no die is rolled.
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
//! Scenario 53's. The placeholder carrier scout (type 2) searches to 300
//! nm and carries no weapon. From a hex, the hexes on its own row to the
//! east lie on 090 and to the west on 270, and (0,4) and (0,5) from (0,0)
//! lie on 150. The placeholder carrier (class 4): 3000 t of fuel over 9000
//! nm, 16 kn at cruise.
//!
//! ## Inputs
//!
//! A placeholder map of open sea, with S at (5,0) a coast hex with a port:
//! Allied base 1, with 5000 t of fuel. The game is 2 days. There is no
//! Japanese unit and no Japanese base.
//!
//! Task force 1, Allied: one placeholder carrier, ship 1, stopped at sea
//! at (0,0), full of fuel, on the carrier mission, at normal readiness,
//! with the stance engage and base 1 as its home port. Aboard her:
//!
//! | Group | Type          | Ready aircraft | At the start |
//! | ----- | ------------- | -------------- | ------------ |
//! | 1     | fighter       | 12             | resting, no setting |
//! | 2     | carrier scout | 6              | resting, no arc |
//! | 3     | dive bomber   | 10             | resting, no setting |
//!
//! The Allied side starts holding scenario 53's three contacts: contact 1,
//! a carrier at (0,4), age 2; contact 2, 2 cruisers at (3,0), age 1;
//! contact 3, 3 transports at (-3,0), age 0.
//!
//! The Allied side's commands of day 1: intent 1, deny the waters within 0
//! hexes of (0,5); its risk, press; its forces limit, task force 1; leave
//! to prepare and leave to launch both given.
//!
//! ## The tree on the first morning
//!
//! | Intent | Kind | Parent | Maker | Addressee |
//! | ------ | ---- | ------ | ----- | --------- |
//! | 1      | deny the waters at (0,5) | none | the commander | the area |
//! | 2      | operate task force 1 from (0,0) | 1 | the staff | task force 1 |
//!
//! Base 1 has no air group, so no task is formed for it.
//!
//! ## The orders of day 1
//!
//! All serve intent 2 and were decided at the level of task force 1,
//! except the sizing order, whose reason says the group decided.
//!
//! - **The force,** two orders: battle readiness; and rules of engagement
//!   with the stance avoid and a break-off level of 0.75. It is in its hex
//!   and is given no route. Fuel: the passage home is 200 nm, 66.666666 t
//!   at press against 3000 aboard.
//! - **Group 1,** two orders: combat air patrol, and a patrol share of
//!   0.25. It keeps 12 x 0.25 = 3 overhead in each daylight pulse.
//! - **Group 2,** two orders: the search mission, and the arc. It can
//!   search and cannot attack a ship. The threats are the three contacts,
//!   on 150, 090 and 270, and the waters on 150. The narrowest arc that
//!   holds them runs from 090 to 270. Widened by 30 at each end it is 060
//!   to 300, at 300 nm.
//! - **Group 3,** four orders, as scenario 53's bombers: the naval strike
//!   mission; the oldest contact, 2; extended reach; strike sizing on.
//!
//! Ten orders on day 1.
//!
//! ## The strikes of day 1
//!
//! As scenario 53, with one group. In the morning contact 1 is 3 pulses
//! old and does not qualify; group 3 sends its 10 at contact 2, whose need
//! is 24. The raid finds nothing and contact 2 is dropped. In the
//! afternoon contact 3 is 2 pulses old; group 3 sends its 10 at it, whose
//! need is 18. It finds nothing and contact 3 is dropped. At the end of
//! the afternoon contact 1 reaches 5 pulses and is dropped.
//!
//! ## Day 2
//!
//! The Allied side holds no contact. The one threat is the waters, on 150:
//! the arc is 110 to 180. One order: group 2's arc, 110 to 180 at 300 nm.
//! Nothing is launched.
//!
//! ## At the end
//!
//! Two strikes of 10. Eleven staff orders. The force has not moved and has
//! burnt no fuel under way. Group 2 searched in all four daylight pulses
//! with its 6 and sighted nothing.
//!
//! # Properties
//!
//! - **Balanced.** Group 1's share is 0.5 and it keeps 6 overhead. Group 3
//!   is given two orders, the mission and sizing on, and launches nothing.
//!   The rules of engagement order has a break-off level of 0.5. Day 1 has
//!   8 orders.
//! - **No scout.** With no group 2 and a second dive bomber group, group 4,
//!   of 10: no group can only search, so group 3, the lowest that can do
//!   both, is set to search and group 4 to strike. The placeholder dive
//!   bomber has no search range, so this is a test of the pure decision in
//!   a view built for it, with a type that has both.
//! - **Fuel takes her home.** At balanced, with ship 1 holding 79 t: the
//!   passage home needs 66.666666 x 1.2 = 80 t. On day 1 the force is
//!   routed to base 1 and the reason gives 79 against 80. Its groups are
//!   given their orders as above. With 81 t it stays.
//! - **A commander's hex.** With the commander stating "operate task force
//!   1 from (2,0)" as intent 1 and nothing else: on day 1 the force is
//!   routed to (2,0) at cruise, and the air orders are those of balanced.
//! - **A raid on the force raises the patrol.** In a view built for it,
//!   with an air attack on the force two days before: the share ordered at
//!   balanced is 0.75.
//! - **Nothing to fly.** With no air group aboard, no task is formed. With
//!   every group at no ready aircraft, the staff hands in on day 1 that it
//!   cannot meet intent 2.
//! - **A surface force and a carrier force named together.** With task
//!   force 2, two cruisers on the surface combat mission, also in the
//!   forces limit: the staff forms a stand task for it (decision 0124) and
//!   the carrier task for force 1, and orders each by its own doctrine.
//! - A held force or group is left alone; the same start gives the same
//!   orders; two worlds that differ only in what the Allied side cannot
//!   see give the same intents, orders and reasons.
//! - Every staffed scenario built before this one gives the orders it
//!   gave.
