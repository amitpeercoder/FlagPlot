//! Scenario 134: the estimate of the enemy's boats, as a reading (decision
//! 0223 as amended on review, stage D1, with the architect's rulings in its
//! section "Stage D1: the statements, as written").
//!
//! # The architect's statements
//!
//! Written by the architect on Nov 23, 2026, before any code. Nobody else
//! changes a statement, a hex or a count here; a mismatch goes back to the
//! architect. The areas and counts are worked by hand below.
//!
//! ## Figures
//!
//! The frozen step 1 figures with `position_error_chance` 0, through
//! `common::altered_reference`, so that a report of a submarine names her
//! own hex. The submarine figures are the scenario's own
//! (`submarines_placeholder_scenario_134.toml`): scenario 128's with
//! `submarine_surfaced_share` 1 and `air_attack_damages_chance` 0;
//! `submarine_report_drop_age_pulses` is 6. The doctrine's figure
//! `estimate_join_hexes` is 2, a judgement, in the code. The data is
//! placeholder data.
//!
//! Hexes are axial (q, r); the distance between two is (|dq| + |dr| +
//! |dq + dr|) / 2; hex order is by q, then r. A hex is 40 nm.
//!
//! ## The game
//!
//! `data/scenarios/the-estimate-of-boats/134_the_estimate_of_boats.toml`,
//! 4 days, on scenario 131's map. Japanese base 1 at (-9,0). Allied base 2
//! at (8,0) with group 1, 100 flying boats; Allied base 3 at (4,0), with
//! nothing. Eight Japanese boats, each on her station, numbered in the
//! order of their hexes:
//!
//! | Boat | Hex | From base 2 | Bearing from base 2 |
//! | --- | --- | --- | --- |
//! | 1 | (-4,2) | 445 nm | 261 |
//! | 2 | (-4,3) | 433 nm | 256 |
//! | 3 | (-3,3) | 394 nm | 255 |
//! | 4 | (-2,2) | 367 nm | 259 |
//! | 5 | (-1,2) | 327 nm | 258 |
//! | 6 | (0,-1) | 342 nm | 276 |
//! | 7 | (0,0) | 320 nm | 270 |
//! | 8 | (6,-6) | 288 nm | 316 |
//!
//! Group 1's orders: day 1, the search mission and the whole circle to 600
//! nm; day 2, the arc 280 to 330; day 3, rest. No commander's command is
//! given and neither side has a staff's order to give. No Allied ship is at
//! sea, so no boat attacks.
//!
//! **What is reported.** A boat on her station in a searching group's arc
//! and range is sighted at the chance of sighting a force there times the
//! surfaced share. With 100 flying boats the first is 57.295779 x 100 x 40
//! = 229,183.116 over the distance times the arc's width, at most 445 x 360
//! = 160,200: 1. The share is 1. So each boat in the arc is reported in
//! each daylight pulse the group flies, at her own hex, and is never
//! damaged. A side holds one report for a hex, the newer.
//!
//! - Day 1, morning and afternoon: all eight, the whole circle.
//! - Day 2, morning and afternoon: boat 8 alone. Her bearing, 316, is
//!   inside 280 to 330; the nearest of the others, boat 6 on 276, is 4
//!   degrees outside it.
//! - Days 3 and 4: none.
//!
//! **The ages.** A report made in the afternoon of day 1 is 0 pulses old at
//! the end of that day, 3 at the end of day 2, and reaches 6 at the end of
//! the afternoon of day 3, when it is dropped. Boat 8's report of the
//! afternoon of day 2 is dropped at the end of the afternoon of day 4.
//!
//! | After day | Reports held |
//! | --- | --- |
//! | 1 | all eight hexes, each of day 1 |
//! | 2 | all eight; (6,-6) of day 2, the others of day 1 |
//! | 3 | (6,-6), of day 2 |
//! | 4 | none |
//!
//! ## The areas, worked by hand
//!
//! The reports are taken in hex order. A report within 2 hexes of the first
//! report of an area joins it, the earliest-begun where there are two; else
//! it begins an area. A report is counted where it is more than one hex
//! from every report counted before it in its area.
//!
//! | Report | To the first report of A, (-4,2) | Of B, (-1,2) | Of C, (0,-1) | So | Counted |
//! | --- | --- | --- | --- | --- | --- |
//! | (-4,2) | | | | begins A | yes: A is 1 |
//! | (-4,3) | 1 | | | joins A | no: 1 from (-4,2) |
//! | (-3,3) | 2 | | | joins A | yes, 2 from (-4,2): A is 2 |
//! | (-2,2) | 2 | | | joins A | no: 1 from (-3,3) |
//! | (-1,2) | 3 | | | begins B, though it is next to (-2,2) of A | yes: B is 1 |
//! | (0,-1) | 4 | 3 | | begins C | yes: C is 1 |
//! | (0,0) | 4 | 2 | 1 | joins B, the earlier begun of B and C | yes, 2 from (-1,2): B is 2 |
//! | (6,-6) | 10 | 8 | 6 | begins D | yes: D is 1 |
//!
//! The distances: (-4,3) from (-4,2) is (0 + 1 + 1) / 2 = 1. (-3,3) from
//! (-4,2): (1 + 1 + 2) / 2 = 2. (-2,2) from (-4,2): (2 + 0 + 2) / 2 = 2,
//! and from (-3,3): (1 + 1 + 0) / 2 = 1. (-1,2) from (-4,2): 3. (0,-1) from
//! (-4,2): (4 + 3 + 1) / 2 = 4, and from (-1,2): (1 + 3 + 2) / 2 = 3. (0,0)
//! from (-4,2): (4 + 2 + 2) / 2 = 4; from (-1,2): (1 + 2 + 1) / 2 = 2; from
//! (0,-1): 1. (6,-6) from (-4,2): (10 + 8 + 2) / 2 = 10; from (-1,2):
//! (7 + 8 + 1) / 2 = 8; from (0,-1): (6 + 5 + 1) / 2 = 6.
//!
//! ## Statements
//!
//! 1. **After day 1** the Allied side's estimate has four areas, in the
//!    order they were begun, each on the basis "reported":
//!
//!    | Area | Centre | Its reports, in hex order | Count | Days |
//!    | --- | --- | --- | --- | --- |
//!    | A | (-4,2) | (-4,2), (-4,3), (-3,3), (-2,2) | about 2 | 1 |
//!    | B | (-1,2) | (-1,2), (0,0) | about 2 | 1 |
//!    | C | (0,-1) | (0,-1) | about 1 | 1 |
//!    | D | (6,-6) | (6,-6) | about 1 | 1 |
//!
//!    Each area is told with its centre, the hex of its first report, and
//!    the distance 2.
//! 2. **No area joins another.** (-1,2) lies next to (-2,2), a report of
//!    area A, and begins area B: it is 3 hexes from A's first report.
//! 3. **The earlier area takes a report within 2 of two.** (0,0) is 2 from
//!    B's first report and 1 from C's, and joins B.
//! 4. **The count is said "about",** never "at least", in every word a
//!    seat is given, and a count is a whole number of 1 or more.
//! 5. **After day 2** the areas are those of statement 1 but for area D's
//!    day, which is 2.
//! 6. **After day 3** there is one area: D, centre (6,-6), about 1, day 2.
//!    **After day 4** there is none, and the estimate holds nothing.
//! 7. **By habit** (the game with `[side.allied]` and
//!    `expects_boats_off_own_bases = true`): beside the reported areas of
//!    each day the estimate names, on the basis "by habit" and with no
//!    count, Allied base 2 at (8,0) and Allied base 3 at (4,0), in base
//!    number order, as waters where boats may be expected. They are named
//!    on every day, after day 4 too, when no report is held. In the game
//!    as filed, where the scenario says nothing, none is named.
//! 8. **The other side.** The Japanese side holds no report and its
//!    estimate holds nothing, in the game as filed and in the game of
//!    statement 7. In the game with `[side.japanese]` and
//!    `expects_boats_off_own_bases = true` its estimate names Japanese base
//!    1 at (-9,0) by habit and nothing else, and the Allied side's is that
//!    of statements 1, 5 and 6.
//! 9. **Told in `list_contacts`.** The Allied seat's `list_contacts` gives
//!    the estimate as `submarine_estimate`, beside `submarine_reports` and
//!    `suspected_waters`, which are what they were: after day 1 the eight
//!    reports, each with its place, day and age, and the four areas of
//!    statement 1. `submarine_estimate` is left out when the estimate holds
//!    nothing. With an area given to the tool, the areas are those whose
//!    centre lies in it, and a base named by habit is given where its hex
//!    lies in it: after day 1, for the hexes within 2 of (0,0), areas B and
//!    C and no other.
//! 10. **Suspected waters stand beside it** (the game with one
//!     `[[suspected_waters]]` entry for the Allied side, the waters within
//!     3 of (-4,2), told on day 1): the entry is given as it was, and the
//!     estimate is that of statement 1: nothing is joined to it or counted
//!     from it.
//! 11. **One command's picture** (the game with `command_levels = true`
//!     and `reports_placeholder_scenario_134.toml`, in which no report
//!     arrives at another command). After day 1 the estimate read from base
//!     2's picture is that of statement 1, and the estimate read from the
//!     area command's picture has no reported area. The Allied seat at the
//!     area is given no `submarine_estimate`; with the habit of statement 7
//!     it is given the two bases and no reported area.
//! 12. **A reading and no more.** The estimate is kept nowhere: the state
//!     layout, the state hash, the saved game and the event log of the game
//!     are what they are with the reading never asked for, and asking for
//!     it twice gives the same answer. No event is recorded for it.
//! 13. **It drives nothing.** No order, command, reason or notice of any
//!     staff differs for it: the comparison of every run on file, live and
//!     frozen, shows no difference from the commit this is built on, the
//!     hashes included. The staff's regard for submarine waters reads the
//!     reports and the suspected waters as it did (scenario 129's and
//!     scenario 131's statements hold as written).
//! 14. **Nothing it cannot see.** The game with a ninth Japanese boat at
//!     (-10,8), 625 nm from base 2 and never reported, and the game with
//!     boat 1's salvoes cut to 1, give the Allied side the estimate of
//!     statements 1, 5 and 6 on every day. The function that makes the
//!     estimate takes a picture, the side's own bases and the side's own
//!     doctrine, and neither a world nor the umpire's view.
//! 15. **The frozen figures are untouched** and no rule's figure file gains
//!     a key: `estimate_join_hexes` is a constant, marked a judgement where
//!     it is written.
//!
//! If the game cannot be set up as written (a seat's `list_contacts` read
//! at a base's command, a boat sighted at a chance of 1), the build says so
//! and it comes back to the architect; no statement is changed to fit.

#[test]
#[ignore = "statement written, not built"]
fn after_day_1_four_areas_with_their_counts() {}

#[test]
#[ignore = "statement written, not built"]
fn no_area_joins_another_and_the_earlier_takes_a_report_within_two() {}

#[test]
#[ignore = "statement written, not built"]
fn the_count_is_said_about() {}

#[test]
#[ignore = "statement written, not built"]
fn the_days_of_the_reports_and_the_areas_as_reports_are_dropped() {}

#[test]
#[ignore = "statement written, not built"]
fn by_habit_a_sides_own_bases_where_its_scenario_says_so() {}

#[test]
#[ignore = "statement written, not built"]
fn the_other_side_is_given_nothing_of_it() {}

#[test]
#[ignore = "statement written, not built"]
fn told_in_list_contacts_beside_the_reports_and_the_suspected_waters() {}

#[test]
#[ignore = "statement written, not built"]
fn one_commands_picture_and_nothing_else() {}

#[test]
#[ignore = "statement written, not built"]
fn a_reading_kept_nowhere_that_drives_nothing() {}

#[test]
#[ignore = "statement written, not built"]
fn nothing_the_side_cannot_see_changes_it() {}
