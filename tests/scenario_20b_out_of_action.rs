//! Scenario 20b: ships out of action on the objectives scale (decision 0088).
//!
//! THE HAND CALCULATION, written by the architect on Nov 13, 2026, before any
//! code. Nobody else changes an expected number here; a mismatch goes back to
//! the architect. Derived quantities are written cut to six places, as the
//! engine's fixed point cuts them.
//!
//! # The rule
//!
//! A ship's days out = her days of repair at the end (decision 0069's yard
//! function, without the passage; 0 if she is at the yard at the end)
//! + the days of repair she left with each time she left for a yard.
//! Her share = min(days out, horizon) / horizon. Her points = her kind's
//! points x her share. A ship sunk is counted as sunk and not here.
//! The horizon is the scenario's days unless `horizon_days` states a longer.
//!
//! # Figures
//!
//! The frozen figures, with decision 0069's five at their placeholders:
//! yard days 2 for a point of flotation damage, 5 for a system degraded,
//! 30 for a system out; `forward_repair_max_flotation` 10;
//! `forward_repair_system_points` 10. Days of repair =
//! ceil(2 x flotation + 5 x degraded + 30 x out). Fire adds nothing.
//!
//! # The scenario, as written
//!
//! 6 days. No fight: the two sides never meet and nothing searches. A
//! placeholder map; every force is stopped for the whole game unless it is
//! sent to the yard.
//!
//! Japanese bases: base 1, a port for the yard with a passage of 1 day and
//! no repair figure; base 2, which mends 1 point a day and is not a port;
//! base 3, with neither. Allied base 4, with neither.
//!
//! | Ship | Side | Kind | Force, at | Flotation | Systems not working |
//! | --- | --- | --- | --- | --- | --- |
//! | 1 | Japanese | carrier | 1, base 3 | 12 | flight deck degraded |
//! | 2 | Japanese | cruiser | 1, base 3 | 0 | propulsion out |
//! | 3 | Japanese | cruiser | 1, base 3 | 40 | none |
//! | 4 | Japanese | cruiser | 1, base 3 | 6 | fire control degraded |
//! | 5 | Japanese | destroyer | 2, base 1 | 1 | none |
//! | 6 | Japanese | destroyer | 3, base 1 | 3.25 | torpedo mounts degraded |
//! | 7 | Japanese | destroyer | 1, base 3 | 0 | none |
//! | 8 | Japanese | cruiser | 4, base 2 | 8 | none |
//! | 9 | Allied | battleship | 5, base 4 | 20 | main battery out |
//!
//! No ship has fire damage.
//!
//! Orders: day 2, force 2, `send_to_yard`. Day 4, force 3, `send_to_yard`.
//!
//! Objectives. Levels 3, 8, 16. `horizon_days = 60`.
//! Allied: `ships_sunk` (carrier 10, battleship 8, cruiser 4, destroyer 1)
//! and `ships_out_of_action` (carrier 5, battleship 4, cruiser 2,
//! destroyer 0.5). Japanese: the same two.
//!
//! # What happens
//!
//! - Force 2, day 2: ship 5's days of repair are ceil(2 x 1) = 2. Days away
//!   1 + 2 = 3. She is back on day 5, whole. The event gives her 2.
//! - Force 3, day 4: ship 6's days of repair are ceil(6.5 + 5) =
//!   ceil(11.5) = 12. Days away 13; the day of return would be 17, after
//!   day 6, so she never returns and ends at the yard. The event gives her 12.
//! - Force 4 at base 2: forward repair of 1 point at the start of each of
//!   days 1 to 6 (after the day's orders). Ship 8's flotation goes
//!   8, 7, 6, 5, 4, 3, 2. She ends with 2.
//! - Nothing else changes.
//!
//! # Days out at the end
//!
//! | Ship | At the end | Left with | Days out |
//! | --- | --- | --- | --- |
//! | 1 | ceil(24 + 5) = 29 | 0 | 29 |
//! | 2 | 30 | 0 | 30 |
//! | 3 | 80 | 0 | 80 |
//! | 4 | ceil(12 + 5) = 17 | 0 | 17 |
//! | 5 | 0 (whole) | 2 | 2 |
//! | 6 | 0 (at the yard: not counted twice) | 12 | 12 |
//! | 7 | 0 | 0 | 0: not listed |
//! | 8 | ceil(2 x 2) = 4 | 0 | 4 |
//! | 9 | ceil(40 + 30) = 70 | 0 | 70 |
//!
//! # Case 1: the horizon stated, 60 days
//!
//! | Ship | Share | Points |
//! | --- | --- | --- |
//! | 1 | 29 / 60 = 0.483333 | 5 x 0.483333 = 2.416665 |
//! | 2 | 30 / 60 = 0.5 | 2 x 0.5 = 1 |
//! | 3 | min(80, 60) / 60 = 1 | 2 |
//! | 4 | 17 / 60 = 0.283333 | 2 x 0.283333 = 0.566666 |
//! | 5 | 2 / 60 = 0.033333 | 0.5 x 0.033333 = 0.016666 |
//! | 6 | 12 / 60 = 0.2 | 0.5 x 0.2 = 0.1 |
//! | 8 | 4 / 60 = 0.066666 | 2 x 0.066666 = 0.133332 |
//! | 9 (for the Japanese) | min(70, 60) / 60 = 1 | 4 |
//!
//! Allied: out of action 7 ships (carrier 1, cruiser 4, destroyer 2),
//! 2.416665 + 1 + 2 + 0.566666 + 0.016666 + 0.1 + 0.133332 = 6.233329.
//! Sunk 0. Total 6.233329.
//! Japanese: out of action 1 ship (battleship), 4. Sunk 0. Total 4.
//! Allied margin 2.233329: below the minor level of 3, a stalemate.
//!
//! # Case 2: no horizon stated, so 6 days
//!
//! Every ship with 6 days out or more has a share of 1.
//!
//! | Ship | Share | Points |
//! | --- | --- | --- |
//! | 1 | 1 | 5 |
//! | 2 | 1 | 2 |
//! | 3 | 1 | 2 |
//! | 4 | 1 | 2 |
//! | 5 | 2 / 6 = 0.333333 | 0.5 x 0.333333 = 0.166666 |
//! | 6 | 1 | 0.5 |
//! | 8 | 4 / 6 = 0.666666 | 2 x 0.666666 = 1.333332 |
//! | 9 | 1 | 4 |
//!
//! Allied 5 + 2 + 2 + 2 + 0.166666 + 0.5 + 1.333332 = 12.999998.
//! Japanese 4. Allied margin 8.999998: at least the major level of 8 and
//! below the decisive 16, a major Allied victory.
//!
//! # Case 3: the horizon stated, 1,300 days
//!
//! | Ship | Share | Points |
//! | --- | --- | --- |
//! | 1 | 29 / 1300 = 0.022307 | 5 x 0.022307 = 0.111535 |
//! | 2 | 30 / 1300 = 0.023076 | 2 x 0.023076 = 0.046152 |
//! | 3 | 80 / 1300 = 0.061538 | 2 x 0.061538 = 0.123076 |
//! | 4 | 17 / 1300 = 0.013076 | 2 x 0.013076 = 0.026152 |
//! | 5 | 2 / 1300 = 0.001538 | 0.5 x 0.001538 = 0.000769 |
//! | 6 | 12 / 1300 = 0.009230 | 0.5 x 0.009230 = 0.004615 |
//! | 8 | 4 / 1300 = 0.003076 | 2 x 0.003076 = 0.006152 |
//! | 9 | 70 / 1300 = 0.053846 | 4 x 0.053846 = 0.215384 |
//!
//! Allied 0.111535 + 0.046152 + 0.123076 + 0.026152 + 0.000769 + 0.004615
//! + 0.006152 = 0.318451. Japanese 0.215384. Allied margin 0.103067: a
//! stalemate.
//!
//! # Refusals of the loader (each a variant of the scenario file)
//!
//! - `horizon_days = 5` with `days = 6`: refused.
//! - `horizon_days = 0`: refused.
//! - The Japanese side given `ships_crippled` beside the Allied
//!   `ships_out_of_action`: refused.
//!
//! # Unit tests, for what the scenario cannot reach (in `objectives.rs`)
//!
//! - A ship sunk after a yard trip is counted as sunk and not listed out of
//!   action.
//! - A ship that left with 45 days, came back whole and ends with flotation
//!   3 has 45 + 6 = 51 days out; at a horizon of 60 a cruiser of 2 points
//!   scores 2 x 0.85 = 1.7.
//! - A ship that left twice, with 20 and 50 days, has 70 days out and a
//!   share of 1 at a horizon of 60.
//! - `known_progress` gives nothing for the objective, on the basis it gives
//!   for `ships_crippled`.
//!
//! # Properties
//!
//! - The result is the same for every seed (seeds 1 to 50): the rule and the
//!   scenario make no draw that bears on it.
//! - A side's points for the objective never exceed the sum of its points
//!   for each enemy ship afloat.
//! - Every scenario that does not use the objective has the result, the
//!   event log and the hashes it had before.
