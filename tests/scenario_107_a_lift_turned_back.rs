//! Scenario 107: a lift turned back. A staff that has sent men by sea to
//! take a place turns them back while enemy carriers threaten the way, and
//! sends them on again when the threat has passed.
//!
//! Proves decision 0191. The statements are the architect's, written
//! before the code. If the code disagrees, the code or the rule is wrong:
//! a mismatch goes back to the architect, and nobody else changes a
//! statement or a number here.
//!
//! # The setting
//!
//! A placeholder scenario made for the test, on the placeholder map, with
//! no real ship or unit. Side J has a base S, and a transport force L at
//! sea with a land unit aboard, three transports and a destroyer, 12
//! hexes from a coast hex P that side A holds. J is given "take the
//! place P", its forces L and the unit, with both leaves. A second J
//! force, K, with a carrier, is formed under the same intent. Side A has
//! a carrier force. Hexes are 40 nm; a fresh contact is one no more than
//! 3 pulses old; the danger's distance is 8 hexes. A lift is turned back
//! when it is under way with men aboard and P is no longer a hex it is to
//! act at on its route.
//!
//! In each statement the lift has been released and is under way to P.
//!
//! # Statements
//!
//! 1. **No danger, no change.** J holds no contact of a carrier. The lift
//!    sails on and lands, at every risk, as it does today.
//! 2. **Preserve the force turns back on the danger.** J holds a fresh
//!    contact reported as a carrier, 6 hexes from the lift's hex. The
//!    next morning the staff routes L to S at its cruise speed. Its men
//!    are aboard. The intent is in force and its plan released. The
//!    reason gives the contact, "carrier", its age in pulses and its
//!    distance, 6 hexes (240 nm), and the risk. L's order to unload is
//!    taken away. The notice
//!    `lift_turned_back` is handed in.
//! 3. **Balanced does not turn back on the sighting alone.** The same
//!    contact, at balanced, with no ship of L or carrier of K lost: L
//!    sails on. The staff's reason says the lift is threatened and has
//!    not been hurt.
//! 4. **Balanced turns back once hurt.** The same, and a transport of L
//!    has been sunk since the plan was released: L is routed to S. The
//!    reason names the ship lost.
//! 5. **A carrier of the side counts.** The same as statement 3, and K's
//!    carrier has been sunk since the release, with no ship of L
//!    touched: L is routed to S. With K's carrier damaged and afloat, L
//!    sails on.
//! 6. **A loss with no danger does not turn it back.** A transport of L
//!    is sunk and J holds no fresh contact of a carrier within 8 hexes of
//!    the lift or of its way to P:
//!    L sails on, at balanced and at preserve the force.
//! 7. **Press never turns back,** with the danger and the hurt of
//!    statement 4.
//! 8. **What counts as the danger.** None of these turns a lift back at
//!    any risk: a contact of a carrier 4 pulses old; a fresh contact of a
//!    carrier 9 hexes from the lift's hex and from every hex of its way
//!    to P; a fresh contact at 6 hexes reported as a battleship that is
//!    in truth a carrier; a fresh carrier contact 3 hexes from the way
//!    home from P and more than 8 from the way to it.
//! 9. **Along the way.** A fresh carrier contact 12 hexes from the
//!    lift's hex and 5 hexes from a hex of the sea route from the lift
//!    to P that is not one of the route's corners: the danger stands.
//! 10. **Going on again, after two clear mornings.** After statement 2,
//!     the contact is not reported again. On the next morning it is 4
//!     pulses old: no danger, and L is not yet sent on. On the morning
//!     after, with no contact of a carrier reported in the last 6
//!     pulses, the take routes L to P again from the hex it is in and
//!     gives it its order to unload. The reason says the danger has
//!     passed for two mornings.
//! 11. **Waiting at its source.** A lift that reaches S while the danger
//!     stands stops there, and no man goes ashore: it has no order to
//!     unload. On the second clear morning it sails for P.
//! 11a. **Preserve the force does not sail into danger.** A lift at S
//!     whose plan is ready, at preserve the force, on a morning of
//!     danger: it is not sent, and the reason says it is held for the
//!     danger. At balanced it sails.
//! 12. **No state.** Saved on the morning after statement 2 and loaded,
//!     the game goes on the same: the staff reads that L is under way
//!     with men aboard on a route that ends at S, and decides from that.
//! 13. **A held lift is left alone.** The commander holds L. With the
//!     danger and the hurt, the staff gives it no order and says it is
//!     held.
//! 14. **A lift that has begun to land is not turned back,** whatever is
//!     sighted.
//! 15. **Not a supply run.** A supply run of "keep a base supplied"
//!     under the same danger does what it does today.
//! 16. **The commander is told and may overrule.** With the standing
//!     interrupt for the notice on, a seat's day stops on the morning L
//!     is turned back. Setting press on the intent sends L on the next
//!     morning though the danger stands.
//! 17. **Truth and belief.** Every decision here is the same when the
//!     enemy's forces are moved in truth and J's contacts are left as
//!     they were.
//!
//! # Properties
//!
//! - With no contact of a carrier held by the side that lifts, no order
//!   of a staff differs from what it gave before this rule.
//! - A lift is never routed to its source by this rule at press.
//! - The men aboard a lift turned back are the men aboard before: none
//!   lands and none is lost by the turning, nor by a going on after it.
//! - Turned back at press implies turned back at balanced, which implies
//!   turned back at preserve the force.
//! - A contact made older or farther never makes a danger that the
//!   newer or nearer one did not.
//!
//! The tests are written by the build under these statements.
