//! Scenario 128: submarines against warships. A boat on her station, the
//! forces that pass through her hex, her salvo, the hunt, aircraft that
//! find her, and what each side is told.
//!
//! Proves decision 0218 as amended on its second review (the section
//! "Amended on a second review", which governs, with rules 1 to 26 where
//! it leaves them standing). The data is placeholder data: every boat,
//! ship, aircraft and figure here is made up for the proof and named
//! "Placeholder". It is several small games, one for each part, so that
//! no part's numbers rest on another's draws.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything stated is
//! asserted exactly. If the code disagrees, the code or the rule is
//! wrong: do not change a statement here to make a test pass. A mismatch
//! goes back to the architect.
//!
//! Every quantity is a fixed-point number of six decimal places and every
//! multiplication and division is cut toward zero at the sixth place, as
//! `Fixed` does. The numbers below are worked so.
//!
//! ## The figures
//!
//! The scenario's own, placeholders all:
//!
//! | Figure | Value |
//! | --- | --- |
//! | `submarine_attacks_per_hour` | 0.02 |
//! | `attacks_ship_dead_in_water` | 5 |
//! | `held_down_pulses` | 2 |
//! | `hunt_damages_chance` | 0.3 |
//! | `hunt_sinks_share` | 0.2 |
//! | `submarine_surfaced_share` | 0.25 |
//! | `air_attack_damages_chance` | 0.25 |
//! | `air_attack_sinks_share` | 0.2 |
//!
//! The placeholder boat class: a salvo of 4, 2 salvoes carried, a passage
//! speed of 10 knots, the placeholder torpedo. The placeholder torpedo: a
//! `submarine_hit_chance` of 0.5 and the damage of scenario 4a's torpedo,
//! so that a hit is worked exactly as that scenario works one.
//!
//! A hex is 40 nm. The night pulse is 12 hours, the morning and the
//! afternoon 6 each.
//!
//! ## Part A: the hours and the chance to attack
//!
//! Boat 1, Japanese, on her station at (5,0), sound, with both salvoes.
//! One Allied task force in each game, of one placeholder carrier and one
//! placeholder destroyer, neither dead in the water. The hourly chance is
//! p = 0.02 and its miss q = 0.98. The miss for a pulse is q multiplied by
//! itself once for each whole hour, then by 1 - p x f for the part f of an
//! hour left over; the chance is one less that.
//!
//! | Game | The force in the pulse | Hours in (5,0) | Miss | Chance |
//! | --- | --- | --- | --- | --- |
//! | A1 | lies stopped at (5,0) all the night | 12 | 0.784713 | 0.215287 |
//! | A2 | lies stopped at (5,0) all the morning | 6 | 0.885841 | 0.114159 |
//! | A3 | crosses (5,0) at 30 knots, entering and leaving in the morning | 40/30 = 1.333333 | 0.973467 | 0.026533 |
//! | A4 | crosses at 20 knots in the morning | 2 | 0.960400 | 0.039600 |
//! | A5 | crosses at 15 knots in the morning | 40/15 = 2.666666 | 0.947594 | 0.052406 |
//! | A6 | arrives at (5,0) with 3 hours of the morning left and stops | 3 | 0.941192 | 0.058808 |
//! | A7 | passes through (4,0) and (6,0) and never enters (5,0) | 0 | none | no draw |
//!
//! Worked: 0.98 squared is 0.960400; cubed 0.941192; the sixth power
//! 0.885841 and the twelfth 0.784713, each product cut at the sixth
//! place. A3: f = 0.333333, p x f = 0.006666, 1 - 0.006666 = 0.993334, and
//! 0.98 x 0.993334 = 0.973467. A5: f = 0.666666, p x f = 0.013333,
//! 0.960400 x 0.986667 = 0.947594.
//!
//! Statements:
//!
//! 1. In each of A1 to A6 exactly one attack chance is recorded for boat
//!    1 and the force in that pulse, with the hours and the chance of the
//!    table. In A7 none is recorded and no draw of the attack is made.
//! 2. The force's hours in a hex are from when it enters to when it
//!    leaves: half a leg before the hex's centre and half after. In A3 the
//!    force is in (5,0) for one hex's steaming, not two.
//! 3. A ship dead in the water multiplies the hourly chance by 5: p =
//!    0.1. Game A8, the force of A2 with its carrier dead in the water:
//!    miss 0.9 to the sixth, 0.531441; chance 0.468559. Game A9, the same
//!    all the night: miss 0.282428 (0.9 to the twelfth, cut at each
//!    product); chance 0.717572.
//! 4. A boat that is held down, on passage, damaged or without a salvo
//!    makes no draw. Games A10 to A13, the force of A1 with the boat in
//!    each state: no attack chance is recorded.
//! 5. A force lying in a hex that holds a base of its own side is not
//!    attacked. Game A14, A1 with an Allied base at (5,0): none recorded.
//! 6. A force with none of a carrier, a battleship, a transport, an
//!    auxiliary or a cruiser is not attacked. Game A15, A1 with two
//!    destroyers only: none recorded.
//!
//! ## Part B: which force, which ship, the salvo
//!
//! 7. Of several forces she attacks one. Game B1: three Allied forces lie
//!    at (5,0) all the night, force 1 of two cruisers, force 2 of a
//!    transport and a destroyer, force 3 of a battleship and a cruiser.
//!    One attack chance is recorded for each, 0.215287, and each has its
//!    own draw. On a seed on which more than one draw succeeds she fires
//!    at force 3 where its draw succeeded, else at force 2, else at force
//!    1: battleship before transport before cruiser. She fires once.
//! 8. She aims at one ship: the first by the order of kinds, a ship dead
//!    in the water before another of its kind, then the lowest number.
//!    Game B2: a force of carriers 1 and 2, carrier 2 dead in the water,
//!    and a battleship: on every seed on which she fires, at carrier 2.
//! 9. A salvo is 4 torpedoes, each its own draw of the boat, the ship and
//!    its place in the salvo, each hitting at 0.5. Over the seeds on which
//!    she fires in game A1 the hits on the aimed ship run from 0 to 4 and
//!    no other ship is ever hit. Her salvoes go from 2 to 1.
//! 10. Each hit is worked as scenario 4a works a torpedo hit, with the
//!    damage of the placeholder torpedo.
//! 11. With one salvo left she fires again at a later pulse and then has
//!    none: she leaves for the base her scenario names, at 10 knots, and
//!    is taken off the map there. She attacks nothing on the way.
//!
//! ## Part C: the hunt
//!
//! 12. A force with a warship that is not the ship aimed at hunts her,
//!    hit or miss. In game A1 on every seed on which she fires she is held
//!    down for the rest of that pulse and the 2 pulses after: no attack
//!    chance is recorded for her in those pulses, and one is recorded in
//!    the third.
//! 13. The hunt is one stream of the boat and the force: a first draw
//!    under 0.3 damages her; of those, a second under 0.2 sinks her. Over
//!    the seeds the test runs each of the three ends happens: sound,
//!    damaged, sunk. The chance she is sunk is 0.3 x 0.2 = 0.06; that she
//!    is damaged and afloat 0.24; untouched 0.7.
//! 14. Damaged, she leaves for home and attacks nothing. Sunk, she is
//!    gone at once and her side is told in that pulse.
//! 15. A force of one ship, the ship aimed at, hunts nothing. Game C1: a
//!    lone placeholder cruiser lies at (5,0): on every seed on which she
//!    fires she is not held down and may fire again in the next pulse.
//!
//! ## Part D: aircraft
//!
//! Game D1. Allied air group 1 at a base at (0,0): 6 placeholder search
//! aircraft, a sweep width of 20 nm, on the search mission with an arc of
//! 60 degrees that holds (5,0), 200 nm away. No ship of either side but
//! the boat.
//!
//! The chance of sighting one force there is 57.295779 x 6 x 20 / (200 x
//! 60) = 6875.493480 / 12000 = 0.572957.
//!
//! 16. Boat 1 on her station at (5,0): in each daylight pulse one chance
//!    is recorded for the boat and the group, 0.572957 x 0.25 = 0.143239.
//!    None is recorded in the night pulse.
//! 17. Boat 1 on passage through (5,0): the chance is 0.572957 whole.
//! 18. Boat 1 on her station at (0,0), the group's own hex: the distance
//!    is taken as one hex, 40 nm, so the chance of sighting a force is
//!    6875.493480 / 2400, cut to 1, and the boat's is 0.25. It is not 1.
//! 19. Sighted, she is reported: the Allied side gains one report of a
//!    submarine, in its own list, at her hex or, where the error of a
//!    report struck, a neighbour. Its contacts of ships are unchanged and
//!    no force of its side is alerted by it.
//! 20. Sighted, she is held down for the rest of that pulse and the next,
//!    and the aircraft attack her in the same pulse: one stream of the
//!    boat and the group, a first draw under 0.25 damages her, of those a
//!    second under 0.2 sinks her. Sunk 0.05, damaged and afloat 0.2,
//!    untouched 0.75. No aircraft is lost, no bomb is used and no sortie
//!    is added to the group's count for the day.
//!
//! ## Part E: what each side is told
//!
//! 21. Of her salvo her own side is told that she fired, the kind she
//!    took her target for, and her claim. It is given no contact of the
//!    force and no report of ships.
//! 22. The side fired on is told of each salvo, hit or miss, with no boat
//!    named and no number of hers, and gains one report of a submarine at
//!    her hex through the report's error.
//! 23. Two reports of a submarine in one hex are one, the newer. A report
//!    of "nothing there" for that hex removes no report of a submarine.
//! 24. A side's view of its own boats is exact; the other side's view
//!    holds no boat, only its reports of submarines. No event of the boat's
//!    passage, station, salvoes, state or held-down count is seen by the
//!    other side.
//! 25. A report of a submarine ages and is dropped by its own figures,
//!    and is in every command's picture where command levels are on.
//!
//! ## Part F: suspected waters
//!
//! 26. Game F1: the Allied side is given suspected waters, (5,0) and 2
//!    hexes round, told on day 2. On day 1 its briefing and view hold
//!    none; from day 2 they hold them, with the words of the belief. The
//!    Japanese side, given none, holds none. The waters do not move when a
//!    boat is reported elsewhere.
//!
//! ## Part G: nothing else moves
//!
//! 27. A scenario with no boat and no suspected waters has the hash it
//!    had: scenario 4a and scenario 5b give their hashes on file.
//! 28. No rule made for a task force touches a boat: in a game where an
//!    Allied surface force with battle readiness lies in her hex all a
//!    night and she does not fire, no surface fight is recorded; where a
//!    carrier's groups on naval strike are in reach of her report, no
//!    strike is launched at it.
//! 29. Every draw of this rule has its own `Draw` value in a subsystem of
//!    its own, and two games that differ only in a boat that never meets a
//!    force give every other draw the same.
//!
//! The station order, holds on a boat, the patrol order, the staff's
//! regard for submarine waters and the table are the third stage's and
//! are proved there.
