//! Scenario 59: a search that can be kept up. A flying boat group under a
//! cover searches to 600 nm every morning for 12 days, where under the
//! rules before it would have been exhausted on the second.
//!
//! Proves decision 0131 against a hand calculation. The data is placeholder
//! data: see `data/scenarios/59_a_search_kept_up.toml`. It runs on the
//! frozen step 1 figures with the fatigue figures loaded and one figure
//! added in the scenario's own file: a search figure of 0.15 for each 100
//! nm. The wear, upkeep and supply figures are not loaded. The only
//! Japanese thing is a base with nothing at it: nothing is sighted and no
//! die is rolled.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Figures
//!
//! A group that flew in a daylight pulse gains 3 for the sortie and, for a
//! search, 0.15 for each 100 nm of it, there and back; for any other
//! sortie, 2. A group that did not fly in a daylight pulse loses 5; never
//! below 0. The placeholder flying boat (type 1) searches to 600 nm: a
//! sortie of 1200 nm, which gains 3 + 12 x 0.15 = 4.8. A group flies its
//! ready aircraft times 100 less its fatigue, over 100, cut to a whole
//! aircraft. From (0,0) the hex (5,0) lies on 090.
//!
//! ## Inputs
//!
//! Scenario 47's map: Allied base 1 at (0,0); Japanese base 2 at (5,0), on
//! the Allied map, with nothing at it. The game is 12 days.
//!
//! One group at base 1: group 1, placeholder flying boats, 6 ready,
//! resting, with no arc and the search-once setting off.
//!
//! The Allied side's commands of day 1: intent 1, cover base 1; leave to
//! prepare and leave to launch both given. No risk: it is balanced. A
//! group rests when its fatigue in the morning is above 45.
//!
//! ## The orders of day 1
//!
//! Three, serving intent 1: the search mission; search once a day, on;
//! and the arc. The one threat is base 2, on 090: the arc is 080 to 090,
//! widened to 050 to 120, at 600 nm, as scenario 47 has it from day 3.
//!
//! ## Day by day
//!
//! Every day is the same. In the morning the group is at fatigue 0 and
//! flies its 6. It gains 4.8. In the afternoon it does not fly and loses
//! 5: it is at 0 again.
//!
//! On every morning of the 12 days its fatigue is 0, it flies 6 aircraft
//! in the morning and none in the afternoon, and it never rests. The staff
//! gives three orders on day 1 and none after.
//!
//! # Properties
//!
//! - **In both pulses, at press.** With the risk of intent 1 set to press
//!   on day 1: the search-once order is not given, and the group searches
//!   in both daylight pulses, gaining 9.6 a day. Its mornings are 0, 9.6,
//!   19.2, 28.8, 38.4, 48, 57.6 and, on day 8, 67.2, above the 60 of
//!   press: it rests, losing 10. Then 57.2 on day 9, when it flies; 66.8
//!   on day 10, when it rests; 56.8 on day 11, when it flies; 66.4 on day
//!   12, when it rests. The staff gives 2 orders on day 1 and one on each
//!   of days 8 to 12: seven.
//! - **Without the search figure.** With no search figure in the file, a
//!   search costs what every sortie costs: 3 + 12 x 2 = 27. Once a day, at
//!   balanced, the group gains 22 a day: its mornings are 0, 22 and 44 on
//!   days 1 to 3, when it flies, and 66 on day 4, above 45. It rests on
//!   days 4, 5 and 6, at 66, 56 and 46; flies on day 7 at 36; rests on
//!   days 8 and 9 at 58 and 48; flies on day 10 at 38; rests on days 11
//!   and 12 at 60 and 50. It searches on days 1, 2, 3, 7 and 10.
//! - **The rule alone.** A group on the search mission with the setting on
//!   records a search in the morning and none in the afternoon; with it
//!   off, in both. A strike group's fatigue for a sortie of 1200 nm is 27
//!   whether or not the search figure is given.
//! - **The least tired bomber searches, on a carrier.** In a view built
//!   for it: a force with no group that can only search and two bomber
//!   groups that can search, at fatigue 10 and 4: the one at 4 searches,
//!   once a day at balanced, and the other strikes. At equal fatigue the
//!   lower numbered searches. A group that cannot fly is passed over.
//! - **The least tired bomber searches, at a base.** In a view built for
//!   it: a base with a cover that has no searcher and a strike task with
//!   two bomber groups that can search: the less tired flies the cover's
//!   arc and the other holds the strike. With one such group, it holds
//!   the strike and nobody searches. With a group at the base that can
//!   only search, it searches and both bombers strike.
//! - **An order to set search once a day** on a group that is not the
//!   side's is refused.
//! - Every hand-calculation test on the frozen figures built before this
//!   one passes untouched: none gives a search figure or the setting.
