//! Scenario 84: the plan as an order. For each intent of the commander's
//! the staff hands in an order: the mission, the phases with what ends
//! each and what each waits for, what the plan needs and has, and the
//! tasks under it. It is read here on mornings of scenarios 79 to 82,
//! whose games are already worked by hand, so no new game is played.
//!
//! Proves decision 0159, rule E (items 15 to 17), step 6 of the opening
//! of stage 4, against a hand calculation. It has no scenario file of its
//! own: it runs scenarios 79, 80, 81 and 82 as their own tests run them,
//! and reads the order at the staff's hour of the morning named, before
//! that morning's decisions are made.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## The order
//!
//! An order is worked when it is asked for, from the side's command
//! state and the staff's own sums, and is stored nowhere. It has:
//!
//! - **the intent** it is of, and its **mission** in words (the intent's
//!   statement, as the staff's reasons give it);
//! - **the plan** and its state;
//! - **the phases,** in order, each with a name, the condition that ends
//!   it, and whether it is done, under way or to come;
//! - **what it waits for:** the things that must be so before the phase
//!   under way can end, each with its kind (men, leave, a day, a force,
//!   an intent, knowledge), whether it is met, and the day it is expected
//!   to be met where the staff can work one;
//! - **what it needs and has:** the men wanted and the men to hand, where
//!   a count of the enemy is held;
//! - **the day it expects** to be ashore, where it can work one, by the
//!   sums the take uses for a latest day;
//! - **the need to know,** as step 4 gives it;
//! - **the tasks** under it: the staff's intents, each with its phase.
//!
//! The phases of a "take a place": **prepare**, which ends when the plan
//! is released; **move**, which ends when the first man is ashore at the
//! place; **land**, which ends when the place is the side's. The phases
//! of a "hold" of a base that is not the side's: **wait**, which ends
//! when the base is the side's; then **hold**, which does not end. A
//! "hold" of a base that is the side's has the one phase, hold.
//!
//! A day on which a task had no order is not in the order.
//!
//! ## Scenario 80, a take that waits (intent 1)
//!
//! | Morning | Plan | Prepare | Move | Land | Waits for | Men wanted, to hand | Expects to be ashore |
//! | --- | --- | --- | --- | --- | --- | --- | --- |
//! | 2 | draft | to come | to come | to come | men: not met, expected day 4 | 1,200, 600 | day 8 |
//! | 5 | preparing | under way | to come | to come | nothing unmet | 1,200, 1,200 | day 8 |
//! | 6 | released | done | under way | to come | nothing | 1,200, 1,200 | day 8 |
//! | 9 | released | done | done | done | nothing | 1,200, 1,200 | none: it is ashore |
//!
//! On morning 2 the take has no task under it. On mornings 5 and 6 its
//! one task is intent 2, ready a landing. On morning 9 base 2 is the
//! side's and the plan is not yet marked complete: that is done by the
//! staff later that morning.
//!
//! The day it expects: a landing begun on day 4 is ready on day 5, takes
//! 3 days to embark and sails on day 8, and the passage is inside a day.
//!
//! With a latest day of 7 (the property of scenario 80), on morning 2:
//! the plan is a draft, it waits for men, not met, with no day expected,
//! and no day ashore is expected.
//!
//! ## Scenario 81, told to wait (intent 1)
//!
//! | Case | Morning | Plan | Prepare | Waits for | Expects to be ashore |
//! | --- | --- | --- | --- | --- | --- |
//! | Main | 3 | preparing | under way | a day: not before day 6, not met, expected day 6 | day 9 |
//! | Main | 7 | released | done | nothing | day 9 |
//! | Leave withheld | 3 | preparing | under way | a day, as above; leave to launch, not met, no day | none |
//! | Wait for a force | 3 | preparing | under way | a force: task force 2, not met, expected day 5 | day 8 |
//! | After another place | 3 | preparing | under way | an intent: intent 3, not met, no day | none |
//!
//! No count is held in scenario 81, so the order gives no men wanted. A
//! thing with no day expected leaves the day ashore without one.
//!
//! ## Scenario 82, how much must be known (intent 1)
//!
//! | Morning | Plan | Prepare | Waits for | The need to know |
//! | --- | --- | --- | --- | --- |
//! | 2 | preparing | under way | knowledge: met | 150, seen from the air, of day 1, 1 day old; met |
//! | 3 | preparing | under way | knowledge: not met, no day | the same count, 2 days old; not met; nobody looking yet |
//! | 4 | preparing | under way | knowledge: met | 150, of day 3, 1 day old; met |
//! | 5 | released | done | nothing | not asked: the plan is released |
//!
//! The men wanted and to hand are 300 and 600 on mornings 3, 4 and 5.
//!
//! ## Scenario 79, fed before it is taken (intent 1, a hold)
//!
//! On the morning of day 3: the phases are wait, under way, ending when
//! base 2 is the side's, and hold, to come. It waits for nothing the
//! staff can give a day to. Its one task is intent 2, keep base 3
//! supplied. No men are wanted and no day is expected.
//!
//! # Properties
//!
//! - **Nothing more is told.** Each order is made from the seat's own
//!   view: two worlds that differ only in what the Allied side cannot
//!   see and has not been told give the same orders.
//! - **Nothing is stored.** Asking for an order changes no hash; a game
//!   in which it is asked on every morning has the daily hashes of the
//!   game in which it is never asked.
//! - **A seat's tool gives it.** The tool answers for an intent of the
//!   commander's by its number, and refuses an intent of the staff's and
//!   an intent that is not the side's with the reason it refuses any
//!   intent the side does not have.
//! - **No "no order".** In none of the orders above is there a line for a
//!   task that was given no order.
