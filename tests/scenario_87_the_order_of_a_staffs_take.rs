//! Scenario 87: the plan order of a take the staff formed. The commander
//! says "control the island"; the take of its airfield is the staff's.
//! The staff hands in a plan order for that take as it does for a take
//! of the commander's, and the seat's tool answers for it.
//!
//! Proves decision 0164 against a hand calculation. It has no scenario
//! file of its own: it runs scenario 76's main game as scenario 76's own
//! test runs it, and reads the order at the staff's hour of the morning
//! named, before that morning's decisions are made, as scenario 84 reads
//! its orders.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## The game
//!
//! Scenario 76's main game (its own hand calculation has every day).
//! Intent 1 is the commander's, control island 1. Intent 2 is the
//! staff's, take the place at B, (4,0), with a plan of its own. Intent 8
//! is the staff's, ready a landing by task force 1 at B, under intent 2.
//! Leave to prepare and leave to launch are given. The side holds no
//! estimate and no count of the enemy at B.
//!
//! The landing is begun on day 1 and is ready on day 2 (loading 100 and
//! rehearsal 100, at 100 a day). The take's plan is released by the
//! decisions of the morning of day 2. Supply is loaded on day 2 and unit
//! 1 on day 3; the force sails on day 4 and its first men are ashore
//! that day. Base 2 is the side's at the end of day 4, and the plan is
//! marked complete by the decisions of the morning of day 5.
//!
//! ## The order of intent 2
//!
//! | Morning | Plan | Prepare | Move | Land | Waits for | Men wanted, to hand | Expects to be ashore |
//! | --- | --- | --- | --- | --- | --- | --- | --- |
//! | 2 | preparing | under way | to come | to come | nothing | none | day 4 |
//! | 3 | released | done | under way | to come | nothing | none | day 4 |
//! | 4 | released | done | under way | to come | nothing | none | day 4 |
//! | 5 | released | done | done | done | nothing | none | none: it is ashore |
//!
//! The mission is the take's statement, as the staff's reasons give it.
//! On each of the four mornings its one task is intent 8, ready a
//! landing, in the phase prepare. No need to know is asked.
//!
//! The day it expects, by the sums of a latest day (the day it starts
//! from, plus the days to embark, plus the days of passage, less 1):
//!
//! - **Morning 2,** preparing: it starts from the day the landing is
//!   ready, day 2. To embark: 1 day for the supply and 1 for unit 1,
//!   ashore at A. The passage, 16 hours, is 1 day. 2 + 2 + 1 - 1 = 4.
//! - **Morning 3,** released: it starts from today, day 3. The supply
//!   was loaded on day 2, so 1 day to embark, for unit 1. 3 + 1 + 1 - 1
//!   = 4.
//! - **Morning 4:** unit 1 is aboard, so no day to embark. 4 + 0 + 1 - 1
//!   = 4.
//! - **Morning 5:** unit 1 is ashore at B and B is the side's: no day.
//!
//! ## The order of intent 1
//!
//! On the morning of day 2 the order of intent 1, control island 1, is
//! what it was before this decision: it has no phases, no men and no
//! day, and intents 2 and 8 are among its tasks.
//!
//! ## The orders together
//!
//! On the morning of day 2 the staff hands in two orders, of intent 1
//! and then of intent 2, and no other.
//!
//! # Properties
//!
//! - **The tool answers for the take.** The seat's tool `order` answers
//!   for intent 2 with the order above, and for intent 1 as before.
//! - **It refuses the rest.** It refuses intent 3, the staff's hold of
//!   base 2, and intent 8, the staff's landing, with the words it
//!   refuses any intent the side does not have.
//! - **Nothing is stored.** Asking for the order of intent 2 on every
//!   morning changes no daily hash of scenario 76's main game.
//! - **Nothing more is told.** The order of intent 2 is made from the
//!   seat's own view: two worlds that differ only in what the Allied
//!   side cannot see and has not been told give the same order.

// The tests are written by the build, to the statements above.
