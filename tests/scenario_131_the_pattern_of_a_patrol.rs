//! Scenario 131: the pattern of a patrol (decision 0223 as amended on review,
//! stages 0 and A).
//!
//! # The architect's statements
//!
//! Written by the architect on Nov 23, 2026, before any code. Nobody else
//! changes a statement or a hex here; a mismatch goes back to the architect.
//! The hexes are worked by hand below.
//!
//! ## The map and the games
//!
//! One placeholder map, `placeholder_patterns.toml`: deep sea for q from -10
//! to 10 and r from -8 to 8, with a coast hex under each base a game has and
//! nothing else unless a game says so. Hexes are axial (q, r); the distance
//! between two hexes is (|dq| + |dr| + |dq + dr|) / 2. Every boat is a
//! placeholder boat of scenario 128's placeholder class, on station at her
//! side's home base at the start, and "boat N" is the side's Nth boat in
//! number order. Each game is three days; the stations are read from the
//! staff's orders of the first morning the patrol is served and from
//! `preview`'s and `propose`'s `stations`.
//!
//! The line games (the Japanese side has the boats; its home base is at
//! (-9, 0); the patrol's centre is (0, 0)):
//!
//! - **L1:** pattern `lines`; one Allied base at (8, 0); waters within 3
//!   hexes; nine boats.
//! - **L2:** as L1 with waters within 6 hexes and fifteen boats.
//! - **L3:** as L1 with (-1, 1) a hex of land and six boats.
//! - **L4:** pattern `lines`; one Allied base, at (4, 0); waters within 3;
//!   three boats.
//! - **L5:** as L1 with a second Allied base at (0, -7) and three boats.
//! - **L6:** as L1 with no pattern given to the Japanese side, and three
//!   boats.
//!
//! The base games (the Allied side has the boats; its home base is at
//! (8, 0); the Japanese bases are base 1 at (-8, 0), base 2 at (-6, 3) and
//! base 3 at (-2, 0); the patrol's centre is (-6, 0), waters within 3):
//!
//! - **B1:** pattern `off_bases`; seven boats.
//! - **B2:** as B1 with (-3, 0) a hex of land and four boats.
//! - **B3:** pattern `off_bases`; the patrol's centre is (4, -4), waters
//!   within 2; two boats.
//!
//! ## Stage 0: a scenario says how a side's staff uses its boats
//!
//! 1. A scenario may give a side `submarine_pattern`, `off_bases` or
//!    `lines`, and `expects_boats_off_own_bases`, true or false. A side
//!    given neither has no pattern and expects nothing.
//! 2. Any other word for the pattern is refused when the scenario is
//!    loaded, naming the side and the word.
//! 3. A side's own picture holds its pattern and what it expects. The other
//!    side's picture, and every answer of every tool to the other side's
//!    seat, holds neither.
//! 4. Every scenario on file that gives no pattern plays as it did: the
//!    comparison of every run with the hashes cut shows no difference, and
//!    scenario 129's stations are what they were.
//!
//! ## Stage A: lines
//!
//! 5. **L1.** The approach is from the Allied base at (8, 0). The boats are
//!    stationed, boat 1 to boat 9, at (0, 0), (-1, 1), (0, -1), (-1, 2),
//!    (1, -2), (-2, 3), (1, -3), (-3, 0) and (2, 1).
//! 6. **L1.** The reason of each of the first seven station orders says the
//!    boat is on the first line across the approach from that base, naming
//!    the base; the reason of the last two says they are spread over the
//!    waters, the line being full.
//! 7. **L2.** The boats are stationed, boat 1 to boat 15, at (0, 0),
//!    (-1, 1), (0, -1), (-1, 2), (1, -2), (-2, 3), (1, -3), (-2, 4),
//!    (2, -4), (-3, 5), (2, -5), (-3, 6), (3, -6), (-5, 0) and (-6, 1). The
//!    reason of the last two says they are on the second line.
//! 8. **L3.** The line steps over the land: the boats are stationed, boat 1
//!    to boat 6, at (0, 0), (-1, 2), (0, -1), (-2, 3), (1, -2) and (1, -3).
//!    No boat is given (-1, 1).
//! 9. **L4.** The only enemy base lies within 5 hexes of the centre, so
//!    there is no approach and no line: the boats are stationed as decision
//!    0218 stations them, at (0, 0), (-3, 0) and (-3, 3), and the reason
//!    says there is no line and why.
//! 10. **L5.** The approach is from the base at (0, -7), the nearer of the
//!     two that lie more than 5 hexes from the centre: the boats are
//!     stationed at (0, 0), (1, -1) and (-1, 0).
//! 11. **L6.** With no pattern the boats are stationed as decision 0218
//!     stations them: at (0, 0), (-3, 0) and (-3, 3).
//! 12. **L1, the second morning.** No boat is given a station order: each
//!     keeps the station she was given, and the one decision to give no
//!     order names each as keeping it.
//!
//! ## Stage A: off bases
//!
//! 13. **B1.** The boats are stationed, boat 1 to boat 7, at (-7, 0),
//!     (-6, 2), (-3, 0), (-8, 1), (-7, 3), (-6, -3) and (-4, -2).
//! 14. **B1.** The reason of boats 1 and 4 says each is off base 1, of
//!     boats 2 and 5 off base 2, and of boat 3 off base 3, each naming the
//!     base; the reason of boats 6 and 7 says they are spread over the
//!     waters. Base 3 is given one boat and not two, and the staff's words
//!     say why: no second hex next to it lies in the waters.
//! 15. **B2.** Base 3 has no hex next to it that ships can enter and that
//!     lies in the waters, and is passed over: the boats are stationed at
//!     (-7, 0), (-6, 2), (-8, 1) and (-7, 3).
//! 16. **B3.** No enemy base lies in the waters or within a hex of them:
//!     the boats are stationed as decision 0218 stations them, at (4, -4)
//!     and (2, -4).
//! 17. In every game no boat is stationed on land, on a base's own hex, or
//!     outside the waters, and no two boats of one patrol share a station.
//!
//! ## The hexes, worked by hand
//!
//! **The line.** For an approach whose vector from the centre is (q, r),
//! the vector across it is (-(q + 2r), 2q + r), divided by the greatest
//! common divisor of its two parts. The line is drawn from the centre to
//! the centre plus that vector, and on from there by the same vector, and
//! the same with the vector's opposite: between two ends N hexes apart the
//! hexes are the N + 1 points at even steps, each rounded to the nearest
//! hex, the lowest hex (q, then r) where two are equally near. The first
//! hand is the vector as worked, the other its opposite. Stations are the
//! centre, then one hex farther along the first hand, then along the other,
//! by turns; when one hand has left the waters the other goes on.
//!
//! *L1, L2, L3, L6's geometry: approach (8, 0).* Across: (-(8), 16), which
//! is (-1, 2) eight times. From (0, 0) to (-1, 2) is 2 hexes: the middle
//! point is (-0.5, 1), equally near (-1, 1) and (0, 1), so (-1, 1); then
//! (-1, 2). On to (-2, 4): middle (-1.5, 3), so (-2, 3); then (-2, 4). On
//! to (-3, 6): middle (-2.5, 5), so (-3, 5); then (-3, 6). The other hand,
//! from (0, 0) to (1, -2): middle (0.5, -1), equally near (0, -1) and
//! (1, -1), so (0, -1); then (1, -2); then (1, -3) (middle (1.5, -3)),
//! (2, -4), (2, -5) (middle (2.5, -5)), (3, -6).
//!
//! Distances from the centre: (-1, 1) 1, (-1, 2) 2, (-2, 3) 3, (-2, 4) 4,
//! (-3, 5) 5, (-3, 6) 6, (-4, 7) 7; (0, -1) 1, (1, -2) 2, (1, -3) 3,
//! (2, -4) 4, (2, -5) 5, (3, -6) 6. So within 3 hexes the first line has 7
//! stations and within 6 it has 13.
//!
//! *L1's last two boats.* The second line would pass through (-5, 0), five
//! hexes from the centre away from the enemy, which is outside waters of 3
//! hexes: there is none. The two boats left are spread: each goes to the
//! hex of the waters farthest from the nearest station already given. No
//! hex of waters of 3 hexes is more than 3 from (0, 0), and four are 3 from
//! every station of the line: (-3, 0), (2, 1), (3, 0) and (3, -1). (For
//! (-3, 0): 3 from (0, 0), 3 from (-1, 1), 3 from (0, -1). For (2, 1): 3
//! from (0, 0), 3 from (-1, 1), 3 from (-1, 2). The other hexes at 3 from
//! the centre are within 2 of a station: (-3, 1) of (-1, 1), (-3, 2) of
//! (-1, 2), (3, -2) of (1, -2), (3, -3) of (1, -3), and so round.) The
//! lowest is (-3, 0). With (-3, 0) given, (2, 1), (3, 0) and (3, -1) are
//! still 3 from every station; the lowest is (2, 1).
//!
//! *L2's last two boats.* The second line passes through (-5, 0), which is
//! 5 from the centre and in waters of 6. From (-5, 0) to (-6, 2): middle
//! (-5.5, 1), equally near (-6, 1) and (-5, 1), so (-6, 1), which is 6 from
//! the centre. Boat 14 has (-5, 0) and boat 15 the next along the first
//! hand, (-6, 1).
//!
//! *L3.* (-1, 1) is land. The first hand's first hex ships can enter is
//! (-1, 2), then (-2, 3); (-2, 4) is outside. The other hand: (0, -1),
//! (1, -2), (1, -3). By turns: (0, 0), (-1, 2), (0, -1), (-2, 3), (1, -2),
//! and the first hand having left the waters, (1, -3).
//!
//! *L4 and L6, the spread of decision 0218.* The centre; then the hex of
//! the waters farthest from it, where all the hexes 3 away tie and the
//! lowest is (-3, 0); then the hex farthest from both: (-3, 3) is 3 from
//! each, and no hex with q below -3 is in the waters, and (-3, 1) and
//! (-3, 2) are 1 and 2 from (-3, 0).
//!
//! *L5: approach (0, -7).* The base at (0, -7) is 7 from the centre and
//! the base at (8, 0) is 8; both are more than 5, and the nearer is taken.
//! Across: (-(0 - 14), 0 - 7) = (14, -7), which is (2, -1) seven times.
//! From (0, 0) to (2, -1) is 2 hexes: the middle point is (1, -0.5),
//! equally near (1, -1) and (1, 0), so (1, -1). The other hand, to
//! (-2, 1): middle (-1, 0.5), equally near (-1, 0) and (-1, 1), so (-1, 0).
//!
//! **Off bases.** *B1.* Distances of the bases from the centre (-6, 0):
//! base 1 at (-8, 0) is 2, base 2 at (-6, 3) is 3, base 3 at (-2, 0) is 4,
//! which is one hex outside the waters. In that order:
//!
//! - Base 1's neighbours and their distances from the centre: (-7, 0) 1,
//!   (-7, -1) 2, (-8, 1) 2, (-8, -1) 3, (-9, 0) 3, (-9, 1) 3. Boat 1 has
//!   (-7, 0).
//! - Base 2's: (-6, 2) 2, (-5, 2) 3, (-7, 3) 3, (-5, 3) 4, (-6, 4) 4,
//!   (-7, 4) 4. Boat 2 has (-6, 2).
//! - Base 3's: (-3, 0) 3, (-3, 1) 4, (-2, -1) 4, (-1, 0) 5, (-2, 1) 5,
//!   (-1, -1) 5. Only (-3, 0) is in the waters. Boat 3 has it.
//! - The second round: base 1's nearest hexes not taken are (-7, -1) and
//!   (-8, 1), both 2 away; the lowest is (-8, 1), boat 4. Base 2's are
//!   (-5, 2) and (-7, 3), both 3; the lowest is (-7, 3), boat 5. Base 3
//!   has none.
//! - The spread, for boats 6 and 7: the hexes of the waters 3 from the
//!   nearest of the five stations are (-6, -3), (-5, -3), (-4, -3),
//!   (-3, -3) and (-4, -2), and none is farther. The lowest is (-6, -3).
//!   With it given, (-4, -2) and (-3, -3) are still 3 from every station
//!   and (-5, -3) and (-4, -3) are not; the lowest is (-4, -2).
//!
//! *B2.* With (-3, 0) land, base 3 has no hex. Boats 1 and 2 as in B1;
//! the second round gives boats 3 and 4 (-8, 1) and (-7, 3).
//!
//! *B3.* The Japanese bases are 11 hexes and more from (4, -4). The centre,
//! then the lowest of the hexes 2 away: those with q = 2 are (2, -4),
//! (2, -3) and (2, -2), and the lowest is (2, -4).
