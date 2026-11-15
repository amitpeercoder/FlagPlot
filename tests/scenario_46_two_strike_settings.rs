//! Scenario 46: two settings of a strike group. Three groups at one base
//! differ only in the oldest contact they launch on; three at a second base
//! differ only in the share of their aircraft they hold back.
//!
//! Proves decision 0114 against a hand calculation. The data is placeholder
//! data: see `data/scenarios/46_two_strike_settings.toml`. It runs on the
//! frozen step 1 figures. There is no enemy unit at all: every strike flies
//! to a contact's hex, finds nothing there and rolls no die.
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
//! `strike_contact_max_age_pulses` 0. `contact_drop_age_pulses` 5.
//! `merge_distance_hexes` 2. The placeholder dive bomber (type 3): a strike
//! radius of 250 nm, a bomb. A hex is 40 nm across. A day is a night pulse
//! and two daylight pulses, morning and afternoon. At the end of every
//! pulse a contact not made or refreshed in it gains one pulse of age.
//! Strikes are decided in a daylight pulse, in group number order, on the
//! side's contacts as they stand. The upkeep, wear, fatigue and supply
//! figures are not loaded.
//!
//! ## Inputs
//!
//! A placeholder map of open sea with two Allied bases: base 1 at (0,0) and
//! base 2 at (20,0), 800 nm apart. The game is 1 day. There is no Japanese
//! unit.
//!
//! Six Allied groups of placeholder dive bombers, all on the naval strike
//! mission, with the default target list, on normal reach:
//!
//! | Group | Base | Ready aircraft | Oldest contact it launches on | Share held back |
//! | ----- | ---- | -------------- | ----------------------------- | --------------- |
//! | 1     | 1    | 10             | not set (the figure, 0)       | not set (0)     |
//! | 2     | 1    | 10             | 1                             | not set         |
//! | 3     | 1    | 10             | 3                             | not set         |
//! | 4     | 2    | 12             | 1                             | 0.333333        |
//! | 5     | 2    | 10             | 1                             | 0.25            |
//! | 6     | 2    | 10             | 1                             | not set         |
//!
//! The Allied side starts holding three contacts, in this order:
//!
//! | Contact | Hex    | Ships | Kind      | Age at the start | From base 1 | From base 2 |
//! | ------- | ------ | ----- | --------- | ---------------- | ----------- | ----------- |
//! | 1       | (3,0)  | 3     | transport | 0                | 120 nm      | 680 nm      |
//! | 2       | (0,4)  | 1     | carrier   | 2                | 160 nm      | out of reach |
//! | 3       | (22,0) | 2     | cruiser   | 0                | 880 nm      | 80 nm       |
//!
//! There is no order.
//!
//! ## The night
//!
//! Nothing is decided at night. At its end each contact gains a pulse:
//! contact 1 is 1 pulse old, contact 2 is 3, contact 3 is 1.
//!
//! ## The morning
//!
//! At base 1, in group order:
//!
//! - **Group 1** launches on a contact no older than 0. Contacts 1 and 2
//!   are in its reach, at ages 1 and 3. Neither qualifies. It does nothing
//!   and records nothing.
//! - **Group 2** launches on a contact no older than 1. Contact 1 qualifies
//!   and contact 2, at 3, does not. It launches its 10 aircraft at contact
//!   1, to (3,0), 120 nm.
//! - **Group 3** launches on a contact no older than 3. Both qualify. The
//!   carrier comes before the transport on its list: it launches its 10
//!   aircraft at contact 2, to (0,4), 160 nm.
//!
//! At base 2 only contact 3 is in reach, at age 1, and all three groups
//! launch on a contact no older than 1:
//!
//! - **Group 4** is able to fly 12 and holds back 0.333333: 12 x 0.666667 =
//!   8.000004, cut to 8. It launches 8 at contact 3, to (22,0), 80 nm.
//! - **Group 5** is able to fly 10 and holds back 0.25: 10 x 0.75 = 7.5,
//!   cut to 7. It launches 7.
//! - **Group 6** holds none back and launches 10.
//!
//! Each launch's breakdown gives the aircraft able to fly, the share held
//! and the aircraft sent: 10, 0, 10 for groups 2, 3 and 6; 12, 0.333333, 8
//! for group 4; 10, 0.25, 7 for group 5.
//!
//! In the air combat step there are three raids, one to each hex, and none
//! finds a ship. Each of the five groups records that its strike found
//! nothing. Contacts 1, 2 and 3 are dropped as not there, each once: for
//! contact 3 by group 4's strike, the first of its raid.
//!
//! ## The afternoon
//!
//! The Allied side holds no contact. No group launches and nothing is
//! recorded for any.
//!
//! ## At the end
//!
//! Five strikes were launched, with 10, 10, 8, 7 and 10 aircraft: 45 in
//! all. Group 1 launched none. Every group has the aircraft it began with,
//! ready: 10, 10, 10, 12, 10 and 10. The Allied side holds no contact.
//!
//! # Properties
//!
//! - With no setting on any group, no group launches on any pulse of the
//!   day: every contact is 1 pulse old or more in the morning. That is the
//!   game as it was before this rule.
//! - With every group's age set to 4, groups 1, 2 and 3 all launch at
//!   contact 2, the carrier, in the morning, 30 aircraft in one raid.
//! - An order for an age of 5, or of a negative number, is refused and
//!   names the range 0 to 4. An order for a share above 1 or below 0 is
//!   refused.
//! - A group that holds back all it has, a share of 1, does not launch and
//!   records nothing; so does a group able to fly 1 with a share of 0.5.
//! - With group 4 on a base strike at a Japanese base in its reach, it
//!   launches 8 of its 12.
//! - The held aircraft are not charged: with the wear, fatigue and supply
//!   figures loaded, group 4 is charged for 8 sorties and not for 12.
//! - The answer to the strike question for groups 1, 2 and 3 before the
//!   morning is none, contact 1 and contact 2.
//! - Every existing scenario's events and end states are what they were.
