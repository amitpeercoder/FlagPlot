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
//! "Made out" is `claim_type_made_out_chance`; "false" is the three false
//! claim chances of scenario 11, all at the one setting.
//!
//! | Day | False 0, made out 1 | False 1, made out 1 | Made out 0, either |
//! | --- | ------------------- | ------------------- | ------------------ |
//! | 1 | none claimed, no type | type 3: 16 | no type |
//! | 2 | type 3: 2 | type 3: 15 | no type |
//! | 3 | none claimed, no type | type 3: 11 | no type |
//!
//! Day 2 at false 1: the 2 really destroyed are of type 3, their own; the
//! 13 false are of type 3, the most numerous type on the field (the only
//! one, 10 aircraft). On days 1 and 3 every claimed aircraft is false and
//! of type 3 by the same rule.
//!
//! The Allied claims, of aircraft shot down, name no type at any setting.
//!
//! The claim's words, day 2, false 1 and made out 1: they hold "15
//! aircraft destroyed on the ground" and name type 3 as decision 0167
//! names a type, with 15 against it, and nothing "not made out". At made
//! out 0 the words are those of scenario 11's run, to the letter.
//!
//! ## Nothing else moves
//!
//! At each setting of the false chances, between made out 0 and made out
//! 1: every claim's numbers are the same, and the event logs are the same
//! but for the Japanese claims' types and words. With made out 0 the logs
//! are the same as with the figure absent from the file (the frozen
//! figures as they stand, where it is 0).
//!
//! ## The rule's own function
//!
//! Given the aircraft really destroyed by type, the number claimed
//! falsely, and the field by type; with the chance at 1, then at 0:
//!
//! | Really destroyed | False | The field | At 1 | At 0 |
//! | --- | --- | --- | --- | --- |
//! | type 3: 2 | 13 | type 3: 10 | type 3: 15 | none |
//! | type 3: 2 | 3 | type 2: 6, type 3: 4 | type 2: 3, type 3: 2 | none |
//! | none | 4 | type 2: 6, type 3: 6 | type 2: 4 (a tie goes to the lower number) | none |
//! | none | 5 | empty | none, 5 not made out | none |
//! | type 4: 1 | 0 | type 4: 1 | type 4: 1 | none |
//!
//! The list is in order of the type's number and holds no pair with 0.
//!
//! # Properties
//!
//! - **A bombardment's claim names no type,** at made out 1: scenario
//!   24's bombardment claim, which is of aircraft on the ground, has an
//!   empty list.
//! - **A type named was on the field.** Over 200 seeds of the base raid
//!   at the placeholder false chances and made out 0.75: every type in a
//!   Japanese claim is type 3, and the numbers named add to no more than
//!   the aircraft claimed on the ground.
//! - **The other side is told nothing:** the Allied side's events hold no
//!   Japanese claim, with types or without.
//! - **A claim passed between commands keeps its types** (decision 0135),
//!   and a picture and a saved game keep them (decision 0170): a world
//!   rebuilt from the picture after day 2 holds the same claim.
//! - **The hash names the field:** `hash::name_every_field` lists it, and
//!   two worlds that differ only in a claim's types hash differently.

// The tests are written by the build, to the statements above.
