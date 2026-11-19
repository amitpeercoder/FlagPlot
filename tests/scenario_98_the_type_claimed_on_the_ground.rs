//! Scenario 98: the type of the aircraft claimed destroyed on the ground.
//!
//! Proves decision 0179 against a hand calculation. It runs on the frozen
//! step 1 figures with chances altered, as scenario 11's test does, on
//! `06a_base_raid.toml`, which is placeholder data.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Inputs
//!
//! The base raid of scenario 11's hand calculation (see it): on three
//! mornings Japanese groups 1 and 2 raid base 2. The aircraft on the
//! ground there are group 4's alone, of aircraft type 3: 10 at the start
//! of day 1 and of day 2, and 8 at the start of day 3. Groups 3 and 5 are
//! in the air and have no damaged aircraft.
//!
//! What the bombs really destroy on the ground: day 1, none; day 2, 2 of
//! group 4; day 3, none. What is claimed on the ground with the false
//! claim chance of bombs at 0: 0, 2 and 0; at 1: 16, 15 and 11.
//!
//! ## The Japanese claim's types, each day
//!
//! "Made out" is `raid_recognised_chance`, the one chance of the rule;
//! "false" is the three false claim chances of scenario 11, all at the
//! one setting. The field each day is type 3 alone.
//!
//! | Day | False 0, made out 1 | False 1, made out 1 | Made out 0, either |
//! | --- | ------------------- | ------------------- | ------------------ |
//! | 1 | none claimed, no type | type 3: 16 | no type |
//! | 2 | type 3: 2 | type 3: 15 | no type |
//! | 3 | none claimed, no type | type 3: 11 | no type |
//!
//! Day 2 at false 1: the 2 really destroyed are of type 3, group 4's;
//! the 13 false are shared over the field, which is type 3 alone. On
//! days 1 and 3 every claimed aircraft is false and of type 3.
//!
//! The Allied claims, of aircraft shot down, name no type at any setting.
//!
//! The claim's words, day 2, false 1 and made out 1: they hold "15
//! aircraft destroyed on the ground" and name type 3 as decision 0167
//! names a type, with 15 against it. At made out 0 the words are those
//! of scenario 11's run, to the letter but for the clause at their end
//! that names the units (decision 0180, built with this record).
//!
//! ## The type is the group's on the ground, not the bombers'
//!
//! The same three mornings with group 4 given aircraft type 2 in the
//! scenario's text (the raiding bombers, group 1, are type 3). The fight
//! is the same, shot for shot, which is asserted. Day 2, made out 1: at
//! false 0 the claim names type 2: 2; at false 1, type 2: 15. No claim
//! names type 3.
//!
//! ## Nothing else moves
//!
//! At each setting of the false chances, between made out 0 and made out
//! 1: every claim's numbers are the same, and the event logs are the same
//! but for the Japanese claims' types and words and the events that
//! decision 0167's recognition records. With made out 0 the logs are
//! those of scenario 11's variants as they run today, but for that
//! clause.
//!
//! ## The rule's own function
//!
//! Given the aircraft really destroyed by type, the number claimed
//! falsely, and the field by type; the list when the types are made out:
//!
//! | Really destroyed | False | The field | The false, shared | The list |
//! | --- | --- | --- | --- | --- |
//! | type 3: 2 | 13 | type 3: 10 | type 3: 13 | type 3: 15 |
//! | type 3: 2 | 3 | type 2: 6, type 3: 4 | 1.8 and 1.2: 1 and 1, the one left to type 2 | type 2: 2, type 3: 3 |
//! | none | 4 | type 2: 6, type 3: 6 | 2 and 2 | type 2: 2, type 3: 2 |
//! | none | 5 | types 2, 3 and 4: 1 each | 1.67 each: 1 each, the two left to types 2 and 3 | type 2: 2, type 3: 2, type 4: 1 |
//! | type 3: 2 | 1 | type 2: 9, type 3: 1 | 0.9 and 0.1: 0 and 0, the one left to type 2 | type 2: 1, type 3: 2 |
//! | type 4: 1 | 0 | type 4: 1 | none | type 4: 1 |
//! | none | 5 | empty | none | empty, and no draw is made |
//!
//! When the types are not made out the list is empty in every row. The
//! list is in order of the type's number and holds no pair with 0; where
//! it is not empty its numbers add to the aircraft claimed.
//!
//! # Properties
//!
//! - **A bombardment's claim names no type,** at made out 1: scenario
//!   24's bombardment claim, which is of aircraft on the ground, has an
//!   empty list.
//! - **A type named was on the field.** Over 200 seeds of the base raid
//!   at the placeholder false chances and made out 0.75: every type in a
//!   Japanese claim is the type of a group that had an aircraft on the
//!   ground at base 2 at the start of that pulse, read from the truth;
//!   and each list is empty or adds to the aircraft claimed on the
//!   ground.
//! - **One draw a claim.** With the chance at 0.75, the fights and every
//!   claim's numbers over those seeds are those at made out 0.
//! - **The other side is told nothing:** the Allied side's events hold no
//!   Japanese claim, with types or without.
//! - **A claim passed between commands keeps its types** (decision 0135),
//!   and a picture keeps them (decision 0170): a world rebuilt from the
//!   picture after day 2 holds the same claim.
//! - **The hash names the field:** `hash::name_every_field` lists it, and
//!   two worlds that differ only in a claim's types hash differently.
//! - **The frozen figures are as they were:** no figures file is changed,
//!   and on them no claim names a type.

// The tests are written by the build, to the statements above.
