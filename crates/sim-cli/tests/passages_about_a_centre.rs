//! The check of passages on a map about a centre (decision 0242, rule 15).
//!
//! # The hand calculation
//!
//! Written by the architect on Nov 28, 2026, before any code. Nobody else
//! changes a number here; a mismatch goes back to the architect.
//!
//! On the map from Papua to Midway (`data/map/papua_to_midway.csv`) the check
//! works no position into a hex. It reads each end's hex from the map's file
//! of located positions, `data/map/papua_to_midway_located.csv`, by the row of
//! `data/oob/place_positions.csv` the passage names.
//!
//! ## One passage, from Midway to Wake
//!
//! - From row 5100, Midway: the located file gives hex (31,-41).
//! - To row 5107, Wake: the located file gives hex (2,-26).
//! - A hex (q, r)'s centre is 40 x (q + r/2) miles east and 34.641016 x r
//!   miles south of hex (0,0)'s. So (31,-41) is at east 420.000, north
//!   1420.282, and (2,-26) at east -440.000, north 900.666.
//! - The two are 29 steps apart: q falls by 29 and r rises by 15, which is 29
//!   steps on a hex grid. Open sea lies between them, so the way is clear.
//! - Where the way is clear a ship is charged the straight line between the
//!   centres (decision 0058): the root of 860.000 squared and 519.616 squared,
//!   1004.79, charged as 1,005 miles.
//!
//! The check reports the passage with its ends as "row 5100, midway, (31,-41)"
//! and "row 5107, wake, (2,-26)" and its distance as 1,005 miles. What it says
//! of the time is what it says for 1,005 miles on any map.
//!
//! ## What stops it
//!
//! - An end given as a bare latitude and longitude, with no row: the check
//!   stops with an error that names the passage and the end and says that on
//!   a map about a centre an end is a row of the place positions file, since
//!   only the map tool works a position into a hex there.
//! - An end whose row is not in the located file: an error that names the
//!   row and the located file.
//! - A map about a centre with no located file beside it: an error that names
//!   the file wanted.
//!
//! ## What does not change
//!
//! On a map of one factor the check works in whole numbers as it did, reads
//! no located file, and reports what it reported: the existing tests of the
//! check hold without a change.
