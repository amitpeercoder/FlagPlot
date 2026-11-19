//! Scenario 96: the window. The same landing is tried at two dates in the
//! same worlds: now, with the one unit that is to hand, and on the day a
//! second unit may first be used. The estimates are set side by side,
//! with what the later date buys and what the enemy did with the days.
//!
//! Proves decision 0177, step 7 of step 3's stage 4, against a hand
//! calculation. The data is placeholder data: see
//! `data/scenarios/96_the_window.toml`, which the build writes to the
//! inputs below. It runs on the frozen step 1 figures with scenario 68's
//! ground figures and three more of its own, so that men dig in: every
//! shot is a whole shot and hits, and no die changes anything.
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
//! Scenario 68's (see its hand calculation, and scenario 95's): one
//! shot for each 100 firing men, every shot hits and takes 50 men; a
//! side is beaten where the other attacked and it has no man left. With
//! three ground figures of this scenario's own: `fieldworks_per_day` 10,
//! `fieldworks_limit` 100 and `cover_in_fight` 0.5. A fighting unit
//! ashore on the defence digs 10 of fieldworks at the end of each day,
//! after that day's fight, to 100 at most. In a fight a defender's loss
//! is cut by its cover over 100 times 0.5, and cut to a whole man: at a
//! cover of 30 a loss of 300 is 300 x (1 - 0.15) = 255; at 70 a loss of
//! 600 is 600 x (1 - 0.35) = 390. Cover does not change the shots of
//! either side, and an attacker has none.
//!
//! With no count of the enemy the staff takes every unit its ships can
//! carry, and ashore it attacks.
//!
//! ## Inputs
//!
//! Scenario 68's map, bases, ships and task force, with these changes:
//!
//! - **Two units.** Unit 1, 600 fighting men at A, usable from day 1.
//!   Unit 2, 600 fighting men at A, usable from day 4. No unit 3.
//! - **No opening estimate.** The Allied side holds no count of B.
//! - **Believed men:** the Allied side believes 200 men are at B,
//!   standing present.
//! - **No commands in the scenario.**
//! - **The trial's figures:** an enemy base is assumed to hold 200 t of
//!   supply and 0 t of fuel; the seed is 1942. A file of sides that
//!   gives every class to both sides.
//!
//! **The course:** for the Allied side: intent 1, take the place at
//! (4,0); its forces limit, task force 1, units 1 and 2 and base 1 as
//! the source; leave to prepare given; leave to launch given. Each
//! command is of day 1 of the course.
//!
//! **The window:** of the Allied side, from the evening before day 1,
//! trial 1, 4 worlds, a size of 8 days.
//!
//! ## The dates
//!
//! The first date is day 1. Of what the intent's forces limit names,
//! task force 1 and unit 1 may be used from day 1 and unit 2 from day 4.
//! No aircraft are due, no force is in a yard and no supply run is due.
//! So there is one later date, day 4. The window has two dates: day 1,
//! played for 8 days, to day 8; and day 4, played for 8 + 3 = 11 days,
//! to day 11.
//!
//! ## The worlds
//!
//! In every world the enemy has base 2 at B with 200 t of supply and
//! land unit 900,001, 200 fighting men on defend at B, from the
//! believed men. Nothing is of unknown standing, so the four worlds are
//! alike, and the Japanese side is given no intent and orders nothing.
//!
//! The 200 men dig: their fieldworks are 10 at the end of day 1, 20
//! after day 2, 30 after day 3, and so on by 10 a day.
//!
//! ## The first date: day 1
//!
//! The staff holds no count: it takes every unit its ships can carry
//! that it may use. On the morning of day 1 that is unit 1. The days
//! are scenario 95's: on day 1 it forms the landing and orders
//! `prepare_landing` with roster 1; on day 2 the landing is ready, the
//! plan is released and 45 t of supply are loaded; on day 3 unit 1 is
//! set to attack and loaded; on day 4 the force sails at hour 72,
//! arrives at hour 88 and lands the 600 men, and the fight is that
//! evening.
//!
//! | The fight of day 4 | Allied | Japanese |
//! | --- | --- | --- |
//! | Men ashore | 600, on attack | 200, on defend, fieldworks 30 |
//! | Shots, all hits | 6 | 2 |
//! | Men lost | 2 x 50 = 100 | 6 x 50 = 300, cut by cover to 255, more than the 200: all 200 |
//! | Left | 500 | none |
//!
//! Base 2 is Allied at the end of day 4: the end is met on day 4, in
//! each of the four worlds. No man is lost after it.
//!
//! ## The second date: day 4
//!
//! Days 1 to 3 are played as things stand: nothing is ordered. On the
//! morning of day 4 the course is handed in, and the staff may use
//! units 1 and 2: it takes both, one to a ship. The days are scenario
//! 80's from its day 4: on day 4 it forms the landing and orders
//! `prepare_landing` with roster 1 and 2; on day 5 the landing is
//! ready, the plan is released and 1,200 x 0.005 x 15 = 90 t of supply
//! are loaded; on day 6 unit 1 is set to attack and loaded; on day 7
//! unit 2; on day 8 the force sails at hour 168, arrives at hour 184
//! and lands 600 men from each ship, and the fight is that evening.
//!
//! | The fight of day 8 | Allied | Japanese |
//! | --- | --- | --- |
//! | Men ashore | 600 and 600, on attack | 200, on defend, fieldworks 70 |
//! | Shots, all hits | 6 and 6 | 2 |
//! | Men lost | 2 x 50 = 100: 50 each | 12 x 50 = 600, cut by cover to 390, more than the 200: all 200 |
//! | Left | 550 and 550 | none |
//!
//! Base 2 is Allied at the end of day 8: the end is met on day 8, in
//! each of the four worlds.
//!
//! ## The window of intent 1
//!
//! | | The first date, day 1 | The second date, day 4 |
//! | --- | --- | --- |
//! | Played to | day 8 | day 11 |
//! | The plan order: men wanted | no count | no count |
//! | The plan order: men to hand | 600 | 1,200 |
//! | The plan order: day expected ashore | day 4 | day 8 |
//! | The enemy at B on the eve: men, lowest, middle, highest | 200, 200, 200 (the evening before day 1) | 200, 200, 200 (the evening of day 3) |
//! | The enemy at B on the eve: cover | 0, 0, 0 | 30, 30, 30 |
//! | Met and kept | 4 of 4 worlds, on day 4, 4 and 4 | 4 of 4, on day 8, 8 and 8 |
//! | The side's men lost to the enemy | 100, 100, 100 | 100, 100, 100 |
//! | The trial's enemy's men lost | 200, 200, 200 | 200, 200, 200 |
//! | Every other cost of either side | 0 | 0 |
//!
//! - **What the second date buys:** unit 2, usable from day 4; 600 more
//!   men to hand, 1,200 against 600.
//! - **World for world,** the second date against the first: met and
//!   kept at both in 4 worlds; at the first alone in none; at the later
//!   alone in none; at neither in none.
//!
//! # Properties
//!
//! - **The same worlds.** The pictures of the two dates' trials are
//!   equal, world for world.
//! - **The first date is the estimate.** The first date's estimate is
//!   the estimate decision 0176 gives of the same course in a trial of 4
//!   worlds for 8 days, field for field.
//! - **Nothing of the side's is due to change.** With unit 2 usable from
//!   day 1: the window has one date, day 1, and says that nothing is due
//!   to change. The staff takes both units on day 1.
//! - **Three dates at most.** With a third unit of 600 men usable from
//!   day 6 and a fourth from day 7, all in the forces limit: the dates
//!   are days 1, 4 and 6. Day 7 is not tried.
//! - **Outside the limit does not count.** With unit 2 left out of the
//!   course's forces limit: the window has one date.
//! - **Beyond the horizon does not count.** With unit 2 usable from day
//!   10 and a size of 8 days: one date.
//! - **A date is told before it is played.** The dates of the main case
//!   are had from the view and the course alone, with no world built.
//! - **The same twice,** and the live game untouched: its hash before
//!   and after the window is the same.
//! - **No live game changes** by the build: no baseline, golden file or
//!   pinned hash moves.

// The tests are written by the build, to the statements above.
