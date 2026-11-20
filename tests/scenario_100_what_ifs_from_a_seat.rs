//! Scenario 100: what-ifs from a seat. A commander at his seat queues a
//! landing and asks three things of it before a day is resolved: what if
//! I give it later, what if I hold it a day, and what does it cost me.
//! He is answered with estimates set side by side; the game is as it was.
//!
//! Proves decision 0181, the second part of step 8 of step 3's stage 4,
//! against a hand calculation. The data is placeholder data: see
//! `data/scenarios/100_what_ifs_from_a_seat.toml`, which the build writes
//! to the inputs below. It runs on the frozen step 1 figures with
//! scenario 96's ground figures, through a session as `sim-cli` opens
//! one, with the Allied seat played and the Japanese seat scripted.
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
//! Scenario 96's (see its hand calculation): one shot for each 100
//! firing men, every shot hits and takes 50 men; a side is beaten where
//! the other attacked and it has no man left, or fewer than 0.2 of the
//! other's, and beaten men at B surrender; a unit on the defence digs 10
//! of fieldworks at the end of each day; in a fight a side's loss is
//! held to the men it has, shared between its units, and each unit's
//! part cut by its cover over 100 times 0.5, to a whole man.
//!
//! ## Inputs
//!
//! Scenario 96's second case, "worlds that differ":
//!
//! - Task force 1, two troop transports, and base 1 at A, Allied. Unit
//!   1, 600 fighting men at A, usable from day 1. Unit 2, 600 fighting
//!   men at A, usable from day 4.
//! - Base 2 at B, (4,0), Japanese, with the live garrison scenario 96's
//!   file gives it.
//! - The Allied side holds no count of B. It believes 200 men are at B,
//!   standing present, and 200 more may be there, standing unknown.
//! - No commands in the scenario.
//! - The trial's figures: an enemy base is assumed to hold 200 t of
//!   supply and 0 t of fuel; the seed is 1942; the size `small` is 4
//!   worlds for 5 days, `standard` 8 worlds for 5 days and `full` 16
//!   worlds for 5 days.
//!
//! So in a trial's worlds 1 and 3 the garrison of B is 200 men, and in
//! worlds 2 and 4 two units of 200.
//!
//! ## What the commander queues
//!
//! On the evening before day 1, at the Allied seat:
//!
//! 1. `set_intent`: take the place at (4,0). It is his intent 1.
//! 2. `set_limit`, forces, on intent 1: task force 1, units 1 and 2, and
//!    base 1 as the source.
//! 3. `set_leave`: leave to launch, given.
//!
//! Leave to prepare is the side's from the start. The side has no other
//! intent.
//!
//! ## A. What if I give it later: the window
//!
//! He asks `trial` of intent 1 with `dates` chosen, at `small`.
//!
//! **What is handed in at each date:** commands 1 and 2, which set the
//! intent and its limit. Command 3, the leave, is a standing command,
//! handed in on day 1 at every date. The course gives no leave: the side
//! holds leave to prepare and the commander has queued leave to launch.
//!
//! **The dates.** The first is day 1. Of what the forces limit names,
//! unit 2 may first be used on day 4, within the 5 days: one later date,
//! day 4. The first date is played to day 5 and the second to day 8.
//!
//! **The answer** is of the kind `window`, and is scenario 96's window
//! of its second case:
//!
//! | | The first date, day 1 | The second date, day 4 |
//! | --- | --- | --- |
//! | Its course taken in on | day 1 | day 4 |
//! | What came due | nothing | unit 2, usable from day 4 |
//! | The men its take chose | 600, unit 1 | 1,200, units 1 and 2 |
//! | Men wanted and to hand | none: no count is held | none |
//! | Day expected ashore | day 4 | day 8 |
//! | Met and kept | 2 of 4 worlds, worlds 1 and 3, on day 4 | 4 of 4, on day 8 |
//! | Not met | 2, worlds 2 and 4, by day 5 | none |
//! | The side's men lost to the enemy | 100, 100, 200 | 100, 100, 200 |
//! | The trial's enemy's men lost to the enemy | 200, 200, 254 | 200, 200, 400 |
//! | Every other cost of either side | 0 | 0 |
//! | The enemy at (4,0) on the eve: men | 200, 200, 400 | 200, 200, 400 |
//! | The enemy at (4,0) on the eve: the least cover | 0, 0, 0 | 30, 30, 30 |
//!
//! - **World for world,** day 4 against day 1: met and kept at both in
//!   2 worlds, 1 and 3; at the first alone in none; at the later alone
//!   in 2, worlds 2 and 4; at neither in none.
//! - The second date's words say that nothing of the order was done
//!   before day 4.
//! - No second trial is played, and the answer names no other plan.
//!
//! ## B. What if I hold it a day: a what-if of the date
//!
//! On the same evening, with the same three commands queued, he asks
//! `preview` of a `set_limit`, "not before day 3", on intent 1. It is
//! his preview 1. He asks `trial` of preview 1 at `small`, and gives no
//! `compare`: for a preview of a limit it is true, and the second trial
//! is the plan as it stands.
//!
//! **The two trials.** Both are given the three standing commands; the
//! first has the draft besides. Neither is given a leave by the trial.
//!
//! **As it stands** is the first date of A: the place is taken on day 4
//! in worlds 1 and 3.
//!
//! **With the draft,** in every world (scenario 81's days, with one
//! unit): on day 1 the staff forms the landing and orders
//! `prepare_landing` with roster 1, the one unit it may use; on day 2
//! the landing is ready and the plan is not released, the reason naming
//! day 3; on day 3 the plan is released and 600 x 0.005 x 15 = 45 t of
//! supply are loaded; on day 4 unit 1 is set to attack and loaded; on
//! day 5 the force sails at hour 96, arrives at hour 112 and lands the
//! 600 men, and the fight is that evening. The garrison has dug at the
//! end of days 1 to 4: its fieldworks are 40.
//!
//! | The fight of day 5, worlds 1 and 3 | Allied | Japanese |
//! | --- | --- | --- |
//! | Men ashore | 600, on attack | 200, on defend, fieldworks 40 |
//! | Shots, all hits | 6 | 2 |
//! | Men lost in the fight | 2 x 50 = 100 | 6 x 50 = 300, held to the 200 there, cut by cover to 200 x (1 - 0.2) = 160 |
//! | Left | 500 | 40 |
//!
//! 40 is fewer than 0.2 x 500 = 100: beaten, and the 40 surrender. The
//! end is met on day 5. The side has lost 100 men and the enemy 200.
//!
//! | The fight of day 5, worlds 2 and 4 | Allied | Japanese |
//! | --- | --- | --- |
//! | Men ashore | 600, on attack | 200 and 200, on defend, fieldworks 40 |
//! | Shots, all hits | 6 | 4 |
//! | Men lost in the fight | 4 x 50 = 200 | 300, 150 from each unit, each cut to 150 x (1 - 0.2) = 120 |
//! | Left | 400 | 80 and 80: 160 |
//!
//! 160 is not fewer than 0.2 x 400 = 80: not beaten. Not met by day 5.
//! The side has lost 200 men and the enemy 240.
//!
//! **The answer** is of the kind `trial`, of preview 1, intent 1:
//!
//! | | With the draft | As it stands |
//! | --- | --- | --- |
//! | Met and kept | 2 of 4 worlds, worlds 1 and 3, the earliest, the middle and the latest day 5 | 2 of 4, worlds 1 and 3, on day 4 |
//! | Not met by day 5 | 2, worlds 2 and 4 | 2, worlds 2 and 4 |
//! | The side's men lost to the enemy | 100, 100, 200 | 100, 100, 200 |
//! | The trial's enemy's men lost to the enemy | 200, 200, 240 | 200, 200, 254 |
//! | Every other cost of either side | 0 | 0 |
//!
//! - **World for world,** the draft against the plan as it stands: met
//!   and kept in both in 2 worlds, 1 and 3; with the draft alone in
//!   none; as it stands alone in none; in neither in 2, worlds 2 and 4.
//! - **Where the worlds part with the draft,** by the 200 men believed,
//!   standing unknown: with them, worlds 2 and 4, met and kept in none,
//!   the side's men lost 200, 200, 200 and the enemy's 240, 240, 240;
//!   without them, worlds 1 and 3, met and kept in 2 on day 5, the
//!   side's men lost 100, 100, 100 and the enemy's 200, 200, 200.
//! - The side has no other plan to weigh, and the answer says so.
//! - The plan order's day expected ashore with the draft is reported by
//!   the build for the architect to state, and not asserted here.
//!
//! ## C. What does it cost me: without it
//!
//! On the same evening he asks `trial` of intent 1 with `compare` true,
//! at `small`.
//!
//! **The two trials.** The first has the three standing commands and an
//! empty course: the first date of A. The second has the standing
//! commands less commands 1 and 2, which leaves the leave: nothing is
//! ordered, no ship sails and no man is lost.
//!
//! **The answer:** how the intent went and what it cost are the first
//! date of A. Beside it, without the order: the side's men lost to the
//! enemy 0, 0, 0 and the trial's enemy's 0, 0, 0, and every other cost
//! 0. The intent is not there to have gone, and no count of worlds is
//! given for it. The side has no other plan to weigh.
//!
//! ## Nothing is changed
//!
//! - The session's hash of the game is the same before each call and
//!   after it. The seat's queue holds the same three commands.
//! - Preview 1 is not spent by its trial: `approve` of it afterwards
//!   queues the limit.
//! - `trials` lists three estimates, A, B and C, numbered 1 to 3 in the
//!   order asked, and gives each again unchanged.
//! - **The live enemy is not read.** The same scenario with the live
//!   garrison of B changed by 300 men, and the same calls: the three
//!   answers are the same, byte for byte.
//!
//! # Properties
//!
//! - **Asked twice, the same.** Each of A, B and C asked again on the
//!   same evening gives the same fields and words.
//! - **Where no `compare` is given.** `trial` of intent 1 with nothing
//!   more plays one trial: the side has no other plan. `trial` of
//!   preview 1 with `compare` false plays one trial, and its figures are
//!   B's "with the draft".
//! - **The same leaves in both trials.** Without command 3: the what-if
//!   of B has B's figures on both sides. Leave to launch is given by the
//!   trial in both trials, and the answer names it as given to the side
//!   by the trial. (Before this record the second trial was given no
//!   leave, and its landing would not have sailed.)
//! - **Another plan to weigh.** With a hold of base 1 queued as well,
//!   his intent 2: `trial` of intent 1 with no `compare` plays the second
//!   trial, without intent 1. Intent 2 is met and kept in 4 worlds with
//!   intent 1 and in 4 without, and is not named as displaced.
//! - **An order in force.** With the three commands taken in on day 1
//!   and the trial asked on the evening of day 1:
//!   - with no `compare`, one trial is played, and the answer says the
//!     order is in force;
//!   - with `compare` true, two are played, of days 2 to 6. The second
//!     is handed the withdrawal of intent 1 on the morning of day 2,
//!     before the staff is asked: nothing sails, the side's men lost
//!     are 0, 0, 0, and the answer says that the order is withdrawn on
//!     the coming morning, that what was already ordered stands, and
//!     that the withdrawal is the trial's. In the
//!     first the place is taken on day 4 in worlds 1 and 3. What worlds
//!     2 and 4 cost by day 6 is reported by the build for the architect
//!     to state, and not asserted here.
//! - **Dates given.** `trial` of intent 1 with `dates` of day 4 gives
//!   A's window, and so does `dates` of days 1 and 4.
//! - **A draft limit at later dates.** `trial` of preview 1 with `dates`
//!   chosen: the course is commands 1 and 2 and then the draft, and the
//!   dates are day 1 and day 4, chosen from the forces limit. Its first
//!   date has B's figures with the draft. The second date's figures are
//!   reported by the build, and not asserted here.
//! - **A draft at later dates.** On a fresh evening he previews the
//!   `set_intent` alone and asks `trial` of that preview with `dates`
//!   chosen. With no forces limit every transport force and fighting
//!   unit of the side counts, so day 4 is a date. It is answered with a
//!   window of two dates; its figures are reported by the build, and not
//!   asserted here.
//! - **Asked before it is played.** A with `describe` true: the answer
//!   gives the standing command, the two commands of the course, the
//!   dates day 1 and day 4 with unit 2 due on day 4, and 4 worlds for 5
//!   days at the first date and 8 at the second. B with `describe` true:
//!   two trials of 4 worlds for 5 days, the second the plan as it
//!   stands. Nothing
//!   is played, the hash is the same, and `trials` lists nothing more.
//! - **Refused, each with its own words and nothing played:**
//!   - `dates` with `compare` true;
//!   - `dates` of an intent in force, named by `intent` or by a preview
//!     of a limit on it;
//!   - `dates` of three later days; of days not ascending; of day 6,
//!     which is after the last day of the first date's play; and `dates`
//!     that is neither `chosen` nor a list of days;
//!   - with a second intent queued, a take of another place, and an
//!     "after intent 1" queued on it: `dates` of intent 1, and `compare`
//!     true of intent 1, each naming the "after";
//!   - a `compare` or a `describe` that is not true or false.
//! - **Nothing of the trial's enemy is named.** Every number of a unit,
//!   a task force or a base in each answer, fields and words, is one the
//!   seat's own tools give in the live game; no run of digits in it is
//!   900001 or 900002, with or without a comma. The enemy on the eve of
//!   a date is men and cover at a place.
//! - **The words** hold no per cent sign and none of "chance", "points",
//!   "better", "should", "recommend" or "advise", and none of the
//!   engine's own names decision 0178's hand calculation lists.
//! - **The answers are checked against their schemas.**
//! - **A game with no trial is as it was:** the recorded games replay
//!   as they did, but for the version of the agent's interface in each
//!   answer. Scenario 97's answers are as they were, but for that
//!   version.

// The tests are written by the build, to the statements above.
