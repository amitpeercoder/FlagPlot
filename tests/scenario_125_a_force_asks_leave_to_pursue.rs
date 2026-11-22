//! Scenario 125: a force asks leave to go after what it has found. A
//! carrier at her hex holds a contact of a carrier beyond a night's
//! steaming and her bombers' reach. The staff asks leave to pursue and the
//! clock stops in the evening, before the day is played. On the
//! commander's yes she sails that night, stays while the contact is in
//! reach, and goes back when it is gone.
//!
//! Proves decision 0214 against a hand calculation. It stands on scenario
//! 58's for the closing rule and on scenario 55's for the orders to the
//! groups, and uses scenario 58's map, ship, groups and figures: the
//! frozen step 1 figures with scenario 53's table of aircraft for each
//! ship; no upkeep, wear, fatigue or supply figures. The data is
//! placeholder data. There is no enemy unit: nothing is struck and no die
//! is rolled.
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
//! As scenario 58: a hex is 40 nm; a night pulse of 12 hours and two
//! daylight pulses of 6; the placeholder carrier makes 16 kn at cruise and
//! 30 kn at most; the placeholder dive bomber has a strike radius of 250
//! nm; `strike_contact_max_age_pulses` 0; a contact gains a pulse of age
//! at the end of each pulse and is dropped at 5; the closing rule takes a
//! contact 3 pulses old or less. `pursuit_nights` 2.
//!
//! A night at full speed is 360 nm, so the closing rule as it stands takes
//! a contact within 360 and 250, 610 nm. With the bound lifted the night
//! is taken as two, 720 nm, and the test is 720 and 250, 970 nm.
//!
//! ## Inputs
//!
//! Scenario 58's, with these differences. The game is 4 days. The Allied
//! side starts holding one contact: 1 ship, a carrier, at (16,0), age 0,
//! 640 nm from the force. The Allied side's commands of day 1: intent 1,
//! operate task force 1 from (0,0); its risk, balanced; leave to prepare
//! and leave to launch both given; leave to pursue, ask.
//!
//! ## Day 1, the asking
//!
//! The rule as it stands: the contact is fresh and a carrier, which
//! balanced closes on, and 640 nm away against 610. It is too far, she has
//! nothing in reach, and she closes on nothing. So a find is looked for.
//! Worked again with the night as two: 640 is within 970. The contact is
//! the find. Along the row, (9,0) is 280 nm from the contact and (10,0) is
//! 240: she would close to (10,0), 400 nm from her, 13 hours and a third
//! at 30 kn (asserted as 400 divided by 30 in the game's own arithmetic).
//!
//! The staff hands in the day's orders as the rule as it stands gives
//! them, with no route and no speed order to the force (scenario 58's
//! property "Too far" at balanced, order for order), and one notice: task
//! force 1 asks leave to pursue; contact 1, a carrier, age 0, reported at
//! (16,0); to (10,0); the hours above; the order left, intent 1. The
//! standing interrupt for it fires, its subject task force 1.
//!
//! A seat that runs time is stopped here: the answer says it stopped
//! before day 1, for that interrupt, and no pulse of day 1 has been
//! resolved (the day is 1, the hour 0, the force at (0,0)).
//!
//! ## Day 1, the answer
//!
//! The commander answers yes for task force 1 and contact 1 at that stop.
//! It is taken in at this same dusk, before the staff is asked. The staff
//! now gives the force what scenario 58 gives at balanced when it closes,
//! order for order, with the route to (10,0): full speed and that route
//! among them. The reason of the route says it is a pursuit, by the
//! commander's leave, and that intent 1 waits; the event of the answer is
//! among the route order's causes. There is no notice of asking and none
//! that the force pursues. The side's leaves by answer: task force 1.
//!
//! She sails at the start of the night, is at (9,0) at hour 12, reaches
//! (10,0) at hour 13 and a third, in the morning, and stops. Group 3
//! launches nothing on any day: the contact is never 0 pulses old in a
//! daylight pulse. At the end of day 1 the contact is 3 pulses old.
//!
//! ## Day 2
//!
//! The contact is 3 pulses old, fresh, and 240 nm from (10,0): it is in
//! reach, and she stays. No order to the force, no notice. The leave
//! stands, since she has a contact. The contact is 4 pulses old at the end
//! of the night and is dropped at the end of the morning, at 5.
//!
//! ## Day 3
//!
//! The side holds no contact. She has neither a find nor a contact to
//! close on: the leave ends, and the staff hands in the notice that task
//! force 1's pursuit is ended. She goes back as scenario 58's day 2: two
//! orders, the route to (0,0), 400 nm, at the full speed she has, and
//! group 2's arc, the whole circle. She sails at hour 48 and arrives at
//! hour 61 and a third. The side's leaves by answer: none.
//!
//! ## Day 4
//!
//! She is at her hex. No order and no notice.
//!
//! ## At the end
//!
//! The force is at (0,0), stopped, having steamed 800 nm at 30 kn. Her
//! fuel is asserted as scenario 58's is: she burnt what a force of the
//! same two passages at full speed burns. The game as answered records
//! one notice of this rule: the pursuit ended, on day 3. The asking is not
//! recorded in it: it was shown at the stop as what the staff would hand
//! in, and the answer came before the staff was asked. A game not answered
//! records the asking (property "Not answered").
//!
//! # Properties
//!
//! - **Not answered.** With the interrupt switched off and no answer: day
//!   1 has the asking and no route. At dusk of day 2 the contact is 3
//!   pulses old and still 640 nm away: the find stands and the staff hands
//!   in the asking again. On day 3 there is no contact and no notice. She
//!   never leaves (0,0). The interrupt switched off gave no leave.
//! - **Refused.** With a no at the stop: no route. At dusk of day 2 the
//!   find is not asked about again, the refusal being kept for contact 1.
//!   She never leaves (0,0).
//! - **Go.** With the leave to pursue at go: no asking, no interrupt and
//!   no stop; a notice that task force 1 pursues, with the same contact,
//!   hex and hours; the same orders and the same passage as the game as
//!   answered; the reason says the leave was taken. On day 3 the notice
//!   that the pursuit is ended.
//! - **Stay.** With the leave at stay: no notice on any day, and every
//!   order of every day is what the staff gave before this rule.
//! - **The default.** With no leave to pursue stated: where launching
//!   waits for the commander by default it is ask, and where the leaves
//!   are given because there is no commander it is go.
//! - **Within a night.** With the contact at (12,0), 480 nm: she closes as
//!   scenario 58 at balanced, to (6,0), with no notice at ask, go or stay.
//! - **Beyond two nights.** With the contact at (25,0), 1000 nm against
//!   970: no find, no notice, at ask or go.
//! - **A kind the risk does not close on.** With the contact a cruiser at
//!   (16,0): at balanced there is no find. At press, with ask, there is,
//!   and the asking names it a cruiser.
//! - **Outside the area.** With an area limit on intent 1 that does not
//!   hold (16,0): no find, at ask or go.
//! - **Something in reach.** With a second contact, a carrier at (5,0),
//!   200 nm: she has a contact in reach, the rule as it stands stays, and
//!   no find is looked for.
//! - **The waters first.** At press, the order's waters first and holding
//!   (20,0) and not (12,0); a destroyer contact at (20,0), 800 nm, and a
//!   carrier contact at (12,0), 480 nm. As it stands the destroyer, in the
//!   waters, is first and is too far: she closes on nothing. With the
//!   night as two the rule takes the destroyer; with the waters set aside
//!   it takes the carrier. The more important kind is the find: the
//!   carrier, to (6,0). With the leave at ask the staff asks. With the
//!   leave at go the staff still asks and does not go, the waters being a
//!   limit the commander set. On a yes she closes to (6,0).
//! - **Seen again under a new number.** In the game as answered, with a
//!   new contact handed to the side at dusk of day 3, contact 2, a carrier
//!   at (26,0), 640 nm from (10,0): the leave stands without a new asking
//!   and she closes to (20,0). With that contact at (22,0), 480 nm, she
//!   closes to (16,0) by the rule as it stands. (If a test cannot hand a
//!   side a contact after the start, the build says so and this comes back
//!   to the architect.)
//! - **A force covering a landing** has no find, and **a held force** has
//!   none and is left alone.
//! - **With command levels on,** the asking is handed in only where the
//!   commander's own picture holds the contact.
//! - The same start gives the same orders and notices; two worlds that
//!   differ only in what the Allied side cannot see give the same.
//! - Every staffed scenario built before this one, with the leave at stay
//!   for every order, gives the orders it gave.
