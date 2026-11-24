//! Scenario 136: supply at press. Scenario 76's island, asked for at
//! press. The staff lands the men with the supply that lasts until the
//! first run can follow, and the first run follows: it is ordered so that
//! it lands on the day the stock runs out, and nobody goes unfed.
//!
//! Proves decision 0230 against a hand calculation. It stands on scenario
//! 76's, whose tree, landing and supply sums it repeats with one command
//! more. The data is placeholder data: see
//! `data/scenarios/supply-at-press/136_supply_at_press.toml`. It runs on
//! the frozen step 1 figures with scenario 76's ground figures file. No
//! enemy man is ashore anywhere, so no die is rolled.
//!
//! **Written before the rule is built.** Every test below is empty and
//! marked `#[ignore = "statement written, not built"]`. The build fills
//! each to the statement above it and takes the mark off. Until then the
//! scenario file plays the game before the rule, which the section
//! "Before the rule" gives.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! The run of day 7 and every stock below were worked by hand and then
//! checked on the engine as it stood on Nov 24, 2026, before the rule, by
//! a copy of the scenario in which the commander himself states "keep
//! base 2 supplied" with a floor of 1 day on day 5, so that the hold forms
//! none: the supply doctrine served it and gave these figures. The numbers
//! of the intents and plans in the game with the rule, where the staff
//! forms the supply, could be checked on nothing.
//!
//! ## Inputs
//!
//! Scenario 76's, whole: A at (0,0), port size 5; B at (4,0), port size
//! 0, 160 nm from A; C at (0,6), port size 0. Base 1 at A, Allied, 1,000
//! t. Base 2 at B, Japanese, on the Allied map, empty. Base 3 at C,
//! Allied, 100 t. Task force 1, ships 1 and 2, placeholder troop
//! transports, at base 1. Unit 1, 600 men, at A; unit 3, 300 men, at C.
//! Island 1: base 2, then base 3. A man eats 0.005 t a day. The game is
//! 14 days.
//!
//! The Allied side's commands of day 1: intent 1, control island 1; the
//! risk of intent 1 set to press; both leaves given.
//!
//! ## The floor at press
//!
//! The margin of the check that a place can be held is 0 days at press,
//! as before. The floor of a "keep supplied" the staff forms is that
//! margin and never less than 1 day: 10 days at preserve the force, 5 at
//! balanced, 1 at press.
//!
//! ## The tree
//!
//! As scenario 76's, with the two supplies formed at a floor of 1 day:
//!
//! | Intent | Kind | Under | Formed |
//! | ------ | ---- | ----- | ------ |
//! | 1 | Control island 1, at press | none | the commander, day 1 |
//! | 2 | Take the place at base 2 | 1 | day 1 |
//! | 3 | Hold base 2 | 1 | day 1 |
//! | 4 | Hold base 3 | 1 | day 1 |
//! | 5 | Deny the waters within 3 hexes of (4,0) | 1 | day 1 |
//! | 6 | Neutralise base 2 | 1 | day 1; withdrawn on day 5 |
//! | 7 | Keep base 3 supplied, floor 1 day | 4 | day 1 |
//! | 8 | Ready a landing by task force 1 at B | 2 | day 1 |
//! | 9 | Keep base 2 supplied, floor 1 day | 3 | day 5 |
//!
//! ## The landing
//!
//! The check that the place can be held: the need is 600 x 0.005 = 3 t a
//! day; supply follows in 4 days; the margin at press is 0; the supply
//! that lands with the men is 3 x 4 = 12 t. The landing is scenario 76's
//! but for that weight:
//!
//! | Day | The staff's orders | What happens |
//! | --- | --- | --- |
//! | 1 | `prepare_landing` at B, roster 1 | |
//! | 2 | A load of supply to a weight of 12 t | Ship 1 loads 12 t |
//! | 3 | Unit 1 set to attack. Load unit 1 | |
//! | 4 | A route to B and back to A, acting at B, and an unload order | Ship 1 lands 160 men in the morning, 440 in the afternoon and then 10 t. Base 4, "Landing at (4,0)", is formed. Base 2 changes side at the end of the day |
//! | 5 | The take's plan is marked complete. Unit 1 set to defend | Ship 1 lands 2 t at base 2 in the night. The force steams for A |
//! | 6 | | The force arrives at A in the night |
//!
//! ## The run that follows
//!
//! Intent 9 is formed on the morning of day 5 and first served on day 6.
//! The use at B is 3 t a day and the floor's stock 1 x 3 = 3 t. The stock
//! in the hex is that of bases 2 and 4 together.
//!
//! - **Day 6.** The stock is 9 t (2 t and 7 t). A run ordered today would
//!   be due on day 7, and the reading checked is day 8's: 9 - 3 - 3 = 3
//!   t, not below 3 t. No run is needed. (Task force 1 is at sea this
//!   morning; that binds nothing, since none is needed.)
//! - **Day 7.** The stock is 6 t. A run ordered today is due on day 8,
//!   and the reading checked is day 9's: 6 - 3 - 3 = 0 t, below 3 t. A
//!   run is needed. Its load is the floor's stock on the day it arrives,
//!   3 t, less the reading of that day, 3 t, which is 0, plus 15 days of
//!   use: 45 t. The staff adds run 1 of intent 9's plan, 45 t, loading
//!   day 7 and due day 8, commits both ships from day 7 to day 10, and
//!   orders a load of supply to a weight of 45 t. Ship 1 loads 45 t that
//!   night.
//! - **Day 8.** A route to B and back to A, acting at B, and an unload
//!   order. Ship 1 lands 40 t at base 2 in the morning and 5 t in the
//!   afternoon.
//! - **No second run.** One is next needed on the first morning the
//!   stock is below 9 t. It is 30 t on the morning of day 14.
//!
//! Ten orders in all: seven of the landing and three of the run. No staff
//! order or command is refused.
//!
//! ## Supply, at the end of each day
//!
//! | Day | Base 1 | Base 2 | Base 4 | Base 3 |
//! | --- | ------ | ------ | ------ | ------ |
//! | 1 | 997 | 0 | | 98.5 |
//! | 2 | 982 | 0 | | 97 |
//! | 3 | 979 | 0 | | 95.5 |
//! | 4 | 979 | 0 | 10 | 94 |
//! | 5 | 979 | 2 | 7 | 92.5 |
//! | 6 | 979 | 0 | 6 | 91 |
//! | 7 | 934 | 0 | 3 | 89.5 |
//! | 8 | 934 | 45 | 0 | 88 |
//! | 9 | 934 | 42 | 0 | 86.5 |
//! | 14 | 934 | 27 | 0 | 79 |
//!
//! Base 1 feeds unit 1 on the nights of days 1 to 3 and loads out 12 t
//! and 45 t: 1,000 - 9 - 57 = 934 t. At B the night's charge is paid by
//! base 2 and then base 4: on the night of day 5 base 2 holds nothing and
//! base 4 pays 3 t; on the night of day 6 base 2 pays its 2 t and base 4
//! 1 t; on the nights of days 7 and 8 base 4 pays 3 t and is empty. The
//! run lands on day 8, after that night's charge. Base 2 pays 3 t on the
//! nights of days 9 to 14: 45 - 18 = 27 t.
//!
//! ## The men
//!
//! Every man is fed on every night. Unit 1 has 600 men at the end and
//! unit 3 has 300. No `MenLostUnfed` is recorded in the game.
//!
//! ## Before the rule
//!
//! For the record, and not asserted: the game the scenario file plays on
//! the engine before decision 0230. The two supplies are formed with a
//! floor of 0 days; no run is ever needed, since no reading is below 0 t;
//! bases 2 and 4 are empty from the end of day 8; on the night of day 9
//! all 600 men are unfed, and unit 1 loses 30, 28, 27, 25, 24 and 23 men
//! on the nights of days 9 to 14: 157 lost, 443 left. Base 1 ends at 979
//! t. Seven orders.
//!
//! # Properties
//!
//! - **Never a night short.** With unit 1 of 500, 550, 650 and 700 men in
//!   turn (so that the stock at B is no whole number of days of use): in
//!   each game no `MenLostUnfed` is recorded and every `MenFed` at B has
//!   no man unfed. Only that is asserted.
//! - **At balanced and at preserve nothing moves.** With the risk command
//!   left out, the game is scenario 76's main case: its ten orders on
//!   their days, the supplies formed with a floor of 5 days, and 919 t,
//!   35 t and 79 t at bases 1, 2 and 3 at the end. With the risk set to
//!   preserve, the supplies are formed with a floor of 10 days.
//! - **The commander's own floor is his.** With the commander's own
//!   intent "keep base 3 supplied, floor 3 days", with a forces limit of
//!   task force 1 and base 1, stated on day 1 beside intent 1: the staff
//!   forms no supply of base 3, and the commander's keeps its floor of 3
//!   days.
//! - **The same twice.** The main case played twice gives the same daily
//!   hashes.
#![allow(clippy::doc_lazy_continuation, clippy::doc_markdown)]

/// The tree: the intents of the table, and the floors of intents 7 and 9,
/// each 1 day.
#[test]
#[ignore = "statement written, not built"]
fn the_tree_and_the_floor_at_press() {}

/// The landing: the seven orders and their days; 12 t loaded on day 2; the
/// men and the 10 t landed on day 4 and the 2 t on day 5.
#[test]
#[ignore = "statement written, not built"]
fn the_landing_at_press() {}

/// The run that follows: none needed on day 6; run 1 of 45 t added on day
/// 7, loading day 7 and due day 8; its three orders; 40 t and 5 t landed
/// at base 2 on day 8; no second run.
#[test]
#[ignore = "statement written, not built"]
fn the_run_that_follows() {}

/// Supply at the end of each day at the four Allied bases, as the table
/// has it and at every day between.
#[test]
#[ignore = "statement written, not built"]
fn supply_at_the_end_of_each_day() {}

/// The men: no man unfed on any night, 600 and 300 at the end, and no
/// `MenLostUnfed` in the game.
#[test]
#[ignore = "statement written, not built"]
fn every_man_is_fed() {}

/// Property: never a night short.
#[test]
#[ignore = "statement written, not built"]
fn never_a_night_short() {}

/// Property: at balanced and at preserve nothing moves.
#[test]
#[ignore = "statement written, not built"]
fn at_balanced_and_at_preserve_nothing_moves() {}

/// Property: the commander's own floor is his.
#[test]
#[ignore = "statement written, not built"]
fn the_commanders_own_floor_is_his() {}

/// Property: the same twice.
#[test]
#[ignore = "statement written, not built"]
fn the_same_twice() {}
