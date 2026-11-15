//! Scenario 49: a strike sized to its target. Eight groups at one base and
//! three contacts in reach: a sized group sends what the report calls for
//! and no more, and goes on to the next contact when one is served.
//!
//! Proves decision 0118 against a hand calculation. The data is placeholder
//! data: see `data/scenarios/49_a_strike_sized_to_its_target.toml`. It runs
//! on the frozen step 1 figures with the table of aircraft for each ship
//! added, as below. There is no enemy unit at all: every strike flies to a
//! contact's hex, finds nothing there and rolls no die.
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
//! As scenario 46: a hex is 40 nm across; the placeholder dive bomber
//! (type 3) has a strike radius of 250 nm; at the end of every pulse a
//! contact not made or refreshed in it gains one pulse of age; strikes are
//! decided in a daylight pulse, in group number order; a group chooses
//! among the contacts it could launch on by the kind first on its target
//! list, then the nearest; the upkeep, wear, fatigue and supply figures are
//! not loaded.
//!
//! Aircraft for each ship reported, in this scenario's own figures and not
//! the live ones: carrier 24, battleship 24, cruiser 12, destroyer 6,
//! transport 6, auxiliary 4.
//!
//! ## Inputs
//!
//! A placeholder map of open sea. Allied base 1 at (0,0). The game is 1
//! day. There is no Japanese unit.
//!
//! Eight Allied groups of placeholder dive bombers at base 1, all on the
//! naval strike mission, with the default target list, on normal reach,
//! each set to launch on a contact up to 3 pulses old:
//!
//! | Group | Ready aircraft | Strike sizing | Share held back |
//! | ----- | -------------- | ------------- | --------------- |
//! | 1     | 10             | on            | not set (0)     |
//! | 2     | 10             | not set (off) | not set         |
//! | 3     | 10             | on            | not set         |
//! | 4     | 12             | on            | 0.25            |
//! | 5     | 10             | on            | not set         |
//! | 6     | 10             | on            | not set         |
//! | 7     | 20             | on            | not set         |
//! | 8     | 10             | on            | not set         |
//!
//! The Allied side starts holding three contacts, numbered from the oldest
//! down:
//!
//! | Contact | Hex    | Ships | Kind      | Age at the start | From base 1 | Need |
//! | ------- | ------ | ----- | --------- | ---------------- | ----------- | ---- |
//! | 1       | (0,4)  | 1     | carrier   | 2                | 160 nm      | 1 x 24 = 24 |
//! | 2       | (3,0)  | 2     | cruiser   | 1                | 120 nm      | 2 x 12 = 24 |
//! | 3       | (-3,0) | 3     | transport | 0                | 120 nm      | 3 x 6 = 18  |
//!
//! ## The night
//!
//! Nothing is decided. At its end the contacts are 3, 2 and 1 pulses old.
//! All three are in every group's reach and no older than 3, so every
//! group could launch on each. Each group's order among them is contact 1,
//! the carrier, then contact 2, then contact 3.
//!
//! ## The morning
//!
//! | Group | Can send | Contact | Its need | Sent there before | It sends | Left on the ground |
//! | ----- | -------- | ------- | -------- | ----------------- | -------- | ------------------ |
//! | 1     | 10       | 1       | 24       | 0                 | 10       | 0 |
//! | 2     | 10       | 1       | not read | not read          | 10       | 0 |
//! | 3     | 10       | 1       | 24       | 20                | 4        | 6 |
//! | 4     | 9        | 2       | 24       | 0                 | 9        | 3 |
//! | 5     | 10       | 2       | 24       | 9                 | 10       | 0 |
//! | 6     | 10       | 2       | 24       | 19                | 5        | 5 |
//! | 7     | 20       | 3       | 18       | 0                 | 18       | 2 |
//! | 8     | 10       | none    |          |                   | 0        | 10 |
//!
//! - **Group 2** has sizing off: it launches all 10 at the first contact,
//!   as it does today. What it sends counts for the groups after it.
//! - **Group 3** finds 20 sent at contact 1 against a need of 24 and sends
//!   4.
//! - **Group 4** is able to fly 12 and holds back 0.25: 12 x 0.75 = 9. It
//!   finds contact 1 served, 24 against 24, and goes on to contact 2.
//! - **Group 6** finds 19 sent at contact 2 and sends 5.
//! - **Group 7** finds contacts 1 and 2 served, 24 against 24 each, and
//!   sends 18 of its 20 at contact 3.
//! - **Group 8** finds all three served. It does not launch and records
//!   nothing.
//!
//! Each sized launch's breakdown gives the aircraft able to fly, the share
//! held, the need, what had been sent and what it sent: 10, 0, 24, 0, 10
//! for group 1; 10, 0, 24, 20, 4 for group 3; 12, 0.25, 24, 0, 9 for group
//! 4; 10, 0, 24, 9, 10 for group 5; 10, 0, 24, 19, 5 for group 6; 20, 0,
//! 18, 0, 18 for group 7. Each is caused by its group's sizing order, with
//! what causes a launch today. Group 2's breakdown and causes are those of
//! scenario 46.
//!
//! In the air combat step there are three raids, one to each hex: 24
//! aircraft at (0,4), 24 at (3,0) and 18 at (-3,0). None finds a ship. Each
//! of the seven groups records that its strike found nothing. Contacts 1,
//! 2 and 3 are dropped as not there, each once, by the first group of its
//! raid: groups 1, 4 and 7.
//!
//! ## The afternoon
//!
//! The Allied side holds no contact. No group launches.
//!
//! ## At the end
//!
//! Seven strikes were launched, with 10, 10, 4, 9, 10, 5 and 18 aircraft:
//! 66 in all. Every group has the aircraft it began with, ready.
//!
//! # Properties
//!
//! - **The game as it was.** With sizing set on no group, every group
//!   launches all it can send at contact 1: 10, 10, 10, 9, 10, 10, 20 and
//!   10, 89 aircraft in one raid. The same with sizing on and no table of
//!   aircraft for each ship in the figures.
//! - **No figure, everything.** With no figure for a carrier, every group
//!   sends all it can at contact 1, which is never served: 89 aircraft in
//!   one raid, as with sizing off. With no figure for a transport and the
//!   others as given, groups 1 to 7 launch as in the main case and group 8
//!   sends its 10 at contact 3: 76 aircraft.
//! - **An unsized group overshoots.** With group 3's sizing off it sends 10
//!   at contact 1, 30 there in all, and every other launch is unchanged: 72
//!   aircraft.
//! - **A wrong report sends the wrong size.** With contact 1 reporting 2
//!   carriers its need is 48: groups 1, 2 and 3 send 10 each, group 4 sends
//!   9 and group 5 sends 9, all at contact 1; group 6 sends 10 and group 7
//!   sends 14 at contact 2; group 8 sends 10 at contact 3. 82 aircraft: 48,
//!   24 and 10.
//! - **Too few is still sent.** With group 1 alone at the base it sends 10
//!   at contact 1, and its breakdown gives a need of 24 and 10 sent.
//! - **A base strike is not sized.** With group 4 on a base strike at a
//!   Japanese base in its reach, it launches 9.
//! - **Aircraft left on the ground are not charged.** With the wear,
//!   fatigue and supply figures loaded, group 3 is charged for 4 sorties
//!   and group 8 for none.
//! - **The strike question** for a group before the morning is answered as
//!   the decision will be made, the groups before it deciding first:
//!   contact 1 and 4 aircraft for group 3, contact 3 and 18 for group 7,
//!   none for group 8.
//! - An order to set sizing on a group that is not the side's is refused.
//! - Every existing scenario's events and end states are what they were.
