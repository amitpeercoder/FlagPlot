//! Scenario 115: refuelling at sea. A force fuels from an oiler's force
//! while both steam slowly together, by day or by night, a few ships at a
//! time; an attack breaks it off; the staff warns of a force running low
//! and sends it to an oiler nearer than home.
//!
//! Proves decision 0201, stage A. The statements are the architect's,
//! written before the code. A mismatch goes back to the architect, and
//! nobody else changes a statement or a number here.
//!
//! # The setting
//!
//! A placeholder scenario made for the test, on a placeholder map of open
//! sea, with no real ship. Hexes are 40 nm and are named (q,r). A day has a
//! night pulse of 12 hours and a morning and an afternoon pulse of 6 hours
//! each.
//!
//! Side A has a force F: a carrier CV with a fuel capacity of 4,000 t, and
//! two destroyers D1 and D2 with 500 t each, in id order CV, D1, D2. It has
//! a force O with one oiler, an auxiliary whose class has the mark, with
//! room for 3,000 t of cargo and 3,000 t of cargo fuel aboard, and her own
//! fuel full. Both cruise at 15 kn. Unless a statement says otherwise, F
//! and O lie stopped in (0,0) with no route, there is no base in the hex
//! and no enemy, and F is under the order to fuel from O.
//!
//! The figures: the fuelling speed is 8 kn; a line passes 250 t an hour; a
//! ship is 1 hour alongside before her fuel passes; an oiler has two lines.
//!
//! Ships are started in the order of rule 13, and no ship starts before one
//! that is earlier in the order has started.
//!
//! # Statements: fuelling
//!
//! CV holds 2,000 t (half), D1 100 t (a fifth) and D2 300 t (three
//! fifths). The order is D1, CV, D2.
//!
//! 1. **A morning.** In the morning pulse, 6 hours:
//!    - D1 starts at once on one line. She is alongside for 1 hour and
//!      then takes the 400 t she lacks in 1.6 hours: full at 2.6 hours.
//!    - CV needs both lines, and has them from 2.6 hours. She is alongside
//!      to 3.6, and takes 500 t an hour for the 2.4 hours left: 1,200 t.
//!    - D2 comes after CV and no line is free before the pulse ends: she
//!      takes nothing.
//!    At the pulse's end D1 holds 500 t, CV 3,200 t, D2 300 t, and the
//!    oiler 1,400 t of cargo fuel. Two events of a ship fuelled at sea are
//!    recorded, of 400 t and 1,200 t, each naming the oiler.
//! 2. **The afternoon after it.** The order is now D2 (three fifths), CV
//!    (four fifths). D2 is alongside for 1 hour and takes 200 t in 0.8
//!    hours: full at 1.8. CV has both lines from 1.8, is alongside to 2.8,
//!    and takes the 800 t she lacks in 1.6 hours: full at 4.4. Every ship
//!    is full, the order ends, and F is idle at cruise with no route. The
//!    oiler holds 400 t.
//! 3. **By night.** The same three ships in a night pulse of 12 hours: D1
//!    is full at 2.6 hours; CV is alongside from 2.6 to 3.6 and takes her
//!    2,000 t in 4 hours, full at 7.6; D2 starts at 7.6, is alongside to
//!    8.6 and takes 200 t in 0.8 hours, full at 9.4. All in one night; the
//!    order ends; the oiler holds 400 t.
//! 4. **The oiler runs out.** With 300 t of cargo fuel, in a morning: D1
//!    takes 300 t and holds 400 t; the oiler holds none; CV and D2 take
//!    nothing; the order ends, and its event says the giver holds no
//!    cargo fuel.
//! 5. **Two oilers.** O has a second oiler, of a higher id, with 3,000 t;
//!    the first holds 300 t. In a morning D1 goes to the first, takes its
//!    300 t and holds 400 t. CV goes to the second, both of whose lines
//!    are free: alongside to 1 hour, then 2,000 t in 4 hours, full at 5.
//!    D2 has no line with fuel before 5, starts on the second at 5, and is
//!    alongside when the pulse ends: she takes nothing.
//! 6. **An oiler takes nothing.** With an oiler of F's own, her own fuel
//!    at half, she is not in the order and takes nothing.
//! 7. **Tons are kept.** In every statement the fuel the ships take is the
//!    cargo fuel the oilers give, and no ship holds more than her
//!    capacity.
//!
//! # Statements: steaming
//!
//! 8. **Slow together.** O has a route east at cruise and F is in her hex
//!    under the order with ships lacking fuel. In the morning pulse both
//!    make 48 nm on O's route and are 8 nm past the centre of (1,0) at
//!    its end, in one hex still; fuel passes as in statement 1, less what
//!    the ships burn in the 48 nm, which is burned before the fuel passes.
//!    The speed F and O report in that pulse is 8 kn. In the night pulse
//!    they make 96 nm. With F's ships full O makes 90 nm in a morning, as
//!    today.
//! 9. **Not together at the start.** F begins a morning in (1,0) under
//!    the order, O stopped in (0,0). F steers for O and reaches her hex
//!    in the pulse. No fuel passes in that pulse and F is not slowed in
//!    it. In the afternoon fuel passes.
//! 10. **Following.** With O on a route and F in her hex under the order
//!     and full but for 1 t in one ship, F takes O's route, and the two
//!     are in one hex at the end of each pulse until the order ends.
//!
//! # Statements: broken off
//!
//! 11. **A raid.** In a morning in which an enemy raid attacks a ship of
//!     F, no fuel passes: every ship of F holds what it held, the oiler
//!     holds 3,000 t, and an event says the fuelling was broken off and
//!     cites the raid's attack. Both forces still made no more than 48 nm.
//!     The same where the raid attacks the oiler. In the afternoon, with
//!     no attack, statement 1's fuelling takes place.
//! 12. **A surface action.** In a pulse in which O fights a surface
//!     action, no fuel passes and the event cites the action.
//!
//! # Statements: the order
//!
//! 13. **Refused.** The order is refused, each with its own reason, where:
//!     the giver has no ship with the mark; the giver is the force
//!     itself; the giver is under the order herself; the giver is an
//!     escort; the giver has an escort; the force is the giver for
//!     another force; the force is an escort; the force has an escort.
//! 14. **It ends.** Besides statements 2 and 4, the order ends, with its
//!     event saying why, when: the oiler is sunk; the oiler is detached
//!     from O; the commander gives F a route. After each F is at cruise
//!     with no route but the one the commander gave, and no fuel passes
//!     in later pulses.
//! 15. **A force that has run dry.** F is out of fuel, adrift where it
//!     stopped, D1 holding nothing.
//!     - O steams through F's hex on a route: no fuel passes, and O is
//!       not slowed.
//!     - O lies stopped in F's hex: fuel passes as in statement 1 with D1
//!       first, and the first ship fuelled sets F idle where it is with
//!       its route dropped.
//! 16. **A port before an oiler.** F and O lie stopped in a hex with a
//!     base of A's that holds fuel and can fuel them. The ships fuel from
//!     the base by the rule in force today, take nothing from the oiler
//!     in that pulse, and the oiler's cargo fuel is unchanged. In a pulse
//!     in which the oiler loads cargo fuel at the base she gives none.
//! 17. **Who knows.** A's own view gives every event of statements 1 to
//!     16. B's view gives none of them.
//!
//! # Statements: the staff
//!
//! 18. **The warning.** A force of A's with no home port has a destroyer
//!     at 120 t of 500: below a quarter, 125 t. The staff's report names
//!     the force and the ship each day. With the destroyer at 130 t it
//!     does not. With no oiler on A's side the staff gives the force no
//!     order because of it.
//! 19. **Sent to the oiler.** A force on a stand, with a home port 10
//!     hexes away, is short of fuel by the stand doctrine's own measure
//!     and would be routed home today. With O 4 hexes away holding cargo
//!     fuel, the staff gives it the order to fuel from O, and its reason
//!     gives both passages. While the order stands the staff gives it no
//!     route. On the morning after the order ends it is routed back to
//!     its station.
//! 20. **Home is nearer, or the oiler is empty.** With O 12 hexes away the
//!     force is routed home as today. With O 4 hexes away holding no
//!     cargo fuel it is routed home as today.
//! 21. **No home port.** A carrier force with no home port and a
//!     destroyer at 120 t of 500, O holding cargo fuel: the staff gives
//!     it the order to fuel from O.
//!
//! # Properties
//!
//! - **No game on file moves.** Every scenario and recorded game on file
//!   gives the events it gave before, and the daily hashes it gave before
//!   but for the state's layout version; the golden files are remade for
//!   the new layout and for nothing else.
//! - **Nothing is drawn.** Fuelling at sea draws no random number.
//! - **A class file with no column for the mark loads,** and no ship of it
//!   has the mark.
