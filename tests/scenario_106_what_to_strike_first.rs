//! Scenario 106: what to strike first. A commander sets, on an intent, the
//! kinds of ship to strike in his order; the staff gives that order to the
//! groups it commands, and a carrier force reads it when it chooses what to
//! close on.
//!
//! Proves decision 0190. The statements are the architect's, written
//! before the code. If the code disagrees, the code or the rule is wrong:
//! a mismatch goes back to the architect, and nobody else changes a
//! statement or a number here.
//!
//! # The setting
//!
//! A placeholder scenario made for the test, on the placeholder map, with
//! no real ship or aircraft. Side A has a carrier force, F, with one
//! bomber group, B, of strike radius 200 nm, and one fighter group. Side
//! J has two forces: C, with a carrier, and T, with three transports and
//! a destroyer. A holds contacts of both, each fresh, reported as a
//! carrier and as a transport. F is given "operate a carrier force" at
//! its own hex, at balanced, with both leaves.
//!
//! Distances are by air from F's hex.
//!
//! # Statements
//!
//! 1. **With no limit, as today.** C at 150 nm and T at 100 nm, both in
//!    reach. B's target list is the six kinds in the order of importance.
//!    B launches at C: the kind first on its list, though T is nearer.
//! 2. **With the limit, the transports.** The same, and the commander
//!    sets on F's intent the targets limit transport, carrier,
//!    battleship, cruiser, destroyer, auxiliary. The next morning the
//!    staff orders B's target list to those six in that order, and the
//!    order's reason names the limit and the intent. B launches at T. In
//!    T's force its aircraft attack a transport and not the destroyer.
//! 3. **It does not hold the group.** After statement 2 the side's
//!    command state holds no hold on B, and on later mornings the staff
//!    still orders B (its mission, its rest when tired).
//! 4. **A kind left out is not launched at.** The limit is transport
//!    alone. T is not held as a contact; C is, in reach. B does not
//!    launch, and the strike's own record of that pulse says no contact
//!    of a kind on the group's list is held.
//! 5. **Read up the tree, the nearest.** The limit is set on the
//!    commander's intent and the strike is flown under a task the staff
//!    formed beneath it: A has a base with a bomber group D, and a deny
//!    of the waters within 1 hex of T's hex, under which the staff forms
//!    a task to strike ships from that base. D has the limit's list. A second
//!    limit set on the staff's own task, carrier first, replaces it for
//!    that task alone.
//! 6. **Cleared.** The limit set with no kinds is cleared: the next
//!    morning the staff orders B's list back to the six kinds in the
//!    order of importance.
//! 7. **The waters come first.** No targets limit. A deny of the waters
//!    within 1 hex of T's hex names F. T, reported as a cruiser, is in
//!    those waters at 150 nm; C, reported as a carrier, is outside them
//!    at 100 nm. The staff orders B's waters to strike first to the
//!    deny's seven hexes. B launches at T, though C is the more
//!    important kind and the nearer. With the deny withdrawn the staff
//!    orders B's waters empty, and B launches at C.
//! 8. **Both parts together.** As statement 7, with the limit of
//!    statement 2 on the deny: B launches at T, and its aircraft attack
//!    a transport of T's force and not the destroyer.
//! 9. **Closing on what is in the waters.** As statement 7 with T at 300
//!    nm, beyond reach, and no carrier contact held. At balanced F closes
//!    on T, to the first hex on its way within 200 nm of T's hex, and the
//!    reason says the contact is in the waters of the deny. With the
//!    deny withdrawn F does not close: nothing of a kind it closes on is
//!    held. At preserve the force F closes on nothing, with the deny or
//!    without.
//! 9a. **A contact outside the waters is ranked as today.** As statement
//!    7 with T moved one hex outside the waters: B launches at C.
//! 9b. **Preserve the force keeps the limit's list.** With the limit of
//!    statement 2 at preserve the force, B's list is the limit's six
//!    kinds and not the risk's three.
//! 9c. **A strike sized to its target sends on what is over.** With B of
//!    60 aircraft, T reported as a cruiser of 4 ships in the waters and C
//!    outside: the aircraft the sizing figures give for T go at T and
//!    the rest at C. The test takes the split from the sizing figures in
//!    the reference file and states both numbers.
//! 10. **Refused:** a limit that names no intent of the side's in force;
//!     a kind named twice. Each with the reason, as the other limits are
//!     refused. A limit on a staff-made task with other kinds than its
//!     parent's is taken, not refused as wider.
//! 11. **What a seat reads.** `command_state` shows the limit on the
//!     intent, the kinds in order. The other side is shown nothing of it.
//! 12. **No game without the limit changes.** The daily hashes of
//!     scenario 5 and of the 39 days' first week, each played by its
//!     staffs, are what they are before this rule, apart from the state
//!     hash's version line. The test takes the hashes of the fields, not
//!     a number written here.
//!
//! # Properties
//!
//! - With no targets limit anywhere, every order a staff hands in is the
//!   order it handed in before this rule, on the scenarios above.
//! - A group's list set from a limit holds exactly the limit's kinds, in
//!   its order, each once.
//! - A limit of all six kinds in the default order gives the orders of
//!   no limit, at balanced and at press.
//! - Moving a kind earlier on the limit never lowers the aircraft sent at
//!   contacts of that kind.
//! - A contact moved into the waters never ranks lower than it did.
//!
//! The tests are written by the build under these statements.
