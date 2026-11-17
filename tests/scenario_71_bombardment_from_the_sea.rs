//! Scenario 71: bombardment from the sea. The commander gives a
//! battleship the task of bombarding an enemy base. The staff sends her
//! in the night, brings her home at the morning, and stops when her
//! rounds are below the level her risk keeps.
//!
//! Proves decision 0146, step 5 of stage 3, second part, against a hand
//! calculation. The data is placeholder data: see
//! `data/scenarios/71_bombardment_from_the_sea.toml`. It runs on the
//! frozen step 1 figures, which give no fuelling rate, so a ship in port
//! is filled in the pulse, and no ammunition in port. Three bombardment
//! figures are its own: `bombardment_minutes` 5, `bombardment_hit_chance`
//! 0 and `bombardment_false_claim_chance` 0. No shell hits and nothing is
//! claimed, so no die changes anything.
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
//! A hex is 40 nm. Day d begins at hour 24 x (d - 1): a night pulse of 12
//! hours and two daylight pulses of 6. The placeholder battleship (class
//! 9): 16 kn at cruise, 2,000 t of fuel over 8,000 nm, which is 0.25 t a
//! mile; 8 guns at 3 rounds a minute, 100 rounds a gun, an outfit of 800.
//! In a night's bombardment she fires 3 x 8 x 5 = 120 shells. At balanced
//! a ship is short of rounds below half her outfit, 400; at press below
//! 200; at preserve the force below 600. At balanced a sortie needs one
//! and a fifth times the fuel of the passage out and home.
//!
//! ## Inputs
//!
//! The placeholder map of scenario 27: A at (0,0), port size 5, and B at
//! (4,0), 160 nm away. The game is 6 days.
//!
//! Base 1, Allied, at A, with 5,000 t of fuel. Base 2, Japanese, at B, on
//! the Allied map, with no gun, aircraft, supply or man. No contact is
//! held. There is no Japanese unit.
//!
//! Task force 1, Allied, on the bombardment mission: ship 1, a
//! placeholder battleship, stopped at base 1 at cruising speed, full of
//! fuel and rounds, at normal readiness, with no home port and the stance
//! avoid.
//!
//! The Allied side's commands of day 1: intent 1, bombard base 2 with
//! task force 1. No risk is set, so balanced. Leave to prepare and leave
//! to launch are both given.
//!
//! ## The sortie
//!
//! A to B is 160 nm: 10 hours at 16 kn, inside the night's 12. Out and
//! home is 320 nm, 80 t; at balanced the ship needs 96 t.
//!
//! A sortie ordered on day d: the force sails at the start of the night,
//! reaches B at hour 10 of it and halts to bombard. It fires 120 shells
//! in that night's bombardment phase. At the start of the morning it
//! steams on for A: in the morning's 6 hours it makes 96 nm, and it
//! arrives 4 hours into the afternoon, 22 hours after it sailed. The ship
//! takes 80 t at A in that pulse.
//!
//! ## Day by day
//!
//! | Day | Rounds at the staff's hour | The staff's commands and orders | Rounds after the night |
//! | --- | --- | --- | --- |
//! | 1 | 800 | Approves plan 1 to preparing and to released. Home port, base 1. Rules of engagement: engage, break-off level 0.5. Battle readiness. Route to B and A, B marked to act at | 680 |
//! | 2 | 680 | Route, as day 1 | 560 |
//! | 3 | 560 | Route | 440 |
//! | 4 | 440 | Route | 320 |
//! | 5 | 320, below 400 | Normal readiness. Notice: cannot meet intent 1 | 320 |
//! | 6 | 320 | none. Notice: cannot meet intent 1 | 320 |
//!
//! The staff gives 4, 1, 1, 1, 1 and 0 orders: 8 in all. No speed order
//! is given: the force is at cruise. The force halts at B at hours 10,
//! 34, 58 and 82, each to bombard, and steams on at hours 12, 36, 60 and
//! 84. Base 1 holds 4,680 t of fuel at the end.
//!
//! The reasons of days 1 to 4 give base 2 at 4 hexes, 10 hours to it,
//! 2,000 t aboard against 96 needed, and the ship's rounds against 400.
//! The reasons of days 5 and 6 give 320 rounds against 400 and no day
//! from which the force may be used: base 1 holds no ammunition. Every
//! order serves intent 1 and was decided at the level of task force 1.
//! No staff order is refused.
//!
//! # Properties
//!
//! - **Press.** With intent 1 at press: the level is 200. After the
//!   night of day 5 the ship has 200 rounds, which is not below it: she
//!   sorties on all six nights and ends with 80 rounds. The break-off
//!   level ordered is 0.75. Orders: 4, then 1 on each of days 2 to 6.
//! - **Preserve the force.** With intent 1 at preserve the force: the
//!   level is 600. She sorties on days 1 and 2 and has 560 rounds on the
//!   morning of day 3: no sortie, normal readiness, and the notice from
//!   that morning. The break-off level ordered is 0.25 and the fuel
//!   needed 120 t. Orders: 4, 1, 1.
//! - **Preserve the force, and a contact.** At preserve the force, with
//!   the Allied side holding a contact of 2 ships at (5,0) that does not
//!   age out before day 2: on day 1 there is no sortie, the reason gives
//!   2 ships against 1, and day 1 has two orders, the home port and the
//!   rules of engagement.
//! - **Under neutralise.** With the commander's intent 1 "neutralise base
//!   2" and a forces limit of task force 1: the staff forms intent 2,
//!   bombard base 2 with task force 1, under it on day 1, and the force's
//!   orders are those of the main case, serving intent 2.
//! - **Under take a place.** With the inputs of scenario 69's property
//!   "No hold stated" added (base 1 with 1,000 t of supply, task force 2
//!   of two placeholder troop transports, units 1 and 2 of 600 men, the
//!   commander's intent 1 "take the place at base 2") and task force 1
//!   named in that intent's forces limit: the staff forms the bombard
//!   task on the morning of day 2, when plan 1 is released, and gives the
//!   battleship her home port and her rules of engagement. The landing
//!   force is routed on day 5 and is due that day: on the morning of day
//!   5 the battleship is set to battle readiness and routed, and fires
//!   120 shells in that night. Base 2 changes side at the end of day 5.
//!   On the morning of day 6 the target is the side's: she is set to
//!   normal readiness and given nothing more. The transports' orders are
//!   the eight of that property.
//! - **Held.** With task force 1 held from day 3: no order from day 3,
//!   and the reason names the hold.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see give the same orders and reasons; a
//!   save on day 3 and a resume give the same game; every scenario built
//!   before this one gives the events it gave.
