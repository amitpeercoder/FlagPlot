//! Scenario 117: the way a force takes. A carrier force the staff has
//! stationed is given a way that keeps it out of the enemy's reach by
//! day, within how late its risk lets it arrive and what its fuel allows.
//!
//! Proves decision 0203, stage 1. The statements are the architect's,
//! written before the code. A mismatch goes back to the architect, and
//! nobody else changes a statement or a number here.
//!
//! # The setting
//!
//! A placeholder scenario made for the test, on a placeholder map of open
//! sea from q -16 to 16 and r -16 to 16, with no real ship. Hexes are 40
//! nm and are named (q,r). It is scenario 116's setting with a wider map
//! and the force further off.
//!
//! Side A has a carrier force C: one carrier and two destroyers, on the
//! carrier mission, stopped at sea at (16,0). Aboard are a bomber group
//! whose normal strike radius is 210 nm, a fighter group and a group that
//! searches. C cruises at 15 kn and has a full speed of 30 kn. Each
//! ship's class has a fuel capacity of 1,000 t and an endurance of 4,000
//! nm, so it burns 0.25 t a mile at cruise and four times that at full
//! speed. C has no home port unless a statement gives it one: base H at
//! (10,2). A has no aircraft at any base.
//!
//! Side B has what each statement gives it. A circle of danger is 15
//! hexes round an enemy base on A's map and 8 hexes round a contact
//! reported as a carrier and three pulses old or less; a hex at exactly
//! that distance is inside. The danger of a hex is how many hexes inside
//! a circle it lies and one more.
//!
//! A day is the night, 18:00 to 06:00, then the morning and the
//! afternoon, six hours each. A route given by the staff is steamed from
//! the start of the night. In twelve hours C makes 180 nm at cruise and
//! 360 nm at full speed; in six, 90 nm and 180 nm.
//!
//! The engine charges a run with no land in the way by the straight line
//! to the waypoint it steers for, to the nearest mile: 40 nm a step along
//! a line of the grid, less off it. It has a force enter a hex half-way
//! along the step into it. A hex entered at the very hour a pulse ends is
//! entered in that pulse.
//!
//! In each statement A's commander has given leave to launch, the order
//! has no forces limit, and C's commander is steady.
//!
//! # Statements: with no enemy known, as today
//!
//! The order is "deny the waters within 2 hexes of (0,0)". By decision
//! 0202 the station is (3,0), 13 hexes from C: 520 nm, 34.7 hours at
//! cruise, more than 12, and 17.3 at full speed.
//!
//! 1. **One point, as today.** At balanced and at press C is given the
//!    route (3,0) and full speed; at preserve the route (3,0) and cruise.
//!    The reason says nothing of a way.
//! 2. **Routed each morning, as today.** At preserve C is still under way
//!    on the morning of day 2, and is given the route (3,0) again that
//!    morning. A way that does not bend is not kept.
//!
//! # Statements: a carrier contact across the way, at preserve
//!
//! A holds a fresh contact reported as a carrier at (13,-7), K. On C's
//! row, r = 0, the hexes from (5,0) to (14,0) are inside K's circle:
//! (5,0) and (14,0) at 8 hexes from K, the eight between at 7. On the row
//! r = 1 the hexes from (5,1) to (13,1) are inside, at 8. The row r = 2
//! is clear. (3,0) is 10 hexes from K and outside, so it is still the
//! station. The order is at preserve, so C closes on nothing and goes at
//! cruise.
//!
//! 3. **The shortest way, by the engine's clock.** Along r = 0 each step
//!    is 40 nm, and C enters the hex of step n after 40n less 20 nm:
//!    hours 1.3, 4, 6.7, 9.3 and 12 for steps 1 to 5, so (15,0) to (11,0)
//!    are entered in the night of day 1; 14.7 and 17.3 for (10,0) and
//!    (9,0), the morning; 20 and 22.7 for (8,0) and (7,0), the afternoon;
//!    25.3 on for (6,0) to (3,0), the night of day 2. It arrives after
//!    34.7 hours, in the night of day 2. By day it enters four hexes
//!    inside K's circle, (10,0), (9,0), (8,0) and (7,0), each 7 hexes
//!    from K; by night six, (14,0) to (11,0), (6,0) and (5,0).
//! 4. **How late it may be.** At preserve the way may arrive no later
//!    than the night of day 3, the same pulse of the day after.
//! 5. **The way taken.** C is given the route (10,2), (6,2), (3,0) and
//!    cruise.
//!    - Its three legs are charged 212 nm ((16,0) to (10,2) is 40 times
//!      the root of 28, 211.7), 160 nm and 174 nm ((6,2) to (3,0) is 40
//!      times the root of 19, 174.4): 546 nm, 26 more than the shortest.
//!    - The engine steams it through (15,0), (14,0), (13,0), (12,0),
//!      (11,1), (10,2), (9,2), (8,2), (7,2), (6,2), (6,1), (6,0), (5,0),
//!      (4,0), (3,0).
//!    - It arrives after 36.4 hours, in the morning of day 2, a pulse
//!      later than the shortest way and inside the limit.
//!    - It enters (15,0) to (11,1) in the night of day 1, (10,2) and
//!      (9,2) in the morning, (8,2), (7,2) and (6,2) in the afternoon,
//!      and (6,1) to (3,0) in the night of day 2. By day it enters no hex
//!      inside K's circle. By night it enters seven: (14,0), (13,0),
//!      (12,0), (11,1), (6,1), (6,0) and (5,0).
//! 6. **The reason.** The reason for the route says it arrives in the
//!    morning of day 2 where the shortest way arrives in the night of day
//!    2; that the shortest way enters four hexes inside the reach of the
//!    contact by day, the nearest 7 hexes from it, and six by night; and
//!    that the way taken enters none by day and seven by night. It names
//!    the contact by its number and its age. It gives no figure of
//!    danger.
//! 7. **Kept.** On the morning of day 2 C is under way, in (6,2), and the
//!    staff gives it no route and no speed. Its route still ends at
//!    (3,0), and it arrives there in the morning of day 2.
//! 8. **The search on the way.** On the morning of day 1 C's search arc
//!    takes in the way ahead for a day at 15 kn, nine steps: (15,0),
//!    (14,0), (13,0), (12,0), (11,1), (10,2), (9,2), (8,2), (7,2), and
//!    the station (3,0), with the waters of the deny. On the morning of
//!    day 2 it takes in (6,1), (6,0), (5,0), (4,0) and (3,0).
//! 9. **A contact grown old casts no circle.** With K four pulses old on
//!    the morning of day 1, C is given the route (3,0), as in statement
//!    1.
//!
//! # Statements: fuel, at preserve
//!
//! Statement 5's setting, with H at (10,2) C's home port. From (3,0) to H
//! is 327 nm, 81.75 t at cruise. The margin at preserve is 1.5.
//!
//! 10. **What each way takes.** The way of statement 5: 546 nm at cruise,
//!     136.5 t, and 81.75 t home, 218.25 t, and 327.4 t with the margin.
//!     The shortest way: 130 t and 81.75 t, 211.75 t, and 317.6 t with
//!     the margin.
//! 11. **In fuel, the way of statement 5.** With every ship at 330 t C is
//!     given the route (10,2), (6,2), (3,0).
//! 12. **Fuel takes the clearest way away.** With every ship at 325 t the
//!     way of statement 5 is not in fuel. C is given the route (10,1),
//!     (6,1), (3,0): its legs are charged 223 nm ((16,0) to (10,1) is 40
//!     times the root of 31, 222.7), 160 nm and 144 nm ((6,1) to (3,0) is
//!     40 times the root of 13, 144.2), 527 nm; 131.75 t and 81.75 t,
//!     213.5 t, and 320.25 t with the margin. It arrives after 35.1
//!     hours, in the night of day 2. By day it enters four hexes inside
//!     K's circle, (10,1), (9,1), (8,1) and (7,1), each on its rim 8
//!     hexes from K, where the shortest way enters four that are each 7
//!     from it. The reason says so, and that a clearer way is out of
//!     fuel.
//! 13. **No clearer way in fuel.** With every ship at 319 t only the
//!     shortest way is in fuel. C is given the route (3,0), and is routed
//!     each morning as today. The reason says the way lies in the reach
//!     of the contact by day and that no clearer way is in fuel.
//!
//! # Statements: an enemy base beside the way
//!
//! The order is "deny the waters within 2 hexes of (-10,0)". B has a base
//! E at (9,-15) on A's map, and A holds no contact. On C's row the hexes
//! from (-6,0) to (9,0) are inside E's circle, each at 15 hexes, its rim.
//! The row r = 1 is clear. By decision 0202 the station is (-7,0): it is
//! 16 hexes from E and outside, and 23 hexes from C, the least passage of
//! any candidate. The shortest way is 920 nm: 61.3 hours at cruise and
//! 30.7 at full speed.
//!
//! 14. **At press, as today.** C is given the route (-7,0) and full
//!     speed, and the reason says nothing of a way.
//! 15. **At balanced, the shortest way by the engine's clock.** At full
//!     speed C enters the hex of step n after 40n less 20 nm at 30 kn:
//!     steps 1 to 9, (15,0) to (7,0), in the night of day 1; steps 10 to
//!     14, (6,0) to (2,0), in the morning (step 14 at hour 18 exactly);
//!     steps 15 to 18, (1,0) to (-2,0), in the afternoon; steps 19 to 23
//!     in the night of day 2. It arrives after 30.7 hours, in the night
//!     of day 2. By day it enters nine hexes inside E's circle, (6,0) to
//!     (-2,0), each 15 hexes from E; by night seven, (9,0), (8,0), (7,0)
//!     and (-3,0) to (-6,0). It may arrive no later than the morning of
//!     day 2.
//! 16. **At balanced, the way taken.** C is given the route (6,1),
//!     (-3,1), (-7,0) and full speed.
//!     - Its legs are charged 382 nm ((16,0) to (6,1) is 40 times the
//!       root of 91, 381.6), 360 nm and 183 nm ((-3,1) to (-7,0) is 40
//!       times the root of 21, 183.3): 925 nm, 5 more than the shortest.
//!     - The engine steams it along r = 0 to (7,0), then (6,1) and along
//!       r = 1 to (-3,1), then (-3,0) and along r = 0 to (-7,0).
//!     - It enters (7,0) after 327.5 nm, hour 10.9, in the night; (6,1)
//!       after 362 nm, hour 12.1, in the morning; (-2,1) after 682 nm,
//!       hour 22.7, in the afternoon; (-3,1) after 722 nm, hour 24.1, in
//!       the night of day 2.
//!     - It arrives after 30.8 hours, in the night of day 2, the pulse
//!       the shortest way arrives in.
//!     - By day it enters no hex inside E's circle. By night it enters
//!       seven: (9,0), (8,0), (7,0), (-3,0), (-4,0), (-5,0) and (-6,0).
//! 17. **At balanced, kept and reasoned.** The reason says both ways
//!     arrive in the night of day 2; that the shortest enters nine hexes
//!     inside the reach of E by day, the nearest 15 hexes from it; and
//!     that the way taken enters none by day. On the morning of day 2 C
//!     is given no route and no speed.
//! 18. **At preserve.** C goes at cruise. The shortest way arrives after
//!     61.3 hours, in the morning of day 3, and by day enters seven hexes
//!     inside E's circle: (9,0), (8,0) and (7,0) on day 1 and (1,0) to
//!     (-2,0) on day 2. C is given the route (9,1), (-3,1), (-7,0) and
//!     cruise: 925 nm (the legs 262, 480 and 183; (16,0) to (9,1) is 40
//!     times the root of 43, 262.3), arriving after 61.7 hours in the
//!     morning of day 3. By day it enters no hex inside E's circle.
//! 19. **In the dark, the shortest way.** With C at (0,0), inside E's
//!     circle, the station is still (-7,0), 7 hexes and 280 nm: 9.3 hours
//!     at full speed. At balanced every hex of the shortest way is
//!     entered in the night of day 1, the six inside E's circle among
//!     them. C is given the route (-7,0), as today, and the reason says
//!     nothing of a way.
//!
//! # Statements: where nothing is chosen
//!
//! 20. **A station of the commander's.** With no deny, and a task of the
//!     commander's own for C to operate carriers from (3,0), at preserve
//!     and with K held as in statement 5, C is given the route (3,0): the
//!     way is chosen only for a station the staff chose.
//! 21. **Home for fuel.** In statement 11's setting with C at its station
//!     and short of fuel by the carrier doctrine's measure, C is routed
//!     home by the one point H.
//!
//! # Properties
//!
//! - **No game on file moves but scenario 116's.** Every other scenario
//!   and recorded game gives the events and the daily hashes it gave
//!   before. Scenario 116's statements all hold.
//! - **The forecast only reports.** The pulse of each hex in a route's
//!   forecast changes no hash and no event.
//! - **Nothing is drawn.** The same view gives the same way.
//! - **The staff reads its own side.** Nothing in a reason for a way
//!   names what an enemy base holds, and no reason gives a figure of
//!   danger.
//! - **A day is not much slower.** `sim-cli time` on a day in which such
//!   a route is given is measured before and after.

// The architect's statement lists are list items in the doc comment above,
// which is left as written.
#![allow(clippy::doc_lazy_continuation)]
