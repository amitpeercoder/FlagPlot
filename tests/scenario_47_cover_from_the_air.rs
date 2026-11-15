//! Scenario 47: the staff covers a base from the air. Two fighter groups,
//! a flying boat group and a dive bomber group at one base are given their
//! missions, patrol share, search arc and rest by the staff from one cover
//! intent, for 12 days.
//!
//! Proves decision 0115 against a hand calculation. The data is placeholder
//! data: see `data/scenarios/47_cover_from_the_air.toml`. It runs on the
//! frozen step 1 figures, with the fatigue figures loaded and the wear,
//! upkeep and supply figures not. The only Japanese thing is a base with
//! nothing at it, so nothing is sighted, no raid comes and no die is rolled.
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
//! Fatigue: a group that flew in a daylight pulse gains 3 and 2 for each
//! 100 nm of its longest sortie, there and back; a group that did not fly
//! in a daylight pulse loses 5; never below 0 or above 100. A patrol is a
//! sortie of no distance: 3. A search at 600 nm is a sortie of 1200 nm: 3 +
//! 24 = 27. A group flies its ready aircraft times 100 less its fatigue,
//! over 100, cut to a whole aircraft. A fighter group on patrol keeps its
//! share of those overhead, cut, and flies a patrol sortie in each daylight
//! pulse in which that is at least one aircraft. A contact gains a pulse of
//! age at the end of each pulse and is dropped when its age reaches 5.
//! Bearings are clockwise from north on a ten-degree grid; from a hex, the
//! hex 5 to its east at (5,0) lies on 090 and the hex at (0,5) on 150, and
//! an arc holds a hex that lies on its edge.
//!
//! The placeholder types: the fighter (type 4), the flying boat (type 1,
//! search range 600 nm) and the dive bomber (type 3, no search range).
//!
//! ## Inputs
//!
//! A placeholder map of open sea. Allied base 1 at (0,0). Japanese base 2
//! at (5,0), on the Allied map, with nothing at it. The game is 12 days.
//!
//! | Group | Type        | Ready aircraft | At the start |
//! | ----- | ----------- | -------------- | ------------ |
//! | 1     | fighter     | 12             | resting, no setting |
//! | 2     | fighter     | 8              | resting, no setting |
//! | 3     | flying boat | 6              | resting, no arc |
//! | 4     | dive bomber | 10             | resting |
//!
//! The Allied side starts holding one contact: 2 ships, a cruiser, at
//! (0,5), age 0.
//!
//! The Allied side's commands of day 1: intent 1, cover base 1; its forces
//! limit, groups 1, 2, 3 and 4; leave to prepare and leave to launch both
//! given. No risk is set: it is balanced. So the patrol share is 0.5 and a
//! group rests when its fatigue in the morning is above 45.
//!
//! ## The search arc
//!
//! - **Days 1 and 2.** The contact is 0 pulses old on the morning of day 1
//!   and 3 on the morning of day 2. The threats are base 2 at (5,0), on
//!   090, and the contact at (0,5), on 150. The narrowest arc that holds
//!   both runs from 090 to 150. Widened by 30 at each end it is 060 to 180.
//! - **From day 3.** The contact was dropped at the end of the morning of
//!   day 2, at 5 pulses. The one threat is base 2 on 090. The arcs of ten
//!   degrees that hold it are 080 to 090 and 090 to 100; the lower start is
//!   taken: 080 to 090. Widened it is 050 to 120.
//!
//! ## Day by day
//!
//! Fatigue is read in the morning, before the day's orders. "Flies" is a
//! day on patrol or on search; "rests" is a day on the rest mission.
//!
//! | Day | Group 1: fatigue, day | Group 2: fatigue, day | Group 3: fatigue, day | The staff's orders |
//! | --- | --------------------- | --------------------- | --------------------- | ------------------ |
//! | 1   | 0, flies              | 0, flies              | 0, flies              | Groups 1 and 2: patrol, share 0.5. Group 3: search, arc 060 to 180 at 600 nm |
//! | 2   | 6, flies              | 6, flies              | 54, rests             | Group 3: rest |
//! | 3   | 12, flies             | 12, flies             | 44, flies             | Group 3: search, arc 050 to 120 at 600 nm |
//! | 4   | 18, flies             | 18, flies             | 98, rests             | Group 3: rest |
//! | 5   | 24, flies             | 24, flies             | 88, rests             | none |
//! | 6   | 30, flies             | 30, flies             | 78, rests             | none |
//! | 7   | 36, flies             | 36, flies             | 68, rests             | none |
//! | 8   | 42, flies             | 42, flies             | 58, rests             | none |
//! | 9   | 48, rests             | 48, flies             | 48, rests             | Group 1: rest |
//! | 10  | 38, flies             | 54, rests             | 38, flies             | Group 1: patrol. Group 2: rest. Group 3: search |
//! | 11  | 44, flies             | 44, flies             | 92, rests             | Group 2: patrol. Group 3: rest |
//! | 12  | 50, rests             | 50, flies             | 82, rests             | Group 1: rest |
//!
//! At the end of day 12: group 1 has fatigue 40, group 2 has 56, group 3
//! has 72 and group 4 has 0.
//!
//! Why, where it is not plain:
//!
//! - **Day 1** has seven orders: for each fighter group the patrol mission
//!   and the share; for group 3 the search mission and the arc. A mission
//!   and a setting are two orders. Group 4 is given none on any day: it is
//!   not a fighter and cannot search, and the day's reason says it is left
//!   for a strike intent.
//! - **Group 3's days in the air.** On day 1 it flies 6 in the morning and
//!   gains 27; in the afternoon 6 x 0.73, cut to 4, fly and it gains 27: 54.
//!   On day 3, at 44, 6 x 0.56, cut to 3, fly: 71; then 6 x 0.29, cut to 1:
//!   98. On day 10, at 38, 3 fly: 65; then 6 x 0.35, cut to 2: 92.
//! - **Day 3.** Group 3 is at 44, not above 45: it flies, and is given the
//!   search mission and the new arc. The arc's reason names the contact as
//!   gone.
//! - **Day 9.** Both fighter groups are at 48, above 45. They are taken
//!   most tired first, then by number: group 1 rests, and group 2, the last
//!   that can fly, is kept on patrol though it is above the level. Group 3
//!   is at 48 and rests: it has the rest mission already, so no order.
//! - **Day 10.** Group 1 is at 38 and flies. Group 2 is at 54 and may now
//!   rest, since group 1 flies.
//! - **Day 11.** Both fighters are at 44 and fly.
//! - **Day 12.** Both are at 50: group 1 rests and group 2 is kept.
//!
//! On every day at least one fighter group flies the patrol: both on days
//! 1 to 8 and 11, group 2 on days 9 and 12, group 1 on day 10. On a day a
//! fighter group flies it gains 6 and on a day it rests it loses 10. Group
//! 3 searches on days 1, 3 and 10 and no others.
//!
//! No staff order is refused. Every order and every morning without one
//! has a reason with these numbers: the fatigue against 45, the share, the
//! threats and the arc.
//!
//! # Properties
//!
//! - **The risk level.** At preserve the force the share is 0.75 and the
//!   level 30: both fighters first reach 36 on the morning of day 7, when
//!   group 1 rests. At press the share is 0.25 and the level 60: neither
//!   fighter rests in the 12 days, the highest morning fatigue being 66 on
//!   day 12, when group 1 rests.
//! - **A raid raises the share.** In a variant with a Japanese bomber group
//!   at base 2 set to strike base 1 on day 3, the share of both fighter
//!   groups is 0.75 on days 4, 5 and 6 and 0.5 again on day 7.
//! - **No threat, the whole circle.** With base 2 off the Allied map and no
//!   contact, group 3's arc is the whole circle.
//! - **Nothing to cover with.** With both fighter groups at no ready
//!   aircraft, the staff hands in that it cannot meet the intent on day 1
//!   and the standing interrupt fires that day and on no later day.
//! - **A held group is left alone:** with group 2 held from day 5, it is
//!   given no order from then, and on day 9 group 1, now the only fighter
//!   the staff may order, is kept on patrol.
//! - **An override.** A commander's order to group 3 on day 2 holds it, and
//!   the staff gives it no order after.
//! - The staff never orders a group outside the intent's forces limit, and
//!   the same start gives the same orders.
//! - Two worlds that differ only in what the Allied side cannot see give
//!   the same orders and reasons.
