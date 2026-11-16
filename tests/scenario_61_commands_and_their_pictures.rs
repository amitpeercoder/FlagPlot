//! Scenario 61: commands and their pictures. Scenario 58, the carrier
//! force that closes on a contact, played with command levels on: it
//! plays as scenario 58 does, and every Allied command holds the side's
//! picture.
//!
//! Proves decision 0134, step 1 of stage 2b, against a hand calculation.
//! The data is placeholder data: `data/scenarios/61_commands_and_their_pictures.toml`
//! is scenario 58's file with command levels turned on and nothing else
//! changed. It runs on the frozen step 1 figures as scenario 58 does.
//! There is no Japanese unit and no Japanese base, so no die is rolled.
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
//! As scenario 58 (`tests/scenario_58_a_carrier_force_closes.rs`): Allied
//! base 1 at (0,5); task force 1, one carrier, at (0,0); one contact held
//! at the start, 1 ship, a carrier, at (12,0), age 0; intent 1, operate
//! task force 1 from (0,0), at press, both leaves given; 3 days.
//!
//! ## The commands
//!
//! At the start the Allied side has three commands: its area command;
//! task force 1, under the area command; base 1, under the area command.
//! The Japanese side has one: its area command. Air groups 1, 2 and 3
//! belong to task force 1.
//!
//! ## The game
//!
//! Every order, every reason, every arrival and every event is scenario
//! 58's: twelve orders on day 1, two on day 2 and none on day 3; the
//! force at (6,0) at hour 8; the strike of 10 in the morning of day 1,
//! which finds nothing; the force back at (0,0) at hour 32.
//!
//! ## The pictures
//!
//! | When | The side's picture | Each of the three Allied commands' |
//! | --- | --- | --- |
//! | The start | Contact 1: 1 ship, a carrier, at (12,0), age 0 | The same |
//! | After the night of day 1 | Contact 1, age 1 | The same |
//! | After the morning of day 1 | No contact: the strike found nothing there and the contact is dropped | No contact |
//! | Every later pulse | No contact | No contact |
//!
//! The Japanese area command's picture holds nothing at any time, as its
//! side's does.
//!
//! The strike of the morning of day 1 is decided on task force 1's
//! picture: its reason, or the launch's record, names task force 1 as the
//! command whose picture was read. The staff's orders to task force 1 and
//! its groups are decided on task force 1's picture, and the reasons that
//! name contact 1 name it by the number it has there, which is 1.
//!
//! # Properties
//!
//! - **On is off.** The same file with command levels off gives the same
//!   events, the same orders and reasons, and the same state in every
//!   field but the commands.
//! - **Balanced.** The balanced property of scenario 58, with command
//!   levels on: ten orders on day 1, none on day 2, two on day 3. Contact
//!   1 is in all three Allied pictures at ages 1, 2 and 3 after the three
//!   pulses of day 1, at 4 after the night of day 2, and is dropped from
//!   all of them at the end of the morning of day 2, at 5 pulses, in the
//!   pulse the side's own drops it.
//! - **A force formed in the game.** With a commander's order on day 2
//!   that detaches nothing (the carrier is alone), no command is added.
//!   The forming of a command is tested where a force can be formed: in a
//!   view built for it, a force detached from task force 1 is a command
//!   under the area command, and its picture is a copy of task force 1's.
//! - **A base that changes side** stops being a command of the side that
//!   lost it and becomes one of the side that took it, with a copy of that
//!   side's area command's picture: in a view built for it.
//! - **Bigger games, on is off.** On live figures, with command levels
//!   turned on and nothing else changed: the Eastern Solomons from intents
//!   (`data/scenarios/second-proof/05a_eastern_solomons.toml`) on seeds 1
//!   to 20, Savo Island (scenario 4) on seeds 1 to 20, and the week of the
//!   slice from intents (`data/scenarios/second-proof/07a_*.toml`) on
//!   seeds 1 to 3, each give the events they give with command levels
//!   off, and at the end of every pulse every command's picture holds what
//!   its side's holds: the same contacts with the same numbers, places,
//!   counts, kinds and ages, the same claims and the same estimates.
//! - **Off is unchanged.** Every scenario built before this one gives the
//!   events it gave.
//! - Nothing that plays is given another command's picture or the truth:
//!   a command's view returns its own picture only.
