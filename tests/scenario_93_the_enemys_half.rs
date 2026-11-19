//! Scenario 93: the enemy's half of a picture, by rule. From what a side
//! holds and believes, `sim-trial` makes the enemy it will be tried
//! against: bases, men, a force for each contact, forces believed in and
//! not seen, each where it could be, and what the enemy is taken to want.
//!
//! Proves decision 0174, the third part of step 4 of step 3's stage 4,
//! against a hand calculation. The data is placeholder data: see
//! `data/scenarios/93_the_enemys_half.toml`, which the build writes to the
//! inputs below. It runs on the frozen step 1 figures. Each case is that
//! scenario with the contacts, estimates and beliefs the case names.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly, but for the one count over worlds, which has its allowed
//! range. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Inputs
//!
//! - **The map:** the placeholder map of scenario 1. Every hex is sea
//!   but the two coast hexes of the bases, and a force may be in any.
//! - **The classes,** from `ship_classes.csv`: 1, a cruiser; 2, a
//!   destroyer; 3, a transport; 4, a carrier of 36 aircraft. All cruise
//!   at 16 kn but the transport, at 10 kn. **The types,** from
//!   `aircraft_types.csv`: 3, a bomber; 4, a fighter.
//! - **A file of sides** that gives each of those classes and types to
//!   both sides.
//! - **Bases:** base 1, Allied, at (0,0); base 2, Japanese, at (8,3),
//!   on the Allied side's map.
//! - **Allied:** task force 1, one ship of class 1, at base 1, with no
//!   order. **Japanese:** nothing afloat. Nobody moves in any case.
//! - **The figures:** the merging distance is 2 hexes; a contact is
//!   dropped at 5 pulses; an enemy base is given 1,000 t of supply and
//!   500 t of fuel. A hex is 40 nm. Counted back from an evening, the
//!   pulses are the afternoon, 6 hours; the morning, 6; the night, 12.
//! - **The trial:** seed 1942, trial 1, and the worlds each case names.
//!   Unless a case says otherwise the picture is of the evening before
//!   day 1 (day 0) and the Allied side is the side tried.
//!
//! What the enemy's half is made of is read from the picture; what was
//! assumed is read from the list that comes with it.
//!
//! ## 1. Nothing held, nothing believed
//!
//! The Allied side holds no contact and believes nothing. In every world
//! the enemy's half is base 2 at (8,3) with 1,000 t of supply and 500 t
//! of fuel, and nothing else: no ship, no force, no group, no man. The
//! assumptions name the base and its two figures, and say that the enemy
//! is given no belief and no intent. Worlds 1 and 2 are the same picture.
//!
//! ## 2. A carrier contact of 5 ships
//!
//! One contact: 5 ships taken for a carrier, at (5,2), age 0.
//!
//! - **Ships,** in the order made: 900,001, class 4, the carrier;
//!   900,002 to 900,005, class 2, destroyers. Each undamaged, with the
//!   fuel its class holds, in task force 900,001.
//! - **Task force 900,001:** Japanese, at (5,2), on the carrier mission,
//!   stopped, at cruise, normal readiness, no route, no home port.
//! - **Air groups,** aboard ship 900,001: 900,001, fighters, 36 / 3 =
//!   12 aircraft of type 4; 900,002, bombers, 36 - 12 = 24 of type 3.
//!   All ready, at rest.
//! - It is at (5,2) in every world: a contact of age 0 is where it was
//!   reported.
//!
//! With the file of sides giving class 4 to the Allied side alone: the
//! carrier is not built, the force is the four destroyers, 900,001 to
//! 900,004, on the carrier mission with no air group, and the
//! assumptions say one carrier was left out for want of a class.
//!
//! ## 3. A transport contact
//!
//! One contact: 3 ships taken for a transport, at (5,2), age 0. Three
//! ships of class 3, 900,001 to 900,003, in task force 900,001 on the
//! transport mission. No destroyer.
//!
//! ## 4. A claim raises the kind; a claim of sinkings takes nothing
//!
//! One contact: 3 ships taken for a cruiser, at (5,2), age 0.
//!
//! - **As it stands:** ship 900,001 of class 1 and ships 900,002 and
//!   900,003 of class 2; task force 900,001 on the surface combat
//!   mission.
//! - **With a claim of the Allied side's,** from the air, of 1 hit on a
//!   carrier at (6,2) on day 0 (the test adds it to the side's half; a
//!   hex is 1 from the contact, inside the 2 of merging): ship 900,001
//!   is of class 4, with its two air groups of 12 and 24; ships 900,002
//!   and 900,003 are of class 2; the mission is carrier. The assumptions
//!   say the claim raised the kind.
//! - **With that claim at (8,2),** 3 hexes away: as it stands, a
//!   cruiser.
//! - **With a claim from the air of 2 ships sunk and 4 hits on a
//!   cruiser at (5,2):** three ships, as it stands. Nothing is taken
//!   away.
//!
//! ## 5. An older contact is where it could have steamed
//!
//! - **One pulse old.** One contact: 3 cruisers at (5,2), age 1. Its
//!   hours are the afternoon's 6; at 16 kn that is 96 nm, 2 steps. In
//!   each of worlds 1 to 64 the force is at a hex no more than 2 steps
//!   from (5,2), one of 19; it is not at the same hex in all of them;
//!   and world 7 asked twice gives the same hex. The assumptions give
//!   the 6 hours, the 16 kn, the 96 nm and the 2 steps.
//! - **Three pulses old,** a day. Age 3: 6 + 6 + 12 = 24 hours, 384 nm,
//!   9 steps. In each of worlds 1 to 64 the force is within 9 steps of
//!   (5,2).
//! - **Slower ships reach less far.** 3 transports at (5,2), age 1:
//!   6 hours at 10 kn, 60 nm, 1 step. In each of worlds 1 to 64 the
//!   force is within 1 step of (5,2), one of 7 hexes.
//!
//! ## 6. Two reports that may be one force
//!
//! Two contacts: the first, 3 cruisers at (2,2), age 2; the second, 2
//! cruisers at (6,2), age 0. They are 4 hexes apart. The pulses between
//! their ages are two, the afternoon and the morning: 12 hours at 16 kn,
//! 192 nm, 4 steps; with the merging distance, 6. 4 is no more than 6:
//! they may be one.
//!
//! - **In a world where they are one:** one force, at (6,2), of 3
//!   ships, one cruiser of class 1 and two destroyers.
//! - **In a world where they are two:** a force of 2 ships at (6,2), one
//!   cruiser and one destroyer; and a force of 3 ships, one cruiser and
//!   two destroyers, within 4 steps of (2,2).
//! - **Over worlds 1 to 1000** the worlds where they are one number
//!   between 437 and 563: an even chance, a mean of 500, a standard
//!   deviation of the root of 250, 15.81, and 3.94 of them either side.
//!   The assumptions of each world say which it is, and the same world
//!   asked twice is the same.
//! - **With the first at (-1,2),** 7 hexes from the second: they are two
//!   forces in every world.
//!
//! ## 7. The forces believed in and not seen
//!
//! No contact. The picture is of the evening of day 2, after two days in
//! which nothing happens. Eight worlds, 1 to 8.
//!
//! - **Believed present:** a cruiser force of 3 ships, last known at
//!   (5,2) before the first day. It is built in all 8 worlds: one
//!   cruiser and two destroyers. Its hours are 2 days, 48; at 16 kn,
//!   768 nm, 19 steps. In every world it is within 19 steps of (5,2).
//!   The assumptions give the 48 hours, the 768 nm and the 19 steps.
//! - **Believed absent:** the same force, standing absent. Built in no
//!   world. The assumptions say the side ruled it out.
//! - **Unknown:** the same force, standing unknown. Built in worlds 2,
//!   4, 6 and 8 and not in worlds 1, 3, 5 and 7: 4 of 8.
//! - **Two unknown,** that cruiser force and then a carrier force of 5
//!   ships, place not known. The first is in worlds 2, 4, 6 and 8; the
//!   second in worlds 3, 4, 7 and 8. Both in 4 and 8; neither in 1 and
//!   5. The carrier force, its place not known, is at base 2's hex,
//!   (8,3), the one base of the enemy's on the map.
//! - **A contact stands for it.** In a picture of the evening before
//!   day 1, where a contact given at the start is still held: the
//!   carrier force unknown and of no known place, and a contact of 2
//!   ships taken for a carrier at (5,2), age 0:
//!   in every world the enemy has one carrier force, the contact's, of
//!   2 ships, and the believed force is not built. The assumptions say
//!   the contact was taken for it. With the contact taken for a
//!   cruiser instead, the believed carrier force is built in its
//!   worlds, beside the contact's cruiser force.
//!
//! ## 8. The men ashore
//!
//! An estimate of the Allied side's: 600 men at (8,3). The enemy's half
//! has land unit 900,001, a fighting unit of 600 men, ashore at (8,3),
//! on defend.
//!
//! ## 9. What the enemy wants
//!
//! The case of 8, and each world built and resolved for two days with
//! both staffs.
//!
//! - **Believed to hold base 2:** at the end of day 1 the Japanese
//!   side's command state has one intent of the commander's, to hold
//!   base 2, set on day 1, and both its leaves are given.
//! - **No belief of what he wants:** the Japanese side has no intent,
//!   and its staff gives no order on either day.
//! - In both, the Japanese side's belief is empty when the world is
//!   built.
//!
//! ## 10. Each hard case gives a world
//!
//! Each of these pictures builds a world that resolves for two days with
//! both staffs, its invariants holding: case 1 (no contact); case 5's
//! day-old contact; case 6 in a world of each kind; case 4's claim of
//! two sunk; case 4's claim that raises a cruiser to a carrier (a
//! contact of the wrong kind); case 7's unknown carrier in a world that
//! has it and in one that has not (an enemy carrier never seen).
//!
//! # Properties
//!
//! - **No peek.** Scenario 68's pair of worlds that differ only in what
//!   the Allied side cannot see, each played to the evening of day 5:
//!   the Allied side's pictures of worlds 1 to 8 are equal, world for
//!   world, field for field, and so are their assumptions.
//! - **The same twice.** The same view, beliefs, seed, trial and world
//!   give the same picture and the same assumptions.
//! - **Another trial, other draws.** Case 5's contact one pulse old, in
//!   trial 2: its hexes over worlds 1 to 64 are not, world for world,
//!   those of trial 1.
//! - **Nothing of the live game is read but the view.** The crate's own
//!   source test passes: it names no umpire, no whole picture and no
//!   rebuild on a live seed.
//! - **The live game is not touched.** Making 64 pictures and building
//!   and resolving their worlds leaves the live game's hash what it was.
//! - **No live game changes** by the build: no baseline, golden file or
//!   pinned hash moves.

// The tests are written by the build, to the statements above.
