//! Scenario 66: units made ready, and a base built up. The staff trains a
//! raw unit and then rehearses it, rehearses a sound one from the first
//! day, loads the transports, builds an airfield, and says when the
//! landing is ready. Nobody gives a unit its activity by hand.
//!
//! Proves decision 0141, step 2 of stage 3, against a hand calculation.
//! The data is placeholder data: see
//! `data/scenarios/66_units_made_ready.toml`. It runs on the frozen step 1
//! figures, with ground figures of its own that differ from the frozen
//! ones in three: `preparation_per_day` 25, `training_days_per_level` 3,
//! `airfield_work` 4000. There is no Japanese unit, so no die is rolled.
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
//! A force in port with its roster gains 25 of loading a day, to 100. A
//! unit that stands ready on that roster gains 25 of rehearsal a day, to
//! 100 (the assault skill's strength is 0 on the frozen figures). A unit
//! that trains counts a day, and at 3 its experience rises by one and the
//! count starts again; the standard is 2 and training goes no higher. A
//! man of a working unit of works skill 2 makes 1 of work a day; a
//! fighting unit set to work makes half that. An airfield takes 4,000.
//! Each is settled at the end of the day.
//!
//! ## Inputs
//!
//! A placeholder map. Allied base 1 at a coast hex, (0,0), with a port,
//! no airfield and no work done on one, and supply for every man for the
//! whole game. The game is 12 days.
//!
//! Task force 1, Allied: two placeholder transports, stopped in port at
//! base 1. Ashore at base 1, every man fed in full:
//!
//! | Unit | Kind | Men | Experience |
//! | --- | --- | --- | --- |
//! | 1, "Placeholder Raw" | fighting | 800 | 1 |
//! | 2, "Placeholder Sound" | fighting | 800 | 2 |
//! | 3, "Placeholder Builders" | working, works skill 2 | 1,000 | 2 |
//!
//! Every unit's activity at the start is what the rules give it: ready
//! for units 1 and 2, work for unit 3 with no job named.
//!
//! The Allied side's commands of day 1: intent 1, ready a landing by task
//! force 1 at (6,0), a coast hex, with a forces limit of units 1 and 2,
//! no risk set, so balanced, and no date; intent 2, build up base 1,
//! wanting an airfield; leave to prepare given for both.
//!
//! ## Day by day
//!
//! | Day | Orders in the morning | Loading after | Unit 1 after | Unit 2 rehearsal after | Airfield work after |
//! | --- | --- | --- | --- | --- | --- |
//! | 1 | `prepare_landing` at (6,0), roster 1 and 2; unit 1, train; unit 3, work on the airfield | 25 | trains: 1 day counted | 25 | 1,000 |
//! | 2 | none | 50 | 2 days | 50 | 2,000 |
//! | 3 | none | 75 | 3 days: experience 2, count 0 | 75 | 3,000 |
//! | 4 | unit 1, ready | 100 | rehearsal 25 | 100 | 4,000: the airfield is built |
//! | 5 | none | 100 | 50 | 100 | |
//! | 6 | none | 100 | 75 | 100 | |
//! | 7 | none | 100 | 100 | 100 | |
//! | 8 to 12 | none | 100 | 100 | 100 | |
//!
//! Four orders in all: three on day 1 and one on day 4.
//!
//! - **Day 1.** Unit 1 is at experience 1, below the standard of 2: at
//!   balanced with no date it trains first. Unit 2 is at the standard and
//!   stands ready already: no order. Unit 3 is given the airfield by name.
//! - **Day 4.** Unit 1 is at the standard: it is set to stand ready, and
//!   from that day rehearses.
//! - **Day 5.** The base has an airfield. Intent 2's one want is met: the
//!   staff gives no order, says so, and hands in the notice that intent 2
//!   is met, on that morning and on no other.
//! - **Day 8.** The loading is 100 and both units' rehearsal is 100: the
//!   staff hands in the notice that intent 1 is ready, on that morning
//!   and on no other.
//!
//! The reason for unit 1's order of day 1 gives its experience, 1, against
//! the standard, 2; 0 days counted of 3; and 4 days of rehearsal to come.
//! The reason for unit 3's order gives 4,000 of work to do, 1,000 a day,
//! 4 days.
//!
//! No staff order is refused.
//!
//! # Properties
//!
//! - **Press.** With intent 1 at press: unit 1 is not set to train. Day 1
//!   has two orders, the landing order and unit 3's job. Both units'
//!   rehearsal and the loading are 50 after day 2, and the notice that
//!   intent 1 is ready is handed in on the morning of day 3.
//! - **Preserve the force, and a raw unit.** With intent 1 at preserve the
//!   force and unit 1 at experience 0: it trains for 6 days, is at the
//!   standard after day 6, is set to stand ready on the morning of day 7,
//!   and its rehearsal is 100 after day 10. The notice that intent 1 is
//!   ready is handed in on the morning of day 11.
//! - **A date, at balanced.** With a date of the morning of day 6: on the
//!   morning of day 1 five days are left and unit 1's rehearsal needs 4:
//!   it trains. On the morning of day 2 four are left and it needs 4: it
//!   is set to stand ready, with 1 day of training counted and its
//!   experience still 1. Its rehearsal is 100 after day 5, and the notice
//!   that intent 1 is ready is handed in on the morning of day 6.
//! - **A date, at preserve the force.** With the same date and preserve
//!   the force: unit 1 trains to the standard as in the main case, and the
//!   task is ready on the morning of day 8. On the morning of day 1 the
//!   staff hands in the notice that it cannot be ready by its date: 2
//!   days late, and what binds is unit 1's training.
//! - **A fighting unit put to work.** With intent 2's forces limit naming
//!   units 2 and 3, and intent 1's naming unit 1 alone: unit 2 is set to
//!   work on the airfield on day 1 and makes 400 a day, half of 800. The
//!   work after days 1, 2 and 3 is 1,400, 2,800 and 4,200: the airfield
//!   is built on day 3.
//! - **One task a unit.** With unit 2 in both forces limits: intent 1,
//!   the higher, has it; it rehearses, and intent 2's reason says so.
//! - **A want already met.** With an airfield at base 1 at the start:
//!   intent 2 gives no order and its notice is handed in on day 1.
//! - **Defences to a number.** With intent 2 wanting an airfield and then
//!   defences of 10 points: from the morning of day 5 unit 3 is given the
//!   defences, one order, and makes 5 points a day at 200 of work a
//!   point: 10 after day 6, and the notice is handed in on the morning of
//!   day 7.
//! - **Without leave to prepare** no order is given and the reason says it
//!   waits for leave.
//! - **A hold and an override.** With unit 1 held from day 1 it is given
//!   no order and the staff says it plans without it. A commander's
//!   `set_activity` to unit 2 on day 2 holds it, and the staff gives it no
//!   order after.
//! - **Before its day.** With unit 2 available from day 3: it is given no
//!   order before day 3 and is not on the roster of the landing order of
//!   day 1; on day 3 the staff gives the landing order again with both
//!   units, and the loading is halved by the unit added, as decision 0095
//!   has it.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see give the same orders and reasons;
//!   every scenario built before this one gives the events it gave.
