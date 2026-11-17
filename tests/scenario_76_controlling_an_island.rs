//! Scenario 76: controlling an island. The commander says "control the
//! island" and nothing else. The island has two places: an airfield the
//! enemy holds with nobody on it, and an anchorage the side holds with a
//! small garrison. The staff forms the tree, takes the airfield, holds
//! both, and feeds them.
//!
//! Proves decision 0151, step 8 of stage 3, second part, against a hand
//! calculation. It stands on scenario 69's for a place taken and held
//! and on scenario 74's for the force the staff assembles. The data is
//! placeholder data: see `data/scenarios/76_controlling_an_island.toml`.
//! It runs on the frozen step 1 figures, with one ground figure of its
//! own, `preparation_per_day` 100. No enemy man is ashore anywhere, so no
//! die is rolled.
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
//! The placeholder map of scenario 27: A at (0,0), port size 5; B at
//! (4,0), port size 0; C at (0,6), port size 0. The game is 14 days.
//!
//! | Base | Side | Hex | Supply |
//! | ---- | ---- | --- | ------ |
//! | 1, "Placeholder base A" | Allied | A | 1,000 t |
//! | 2, "Placeholder airfield B" | Japanese | B | 0 |
//! | 3, "Placeholder anchorage C" | Allied | C | 100 t |
//!
//! Base 2 is on the Allied map and has no gun, aircraft or man. No base
//! has a reserve or fuel. The Allied side holds no estimate and no
//! contact.
//!
//! Island 1, "Placeholder island": base 2, then base 3.
//!
//! Task force 1, Allied, on the transport mission: ships 1 and 2,
//! placeholder troop transports, stopped at base 1.
//!
//! | Unit | Kind | Where | Men |
//! | ---- | ---- | ----- | --- |
//! | 1, "Placeholder First" | fighting | A | 600 |
//! | 3, "Placeholder Garrison" | fighting | C | 300 |
//!
//! Both are Allied, on the defence, experience 2, every skill 2.
//!
//! The Allied side's commands of day 1: intent 1, control island 1. No
//! forces limit and no risk, so balanced. Leave to prepare and leave to
//! launch are both given.
//!
//! ## The tree on the first morning
//!
//! | Kind | Under | Made by |
//! | ---- | ----- | ------- |
//! | Control island 1 | none | the commander |
//! | Take the place at base 2 | control | the staff |
//! | Hold base 2 | control | the staff |
//! | Hold base 3 | control | the staff |
//! | Deny the waters within 3 hexes of (4,0) | control | the staff |
//! | Neutralise base 2 | control | the staff |
//! | Ready a landing by task force 1 at B | the take | the staff |
//! | Keep base 3 supplied, floor 5 days | the hold of base 3 | the staff |
//!
//! The first five formed are intents 2 to 6, in that order. The side
//! has no carrier force: no carrier task is formed. Base 2 is the
//! enemy's and at a named place: it is neutralised until it is taken,
//! and the neutralising finds no bomber and no bombardment force and
//! forms nothing under it. On the morning of day 5, base 2 the side's,
//! the staff withdraws intent 6. No enemy base lies off the island. The
//! source of
//! supply is base 1, the side's only base off the island, where task
//! force 1 lies: each hold's forces limit is set to task force 1 and
//! base 1. The denial finds no force and no air group and forms nothing
//! under it. The hold of base 2 waits. "Keep base 2 supplied" is formed
//! under the hold of base 2 on the morning of day 5.
//!
//! ## The airfield
//!
//! The take chooses by decision 0149: no estimate, so every unit the
//! ships can carry; the source is base 1, the only base with a transport
//! force and a unit; unit 1; task force 1. The check of decision 0144:
//! the need is 600 x 0.005 = 3 t a day; supply follows in 4 days; the
//! margin is 5; the supply that lands with the men is 3 x 9 = 27 t; the
//! first run is 45 t; base 1 holds 997 t on the morning of day 2. It can
//! be held.
//!
//! | Day | Hour | The staff's orders | What happens |
//! | --- | --- | --- | --- |
//! | 1 | 0 | `prepare_landing` at B, roster 1 | Loading 100; rehearsal 100 |
//! | 2 | 24 | The plan of the take is released. A load of supply to a weight of 27 t | |
//! | 3 | 48 | Unit 1 set to attack. Load unit 1 | |
//! | 4 | 72 | A route to B and back to A, acting at B, and an unload order | Sails at hour 72, arrives at hour 88. Ship 1 lands 160 men in the morning, 440 in the afternoon and then 10 t. Base 4, "Landing at (4,0)", is formed. Base 2 changes side at the end of the day |
//! | 5 | 96 | The take's plan is marked complete; notice. Unit 1 set to defend | Ship 1 lands 17 t in 0.85 hours. The force steams for A at hour 108 and arrives at hour 124 |
//! | 8 | 168 | A run for base 2: a load of supply to a weight of 45 t | |
//! | 9 | 192 | A route to B and back to A, acting at B, and an unload order | Sails at hour 192, arrives at hour 208. Ship 1 lands 40 t in the morning and 5 t in the afternoon |
//!
//! Ten orders in all. No staff order or command is refused.
//!
//! ## The supply doctrine's sums
//!
//! - **Base 2.** The use is 3 t a day and the floor's stock 15 t. The
//!   stock at B is 24 t on the morning of day 6, 21 t on day 7 and 18 t
//!   on day 8. A run ordered on a morning arrives the next day, and the
//!   reading checked is the day after, 6 t lower: 18 t, 15 t and 12 t.
//!   The first below 15 is on the morning of day 8. Its load is 0 plus
//!   15 days of use, 45 t.
//! - **Base 3.** The use is 300 x 0.005 = 1.5 t a day and the floor's
//!   stock 7.5 t. The stock is 100 t and falls to 79 t by the end: no
//!   run is needed on any day.
//!
//! ## Supply
//!
//! - **Base 1** feeds 600 men, 3 t, on the nights of days 1 to 3, and
//!   loads out 27 t and 45 t: 1,000 - 9 - 72 = 919 t at the end.
//! - **Base 4** is given 10 t on the afternoon of day 4. On the night of
//!   day 5 base 2 holds nothing and base 4 pays 3 t: 7 t, which it keeps.
//! - **Base 2** takes 17 t on the night of day 5, after that night's
//!   charge. It pays 3 t on each night from day 6: 14, 11, 8 and 5 t
//!   after the nights of days 6 to 9; takes 45 t on day 9: 50 t; and
//!   pays 3 t on the nights of days 10 to 14: 35 t at the end.
//! - **Base 3** pays 1.5 t each night: 79 t at the end.
//!
//! ## The measure
//!
//! A place counts on a day when it holds a base of the side at the end
//! of the day and the days of supply in its hex are at least the floor,
//! 5 days. C holds 100 t falling to 79 t at 1.5 t a day: it counts on
//! every day. B is the side's from the end of day 4. At the end of day 4
//! its hex holds 10 t at 3 t a day, 3.33 days: it does not count. At the
//! end of days 5 to 8 it holds 24, 21, 18 and 15 t: 8, 7, 6 and 5 days.
//! At the end of day 9 it holds 57 t, and 54 t down to 42 t at the end
//! of days 10 to 14. The island is controlled on days 5 to 14: on 10 of
//! 14 days in force.
//!
//! Restated by the architect on Nov 17, 2026, with decision 0151's
//! amendment: the first writing counted any supply at all and gave 11 of
//! 14; it had no neutralising of base 2, and no property of a carrier or
//! of a date.
//!
//! # Properties
//!
//! - **Both places the side's already.** With base 2 Allied, holding
//!   100 t of supply, and unit 1 ashore there: no take and no
//!   neutralising is formed; both holds are formed and each forms its
//!   supply on day 1; the island is controlled on 14 of 14 days.
//! - **A commander's own hold stands.** With the commander's own intent
//!   "hold base 3" stated on day 1 beside intent 1: the staff forms no
//!   hold of base 3 and its reason names the commander's.
//! - **The commander names the source.** With a forces limit on intent 1
//!   that names base 1 as the source: the tree and the orders are those
//!   of the main case.
//! - **No source.** With task force 1 left out: no base can be the
//!   source of supply or of the landing. The holds form no supply and
//!   say so; the take hands in the notice that it cannot meet its intent
//!   on day 1 and gives the reason each morning; the island is
//!   controlled on 0 of 14 days.
//! - **An enemy base near.** With a Japanese base 5 on the Allied map at
//!   (2,3), in no named place's hex: "neutralise base 2" and then
//!   "neutralise base 5" are formed on day 1, after the denial, and each
//!   finds no bomber and no bombardment force. Only the forming is
//!   asserted: base 5 lies 3 hexes from B, so the lift is a run by night
//!   (decision 0145) and its hours are not the main case's.
//! - **A carrier.** With scenario 73's carrier force, its ship and its
//!   three air groups as they are there, stopped at base 1 with base 1
//!   its home port: "operate task force 2 from (0,0)" is formed under
//!   the denial on day 1, the force lying within its bombers' reach of
//!   B, 160 nm. The take forms no carrier task. On the morning of day 2
//!   the take's plan is released and group 3 is set to the base strike
//!   at base 2: it launches at base 2 on days 2, 3 and 4 and on no other
//!   day. The force is given no route on any day: it is not sent home
//!   when the take is met. The landing is the main case's.
//! - **By a date.** With the island asked for by day 10: the take is
//!   formed with the date of day 10, and the task "ready a landing"
//!   under it has that day. Only the forming is asserted.
//! - **Without leave to launch** the plan of intent 1 is approved to
//!   preparing, nothing is formed, and the notice that it waits for
//!   leave to launch is handed in each morning.
//! - **Withdrawn.** With intent 1 withdrawn on day 6: every intent under
//!   it is withdrawn with it, and no order is given from day 6.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see give the same orders and reasons; a
//!   save on day 7 and a resume give the same game; every scenario built
//!   before this one gives the events it gave.
