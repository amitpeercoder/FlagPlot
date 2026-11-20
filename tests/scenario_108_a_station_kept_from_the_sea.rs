//! Scenario 108: a station kept from the sea. A surface force that begins
//! at sea with no home port is stood at its station by the staff from
//! where it is.
//!
//! Proves decision 0192. The statements are the architect's, written
//! before the code. A mismatch goes back to the architect, and nobody else
//! changes a statement or a number here.
//!
//! # The setting
//!
//! A placeholder scenario made for the test, on the placeholder map, with
//! no real ship. Side A has a surface force G of two cruisers and two
//! destroyers, at sea, stopped, with no home port, and one base B. G is
//! given "stand at" a station X. Hexes are 40 nm. G cruises at 15 kn and
//! has a full speed of 30 kn. The night is 12 hours.
//!
//! # Statements
//!
//! 1. **Today's answer is gone.** At balanced, with no contact held, on
//!    the first morning the staff does not say G is away from home. It
//!    routes G to X, and its reason says G has no home port and stands
//!    from where it is. G is at normal readiness. The same at press.
//! 2. **Cruise where the night is enough.** X is 4 hexes from G: 160 nm,
//!    160 / 15 = 10.7 hours, within 12. G is ordered at cruise.
//! 3. **Full where it is not.** X is 8 hexes: 320 nm, 21.3 hours at
//!    cruise, 320 / 30 = 10.7 at full. G is ordered at full speed.
//! 4. **It goes though the night is not enough.** X is 12 hexes: 480 nm,
//!    32 hours at cruise and 16 at full. G is ordered at full speed and
//!    goes; a force with a home port in the same case does not sortie
//!    and hands in the notice that the intent cannot be met.
//! 5. **It arrives and stays.** G arrives at X and lies stopped there,
//!    with no mark and no leg home. It is given no route on the morning
//!    after or the next. On a morning it is under way to X it is given
//!    no route either.
//! 6. **The threat sets its readiness, not whether it goes.** At
//!    balanced, at X, with no contact of ships within 15 hexes of X, G is
//!    at normal readiness. On the morning a contact of ships is held 10
//!    hexes from X, G is set to battle readiness. At press it is at
//!    battle readiness every morning.
//! 6a. **Preserve the force keeps clear of what outnumbers it.** At
//!    preserve the force, with a contact of 9 ships 10 hexes from X
//!    against G's 4: G is not routed to X, and the reason names the
//!    contact and the numbers. With a contact of 3 ships it goes.
//! 7. **Fuel is weighed to a base that can fuel it.** B holds fuel above
//!    its reserve and has a rate, and is 5 hexes from X by sea. With X at
//!    4 hexes, the passage weighed is 160 nm to X and 200 nm from X to
//!    B. At press, where the margin is one, the reason gives both and
//!    the fuel aboard against the fuel needed; with one ship's fuel cut
//!    below that need, G does not go.
//! 7a. **No base that can fuel it.** With B's fuel stock at nothing, the
//!    fuel is not weighed, G goes, and the reason says no base of the
//!    side can fuel it. A nearer base, B2, that holds no fuel is passed
//!    over for B when B holds fuel.
//! 8. **Going in for fuel.** G is at X at press. A destroyer's fuel is
//!    cut below the measure of the passage from X to B at cruise. The
//!    next morning G is routed to B and B is set as its home port, and
//!    the reason names the destroyer.
//! 9. **Then it is a force with a home port.** Stopped at B and fuelled,
//!    G sorties to X by night and returns, as a stand's force does today.
//! 10. **A force with a home port is as today.** A second force, H, of
//!     the same ships, stopped at B with B its home port: every order
//!     and reason the staff gives H is what it gave before this rule.
//!     And H under way at sea with B its home port is answered "away
//!     from home", as today.
//! 10a. **Stopped at sea with a home port.** H stopped at sea on no
//!     route, B its home port: the staff routes H to B at cruise, and
//!     says why. With B taken by the enemy, the staff says H's home port
//!     is no longer its side's and stands H from where it is.
//! 11. **Held.** With G held by the commander, the staff gives it no
//!     order and says it is held.
//! 12. **Under a deny.** A deny of the waters round X whose forces limit
//!     names G forms G's stand task at X, and statements 1 to 5 hold of
//!     it.
//!
//! # Properties
//!
//! - In every scenario with no stand task for a force at sea without a
//!   home port, no order of a staff differs from before this rule.
//! - G is never routed anywhere but X or the base its fuel is weighed to.
//! - A force that goes to its station at balanced goes at press.
//! - More fuel aboard never stops a force going to its station.
//!
//! The tests are written by the build under these statements.
