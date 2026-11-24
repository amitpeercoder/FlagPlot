//! Scenario 135: men ashore beside a place. The commander says "control
//! the island" and nothing else. The island has one place, which the side
//! holds. One hex from it by a land link the side has a dump with 1,000
//! men and four days of food. On land joined to no place of the island it
//! has the twin of that dump. The staff feeds the men beside the place by
//! a run from the island's source, and does nothing for the twin.
//!
//! Proves decision 0229 as amended on review, against a hand calculation.
//! It stands on scenario 76's for an island's tree, its source of supply
//! and the supply doctrine's sums. The data is placeholder data: see
//! `data/scenarios/men-beside-a-place/135_men_beside_a_place.toml`. It
//! runs on the frozen step 1 figures with scenario 76's ground figures
//! file. No enemy man or ship is in reach of anything, so no die is
//! rolled.
//!
//! **Written before the rule is built.** Every test below is empty and
//! marked `#[ignore = "statement written, not built"]`. The build fills
//! each to the statement above it and takes the mark off. Until then the
//! scenario file plays, on the engine as it stands, the game "without the
//! rule": the men at D are lost as the men at T are.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! The supply doctrine's part of it (the day of the run, its load, its
//! hours and the stocks at D) was worked by hand and then checked on the
//! engine as it stood on Nov 24, 2026, before the rule existed, by a copy
//! of the scenario in which D is itself the island's place, so that the
//! supply of D is formed under a hold: the doctrine that serves the two is
//! one, and it gave these figures. The numbers of the intents and of the
//! plans, and everything about when the new intent is formed and
//! withdrawn, are the rule's own and were checked on nothing.
//!
//! ## Inputs
//!
//! The scenario's own map. A at (0,0), port size 5. D at (4,0), P at (5,0)
//! and N at (6,0), port size 0; land links join D to P and P to N. T at
//! (0,4) and J at (6,4), port size 0, joined to nothing. A to D is 160 nm
//! and A to T is 160 nm. The game is 14 days.
//!
//! | Base | Side | Hex | Supply |
//! | ---- | ---- | --- | ------ |
//! | 1, "Placeholder source A" | Allied | A | 1,000 t |
//! | 2, "Placeholder anchorage P" | Allied | P | 100 t |
//! | 3, "Placeholder dump D" | Allied | D | 20 t |
//! | 4, "Placeholder dump T" | Allied | T | 20 t |
//! | 5, "Placeholder shore J" | Japanese | J | 0 |
//!
//! Base 5 is not on the Allied map. No base has a reserve or fuel. The
//! Allied side holds no estimate and no contact.
//!
//! Island 1, "Placeholder island": base 2. Its land is P, D and N.
//!
//! Task force 1, Allied, on the transport mission: ships 1 and 2,
//! placeholder troop transports (10 kn, 3,000 t of cargo each), stopped
//! at base 1.
//!
//! | Unit | Where | Men |
//! | ---- | ----- | --- |
//! | 1, "Placeholder Beside" | D | 1,000 |
//! | 2, "Placeholder Garrison" | P | 300 |
//! | 3, "Placeholder Twin" | T | 1,000 |
//!
//! All are Allied fighting units, on the defence, experience 2, every
//! skill 2. A man eats 0.005 t a day; of the unfed one in twenty is lost
//! each night, cut to a whole man.
//!
//! The Allied side's commands of day 1: intent 1, control island 1. No
//! forces limit and no risk, so balanced. Both leaves are given.
//!
//! ## The tree on the first morning
//!
//! | Intent | Kind | Under | Made by |
//! | ------ | ---- | ----- | ------- |
//! | 1 | Control island 1 | none | the commander |
//! | 2 | Hold base 2 | 1 | the staff |
//! | 3 | Keep base 3 supplied, floor 5 days, no interval | 1 | the staff |
//! | 4 | Deny the waters within 3 hexes of (5,0) | 1 | the staff |
//! | 5 | Keep base 2 supplied, floor 5 days, no interval | 2 | the staff |
//!
//! The place is the side's, so no take is formed; no enemy base is on the
//! side's map, so nothing is neutralised. The source of supply is base 1,
//! which has the most supply of the side's bases in no named place's hex
//! (1,000 t against 20 t at base 3 and 20 t at base 4), with task force 1
//! stopped there and nothing aboard. The hold's forces limit is set to
//! base 1 and task force 1, as today. **Intent 3 is the rule's:** base 3
//! is on the island's land, in no named place's hex, and unit 1 is ashore
//! in its hex. It is formed after the hold and before the denial, with a
//! plan of its own, plan 2, and the staff sets its forces limit to base 1
//! and task force 1 that morning. Intent 5 then has plan 3. No intent
//! names base 4: T is joined to no place of the island. The denial finds
//! no force and no air group, forms nothing and hands in its notice each
//! morning, as it does without the rule.
//!
//! ## The run to D
//!
//! The supply doctrine is asked before the island's tree is formed, so
//! intent 3, formed on the morning of day 1, is first served on day 2.
//!
//! - **Day 2.** Plan 2 is approved and released on the leaves. The use at
//!   D is 1,000 x 0.005 = 5 t a day and the floor's stock 5 x 5 = 25 t.
//!   The stock at D is 15 t this morning. A run ordered today is due on
//!   day 3, and the reading checked is day 4's: 15 - 5 - 5 = 5 t, below
//!   25 t. A run is needed. Its load is the floor's stock on the day it
//!   arrives less the reading of that day, 25 - 10 = 15 t, plus 15 days
//!   of use, 75 t: 90 t. Ship 1's room covers it; the force goes whole.
//!   The staff adds run 1 of plan 2, 90 t, loading day 2 and due day 3,
//!   commits both ships from day 2 to day 5, and orders a load of supply
//!   to a weight of 90 t. Ship 1 loads 90 t that night.
//! - **Day 3.** The staff orders a route to D and back to A, acting at D,
//!   and an unload order. The force sails at hour 48 and arrives at hour
//!   64. Ship 1 lands 40 t in the morning and 50 t in the afternoon.
//! - **Day 4.** The force steams for A and lies stopped there at the end
//!   of the day.
//! - **No second run.** A run is next needed on the first morning the
//!   stock at D is below 35 t (25 t and two days' use). The stock on the
//!   morning of day 14 is 45 t.
//!
//! Three orders in all, each the staff's and each serving intent 3. No
//! staff order or command is refused.
//!
//! ## Supply, at the end of each day
//!
//! | Day | Base 1 | Base 2 | Base 3 (D) | Base 4 (T) |
//! | --- | ------ | ------ | ---------- | ---------- |
//! | 1 | 1,000 | 98.5 | 15 | 15 |
//! | 2 | 910 | 97 | 10 | 10 |
//! | 3 | 910 | 95.5 | 95 | 5 |
//! | 4 | 910 | 94 | 90 | 0 |
//! | 5 | 910 | 92.5 | 85 | 0 |
//! | 14 | 910 | 79 | 40 | 0 |
//!
//! Base 3 pays 5 t each night; on day 3 it pays 5 t in the night and takes
//! 90 t by day: 5 + 90 = 95 t; then 95 - 5 x 11 = 40 t at the end. Base 2
//! pays 300 x 0.005 = 1.5 t each night: 100 - 21 = 79 t, and its supply,
//! intent 5, needs no run on any day. Base 1 feeds no one and loads out
//! 90 t.
//!
//! ## The men
//!
//! - **Unit 1, at D,** is fed in full every night. It loses no man: 1,000
//!   at the end.
//! - **Unit 3, at T,** is the game without the rule. Base 4 feeds it on
//!   the nights of days 1 to 4 and is then empty. From the night of day 5
//!   every man is unfed, and one in twenty is lost each night, cut:
//!
//!   | Night of day | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 |
//!   | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
//!   | Lost | 50 | 47 | 45 | 42 | 40 | 38 | 36 | 35 | 33 | 31 |
//!   | Left | 950 | 903 | 858 | 816 | 776 | 738 | 702 | 667 | 634 | 603 |
//!
//!   397 men lost; 603 at the end. On the engine as it stood before the
//!   rule, unit 1 had these same figures.
//! - **Unit 2, at P,** is fed in full: 300 at the end.
//!
//! ## The measure, and what the plan order says
//!
//! The island's one place is the side's and holds more than 5 days of
//! supply on every day: the island is controlled on 14 of 14 days. Base 3
//! is no part of the measure. The plan order of intent 1 names base 3
//! among the bases fed beside its places, with its hex and the place it
//! is beside, and does not name base 4.
//!
//! # Properties
//!
//! - **It ends when the men go.** With a scenario order of day 6 for unit
//!   1 to march to P: the unit marches on days 6, 7 and 8 and is at D,
//!   fed there, until it arrives on day 8. It carries 5 days of its
//!   charge, 5 x 5 = 25 t, taken from base 3 and put into base 2. Base 3
//!   holds 80 t at the end of day 6, 75 t at the end of day 7, and on day
//!   8 pays 5 t in the night and gives 25 t to the march: 45 t, which it
//!   keeps to the end. Base 2 holds 100 - 1.5 x 8 + 25 = 113 t at the end
//!   of day 8 and then pays 1,300 x 0.005 = 6.5 t a night: 74 t at the
//!   end. On the mornings of days 7 and 8 intent 3 stands. On the morning
//!   of day 9 no man of the side is at D, none marches to it and no run
//!   of plan 2 is loaded or at sea: the staff withdraws intent 3, and plan
//!   2 is complete. No intent names base 3 after, and no run is added.
//!   The orders are the main case's three and the scenario's one.
//! - **Gone and come back.** With that order, and a scenario order of day
//!   9 for unit 1 to march to D: intent 3 is withdrawn on the morning of
//!   day 9, as above, since the day's orders are taken in after the staff
//!   is asked. On the morning of day 10 unit 1 is marching to D: the staff
//!   forms a "keep base 3 supplied" under intent 1 again, intent 6, with
//!   plan 4, and sets its forces limit. The unit arrives on day 11 with
//!   25 t taken from base 2. Base 3 holds 45 + 25 = 70 t at the end of
//!   day 11 and pays 5 t on the nights of days 12 to 14: 55 t at the end.
//!   Base 2 holds 113 - 6.5 x 3 - 25 = 68.5 t at the end of day 11 and
//!   64 t at the end. Intent 6 needs no run: on the morning of day 12 the
//!   stock at D is 70 t and the reading checked 60 t, and on the morning
//!   of day 14 60 t and 50 t, against 25 t. No man is lost.
//! - **Nobody there.** With unit 1 left out: no intent names base 3 on
//!   any day, the staff gives no order in the game, and base 3 holds 20 t
//!   at the end.
//! - **A hex with no base.** With a fourth unit, "Placeholder Bare", 200
//!   Allied men ashore at N: no intent names N; on each morning the staff
//!   records one reason for giving no order that names N, the unit and
//!   its 200 men (the men it then has), and says that no base of the side
//!   stands there to feed them through. The unit loses 10 men on the
//!   night of day 1 and is unfed every night. Everything of the main case
//!   at D, T and P is as written above.
//! - **More at the source is never worse.** With base 1 holding 2,000 t:
//!   base 3 holds at the end of each day what it holds in the main case.
//! - **At press.** With intent 1's risk set to press on day 1: intent 3
//!   is formed with a floor of 0 days and is not refused. Only the
//!   forming is asserted.
//! - **What the side cannot see.** With base 5 holding 500 t and a
//!   Japanese unit of 400 men ashore at J: the Allied staff's commands,
//!   orders and reasons on every day are those of the main case, word for
//!   word.
//! - **The same twice.** The main case played twice gives the same daily
//!   hashes.
//!
//! # The intake's three changes
//!
//! Tested in `sim-core`'s own command tests, each way (decision 0229,
//! amended 1): a "keep supplied" of a base of the side is taken under
//! "control an island", and of a base of the enemy's is refused as
//! `CannotStandUnder`; so formed it has a plan of its own; and a floor of
//! 0 days is taken under "control an island" and still refused for a
//! "keep supplied" that stands alone.
#![allow(clippy::doc_lazy_continuation, clippy::doc_markdown)]

/// The tree on the first morning: intents 2 to 5 as the table has them,
/// plans 2 and 3, intent 3's forces limit, and no intent that names base 4.
#[test]
#[ignore = "statement written, not built"]
fn the_tree_on_the_first_morning() {}

/// The run to D: served first on day 2; run 1 of plan 2, 90 t, loading day
/// 2 and due day 3; the three orders, their days and what they serve; the
/// hours of sailing and arrival; 40 t landed in the morning and 50 t in the
/// afternoon of day 3; the force at A at the end of day 4; no second run.
#[test]
#[ignore = "statement written, not built"]
fn the_run_to_d() {}

/// Supply at the end of each day, at the four Allied bases, as the table
/// has it and at every day between.
#[test]
#[ignore = "statement written, not built"]
fn supply_at_the_end_of_each_day() {}

/// The men: unit 1 loses none; unit 3 loses the men of the table, night by
/// night, 603 left; unit 2 loses none.
#[test]
#[ignore = "statement written, not built"]
fn the_men_beside_the_place_are_fed_and_the_twin_is_not() {}

/// The measure is 14 of 14 days, and the plan order of intent 1 names base
/// 3 as fed beside the place and not base 4.
#[test]
#[ignore = "statement written, not built"]
fn the_measure_and_the_plan_order() {}

/// Property: it ends when the men go.
#[test]
#[ignore = "statement written, not built"]
fn it_ends_when_the_men_go() {}

/// Property: gone and come back.
#[test]
#[ignore = "statement written, not built"]
fn gone_and_come_back() {}

/// Property: nobody there.
#[test]
#[ignore = "statement written, not built"]
fn nobody_there() {}

/// Property: a hex with no base.
#[test]
#[ignore = "statement written, not built"]
fn a_hex_with_no_base() {}

/// Property: more at the source is never worse.
#[test]
#[ignore = "statement written, not built"]
fn more_at_the_source_is_never_worse() {}

/// Property: at press.
#[test]
#[ignore = "statement written, not built"]
fn at_press() {}

/// Property: what the side cannot see.
#[test]
#[ignore = "statement written, not built"]
fn what_the_side_cannot_see() {}

/// Property: the same twice.
#[test]
#[ignore = "statement written, not built"]
fn the_same_twice() {}
