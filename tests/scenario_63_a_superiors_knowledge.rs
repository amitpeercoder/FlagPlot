//! Scenario 63: what a superior knows of his forces, and radio silence.
//! Scenario 58's carrier force, silent by the scenario, closes, strikes
//! and comes home, and its area command holds it where it last reported.
//!
//! Proves decision 0136, step 3 of stage 2b, against a hand calculation.
//! The data is placeholder data: `data/scenarios/63_a_superiors_knowledge.toml`
//! is scenario 58's file with command levels on, report figures of its
//! own and task force 1 set to radio silence. It runs on the frozen step 1
//! figures as scenario 58 does. There is no Japanese unit. Every chance in
//! the report figures is 0 or 1, so no outcome rests on a die.
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
//! As scenario 58. The report figures: on time 1 in every direction.
//!
//! ## The game
//!
//! Every order, arrival and launch is scenario 58's: twelve orders on day
//! 1, two on day 2, none on day 3. Task force 1, at press, is given no
//! order about radio silence: the staff leaves the setting as it finds
//! it. She is at (6,0) from hour 8 of day 1, strikes in the morning of
//! day 1, and is back at (0,0) from hour 32, in the night of day 2.
//!
//! ## What the area command holds of task force 1
//!
//! She is silent, so she sends a state report only in a pulse in which
//! she fights, is attacked or launches: the morning of day 1.
//!
//! | After | She is at | The area command holds her at | As of |
//! | --- | --- | --- | --- |
//! | The start | (0,0) | (0,0) | the start |
//! | The night of day 1 | (6,0) | (0,0) | the start |
//! | The morning of day 1 | (6,0) | (6,0) | the morning of day 1 |
//! | The afternoon of day 1 | (6,0) | (6,0) | the morning of day 1 |
//! | The night of day 2 | (0,0) | (6,0) | the morning of day 1 |
//! | Every later pulse | (0,0) | (6,0) | the morning of day 1 |
//!
//! What it holds of her ship's fuel and of her groups is what they were
//! at the end of the morning of day 1, in every later pulse: group 3 with
//! its 10 in the air or back as the truth then had them, and ship 1's
//! fuel after the 240 nm of the night and not after the 240 nm home.
//!
//! Base 1 is not silent: the area command holds it as of the pulse just
//! ended, after every pulse.
//!
//! ## The reports
//!
//! "Nothing there" at (12,0) is task force 1's, made in the morning of
//! day 1, a pulse in which she launched: she sends it. The area command
//! and base 1 drop contact 1 in the morning, as she does.
//!
//! No "state report late" and no "state report lost" event is recorded:
//! every one sent arrives on time, and those not sent are not sent.
//!
//! # Properties
//!
//! - **Not silent.** With the silence taken out: the area command holds
//!   task force 1 as of the pulse just ended, after every pulse: at (6,0)
//!   after the night of day 1 and at (0,0) after the night of day 2.
//! - **A pulse late.** With the silence taken out and up a pulse late 1:
//!   after each pulse the area command holds her as of the pulse before:
//!   after the night of day 1, as of the start, at (0,0); after the
//!   morning of day 1, as of the night, at (6,0); after the night of day
//!   2, as of the afternoon of day 1, at (6,0); after the morning of day
//!   2, as of the night of day 2, at (0,0). A "state report late" event in
//!   every pulse for each of task force 1 and base 1.
//! - **Never.** With the silence taken out and up never: the area command
//!   holds her at (0,0) as of the start in every pulse, and base 1
//!   likewise. A "state report lost" event in every pulse for each. The
//!   area command never hears "nothing there" and holds contact 1 to 5
//!   pulses.
//! - **A held report is sent when she breaks silence.** In a view built
//!   for it: a silent force that makes a sighting in pulse 3 and launches
//!   a strike in pulse 5 sends the sighting in pulse 5. Her superior's
//!   picture has no contact after pulses 3 and 4 and has it after pulse
//!   5, at age 2.
//! - **The staff sets silence at preserve the force.** With the scenario's
//!   silence taken out and intent 1 at preserve the force: the orders are
//!   those of the same file with command levels off, and one more on day
//!   1, radio silence on, to task force 1. With no report figures loaded
//!   the order is not given.
//! - **The commander's order.** `set_radio_silence` off, to task force 1
//!   on day 2, is taken: from the night of day 2 the area command holds
//!   her as of the pulse just ended.
//! - **The seat.** The area commander's list of his forces gives task
//!   force 1 at (6,0), "as of the morning of day 1", on day 3, and base 1
//!   with no such words.
//! - **An order that meets a different world.** In the main case, a
//!   commander's order on day 3 that routes task force 1 is taken or
//!   refused by the engine as it would be with command levels off; where
//!   it is refused for a cause the held state did not show, the refusal
//!   says what the area command held and that it was of the morning of
//!   day 1. Tested in a view built for it, with a force the area command
//!   holds afloat that is sunk.
//! - **Every chance certain and no silence is step 2.** Scenario 62 and
//!   scenario 61 give what they gave. On live figures with report figures
//!   of on time 1 in every direction, the Eastern Solomons from intents
//!   (seeds 1 to 10) and the week of the slice from intents (seed 1) give
//!   the orders and reasons they give with command levels off, but for
//!   the words decision 0135 added.
//! - **A supply run on stale word.** Scenario 36, keeping a base supplied,
//!   with command levels on and up never: what the area staff does is
//!   reported by the build, with its reasons, for the architect to read
//!   before the step is merged. It is not asserted here.
//! - **Off is unchanged.** Every scenario built before this one gives the
//!   events it gave.
//! - The same start gives the same game. The Japanese side sees none of
//!   the Allied side's state report events.
