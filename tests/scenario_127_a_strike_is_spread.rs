//! Scenario 127: a strike is spread. Three raids attack a force of three
//! carriers in one pulse. The first draws its carrier among those that do
//! not look lost; each raid after it takes the next.
//!
//! Proves decision 0211 as amended and measured (its section of Nov 22,
//! 2026, which governs). The data is placeholder data. It stands on
//! scenario 5b's hand calculation for the decision to strike and the bomb
//! hits.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything stated is
//! asserted exactly. If the code disagrees, the code or the rule is
//! wrong: do not change a statement here to make a test pass. A mismatch
//! goes back to the architect.
//!
//! ## The setting
//!
//! Task force 1, Allied, at (4,0): three placeholder carriers, ships 1, 2
//! and 3. Aboard ship 1, air group 1, 18 dive bombers; aboard ship 2, air
//! group 2, 18 dive bombers, and air group 3, 12 torpedo bombers; aboard
//! ship 3, air group 4, 18 dive bombers. All on the naval strike mission
//! with the default target list, on which a carrier ranks first.
//!
//! Task force 2, Japanese, stopped at (9,0), 200 nm away: three
//! placeholder carriers, ships 11, 12 and 13, and two cruisers, ships 14
//! and 15. No aircraft, no patrol.
//!
//! The Allied side starts holding a contact of task force 2 at (9,0), age
//! 0, so every group launches in the morning. The game is one day. The
//! live figures, with `strike_is_spread` present.
//!
//! ## The raids
//!
//! A raid is one ship's strike at one hex in a pulse. Three raids attack
//! task force 2 in the morning: ship 1's (group 1), ship 2's (groups 2 and
//! 3), ship 3's (group 4). Their order is that of their first groups: 1,
//! 2, 4. All want carriers.
//!
//! ## The first raid draws
//!
//! None of ships 11, 12 and 13 looks lost at the start of the morning's
//! air combat. The first raid makes one draw, d, from 0 up to but not 1,
//! and takes the ship at place floor(3 x d) in the order 11, 12, 13: under
//! one third ship 11, under two thirds ship 12, otherwise ship 13.
//!
//! ## The raids after it
//!
//! The second raid takes the next ship after the first's in the order 11,
//! 12, 13, going round; the third the next after the second's. So the
//! three raids attack three different carriers, on every seed:
//!
//! | First draws | Raid of ship 1 | Raid of ship 2 | Raid of ship 3 |
//! | --- | --- | --- | --- |
//! | ship 11 | 11 | 12 | 13 |
//! | ship 12 | 12 | 13 | 11 |
//! | ship 13 | 13 | 11 | 12 |
//!
//! Each row happens on some seed of those the test runs.
//!
//! Groups 2 and 3, of one raid, attack the same carrier. No group attacks
//! a cruiser. The later raids make no draw.
//!
//! The first raid's attack records, in its breakdown, the ships it could
//! have drawn (11, 12, 13), the draw and the ship drawn; each later
//! raid's, the ship it took by turn and the ship the raid before it took.
//!
//! Every other draw of the pulse, the fighters', the guns' and each
//! group's shots, is the draw the build without this rule makes for that
//! group and force: the test asserts each group's shots and hits are
//! those of the same seed without the rule, where the ship attacked is of
//! the same class. (The three carriers are of one class, so they are.)
//!
//! # Properties
//!
//! - **One looks lost.** With ship 12 starting with her flight deck out:
//!   the first raid draws between ships 11 and 13, under one half ship
//!   11; the second takes the other; the third goes round to the first
//!   raid's ship. Ship 12 is not attacked on any seed.
//! - **Crippled and burning.** The same with ship 12 starting with
//!   flotation damage at `crippled_flotation`, and with fire damage at
//!   `claimed_sunk_fire`, each in place of the deck.
//! - **All look lost.** With all three decks out: the draw is among the
//!   three, and the raids go round as in the table.
//! - **One afloat.** With ships 12 and 13 removed: every raid attacks ship
//!   11; the first still makes its draw, among one.
//! - **Another kind.** With group 4's target list set to rank cruisers
//!   first: the raid of ship 3 takes its own turn among the cruisers, and
//!   being the first that wants cruisers it draws between ships 14 and
//!   15; the raids of ships 1 and 2 are the first and second of those
//!   that want carriers.
//! - **A raid of two kinds.** With group 3's list set to rank cruisers
//!   first and group 2's unchanged: the raid of ship 2 takes its turn
//!   among carriers by its first group, group 2; group 3 attacks the
//!   lowest-numbered cruiser that does not look lost, as a group chooses
//!   today, and takes no turn.
//! - **Start of the step.** With a seed on which the first raid sinks its
//!   carrier: the third raid, going round, is not in question here (three
//!   carriers, three raids); with a fourth raid added (a fourth Allied
//!   carrier, ship 4, group 5) the fourth raid goes round to the first
//!   raid's carrier, sunk or not, the choice being read at the start of
//!   the step.
//! - **Off where the switch is absent.** On the frozen figures every
//!   group attacks ship 11, as before this rule, and every event of the
//!   frozen strike scenarios is byte for byte what it was.
//! - The same seed gives the same game; two worlds that differ only in
//!   what the Allied side cannot see of ships not in task force 2 give
//!   the same choice.
