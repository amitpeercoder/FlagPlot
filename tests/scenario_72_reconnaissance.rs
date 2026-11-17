//! Scenario 72: reconnaissance. The commander asks for an airfield at
//! preserve the force. The staff has no count of the enemy there. It
//! sends an aircraft to look, learns that 150 men hold it, chooses one
//! unit where it had chosen two, and only then launches.
//!
//! Proves decision 0147, step 6 of stage 3, first part, against a hand
//! calculation. The data is placeholder data: see
//! `data/scenarios/72_reconnaissance.toml`. It runs on the frozen step 1
//! figures, with ground figures of its own that differ from the frozen
//! ones in five: `preparation_per_day` 100, `report_error_low` 1,
//! `report_error_high` 1, `reconnaissance_error_low` 1 and
//! `reconnaissance_error_high` 1. A report gives the true count, so no
//! die changes anything. The game ends before the landing.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly, except the supply and the fatigue the flights cost, which
//! are the search rule's and are not this scenario's to test. If the
//! code disagrees, the code or the rule is wrong: do not change a number
//! here to make a test pass. A mismatch goes back to the architect.
//!
//! ## Figures
//!
//! A force with its roster in port gains 100 of loading in a day and a
//! unit ready on it 100 of rehearsal, each to 100. A change of roster
//! keeps half the loading. At preserve the force the staff wants 3 men
//! for each man of its estimate, a landing is ready at a loading and a
//! rehearsal of 100, and a "take a place" may launch on a count seen
//! from the air or by men in contact no more than 3 days old. A man eats
//! 0.005 t a day.
//!
//! ## Inputs
//!
//! The placeholder map of scenario 27: A at (0,0), port size 5, and B at
//! (4,0), port size 0, 4 hexes away. The game is 4 days.
//!
//! Base 1, Allied, at A, with an airfield and 1,000 t of supply. Base 2,
//! Japanese, at B, on the Allied map, with 100 t of supply.
//!
//! Air group 1, Allied, at base 1: 6 ready aircraft of a placeholder type
//! of the frozen figures that can search and whose search range is 4
//! hexes or more, at rest.
//!
//! Task force 1, Allied, on the transport mission: two placeholder troop
//! transports of 600 berths, stopped at base 1.
//!
//! | Unit | Side | Where | Men | Experience | Assault skill |
//! | ---- | ---- | ----- | --- | ---------- | ------------- |
//! | 1, "Placeholder First" | Allied | A | 600 | 2 | 2 |
//! | 2, "Placeholder Second" | Allied | A | 600 | 2 | 2 |
//! | 10, "Placeholder Garrison" | Japanese | B | 150 | 2 | |
//!
//! The Allied side holds no estimate. Its commands of day 1: intent 1,
//! take the place at base 2, at preserve the force, with a forces limit
//! of task force 1, units 1 and 2, air group 1 and base 1 as the source.
//! Leave to prepare and leave to launch are both given.
//!
//! ## Day by day
//!
//! | Day | The estimate at the staff's hour | The staff's commands and orders | What happens |
//! | --- | --- | --- | --- |
//! | 1 | none | Approves plan 1 to preparing. Forms intent 2, ready a landing by task force 1 at B, with a forces limit of units 1 and 2. Orders `prepare_landing` at B, roster 1 and 2. Sets air group 1 to reconnaissance of B | In the morning one aircraft flies 4 hexes and reports 150 men, seen from the air. Loading 100; each unit's rehearsal 100 |
//! | 2 | 150, seen from the air, of day 1 | Sets the forces limit of intent 2 to unit 1. Orders `prepare_landing` at B, roster 1. Sets air group 1 to rest | The loading is cut to 50 by the change and gains to 100. Unit 2 leaves the roster |
//! | 3 | the same | Notice: intent 2 is ready. Approves plan 1 to released. Orders a load of supply to a weight of 45 t | |
//! | 4 | the same | Unit 1 set to attack. Load unit 1 | |
//!
//! - **Day 1.** With no estimate the staff takes every unit the ships can
//!   carry, units 1 and 2, and says it goes with no count. The need is
//!   not met: it sends air group 1, the only group in the forces limit.
//! - **Day 2.** The men wanted are 150 x 3 = 450. Unit 1 has 600: it is
//!   enough, and unit 2 is left out. The need is met, by a count seen
//!   from the air one day old. The landing is not ready: the loading
//!   after the morning's order is 50.
//! - **Day 3.** The loading is 100 and unit 1's rehearsal is 100: ready,
//!   and the count is 2 days old. The supply is 600 x 0.005 x 15 = 45 t.
//!
//! Seven orders in all, and five staff commands. Air group 1 flies once,
//! on day 1. No staff order is refused.
//!
//! # Properties
//!
//! - **The grades.** With an opening estimate of 500 men at B: it is of
//!   the grade told, and at preserve the force it does not meet the need.
//!   Day 1 is as in the main case but for the choice, which is the same
//!   two units on 1,500 men wanted. On day 2 the estimate is 150, seen
//!   from the air, and the day is as in the main case.
//! - **Balanced.** At balanced the men wanted on day 2 are 300, unit 1 is
//!   chosen alone, and the days are as in the main case.
//! - **Press.** At press no group is sent on any day. The landing is
//!   ready on the morning of day 2 with units 1 and 2: the plan is
//!   released and the reason says it goes on no count. The load is to
//!   90 t.
//! - **Nothing can look, at preserve the force.** With air group 1 left
//!   out of the forces limit: no group is sent. On the morning of day 2
//!   the landing is otherwise ready: the plan is not released, and the
//!   staff hands in the notice that it cannot meet intent 1, on that
//!   morning and on each after, saying that nothing it may use can look.
//! - **Nothing can look, at balanced.** The same, at balanced: on the
//!   morning of day 2 the plan is released with units 1 and 2, and the
//!   reason says it goes on no count.
//! - **Men in contact.** With an Allied fighting unit 3 of 100 men ashore
//!   at B on the defence from the start, fed from an Allied base 3 there:
//!   air group 1 is not sent on any day. The ground report of the end of
//!   day 1 gives 150 men, by men in contact, and day 2 is as in the main
//!   case but for the order to the group.
//! - **An empty place.** With unit 10 left out: the flight of day 1
//!   reports 0 men. On day 2 the staff chooses one unit, unit 1.
//! - **The frozen figures give no flight.** With the two reconnaissance
//!   figures left out, a group on the mission does not fly and no report
//!   is made.
//! - **The enemy sees nothing of the flight:** the Japanese side's events
//!   hold neither the flight nor the report.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see and has not been told give the same
//!   orders and reasons; a save on day 2 and a resume give the same game;
//!   every scenario built before this one gives the events it gave.
