//! Scenario 129: submarines, the patrol order and the staff. Where a
//! side's boats are put by an order, what holds a boat from the staff,
//! and how the staff keeps a standing force out of submarine waters.
//!
//! Proves the third stage of decision 0218: rules 11 and 12, 34, 54 and
//! 55 of the section "Amended on a second review", and the rulings on
//! rows 61, 62 and 68 of the differences. The data is placeholder data.
//! It stands on scenario 128 for everything a boat does once she has a
//! station.
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
//! An open sea of hexes q from 0 to 12 and r from -6 to 6, all navigable
//! but where a game says. The Japanese side has a base at (12,0) and the
//! placeholder boats of scenario 128, numbered from 1, all at (12,0) at
//! the start, sound, with both salvoes. The Japanese side has a commander
//! and a staff.
//!
//! Distance is the count of hexes. Hexes are ordered by q, then by r: the
//! lowest hex is the one that comes first so.
//!
//! ## Part A: the stations of a patrol order
//!
//! The order: patrol the waters within 2 hexes of (5,0) with submarines.
//! The waters are 19 hexes. The stations are given so: the first is the
//! centre; each next is the navigable hex of the waters farthest from the
//! nearest station already given, the lowest hex where they tie.
//!
//! Worked, each station with its distance from the nearest before it:
//!
//! | Station | Hex | Distance |
//! | --- | --- | --- |
//! | 1 | (5,0) | the centre |
//! | 2 | (3,0) | 2 |
//! | 3 | (3,2) | 2 |
//! | 4 | (5,-2) | 2 |
//! | 5 | (5,2) | 2 |
//! | 6 | (7,-2) | 2 |
//! | 7 | (7,0) | 2 |
//!
//! After the seventh every hex of the waters left is 1 from a station.
//! Stations go to boats in the boats' number order.
//!
//! 1. Game A1, seven boats and the order with no boat named: at the first
//!    dusk the staff gives boat n the station of row n. Each order is a
//!    station order of the engine, caused by the patrol order, and the
//!    staff's reason for each says the boat and her station.
//! 2. Game A2, three boats: stations 1 to 3 of the table.
//! 3. Game A3, twenty boats: the nineteenth station is the last hex of
//!    the waters, and boat 20 shares station 1, (5,0).
//! 4. Game A4, as A1 with (5,0) land: the first station is the navigable
//!    hex of the waters nearest the centre, the lowest where they tie:
//!    (4,0). No station is on land.
//! 5. Game A5, as A1 with the order naming boats 2, 5 and 6: those three
//!    take stations 1 to 3 in their number order and no other boat is
//!    ordered.
//! 6. A boat already on the station the order would give her is given no
//!    order again: on the second dusk of A1 the staff hands in none for
//!    the boats, and its reason says each keeps her station.
//! 7. A boat that is damaged, going home, home or sunk is passed over and
//!    her station goes to no one: in A1 with boat 3 damaged at the start,
//!    boats 1, 2, 4, 5, 6 and 7 take stations 1, 2, 4, 5, 6 and 7 as
//!    before. Stations do not move up.
//! 8. The preview of the order says each boat's station before it is
//!    given, and gives the same stations.
//!
//! ## Part B: whose hands a boat is in
//!
//! 9. A direct order to a boat to take a station holds her from the
//!    staff, as a direct order holds a force. Game B1, A1 with the
//!    commander ordering boat 2 to (9,3) at the first dusk: boat 2 goes to
//!    (9,3); the staff orders boats 1, 3, 4, 5, 6 and 7 to stations 1, 3,
//!    4, 5, 6 and 7 and says in its reason that boat 2 is the commander's.
//! 10. A hold may name a boat. Held with no order she keeps what she was
//!    doing and the staff gives her none. Released, she is the staff's at
//!    the next dusk and takes the station her place in the order gives.
//! 11. An order's forces limit may name boats: only they serve it (A5).
//!    A boat named by the forces limit of another order is not used by the
//!    patrol order.
//! 12. A side with no commander is given its patrol order by its scenario
//!    and its staff serves it the same way.
//!
//! ## Part C: the staff and submarine waters
//!
//! The Allied side has a commander and a staff, a carrier force free to
//! be stationed, and an order to deny the waters within 1 hex of (6,0).
//! With nothing else to weigh, the station the staff's rule chooses for
//! the carrier force is hex S; the test finds S by running game C1 and
//! reads the staff's reason for it. The statements are about what changes.
//!
//! 13. Game C1, no submarine belief: the carrier force is stationed at S,
//!    and no reason speaks of submarines.
//! 14. Game C2, C1 with suspected waters told on day 1 whose circle holds
//!    S and not every hex the rule could choose: the staff stations the
//!    force at the hex its rule ranks next that is outside the waters,
//!    and its reason says S was passed over for submarine waters.
//! 15. Game C3, C1 with a report of a submarine held at a neighbour of S:
//!    the same, S being within one hex of the report.
//! 16. Game C4, C1 with suspected waters that hold every hex the rule
//!    could choose: the staff stations the force at S and its reason says
//!    no place outside submarine waters serves.
//! 17. In C2 and C3 the force's route is as the route rule gives it; a
//!    route is not bent for submarine waters.
//! 18. The same regard is had by the doctrine that chooses where a force
//!    stands by night and where a force waits, each proved on one game as
//!    statements 14 and 16 are.
//! 19. A side's staff reads only what the side believes: with an enemy
//!    boat lying at S that the side holds no report of and no suspected
//!    waters, the staff stations the force at S (C1 with the boat added).
//!
//! ## Part D: what a scenario may not give
//!
//! 20. A scenario that gives a boat no salvo is refused when it is read,
//!    with her number in the reason.
//! 21. A scenario whose boat is to go home to a base that no sea route
//!    reaches from her station is refused when it is read.
//!
//! ## Part E: nothing else moves
//!
//! 22. A game with no boat, no report of a submarine and no suspected
//!    waters gives the orders, the reasons and the hash it gave: the
//!    staffed scenarios on file are compared before and after.
