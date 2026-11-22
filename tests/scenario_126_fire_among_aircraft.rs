//! Scenario 126: fire among a carrier's aircraft. A carrier's bombers
//! strike a carrier that has her own bombers aboard. A raid whose bombs
//! hit may set them alight; the fire is fought from the next pulse, rises
//! or falls a level at each pulse's end, and loses her if it rises above
//! the top.
//!
//! Proves decision 0212 as amended and measured (its sections of Nov 22,
//! 2026, which govern), with a figure by side from decision 0213. The data
//! is placeholder data. It stands on scenario 5b's setting and hand
//! calculation for the search, the decision to strike and the bomb hits.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything stated as a
//! number is asserted exactly, in the game's own arithmetic. If the code
//! disagrees, the code or the rule is wrong: do not change a number or a
//! statement here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## The setting
//!
//! Scenario 5b's, with these differences. The game is 3 days. Carrier B,
//! ship 3, Japanese, has aboard air group 3, 20 placeholder dive bombers,
//! Japanese, on the rest mission, and air group 4, 10 placeholder
//! fighters, on the rest mission: neither flies. The figures are the live
//! figures' shape with these values of the scenario's own (a copy of the
//! air combat figures, as scenario files of its kind have):
//! `aircraft_fire_chance` Japanese 0.6, Allied 0.15;
//! `fire_spreads_chance` 0.5; `aircraft_fire_levels` 3.
//!
//! ## The share
//!
//! Carrier B's strike aircraft are the 20 dive bombers of group 3: the
//! fighters are not counted. None flies in the pulse. Aboard 20 of 20: the
//! share is 1.
//!
//! ## A raid's chance
//!
//! For a raid that puts b bomb hits on carrier B and does not sink her,
//! the chance is 1 - (1 - 0.6 x 1)^b:
//!
//! - 1 hit: 0.6
//! - 2 hits: 1 - 0.4^2 = 0.84
//! - 3 hits: 1 - 0.4^3 = 0.936
//! - 4 hits: 1 - 0.4^4 = 0.9744
//!
//! A hit counts whichever track of damage it landed on. A raid with no
//! hit makes no draw and records nothing.
//!
//! The record of the fire set (for every raid that drew, whether or not
//! it set one) gives: the bombs that hit, 20 bombers aboard, 0 flying,
//! the share 1, the chance above for its hits, the draw, and her fire
//! among aircraft after. It cites the raid's attacks on her.
//!
//! ## What the draw does
//!
//! Over the seeds the test runs, in every pulse in which a raid hit
//! carrier B: where the draw recorded is under the chance, her fire among
//! aircraft is 3 after it, and the pulse it was set is that pulse; where
//! it is not, her fire is what it was. Both happen on some seed.
//!
//! ## The fire fought
//!
//! At the end of the pulse in which a fire was set or raised to the top
//! by a raid: no draw, and no record of a rise or fall.
//!
//! At the end of each pulse after it, while she is afloat and her fire is
//! 1 or more: one draw. Under 0.5 it rises a level; otherwise it falls
//! one. The record gives the level before, the draw, and the level after.
//!
//! - From 3, a rise is above the top: she is lost. `ShipSunk` is recorded
//!   with fire as its cause, citing the fire's record, and with her go her
//!   30 aircraft and her men as a sinking takes them.
//! - From 3, a fall leaves 2; from 2, a rise 3 and a fall 1; from 1, a
//!   rise 2 and a fall nothing: the fire is out, no draw is made after it,
//!   and she flies again.
//! - The night's end is a pulse's end like the others.
//!
//! The chances this comes to, stated and not asserted on seeds: a fire set
//! at the top is lost at the end of the next pulse one time in two, and in
//! the end three times in four.
//!
//! ## A carrier with a fire among her aircraft
//!
//! While her fire is 1 or more she flies nothing off and lands nothing on:
//! her groups launch nothing whatever their mission, and aircraft of hers
//! in the air when it is set look for another deck or a base as when a
//! deck is out. Her flight deck's own state is not changed by the fire,
//! her fire damage is not changed by it, she is not counted as crippled
//! for it, and she does not "look lost" for it (decision 0211).
//!
//! ## Bombed again
//!
//! A raid that sets a fire on a carrier already burning puts it at 3
//! again, and that pulse is the one it was last set in: no draw at its
//! end.
//!
//! # Properties
//!
//! - **Half away.** With group 3 at 20 dive bombers of which 10 fly in the
//!   pulse (the test arranges a launch that takes 10): the share is 0.5,
//!   and the chances are 0.3 for 1 hit, 0.51 for 2, 0.657 for 3.
//! - **All away, and none.** With all 20 flying the share is nothing, the
//!   chance is nothing, and no draw is made. With no bomber in her groups
//!   (group 3 removed, the fighters kept) the same.
//! - **The other navy.** With the sides exchanged, so that the carrier
//!   bombed is Allied with 20 bombers aboard: 0.15 for 1 hit, 0.2775 for
//!   2, 0.385875 for 3; with 10 of 20 flying, 0.075, 0.144375 and
//!   0.208546875.
//! - **Not a carrier.** A cruiser of task force 2 with a floatplane group
//!   aboard, bombed: no draw, no record, nothing of this rule.
//! - **A torpedo and a shell set no such fire.** With the attackers
//!   torpedo bombers that hit with torpedoes: no draw.
//! - **Fire damage is as before.** Every ship's fire damage on every seed
//!   is what the build without this rule gives, on every pulse up to the
//!   first at which a carrier is lost to a fire among aircraft or closed
//!   by one.
//! - **Sunk by the raid.** A raid whose hits sink her makes no draw.
//! - **Off where the figures are absent.** On the frozen figures, which
//!   have none of the four, every event of scenario 5b is byte for byte
//!   what it was.
//! - **By side.** With no value by side given, both sides read the plain
//!   value; a side's view gives its own value in both places and not the
//!   enemy's (decision 0213).
//! - The same seed gives the same game; the hash holds her fire among
//!   aircraft and the pulse it was set, and a save and replay agree.
