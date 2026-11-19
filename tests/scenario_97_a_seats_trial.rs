//! Scenario 97: a seat's trial. A commander at his seat queues a landing
//! and, before a day is resolved, asks what it would cost. He is given
//! the estimate, in his own words and numbers; the game is as it was.
//!
//! Proves decision 0178, the first part of step 8 of step 3's stage 4,
//! against a hand calculation. The data is placeholder data: see
//! `data/scenarios/97_a_seats_trial.toml`, which the build writes to the
//! inputs below. It runs on the frozen step 1 figures, through a session
//! as `sim-cli` opens one, with the Allied seat played and the Japanese
//! seat scripted.
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
//! Scenario 95 (see its hand calculation), with one change: its file of
//! the trial's figures names the size `small` as 4 worlds for 8 days,
//! `standard` as 8 worlds for 8 days and `full` as 16 worlds for 8 days.
//! So: unit 1, 600 fighting men at A, and task force 1 of two troop
//! transports there; base 2 at B, Japanese, held in the live game by
//! 1,200 men; the Allied side's opening estimate of B is 200 men, and it
//! believes 400 more may be there, standing unknown; an enemy base is
//! assumed to hold 200 t of supply; scenario 68's ground figures, with
//! no die. The scenario has no commands.
//!
//! ## What the commander does
//!
//! On the evening before day 1, at the Allied seat, by the seat's own
//! tools:
//!
//! 1. `set_intent`: take the place at (4,0). It is his intent 1.
//! 2. `set_limit`, forces, on intent 1: task force 1, unit 1, and base 1
//!    as the source.
//! 3. `set_leave`: leave to launch, given.
//! 4. `trial`, of intent 1, at the size `small`.
//!
//! The three commands are queued for the morning and are not yet taken
//! in. Leave to prepare is the side's from the start.
//!
//! ## The course
//!
//! The standing commands are the three he queued, in the order queued;
//! the scenario has none of its own. Intent 1 is queued, so it may be
//! tried. Both leaves are the side's or queued: the course adds nothing.
//! The trial is trial 1, of 4 worlds for 8 days.
//!
//! This is scenario 95's trial: the same picture, the same beliefs and
//! figures, the same worlds, and the same four commands taken in on the
//! first morning (scenario 95's course gives leave to prepare as well,
//! which the side already has).
//!
//! ## The answer
//!
//! Of the kind `trial`. Its estimate is number 1. What was asked:
//! intent 1, the size `small`, 4 worlds for 8 days. Its fields are
//! scenario 95's estimate of intent 1:
//!
//! - **How it went:** 4 of 4 worlds played; met and kept in 2, worlds 1
//!   and 3, the earliest, the middle and the latest day 4; met and lost
//!   again in none; not met by day 8 in 2, worlds 2 and 4.
//! - **What it cost the side:** men lost to the enemy 100, 100, 300;
//!   every other cost 0, 0, 0.
//! - **What the trial's enemy lost:** men lost to the enemy 200, 200,
//!   300; every other cost 0, 0, 0.
//! - **What it needs and has:** men wanted 400, to hand 600; it waits
//!   for nothing; the day expected ashore is day 4; a count of the
//!   enemy at the place is asked for and not held.
//! - **What was assumed:** the enemy's base at B with 200 t of supply
//!   and 0 t of fuel, in 4 worlds; 200 men at B from the side's
//!   estimate, in 4; 400 men believed at B, standing unknown, in 2 and
//!   not in 2; the enemy given no belief, in 4; the enemy given no
//!   intent, in 4. No leave is named as given by the course: the
//!   commander queued it himself.
//! - **Where the worlds part,** by the 400 men believed at B: with
//!   them, 2 worlds, met and kept in none, the side's men lost to the
//!   enemy 300, 300, 300; without them, 2 worlds, met and kept in 2 on
//!   day 4, the side's men lost 100, 100, 100.
//!
//! In the answer the base is named as the seat's tools name a base, the
//! place as they give a place, the task under the plan order in the
//! words the tool `order` uses for it ("ready a landing at ... by ..."),
//! and task force 1 and unit 1 by the side's own numbers. No number of
//! a unit made for the enemy (900,001 or 900,002) is anywhere in it.
//!
//! Its words name "2 of 4 worlds"; they hold no per cent sign, neither
//! "chance" nor "points", and none of these marks of the engine's own
//! names: "TaskForceId", "LandUnitId", "Hex {", "ReadyLanding",
//! "TakePlace", "intent:", "base:" followed by a number alone.
//!
//! ## Nothing is changed
//!
//! - The session's hash of the game is the same before the call and
//!   after it.
//! - The seat's queue holds the same three commands, and `command_state`
//!   and `staff_orders` answer as they did before the call.
//! - The day is then resolved as the commander meant, and days 1 to 4
//!   of the live game are those of the game in which `trial` was never
//!   called: the same daily hashes. (In the live game the 600 men meet
//!   1,200 on day 4, not the 200 or 600 of the trial: the trial was not
//!   told.)
//!
//! # Properties
//!
//! - **Asked twice, the same.** A second `trial` of intent 1 at `small`
//!   on the same evening is answered with the same fields and words, as
//!   estimate 2. `estimates` then lists two, each with the evening it
//!   was asked on and what was asked; and gives estimate 1 again by its
//!   number, equal to the first answer.
//! - **A larger size is more of the same worlds.** At `standard`, 8
//!   worlds: met and kept in 4, worlds 1, 3, 5 and 7; not met in 4;
//!   the side's men lost to the enemy 100, 100, 300. Worlds 1 to 4 of
//!   it have the figures they had at `small`.
//! - **A draft, tried.** On a fresh evening the commander asks
//!   `preview` of the `set_intent` alone, and then `trial` of that
//!   preview. It is tried and answered; nothing is queued; the preview
//!   is not spent and may still be approved. Its figures are reported
//!   by the build for the architect to state, and not asserted here:
//!   with no forces limit the staff assembles the force itself
//!   (decision 0149), and with leave to launch given by the course and
//!   not by the side, which the answer names.
//! - **An intent in force.** With the three commands taken in on day 1
//!   and the trial asked on the evening of day 1: the standing commands
//!   are none, the course is empty, and the trial is of days 2 to 9.
//!   The landing is ashore on day 4 in every world as before: met and
//!   kept in 2 of 4 on day 4, not met by day 9 in 2.
//! - **Refused, each with its own words and nothing played:**
//!   - at the seat of a command, in a scenario with command levels;
//!   - in a scenario that names no file of the trial's figures: the
//!     game has no trial;
//!   - while a day is open, after the night pulse of day 1 with the
//!     clock stopped;
//!   - with both `intent` and `preview`, and with neither;
//!   - intent 7, which the side does not have;
//!   - the staff's own intent, the landing it formed, on the evening of
//!     day 1;
//!   - a preview that is void, a command having been queued after it;
//!   - the size `vast`.
//! - **The tools are listed** for a seat with a staff in a game that
//!   has a trial, and `trial` is refused by name in one that has none.
//! - **The agent's library names no picture and no world built from
//!   one:** its source test passes unchanged in what it bans.
//! - **A game with no trial is as it was:** the recorded games replay
//!   as they did, but for the version of the agent's interface in each
//!   answer.

// The tests are written by the build, to the statements above.
