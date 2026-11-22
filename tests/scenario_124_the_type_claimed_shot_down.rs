//! Scenario 124: the type of the aircraft claimed shot down.
//!
//! Proves decision 0210 against a hand calculation. It runs on the frozen
//! step 1 figures with chances altered, as scenarios 11 and 98 do, on
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
//! mornings Japanese group 1 (bombers, aircraft type 3) and group 2
//! (escorts, type 4) raid base 2; Allied group 3 (type 4) patrols over it
//! and is the only group on patrol there; the base's guns fire. The
//! firings, from scenario 11:
//!
//! | Day | Firing | The pool fired on | Really shot down | Missed |
//! | --- | ------ | ----------------- | ---------------- | ------ |
//! | 1 | escorts at the patrol | group 3, type 4 | 3 | 3 |
//! | 1 | patrol at the escorts | group 2, type 4 | 1 | 5 |
//! | 1 | patrol at the bombers | group 1, type 3 | 2 | 4 |
//! | 1 | base guns | group 1, type 3 | 0 | 20 |
//! | 2 | escorts at the patrol | group 3, type 4 | 2 | 4 |
//! | 2 | patrol at the escorts | group 2, type 4 | 1 | 5 |
//! | 2 | patrol at the bombers | group 1, type 3 | 1 | 2 |
//! | 2 | base guns | group 1, type 3 | 0 | 20 |
//! | 3 | escorts at the patrol | group 3, type 4 | 0 | 5 |
//! | 3 | patrol at the escorts | group 2, type 4 | 1 | 4 |
//! | 3 | patrol at the bombers | group 1, type 3 | 1 | 1 |
//! | 3 | base guns | group 1, type 3 | 3 | 17 |
//!
//! "Made out" is `raid_recognised_chance`; "false" is the three false
//! claim chances of scenario 11, all at the one setting. At false 0
//! nothing false is claimed; at false 1 every miss is.
//!
//! ## The Japanese claim's types in the air, each day
//!
//! Every aircraft it claims shot down is of the patrol, type 4.
//!
//! | Day | False 0, made out 1 | False 1, made out 1 | Made out 0, either |
//! | --- | ------------------- | ------------------- | ------------------ |
//! | 1 | type 4: 3 | type 4: 6 | no type |
//! | 2 | type 4: 2 | type 4: 6 | no type |
//! | 3 | none claimed, no type | type 4: 5 | no type |
//!
//! Its types on the ground are those of scenario 98, unchanged, and the
//! one draw decides both lists: on a day it names types in the air it
//! names its types on the ground, and on a day it names neither.
//!
//! ## The Allied claim's types in the air, each day
//!
//! The patrol's fire at the escorts names type 4; its fire at the bombers
//! and the guns' fire name type 3.
//!
//! | Day | False 0, made out 1 | False 1, made out 1 | Made out 0, either |
//! | --- | ------------------- | ------------------- | ------------------ |
//! | 1 | type 3: 2, type 4: 1 | type 3: 26, type 4: 6 | no type |
//! | 2 | type 3: 1, type 4: 1 | type 3: 23, type 4: 6 | no type |
//! | 3 | type 3: 4, type 4: 1 | type 3: 22, type 4: 5 | no type |
//!
//! Day 1 at false 1: at the bombers 2 and 4, and the guns 0 and 20, are
//! 26 of type 3; at the escorts 1 and 5 are 6 of type 4; 32 in all, the
//! claim's number. Day 2: 1 and 2 and 20 are 23; 1 and 5 are 6; 29. Day
//! 3: 1 and 1, and 3 and 17, are 22; 1 and 4 are 5; 27.
//!
//! ## The words
//!
//! Day 2, false 1 and made out 1. The Japanese claim: "6 aircraft shot
//! down (6 of aircraft_type:4), 15 aircraft destroyed on the ground (15
//! of aircraft_type:3)". The Allied claim: "29 aircraft shot down (23 of
//! aircraft_type:3, 6 of aircraft_type:4)". At made out 0 both are the
//! words of scenario 11's run, as scenario 98 has them.
//!
//! ## The type is that of the group fired on
//!
//! The same three mornings with group 2, the escorts, given aircraft type
//! 2 in the scenario's text. The fight is the same, shot for shot, which
//! is asserted. Day 1, false 1, made out 1: the Allied claim names type
//! 2: 6 and type 3: 26, and no type 4. The Japanese claim still names
//! type 4: 6, the patrol's.
//!
//! ## Guns name no escort
//!
//! The same three mornings with group 3 taken out of the scenario, so
//! that nothing patrols over base 2: the escorts fire at nothing and
//! nothing fires at them. At every setting, on every day, no Allied claim
//! names type 4, and every type an Allied claim names is type 3; no
//! Japanese claim names a type in the air, since it claims nothing shot
//! down.
//!
//! ## Firing by firing
//!
//! The function is decision 0179's, used once for each firing; scenario
//! 98's table of it stands. A claim's list is the sum of its firings'
//! lists, type by type:
//!
//! | Firing | Really shot down | False | The pool | Its list |
//! | --- | --- | --- | --- | --- |
//! | first | type 3: 2 | 3 | type 2: 6, type 3: 4 | type 2: 2, type 3: 3 |
//! | second | none | 4 | type 4: 5 | type 4: 4 |
//! | the claim | | | | type 2: 2, type 3: 3, type 4: 4 |
//!
//! # Properties
//!
//! - **A type named was fired on.** Over 200 seeds of the base raid at
//!   the placeholder false chances and made out 0.75: every type in a
//!   claim's list in the air is the type of a group in a pool one of the
//!   claimant's firings of that raid was at, read from the truth; each
//!   list is empty or adds to the aircraft claimed shot down.
//! - **One draw a claim, and the old draw is the old draw.** Over those
//!   seeds, with the chance at 0.75: the fights and every claim's numbers
//!   are those at made out 0, and each Japanese claim's types on the
//!   ground are what the code gave before this change at the same seeds,
//!   which the build records from the commit before and pins.
//! - **Both lists or neither.** Over those seeds, a claim that names
//!   types on the ground and claims aircraft shot down names types in the
//!   air, and one that names none on the ground where it could names none
//!   in the air.
//! - **A surface fight's claim and a bombardment's claim name no type.**
//! - **The other side is told nothing:** neither side's events hold the
//!   other's claim, with types or without.
//! - **A claim passed between commands keeps its types, and a picture
//!   keeps them.**
//! - **The hash names the field,** and two worlds that differ only in a
//!   claim's types in the air hash differently.
//! - **The frozen figures are as they were:** no figures file is changed,
//!   on them no claim names a type, and no breakdown of a claim is other
//!   than it was.

// The architect's statement lists are list items in the doc comment above,
// which is left as written.
#![allow(clippy::doc_lazy_continuation)]
