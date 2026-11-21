//! Scenario 116: carriers serve an order for an end. A commander's order
//! to deny an area or to neutralise a base uses a free carrier force, and
//! the staff chooses where it works from by the risk, the enemy and the
//! fuel.
//!
//! Proves decision 0202. The statements are the architect's, written
//! before the code. A mismatch goes back to the architect, and nobody else
//! changes a statement or a number here.
//!
//! # The setting
//!
//! A placeholder scenario made for the test, on a placeholder map of open
//! sea, with no real ship. Hexes are 40 nm and are named (q,r); the
//! distance between two hexes is in hex steps, and on open sea the way by
//! sea is as many steps.
//!
//! Side A has a carrier force C: one carrier and two destroyers, on the
//! carrier mission, stopped at sea at (10,0). Aboard are a bomber group
//! whose normal strike radius is 210 nm (five hexes are in reach, 200 nm;
//! six are not, 240 nm), a fighter group and a group that searches. C
//! cruises at 15 kn and has a full speed of 30 kn. Each ship's class has a
//! fuel capacity of 1,000 t and an endurance of 4,000 nm, so it burns
//! 0.25 t a mile at cruise and four times that at full speed. A's home
//! port for C, where one is given, is base H at (10,2). A has no aircraft
//! at any base.
//!
//! Side B has what each statement gives it. The fuel margins are 1.5 at
//! preserve, 1.2 at balanced and 1.0 at press. A circle of danger is 15
//! hexes round an enemy base on A's map and 8 hexes round a fresh contact
//! reported as a carrier; a hex at exactly that distance is inside.
//!
//! In each statement A's commander has given leave to launch, unless it
//! says otherwise, and the order has no forces limit.
//!
//! # Statements: deny
//!
//! The order is "deny the waters within 2 hexes of (0,0)".
//!
//! 1. **The candidates.** 210 nm less 40 nm for each of the 2 hexes is
//!    130 nm: three hexes are in reach (120 nm), four are not. The
//!    candidates are the hexes within 3 of (0,0).
//! 2. **With no enemy known, the nearest.** B has no base on A's map and
//!    A holds no contact. At preserve, at balanced and at press the staff
//!    forms a task to operate C from (3,0), the candidate with the least
//!    passage, 7 hexes. Its reason gives the station, 3 hexes east of the
//!    centre of the waters, says no circle of danger reaches it, and
//!    gives the passage.
//! 3. **The speed is the carrier doctrine's.** 7 hexes are 280 nm: 18.7
//!    hours at cruise, more than 12, and 9.3 at full speed. At balanced
//!    and at press C is ordered there at full speed; at preserve at
//!    cruise.
//! 4. **Already in reach, it stays.** With C at (2,0), the candidate with
//!    the least passage is the hex it is in. The task names (2,0) and C
//!    is given no route.
//! 5. **Waters wider than the bombers reach.** For "within 6 hexes of
//!    (0,0)", 210 less 240 is less than one hex. The candidates are the
//!    hexes within 5 of (0,0), and C is sent to (5,0), 5 hexes of passage.
//! 6. **A carrier contact casts a circle.** A holds a fresh contact
//!    reported as a carrier at (8,-6). (3,0) is 6 hexes from it and so
//!    inside its circle, by 2. Of the candidates, (0,3) is 9 hexes from
//!    the contact, outside by 1, and 10 hexes of passage from C; no
//!    candidate outside the circle has less.
//!    - At preserve and at balanced C is sent to (0,3). The reason says
//!      it lies outside the circle of that contact, by number and age,
//!      and that a nearer station, (3,0), lies inside it.
//!    - At press C is sent to (3,0), and the reason says it stands inside
//!      the contact's circle.
//! 7. **What casts no circle.** Each of these, in place of statement 6's
//!    contact, leaves the station at (3,0) at every risk: the same
//!    contact reported as a cruiser; the same contact grown too old to
//!    be fresh; a fresh carrier contact at (1,0), inside the waters the
//!    order denies.
//! 8. **An enemy base, and no way clear of it.** B has a base E at
//!    (-10,0) on A's map and A holds no contact. Every candidate is
//!    within 13 hexes of E and so inside its circle.
//!    - At preserve no task is formed for C. The reason says E's circle
//!      holds every candidate. With no base of A's that reaches, the
//!      staff hands in that it cannot meet the order, and names both the
//!      bases and the carrier force.
//!    - At balanced C is sent to (3,0): 13 hexes from E, inside by 2, the
//!      least deep of any candidate, and of the four candidates equally
//!      deep the one with the least passage. The reason says it stands
//!      in danger of E.
//!    - At press C is sent to (3,0), the least passage.
//! 9. **The waters and the targets are read.** At its station under the
//!    deny, the hexes of the waters are in C's search arc; and with the
//!    order's targets limit set to transports first, C's bombers are
//!    given transports first.
//! 10. **Its strikes count.** A strike by C's bombers on a ship in the
//!     waters is counted in the deny's measure.
//!
//! # Statements: neutralise
//!
//! B has a base E at (0,0) on A's map. The order is "neutralise E".
//!
//! 11. **The candidates** are the hexes within 5 of (0,0). Every one is
//!     inside E's own circle.
//! 12. **At preserve the carriers do not go.** No task is formed for C,
//!     and the reason says a raid on E is flown from inside E's circle.
//! 13. **At balanced, the edge of the reach.** The candidates 5 hexes
//!     from E are the least deep, inside by 10. Of those, (5,0) has the
//!     least passage from C, 5 hexes. C is sent to (5,0), and the reason
//!     says it stands in danger of E. At press it is sent to (5,0) too.
//! 14. **The bombers raid the base.** At (5,0), with no fresh contact of
//!     ships within 210 nm, C's bombers are set to strike E. On a morning
//!     A holds a fresh contact of ships 4 hexes from C, they are set to
//!     strike ships and not E.
//! 15. **The pair is allowed.** The task to operate carriers stands under
//!     the neutralise; today the pair is refused.
//!
//! # Statements: fuel
//!
//! The order is statement 2's deny, the station (3,0), and H at (10,2) is
//! C's home port: 9 hexes from (3,0), 360 nm.
//!
//! 16. **What the passage takes.** Out at full speed, 280 nm at 1 t a
//!     mile, 280 t; home at cruise, 360 nm at 0.25 t a mile, 90 t: 370 t.
//!     At press, with every ship holding 400 t, C is sent, and the reason
//!     gives 370 t needed against the least aboard.
//! 17. **The margin.** At balanced the need is 370 times 1.2, 444 t. With
//!     a destroyer at 400 t no candidate is in fuel: the cheapest other,
//!     (2,1), is 8 hexes out and 9 home, 320 t and 90 t, 410 t, and 492 t
//!     with the margin. No task is formed for C, and the reason says no
//!     candidate is in fuel and names the destroyer. With that destroyer
//!     at 450 t C is sent to (3,0).
//! 18. **Preserve sails at cruise.** At preserve the passage out is at
//!     cruise: 70 t out and 90 t home, 160 t, and 240 t with the margin
//!     of 1.5. With every ship at 250 t C is sent.
//! 19. **In port, it is weighed full.** With C stopped at H, H its home
//!     port, H holding fuel and able to fuel it, and its ships at 100 t:
//!     the task is formed, the station chosen as if the ships were full,
//!     and C fuels before it sails, as the carrier doctrine has it.
//! 20. **No home port.** With no home port C is sent as in statement 2 at
//!     every risk, and the reason says its fuel could not be measured
//!     against a passage home.
//!
//! # Statements: which force, and the commander's word
//!
//! 21. **Not free.** In each of these the staff forms no task for C under
//!     the deny, and C is given no order by it: C has an order of the
//!     commander's own to operate from (10,0); another order's forces
//!     limit names C; C is held.
//! 22. **A hold lifted.** With C held when the deny is given, and the
//!     hold lifted on day 3, the task is formed on the morning of day 4
//!     and the station chosen then, from where C is that morning.
//! 23. **A force the commander named keeps its hex.** With the deny's
//!     forces limit naming C, the task names (10,0), the hex C is in,
//!     and no station is chosen: decision 0125, as today.
//! 24. **One force, two orders.** A gives the deny of statement 2 and the
//!     neutralise of statement 11 at balanced. With the neutralise at the
//!     higher priority C serves the neutralise, and the deny's reason
//!     says the one carrier force is taken. With the two at one priority,
//!     the order with the lower number has it.
//! 25. **Leave withheld.** With leave to launch withheld, no task is
//!     formed and C is given no order. The preview of the order names C
//!     and (3,0). On the morning after the commander approves the plan,
//!     the task is formed and C is routed.
//! 26. **A direct order to a tasked force holds it.** With the task
//!     formed and C under way, the commander routes C himself: the staff
//!     gives C no route the next morning, by the override, as today.
//!
//! # Statements: the search on the way
//!
//! 27. **The way ahead is searched.** On the morning C is ordered from
//!     (10,0) to (3,0), its search arc takes in the hexes of its way,
//!     (9,0) to (3,0), with the waters of the deny; the reason for the
//!     arc names the way ahead to the station. On the morning after, at
//!     (3,0), the arc has the waters and no way.
//!
//! # Properties
//!
//! - **No game on file moves.** Every scenario and recorded game on file
//!   gives the events and the daily hashes it gave before.
//! - **Nothing is drawn.** The choice of a station draws no random number:
//!   the same view gives the same station.
//! - **The staff reads its own side.** Nothing in a reason for a station
//!   names what an enemy base holds.
