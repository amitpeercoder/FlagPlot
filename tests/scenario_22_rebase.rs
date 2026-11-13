//! Scenario 22: an air group is moved to another base by order.
//!
//! Proves decision 0091 (rebasing an air group by order) against a hand
//! calculation. The data is placeholder data: see
//! `data/scenarios/22_rebase.toml`. The test runs on the frozen step 1
//! figures (`data/reference/step1`).
//!
//! # Hand calculation
//!
//! Written by the architect before the code. The tests below assert these
//! numbers exactly. If the code disagrees, the code or the rule is wrong:
//! do not change a number here to make a test pass. A mismatch goes back to
//! the architect.
//!
//! ## Inputs
//!
//! A placeholder map with three Allied bases on one row: base 1 at (0,0),
//! base 2 at (5,0) and base 3 at (13,0). A hex is 40 nm, so base 2 is 200 nm
//! from base 1 and base 3 is 520 nm from base 1 and 320 nm from base 2. No
//! base repairs aircraft. There is no Japanese unit. The game is 3 days.
//!
//! - Air group 1: placeholder flying boat (search range 600 nm), at base 1,
//!   6 ready and 2 damaged, on a search mission that it can fly every day.
//! - Air group 2: placeholder fighter (strike range 250 nm, search range 0),
//!   at base 1, 12 ready.
//!
//! Transfer range is twice the longer of the two ranges: 1,200 nm for the
//! flying boat and 500 nm for the fighter.
//!
//! Orders, all on day 2:
//!
//! - Rebase group 1 to base 2.
//! - Rebase group 2 to base 3.
//! - Rebase group 2 to base 1.
//!
//! ## What happens
//!
//! Day 1. Group 1 flies its search from (0,0) in both daylight pulses. At
//! the end of the day it is at base 1 with 6 ready and 2 damaged.
//!
//! Day 2, at intake:
//!
//! - Group 1 to base 2: 200 nm is within 1,200. Accepted. The group is at
//!   base 2 with 6 ready and 0 damaged. 2 damaged aircraft are written off
//!   at base 1.
//! - Group 2 to base 3: 520 nm is beyond 500. Refused, as beyond its
//!   transfer range.
//! - Group 2 to base 1: refused, as already there.
//!
//! Group 1 flies no search in either daylight pulse of day 2. Group 2 is at
//! base 1 with 12 ready throughout.
//!
//! Day 3. Group 1 flies its search from (5,0) in both daylight pulses.
//!
//! At the end: group 1 at base 2, 6 ready, 0 damaged; group 2 at base 1, 12
//! ready; 2 aircraft written off in all, both of group 1 on day 2.
//!
//! # Properties
//!
//! - Aircraft are conserved: a group's ready and damaged aircraft before a
//!   move equal its ready aircraft after it plus those written off.
//! - The enemy's view is the same in two worlds that differ only in a rebase
//!   order.
//! - A scenario with no rebase order is unchanged, apart from the hash
//!   version.
