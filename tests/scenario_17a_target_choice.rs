//! Which enemy ship a ship fires at (decision 0078).
//!
//! Guns go like at like: a ship fires at enemy ships of its own kind; if none
//! is afloat, at those of the nearest more important kind; if none of those,
//! at those of the most important kind afloat; and among them at the ship in
//! place (p modulo their number), where p is the firing ship's place among
//! the ships of its own kind in its force. Torpedoes go at the enemy ships of
//! the most important kind afloat, at the ship in place (i modulo their
//! number), where i is the firing ship's place in its force. Kinds in order
//! of importance: carrier, battleship, cruiser, destroyer, transport,
//! auxiliary.
//!
//! # The hand calculation (the architect's; nobody else changes a number)
//!
//! Checked by the coordinating architect on Nov 13, 2026, before any code.
//!
//! ## Common inputs
//!
//! The frozen step 1 figures and the placeholder classes
//! (`data/reference/step1`). A cruiser C has 8 guns at 3 a minute, gun damage
//! 20, 8 tubes, torpedo damage 200, toughness 400. A destroyer D has 4 guns
//! at 6 a minute, gun damage 5, 8 tubes, torpedo damage 200, toughness 100. A
//! cargo ship T (kind transport) has no guns and no tubes, toughness 300. A
//! round is 5 minutes.
//!
//! Every case is one fight by day with both forces at battle readiness, so
//! each sights in round 1 at chance 1 and nobody is surprised. Nothing burns
//! in round 1. Experience and the shares of a force not ready are off on the
//! frozen figures.
//!
//! So every gun salvo of a C or a D is 120 shots (8 x 3 x 5 and 4 x 6 x 5)
//! x 0.05 = 6 expected hits, and every torpedo salvo is 8 shots x 0.02 = 0.16
//! expected, fired once, in round 1.
//!
//! Points a hit, 100 x damage / toughness:
//!
//! | Weapon | On C | On D | On T |
//! | --- | --- | --- | --- |
//! | C's gun | 5 | 20 | 6.666666 |
//! | D's gun | 1.25 | 5 | 1.666666 |
//! | A torpedo | 50 | 200 | 66.666666 |
//!
//! Ships are numbered in ID order, force X first. p is a ship's place among
//! the ships of its own kind in its force and i its place in its force, both
//! from 0. Unless a case says otherwise only round 1 is asserted, and each
//! force's orders keep it in the fight (a break-off level of 1).
//!
//! ## Case 1: guns with the firer's own kind present
//!
//! X: C1, C2, D3, D4. Y: C5, D6, D7, D8.
//!
//! | Firer | Guns at | Why | Torpedoes at | Why |
//! | --- | --- | --- | --- | --- |
//! | C1 | C5 | Y's cruisers [C5], p 0 | C5 | Y's most important kind, cruisers [C5], i 0 |
//! | C2 | C5 | p 1, modulo 1 | C5 | i 1 |
//! | D3 | D6 | Y's destroyers [D6, D7, D8], p 0 | C5 | i 2 |
//! | D4 | D7 | p 1 | C5 | i 3 |
//! | C5 | C1 | X's cruisers [C1, C2], p 0 | C1 | X's cruisers [C1, C2], i 0 |
//! | D6 | D3 | X's destroyers [D3, D4], p 0 | C2 | i 1 |
//! | D7 | D4 | p 1 | C1 | i 2 |
//! | D8 | D3 | p 2, modulo 2 | C2 | i 3 |
//!
//! ## Case 2: guns with no own kind, and a more important kind afloat
//!
//! X: D1, D2, D3. Y: C4, C5, T6, T7.
//!
//! - X's guns: Y has no destroyer; the nearest more important kind afloat is
//!   cruiser [C4, C5]. By p 0, 1, 2: D1 at C4, D2 at C5, D3 at C4.
//! - X's torpedoes at the most important kind, cruisers, by i: C4, C5, C4.
//! - Y's guns: C4 and C5 have no cruiser in X and nothing more important, so
//!   they fire at the most important kind afloat, destroyers [D1, D2, D3]:
//!   C4 (p 0) at D1, C5 (p 1) at D2.
//! - Y's torpedoes at X's destroyers by i 0 and 1: C4 at D1, C5 at D2.
//! - T6 and T7 fire nothing, and nobody fires at a cargo ship.
//!
//! ## Case 3: guns with no own kind and nothing more important
//!
//! X: C1, C2. Y: D3, D4, T5, T6.
//!
//! - X's guns: Y has no cruiser and nothing more important, so the most
//!   important kind afloat, destroyers [D3, D4]: C1 at D3, C2 at D4. Not the
//!   cargo ships.
//! - X's torpedoes the same: C1 at D3, C2 at D4.
//! - Y's guns: D3 (p 0) and D4 (p 1) have no destroyer in X; the nearest more
//!   important kind is cruiser: D3 at C1, D4 at C2.
//! - Y's torpedoes by i: D3 at C1, D4 at C2.
//!
//! ## Case 4: torpedoes at the most important kind while the guns go elsewhere
//!
//! X: D1, D2. Y: C3, D4, D5.
//!
//! - X's guns at Y's destroyers [D4, D5]: D1 at D4, D2 at D5. 6 expected
//!   each, 5 points a hit.
//! - X's torpedoes at Y's cruisers [C3]: both at C3. 0.16 expected each, 50
//!   points a hit.
//! - So D1 and D2 each have a gun salvo and a torpedo salvo at different
//!   ships in round 1, the gun salvo recorded first.
//! - Y's guns: C3 has no cruiser in X and nothing more important: destroyers
//!   [D1, D2], p 0: D1. D4 (p 0) at D1, D5 (p 1) at D2.
//! - Y's torpedoes at X's most important kind, destroyers, by i 0, 1, 2: C3
//!   at D1, D4 at D2, D5 at D1.
//!
//! ## Case 5: more firers than targets, and fewer
//!
//! X: C1, C2, C3, C4, C5. Y: C6, C7.
//!
//! - X's guns and torpedoes (one kind, so p and i are equal): C6, C7, C6, C7,
//!   C6.
//! - Y's: C6 at C1, C7 at C2. C3, C4 and C5 are not fired on.
//!
//! This is the pairing the rule gave before decision 0078, exactly.
//!
//! ## Case 6: a target sunk in the round, and the places counted again
//!
//! This case alone alters the figures and a class, so that it is determined:
//! `gun_hit_chance_day` is 1, `system_hit_chance_per_point` is 0 (so no hit
//! knocks a system down and C1's second salvo is whole), and C1 is of a
//! cruiser class of the test's own with toughness 4000, the placeholder
//! cruiser otherwise. Both forces' break-off level is 1, so that Y, with one
//! ship of two lost, fights on.
//!
//! X: C1, T2. Y: D3, D4.
//!
//! Round 1.
//! - C1's guns: Y has no cruiser and nothing more important: destroyers [D3,
//!   D4], p 0: D3. 120 shots, 120 hits, 100 x 20 / 100 = 20 points each: D3
//!   is sunk at the round's end.
//! - C1's torpedoes at the most important kind afloat, destroyers, i 0: D3.
//! - T2 fires nothing.
//! - Y's guns: D3 (p 0) and D4 (p 1) have no destroyer in X; the nearest more
//!   important kind is cruiser [C1]: both at C1. 120 hits each, 100 x 5 /
//!   4000 = 0.125 points a hit: 240 x 0.125 = 30 points in all. C1 lives.
//! - Y's torpedoes at X's most important kind, cruisers [C1]: both at C1.
//! - D3, sunk in round 1, fired in round 1.
//!
//! Round 2. Y afloat is [D4].
//! - C1's guns at Y's destroyers [D4]: D4. 120 shots.
//! - D4, now p 0, at C1. 120 shots.
//! - No torpedo salvo: both sides fired theirs in round 1.
//!
//! ## Case 7: an unarmed ship counting for its place
//!
//! X: T1, C2, C3. Y: C4, C5, C6.
//!
//! - X's guns go by place among the firer's own kind, so the cargo ship does
//!   not shift them: C2 (p 0) at C4, C3 (p 1) at C5.
//! - X's torpedoes go by place in the force, where T1 holds place 0: C2 (i 1)
//!   at C5, C3 (i 2) at C6.
//! - Y's guns: C4, C5, C6 (p 0, 1, 2) at X's cruisers [C2, C3]: C2, C3, C2.
//! - Y's torpedoes at X's most important kind afloat, cruisers [C2, C3], by
//!   i 0, 1, 2: C2, C3, C2.
//! - The cargo ship is not fired on while a cruiser is afloat.
//!
//! ## Case 8: mixed kinds against one ship, and the order of the salvos
//!
//! X: C1, D2. Y: C3.
//!
//! - X's guns: C1 at Y's cruisers [C3]: C3. D2 has no destroyer in Y; the
//!   nearest more important kind is cruiser: C3.
//! - X's torpedoes at the most important kind, cruisers: both at C3.
//! - Y's guns: C3 (p 0) at X's cruisers [C1]: C1. Her torpedoes at X's most
//!   important kind, cruisers: C1. D2 is not fired on.
//!
//! The rule before decision 0078 gives the same targets (C1 and D2 at C3; C3
//! at C1). So the event log of this fight is the log the same scenario gave
//! before the change, byte for byte: the salvos in the order C1 gun, C1
//! torpedo, D2 gun, D2 torpedo, C3 gun, C3 torpedo, on the same streams. The
//! log and the hash were captured on main before the change and are held in
//! the test.
//!
//! ## For every case
//!
//! The test reads each `Salvo` event's firer, target, weapon, shots, chance
//! and expected hits, and the points of each hit from the damage events, and
//! asserts that no salvo exists that is not listed above.
