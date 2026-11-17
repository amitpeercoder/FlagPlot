//! Scenario 64: commanders and initiative. One carrier force at balanced
//! holds a cruiser it sighted itself. A steady commander stays; a bold one
//! closes; and a cautious one will not close even on a carrier.
//!
//! Proves decision 0137, step 4 of stage 2b, against a hand calculation.
//! The data is placeholder data: `data/scenarios/64_commanders_and_initiative.toml`
//! is scenario 58's file with command levels on, report figures of on
//! time 1 in every direction, intent 1 with no risk set, so balanced, and
//! the starting contact a cruiser that task force 1 saw: it is in task
//! force 1's picture alone, as its own. It runs on the frozen step 1
//! figures as scenario 58 does. There is no Japanese unit, so no die is
//! rolled.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Figures and inputs
//!
//! As scenario 58: task force 1, one carrier, at (0,0), with groups 1, 2
//! and 3; base 1 at (0,5); 3 days. The contact: 1 ship, a cruiser, at
//! (12,0), age 0, 480 nm away against a reach of 250 and a night at full
//! speed of 360. Balanced closes on a carrier; press closes on any kind;
//! preserve the force closes on nothing.
//!
//! ## The main case: a bold commander
//!
//! Task force 1's commander is "Placeholder Bold", bold.
//!
//! **Day 1.** The contact is his own. He weighs it at press, one level
//! bolder than balanced: press closes on a cruiser. He closes, as scenario
//! 58's force does at balanced on a carrier: the first hex within reach
//! is (6,0), full speed, there at hour 8. Everything else is balanced's:
//! a break-off level of 0.5, a patrol share of 0.5, and two orders to
//! group 3. Ten orders: the force, four (battle readiness, rules of
//! engagement, full speed, the route to (6,0)); group 1, two; group 2,
//! two; group 3, two. She launches nothing: the contact is a pulse old in
//! the morning against the figure of 0.
//!
//! The reason for the route names Placeholder Bold, bold, says the
//! contact is task force 1's own, and says it was weighed at press where
//! the task's risk is balanced.
//!
//! **Day 2.** She is at (6,0), having left her hex to close. The contact
//! is 3 pulses old and 240 nm away, in reach, and, weighed at press, of a
//! kind he closes on: she stays. No order. The contact is dropped at the
//! end of the morning, at 5 pulses.
//!
//! **Day 3.** She holds none and goes back: two orders, the route to
//! (0,0) and group 2's arc, the whole circle. She arrives at hour 56.
//!
//! Ten orders, none and two: twelve.
//!
//! ## The pictures
//!
//! Contact 1 is in task force 1's picture and in the side's own. The area
//! command and base 1 hold no contact at any time: a contact a scenario
//! says one command saw is sent to no one.
//!
//! # Properties
//!
//! - **Steady.** With no commander named: balanced does not close on a
//!   cruiser. She stays at (0,0) and is given eight orders on day 1 (the
//!   ten less the speed and the route), none on day 2, and one on day 3,
//!   group 2's arc, the whole circle, the contact having been dropped at
//!   the end of the morning of day 2. The reason for not closing is
//!   scenario 58's for a kind it does not close on, and names no
//!   commander.
//! - **Cautious, and a carrier.** With the contact a carrier and the
//!   commander "Placeholder Cautious", cautious: he weighs his own contact
//!   at preserve the force, which closes on nothing. She stays: eight
//!   orders on day 1. The reason names him, cautious, and the level. With
//!   no commander and the same carrier she closes: ten orders, as scenario
//!   58 at balanced.
//! - **A contact he was told of.** With the bold commander and the cruiser
//!   an ordinary starting contact, in every command's picture: it is not
//!   his own. He weighs it at balanced and stays: eight orders on day 1,
//!   and the reason names no rating.
//! - **Press stays press, preserve stays preserve.** With the bold
//!   commander and intent 1 at press: the orders are scenario 58's twelve
//!   on day 1. With a cautious commander and intent 1 at preserve the
//!   force: the orders are those of the same file with no commander.
//! - **Bold at preserve the force.** With the bold commander, intent 1 at
//!   preserve the force and the contact a carrier, his own: he weighs it
//!   at balanced, which closes on a carrier, and the speed is preserve's:
//!   always cruise. She is routed to (6,0) with no speed order and arrives
//!   at hour 15, in the morning of day 1.
//! - **Command levels off.** The main case's file with command levels off:
//!   the commander is not read. The contact is the side's, a cruiser, at
//!   balanced: she stays, eight orders on day 1.
//! - **A rating without its note is refused** by the loader, and so is a
//!   commander for a command the scenario does not have.
//! - **Nothing else reads a rating.** Scenarios 61, 62 and 63 give what
//!   they gave, and every scenario built before this one gives the events
//!   it gave.
//! - The same start gives the same game.
