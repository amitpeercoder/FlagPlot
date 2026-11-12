//! Scenario 13a: strike groups with target lists choose among three
//! contacts, and one holds its strike.
//!
//! Stage 1 of decision 0063. It proves the target list as an order: which
//! contact a group goes for, a group that holds, which force a raid attacks
//! and which ship each group attacks. Nobody opposes the strikes: there are
//! no fighters and no anti-aircraft fire.
//!
//! The data is placeholder data: see
//! `data/scenarios/13a_target_priority.toml`. The rule is
//! `docs/decisions/0063-strike-target-priority.md`.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. The tests below assert these
//! numbers. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass.
//!
//! ## The figures
//!
//! The frozen step 1 figures, with one changed for this test through
//! `common::altered_reference`: `air_combat.toml:
//! strike_contact_max_age_pulses` is 1, not 0. A contact a side starts the
//! game holding at age 0 is aged at the end of the night pulse, so it is 1
//! pulse old at the morning's decision. With the figure at 1 it can be
//! struck in the morning, and the morning needs no search and no random
//! report. Everything asserted exactly is of the morning pulse of day 1.
//!
//! ## The setting
//!
//! The map is `placeholder_search.toml`. One day.
//!
//! The Allied carrier force, task force 1, lies stopped at (4,0): carrier A,
//! ship 1 (class 4, 36 aircraft), and a destroyer, ship 2. Aboard carrier A
//! are six groups of 6 dive bombers each (type 3: radius 250 nm, a bomb of
//! damage 60), air groups 1 to 6, all ordered to strike ships.
//!
//! The Japanese forces all lie stopped, and have no aircraft:
//!
//! | Force | Hex    | Ships                                  | Air distance from (4,0) |
//! | ----- | ------ | -------------------------------------- | ----------------------- |
//! | 2     | (7,0)  | Cruiser, ship 3; destroyer, ship 4     | 120 nm                  |
//! | 3     | (7,3)  | Cruiser, ship 5; destroyer, ship 6     | 208 nm                  |
//! | 4     | (7,3)  | Cargo ship, ship 7                     | 208 nm                  |
//! | 5     | (9,-3) | Cruiser, ship 8; destroyer, ship 9     | 174 nm                  |
//!
//! The distances: to (7,0), 3 hexes along an axis, 120. To (7,3), dq 3 and
//! dr 3, 40 x the square root of (9 + 9 + 9) = 40 x 5.196152 = 207.85, to
//! the nearest mile 208. To (9,-3), dq 5 and dr -3, 40 x the square root of
//! (25 + 9 - 15) = 40 x 4.358899 = 174.36, 174.
//!
//! The Allied side starts holding three contacts, each at age 0, entered in
//! this order, so they are contacts 1, 2 and 3:
//!
//! | Contact | Hex    | Kind      | Ships | What is really there         |
//! | ------- | ------ | --------- | ----- | ---------------------------- |
//! | 1       | (7,0)  | cruiser   | 2     | Force 2                      |
//! | 2       | (7,3)  | transport | 3     | Forces 3 and 4               |
//! | 3       | (9,-3) | carrier   | 2     | Force 5, which has no carrier |
//!
//! The three hexes are 3, 3 and 6 hexes from one another, more than the
//! merge distance of 2, so each entry makes its own contact.
//!
//! ## The orders
//!
//! | Group | Target list             |
//! | ----- | ----------------------- |
//! | 1     | None given: the default |
//! | 2     | transport, cruiser      |
//! | 3     | battleship              |
//! | 4     | cruiser                 |
//! | 5     | destroyer, cruiser      |
//! | 6     | carrier, transport      |
//!
//! ## The morning's decisions
//!
//! All three contacts are 1 pulse old and within 250 nm. A group takes the
//! contact whose kind comes first on its list, then the nearest, then the
//! lowest number. A kind not on its list is never launched at.
//!
//! | Group | Contact taken | Why                                                        | Launch                 |
//! | ----- | ------------- | ---------------------------------------------------------- | ---------------------- |
//! | 1     | 3             | The default list has carrier first, as the rule was        | 6 to (9,-3), 174 nm    |
//! | 2     | 2             | Transport is first on its list, though contact 1 is nearer | 6 to (7,3), 208 nm     |
//! | 3     | None          | No contact is a battleship                                 | None. Nothing recorded |
//! | 4     | 1             | The only cruiser                                           | 6 to (7,0), 120 nm     |
//! | 5     | 1             | No contact is a destroyer; cruiser is next on its list     | 6 to (7,0), 120 nm     |
//! | 6     | 3             | Carrier is first on its list                               | 6 to (9,-3), 174 nm    |
//!
//! Five `StrikeLaunched` events, in group order, and none for group 3. Each
//! cites the group's mission order, the group's target order if it was
//! given one (so group 1's cites none), and the report that made its
//! contact.
//!
//! ## The raids
//!
//! Strikes from one place at one hex are one raid, so there are three, in
//! the order of their first groups: groups 1 and 6 at (9,-3); group 2 at
//! (7,3); groups 4 and 5 at (7,0).
//!
//! A group ranks ships by its list first, in the list's order, and then by
//! the kinds not on its list, in the order of importance. The raid attacks
//! the force at its hex that holds the best-ranked ship afloat by the list
//! of the raid's first group, the lowest force number on a tie. Each group
//! attacks the best-ranked ship afloat in that force by its own list, the
//! lowest ship number on a tie.
//!
//! | Raid at | Force attacked | Why                                                                 |
//! | ------- | -------------- | ------------------------------------------------------------------- |
//! | (9,-3)  | 5              | The only force there                                                |
//! | (7,3)   | 4              | Group 2's list has transport first, and force 4 holds the cargo ship. By the order of importance it would be force 3, for its cruiser |
//! | (7,0)   | 2              | The only force there                                                |
//!
//! | Group | Ship attacked | Why                                                                          |
//! | ----- | ------------- | ---------------------------------------------------------------------------- |
//! | 1     | 8, cruiser    | The most important ship of force 5                                           |
//! | 2     | 7, cargo ship | Transport is first on its list                                               |
//! | 4     | 3, cruiser    | Cruiser is its list                                                          |
//! | 5     | 4, destroyer  | Destroyer is first on its list. Two groups of one raid attack different ships |
//! | 6     | 8, cruiser    | Force 5 has no carrier and no transport. Off its list the order of importance gives battleship, then cruiser. A strike sent at the wrong kind attacks what it finds |
//!
//! Each attack is 6 shots at a bomb's chance of 0.2: 1.2 expected hits,
//! from 0 to 6. So five `AirAttack` events in group order, each with the
//! force and ship above, a bomb, shots 6, chance 0.2, expected 1.2.
//!
//! A hit does 100 x 60 / toughness points: 15 on a cruiser (400), 20 on the
//! cargo ship (300), 60 on a destroyer (100).
//!
//! No aircraft is lost: nothing fires at them. After the morning, damage is
//! on ships 3, 4, 7 and 8 only, and ships 5, 6 and 9 are untouched.
//!
//! Over seeds 1 to 1000, each group's morning hits in all are within four
//! standard deviations of 1,200. One run's hits have a standard deviation
//! of the square root of (6 x 0.2 x 0.8) = 0.979796, so 1,000 runs have
//! 30.98, and the allowed range is 1,077 to 1,323.
//!
//! ## With no target order
//!
//! The same scenario with the five target orders taken out is the rule as
//! it was. All six groups take contact 3, the carrier, and fly as one raid
//! of 36 to (9,-3). All attack ship 8: six `AirAttack` events of 6 shots
//! each, 7.2 expected hits in all. No `StrikeLaunched` cites a target
//! order. Ships 3 to 7 are untouched in the morning.
//!
//! ## The afternoon
//!
//! Not asserted exactly. A strike that attacked sends a report, which may
//! name a neighbouring hex or another kind, and by the afternoon the
//! contacts differ from seed to seed. The properties below hold in both
//! pulses.
//!
//! # Properties
//!
//! - Over seeds 1 to 1000 and both pulses, a group is never launched at a
//!   contact whose estimated kind, when it was launched, is not on its
//!   list. Group 3 is never launched.
//! - A group attacks a ship whose kind is off its list only when no ship
//!   afloat in the force attacked is of a kind on its list.
//! - Every group of a raid attacks the same force.
//! - Giving a group the default list by order changes nothing but the
//!   cause its launch cites: the same launches, attacks and hits, seed for
//!   seed, as with no order.
//! - A target order with an empty list, or with a kind named twice, is
//!   refused, and the group keeps the list it had.
//! - The aircraft a group flies never exceed its aircraft, and no track is
//!   above 100.
//! - A replay of any seed reproduces the daily hash.

mod common;
