//! Scenario 62: reports between commands. Two carrier forces hold the same
//! contact. One strikes it and finds nothing there. The other is never
//! told, and for a day the two decide differently, each on what it holds.
//!
//! Proves decision 0135, step 2 of stage 2b, against a hand calculation.
//! The data is placeholder data: `data/scenarios/62_reports_between_commands.toml`
//! is scenario 58's with a second carrier force, command levels on and
//! report figures of its own. It runs on the frozen step 1 figures as
//! scenario 58 does. There is no Japanese unit and no Japanese base. Every
//! chance in the report figures is 0 or 1, so no outcome rests on a die.
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
//! As scenario 58: a hex is 40 nm; the placeholder dive bomber's strike
//! radius is 250 nm; a contact gains a pulse of age at the end of each
//! pulse and is dropped when its age reaches 5. The report figures of this
//! scenario: up, on time 1; sideways, on time 0 and a pulse late 0, so
//! never; down, on time 0 and a pulse late 0, so never. Radio reach: 600
//! nm for a carrier, 1,000 nm for a base.
//!
//! ## Inputs
//!
//! Scenario 58's map, base 1 at (0,5), task force 1 with ship 1 and air
//! groups 1, 2 and 3 at (0,0), and the one contact the Allied side starts
//! with: 1 ship, a carrier, at (12,0), age 0. Added: task force 2, one
//! placeholder carrier, ship 2, stopped at sea at (7,0), full of fuel, on
//! the carrier mission, at normal readiness, at cruising speed, with the
//! stance engage and base 1 as its home port. Aboard her: group 4, 12
//! fighters; group 5, 6 carrier scouts; group 6, 10 dive bombers; all
//! resting. The game is 3 days.
//!
//! The Allied side's commands of day 1: intent 1, operate task force 1
//! from (0,0), its risk press; intent 2, operate task force 2 from (7,0),
//! with no risk set, so balanced; leave to prepare and leave to launch
//! given for both.
//!
//! The Allied commands: the area command, task force 1, task force 2 and
//! base 1. Each starts with contact 1 in its picture.
//!
//! ## Day 1
//!
//! Task force 1 does what it does in scenario 58: twelve orders, full
//! speed to (6,0), there at hour 8, and in the morning group 3 sends its
//! 10 at the contact, 240 nm away, and finds nothing.
//!
//! Task force 2 is at its hex. The contact is 200 nm away, within reach:
//! it does not close. It is given eight orders, as a carrier force at its
//! hex at balanced is (scenario 55): battle readiness; rules of engagement
//! with the stance avoid and a break-off level of 0.5; group 4, combat air
//! patrol and a patrol share of 0.5; group 5, the search mission and the
//! arc, 050 to 120 at 300 nm, the contact lying on 090; group 6, the naval
//! strike mission and strike sizing on. It launches nothing: the contact
//! is a pulse old in the morning, against the figure of 0.
//!
//! Twenty orders on day 1.
//!
//! **The report.** "Nothing there" at (12,0) is task force 1's. In the
//! morning it drops contact 1 from its picture and sends the report:
//!
//! - up, to the area command: on time. The area command drops contact 1 in
//!   the morning.
//! - sideways, to task force 2, 40 nm away, and to base 1, 223 nm away,
//!   both within 600 nm: never.
//! - The area command sends it down, to task force 2 and to base 1: never.
//!
//! Two "report sent" events in the morning of day 1, the first with three
//! listeners and the second with two.
//!
//! ## The pictures
//!
//! Contact 1, by its age, or "none":
//!
//! | After | Task force 1 | The area command | Task force 2 | Base 1 | The side's own |
//! | --- | --- | --- | --- | --- | --- |
//! | The start | 0 | 0 | 0 | 0 | 0 |
//! | The night of day 1 | 1 | 1 | 1 | 1 | 1 |
//! | The morning of day 1 | none | none | 2 | 2 | none |
//! | The afternoon of day 1 | none | none | 3 | 3 | none |
//! | The night of day 2 | none | none | 4 | 4 | none |
//! | The morning of day 2 | none | none | none | none | none |
//!
//! Task force 2 and base 1 drop it at 5 pulses, having never been told.
//!
//! ## Day 2
//!
//! Task force 1 holds no contact: two orders, the route back to (0,0) and
//! group 2's arc, the whole circle, as in scenario 58.
//!
//! Task force 2 holds contact 1, 3 pulses old, 200 nm away: no order. Its
//! reason names contact 1 by its number in task force 2's picture, its
//! age and its distance, and names task force 2 as the command that
//! holds it.
//!
//! Two orders on day 2.
//!
//! ## Day 3
//!
//! Task force 1 is at its hex: no order. Task force 2 holds no contact:
//! one order, group 5's arc, the whole circle.
//!
//! One order on day 3. Twenty-three in all.
//!
//! # Properties
//!
//! - **Told sideways, on time.** With sideways on time 1: task force 2
//!   and base 1 drop contact 1 in the morning of day 1. Task force 2 is
//!   given group 5's arc, the whole circle, on day 2 and nothing on day
//!   3. The orders are 20, 3 and 0.
//! - **Told down, a pulse late.** With sideways never and down a pulse
//!   late 1: task force 2 and base 1 hold contact 1 at age 2 after the
//!   morning of day 1 and drop it at the start of the afternoon. The
//!   orders are 20, 3 and 0.
//! - **Out of reach.** With sideways on time 1 and a carrier's reach 30
//!   nm: task force 2, 40 nm from task force 1, is not sent the report
//!   sideways, and base 1 is not; the game is the main case. With the
//!   reach 40 nm: task force 2 is sent it and drops the contact in the
//!   morning; base 1, 223 nm away, is not, and holds it to 5 pulses.
//! - **Heard once.** With sideways on time 1 and down on time 1: task
//!   force 2 is sent the report by both roads. It is applied once, and the
//!   game is that of "told sideways, on time".
//! - **A late sighting is as old as it is.** In a view built for it: a
//!   sighting made in pulse 4 that reaches a picture holding no contact at
//!   the start of pulse 5 makes a contact at age 1. It is 1 at the end of
//!   pulse 5 and 2 at the end of pulse 6, as it is in the picture of the
//!   command that made the sighting.
//! - **Newer word stands.** In a view built for it: a picture holds a
//!   contact made by a sighting of pulse 5 at (3,0). A sighting of pulse 4
//!   at (3,0) that arrives in pulse 5 after it is heard and changes
//!   nothing: the contact keeps its count, its kind and its age.
//! - **Every chance certain is step 1.** With on time 1 in every
//!   direction: in this scenario, and on live figures with these report
//!   figures in the Eastern Solomons from intents
//!   (`data/scenarios/second-proof/05a_eastern_solomons.toml`, seeds 1 to
//!   10) and the week of the slice from intents (seed 1), every command's
//!   picture holds what its side's holds at the end of every pulse, and
//!   the events are those of the same game with command levels off, but
//!   for the "report sent" events.
//! - **No report figures.** Scenario 61 gives what it gave.
//! - **Off is unchanged.** Every scenario built before this one gives the
//!   events it gave.
//! - The Japanese side sees no Allied "report sent" event. The same start
//!   gives the same game. Two worlds that differ only in what the Allied
//!   side cannot see give the same orders and reasons.
