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
//! 1. **Today's answer is gone.** At press, on the first morning, the
//!    staff does not say G is away from home. It routes G to X, the
//!    station marked to stand at, and its reason says G has no home port
//!    and stands from where it is.
//! 2. **Cruise where the night is enough.** X is 4 hexes from G: 160 nm,
//!    160 / 15 = 10.7 hours, within 12. G is ordered at cruise.
//! 3. **Full where it is not.** X is 8 hexes: 320 nm, 21.3 hours at
//!    cruise, 320 / 30 = 10.7 at full. G is ordered at full speed.
//! 4. **It goes though the night is not enough.** X is 12 hexes: 480 nm,
//!    32 hours at cruise and 16 at full. G is ordered at full speed and
//!    goes; a force with a home port in the same case does not sortie
//!    and hands in the notice that the intent cannot be met.
//! 5. **It stays.** On the morning after it arrives G is at X. It is
//!    given no route on that morning or the next, at press. It is at
//!    battle readiness.
//! 6. **Balanced waits for a threat.** At balanced with no contact of
//!    ships within 15 hexes of X, G is given no route and stays where it
//!    is, and the reason says there is no threat. On the morning a
//!    contact of ships is held 10 hexes from X, G is routed to X.
//! 7. **Fuel is weighed to the nearest base.** B is 5 hexes from X by
//!    sea. With X at 4 hexes, the passage weighed is 160 nm to X and 200
//!    nm from X to B. The reason gives both and the fuel aboard against
//!    the fuel needed. With every ship's fuel cut to less than that need
//!    times the balanced margin of one and a fifth, G does not go.
//! 8. **Going in for fuel.** G is at X at press. A destroyer's fuel is
//!    cut below the measure of the passage from X to B at cruise. The
//!    next morning G is routed to B and B is set as its home port, and
//!    the reason names the destroyer.
//! 9. **Then it is a force with a home port.** Stopped at B and fuelled,
//!    G sorties to X by night and returns, as a stand's force does today.
//! 10. **A force with a home port is as today.** A second force, H, of
//!     the same ships, stopped at B with B its home port: every order
//!     and reason the staff gives H is what it gave before this rule.
//!     And H at sea with B its home port is answered "away from home",
//!     as today.
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
//! - G is never routed anywhere but X or the nearest base of its side.
//!
//! The tests are written by the build under these statements.
