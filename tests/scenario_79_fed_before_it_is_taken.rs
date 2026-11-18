//! Scenario 79: fed before it is taken. Men are ashore beside an airfield
//! the enemy still holds, living on what their landing brought. The
//! commander has asked that the airfield be held. The staff feeds the men
//! through the base their own side has in the hex, and sends the first run
//! of supply when it is needed, without waiting for the airfield to change
//! hands.
//!
//! Proves decision 0159, rule A (items 1 to 3), step 1 of the opening of
//! stage 4, against a hand calculation. The data is placeholder data: see
//! `data/scenarios/79_fed_before_it_is_taken.toml`. It runs on the frozen
//! step 1 figures. Nobody is on the attack, so nobody fights and no die is
//! rolled.
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
//! As scenario 69's. A man eats 0.005 t a day, paid at the start of the
//! night pulse by his side's bases in his hex, in base order. Supply
//! landed goes into the side's lowest-numbered base in the hex. A size 0
//! port works by lighter at 20 t an hour for each ship. Day d begins at
//! hour 24 x (d - 1): a night pulse of 12 hours and two daylight pulses
//! of 6. In a pulse ships move before cargo is worked.
//!
//! The staff's figures: at balanced the margin is 5 days; a supply intent
//! with no interval uses 15 days.
//!
//! ## Inputs
//!
//! The placeholder map of scenario 27: A at (0,0), a coast hex with port
//! size 5, and B at (4,0), a coast hex with port size 0, 160 nm apart: 16
//! hours at 10 kn. The game is 14 days.
//!
//! | Base | Side | Hex | Supply | Fuel |
//! | ---- | ---- | --- | ------ | ---- |
//! | 1, "Placeholder base A" | Allied | A | 1,000 t | 0 |
//! | 2, "Placeholder airfield B" | Japanese | B | 100 t | 0 |
//! | 3, "Placeholder beach at B" | Allied | B | 48 t | 0 |
//!
//! Base 2 has no gun and no aircraft and is on the Allied map. Base 3 is
//! the base a landing forms: no airfield, not on the map. No base has a
//! reserve. The Allied side holds no estimate of the enemy.
//!
//! Task force 1, Allied, on the transport mission: ships 1 and 2,
//! placeholder troop transports (600 men, 3,000 t of cargo, 10 kn, 0.1 t
//! of fuel a mile, 1,000 t of fuel), stopped at base 1, full of fuel,
//! nothing aboard.
//!
//! Units 1 and 2, Allied, "Placeholder First" and "Placeholder Second":
//! fighting units of 600 men each, ashore at B on the defence. Unit 3,
//! Japanese, "Placeholder garrison": a fighting unit of 400 men ashore at
//! B on the defence.
//!
//! The Allied side's command of day 1: intent 1, hold base 2, with a
//! forces limit of task force 1 and base 1 as the source. No risk is set,
//! so balanced. Leave to prepare and leave to launch are both given. The
//! Japanese side is given nothing.
//!
//! ## What the staff does
//!
//! Base 2 is not the Allied side's, and the Allied side has base 3 in the
//! same hex with 1,200 men ashore there. So on day 1 the staff forms
//! intent 2, keep base 3 supplied, under intent 1, with a floor of 5 days
//! (rule A, item 1). The hold's reason that day says that base 2 is not
//! the side's, that the staff waits for it, and that the men there are
//! fed through base 3 (item 3). It gives no order to unit 1 or unit 2.
//!
//! The use at B is the Allied men ashore, 1,200 x 0.005 = 6 t a day. The
//! floor's stock is 5 x 6 = 30 t. The stock the staff reads on a morning
//! is base 3's before that night's charge. A run ordered on a morning
//! loads that day, sails the next and arrives that same day; the reading
//! checked is the day after.
//!
//! | Morning | The stock at base 3 | The day checked | Its reading | The staff |
//! | --- | --- | --- | --- | --- |
//! | 2 | 42 t | 4 | 42 - 12 = 30 t | Not below 30: no run |
//! | 3 | 36 t | 5 | 36 - 12 = 24 t | Below 30: a run |
//!
//! On the morning of day 1 no run is ordered either: whether the staff
//! works the sum on the morning it forms the intent is not asserted (the
//! reading would be 48 - 12 = 36 t, not below 30).
//!
//! The run's load: the floor's stock on the day it arrives, day 4, 30 t,
//! less that day's reading, 30 t, which is 0; plus 15 days of use, 90 t.
//!
//! | Day | Hour | The staff's orders | What happens |
//! | --- | --- | --- | --- |
//! | 3 | 48 | A load of supply to a weight of 90 t | Ship 1 loads 90 t at A |
//! | 4 | 72 | A route to B and back to A, acting at B, and an unload order | Sails at hour 72, arrives at hour 88. Ship 1 lands 40 t in the morning and 50 t in the afternoon, into base 3 |
//! | 5 | 96 | none | The force steams for A at hour 96 and arrives at hour 112 |
//!
//! Three orders to task force 1 in all, and none to a land unit. No staff
//! order or command is refused. After the run, a morning's stock is
//! 114 - 6 x (d - 5) for day d from 5 to 14, never below 42 t, so no
//! second run is called in the game (a run is called when the morning's
//! stock is below 42 t).
//!
//! ## Supply, fuel and men
//!
//! - **Base 3** pays 6 t on each of the 14 nights. After the nights of
//!   days 1 to 4 it holds 42, 36, 30 and 24 t. It takes 90 t on day 4:
//!   114 t. It pays 6 t on the nights of days 5 to 14: 54 t at the end.
//! - **Base 1** feeds nobody. It loads out 90 t: 910 t at the end.
//! - **Base 2** feeds 400 men, 2 t a night, for 14 nights: 72 t at the
//!   end. It is Japanese at the end.
//! - **Each ship** steams 320 nm and burns 32 t: 968 t at the end.
//! - **Every man is fed every night.** Units 1 and 2 have 600 men each at
//!   the end and unit 3 has 400. No man is lost, landed or embarked.
//!
//! # Properties
//!
//! - **No hold stated.** With intent 1 left out, no intent is formed and
//!   no order is given in the game. Base 3 pays 6 t on the nights of days
//!   1 to 8 and then holds nothing. On the night of day 9 it is asked for
//!   6 t and pays none, and 1,200 Allied men are unfed. (What hunger then
//!   takes is the engine's rule and is not asserted here.)
//! - **No man ashore.** With units 1 and 2 left out and intent 1 stated,
//!   the staff forms no intent and gives no order: the hold waits for
//!   base 2 to be the side's, as before this decision.
//! - **No source.** With base 1 left out of intent 1's forces limit, no
//!   "keep supplied" is formed, and the hold's reason says that its
//!   forces limit names no source.
//! - **Formed once.** In the main case exactly one "keep supplied" intent
//!   is in force for the hex on every day from 2 to 14 (item 2).
//! - **Stated as before.** The Japanese side, given nothing, is given no
//!   order by its staff on any day.
