//! Scenario 57: going back to rearm. Scenario 54's force, with one of its
//! two cruisers short of rounds: the staff keeps the force in port on the
//! first night, she rearms, and the force sorties on the second.
//!
//! Proves decision 0127 against a hand calculation, with decision 0122's
//! rule beneath it. It stands on scenario 54's for the sortie's hours,
//! places and fuel. The data is placeholder data: see
//! `data/scenarios/57_going_back_to_rearm.toml`. It runs on the frozen
//! step 1 figures with scenario 52's rearming rates and weights. There is
//! no enemy unit and no random draw.
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
//! Scenario 54's, and scenario 52's: a size 1 port has one berth, where
//! ammunition is worked at 5 t an hour; a round weighs 0.1 t for a
//! cruiser, so a cruiser at the berth takes 50 rounds an hour; a torpedo
//! weighs 2 t. The placeholder cruiser's outfit is 800 rounds, and she has
//! 8 tubes.
//!
//! ## Inputs
//!
//! Scenario 54's map, base 1 at S with port size 1, the waters at (3,0),
//! task force 1 of ships 1 and 2, the contact of 3 ships at (10,0), the
//! intent and its forces limit, 4 days. Balanced.
//!
//! Changed: base 1 holds 100 t of ammunition. Ship 1 has 300 rounds
//! aboard; ship 2 has her 800. No tube of either has fired.
//!
//! ## Day 1
//!
//! There is a threat, and the force has the fuel. At balanced a ship is
//! short below a half of her outfit: 400 rounds. Ship 1 has 300: the force
//! is short and does not sortie.
//!
//! Ship 1 lacks 500 rounds, 50 t. She is the only ship at base 1 with
//! work and has the berth: 500 rounds in 10 hours. She will be no longer
//! short at hour 2, with 400 aboard, and full at hour 10. The reason gives
//! 300 against 400 and day 2 as the day from which the force may be used.
//!
//! The staff's orders of day 1 are two: the home port, and the rules of
//! engagement. The force is not set to battle readiness: it does not
//! sortie.
//!
//! In the night of day 1 ship 1 takes 500 rounds, 50 t, and is full at
//! hour 10. Base 1 holds 50 t of ammunition.
//!
//! ## Day 2
//!
//! The contact is held, at age 3. The force is short of nothing. It
//! sorties, as scenario 54's first sortie a day later: two orders, battle
//! readiness and the route; it halts at (3,0) at hour 31.5, steams on at
//! hour 36 and is at S, stopped and full of fuel, at hour 43.5.
//!
//! ## Days 3 and 4
//!
//! No contact is held. On day 3, one order: normal readiness. On day 4,
//! none.
//!
//! The staff gives 2, 2, 1 and 0 orders: 5 in all. No notice is handed
//! in.
//!
//! # Properties
//!
//! - **Press.** A ship is short below a quarter, 200 rounds. Ship 1 has
//!   300: the force sorties on day 1 and on every night after. She sails
//!   at the start of the night of day 1 before any work is done, and has
//!   taken no round when she sails.
//! - **Preserve the force.** With the contact reporting 2 ships, so that
//!   the threat counts: a ship is short below three quarters, 600 rounds.
//!   The force is short on day 1 and sorties on day 2.
//! - **No ammunition ashore.** With base 1 holding none: the force does
//!   not sortie on day 1 or day 2, the reason gives no day, and the staff
//!   hands in on day 1 that it cannot meet intent 2.
//! - **Torpedoes.** With ship 1 full of rounds and 5 of her 8 tubes fired:
//!   at balanced more than a half are fired and the force is short. She
//!   takes 5 torpedoes, 10 t, in 2 hours of the night of day 1, and the
//!   force sorties on day 2. With 4 fired she is not short and the force
//!   sorties on day 1. At preserve the force one fired tube makes her
//!   short; at press no number does.
//! - **The later of fuel and ammunition.** With ship 1 at 300 rounds and
//!   both ships at 70 t of fuel, against the 72 the sortie needs: the
//!   reason of day 1 gives both shortages and day 2.
//! - **A short escort is not free.** In scenario 56, with ship 3 at a
//!   tenth of her rounds: the first run sails alone, and its reason gives
//!   her rounds against the level.
//! - **The marker.** A force whose ships carry 9,999 rounds a gun is never
//!   short of rounds.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see give the same orders and reasons.
