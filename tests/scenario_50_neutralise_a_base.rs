//! Scenario 50: the commander states one end, "neutralise base 2", and the
//! staff at the area's level gives base 1 the task of raiding it, and at
//! the base's level decides which mornings the raid goes.
//!
//! Proves decision 0120 against a hand calculation. The data is placeholder
//! data: see `data/scenarios/50_neutralise_a_base.toml`. It runs on the
//! frozen step 1 figures, with the fatigue figures loaded and the wear,
//! upkeep and supply figures not. Base 2 has nothing at it: no patrol
//! rises, no gun fires and no aircraft is lost. The bombs' hits on base 2
//! are rolled, and nothing here depends on them or asserts them.
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
//! Fatigue, as scenario 47: a group that flew in a daylight pulse gains 3
//! and 2 for each 100 nm of its longest sortie, there and back; a group
//! that did not fly in a daylight pulse loses 5; never below 0. A group is
//! able to fly its ready aircraft times 100 less its fatigue, over 100, cut
//! to a whole aircraft. A base strike launches once a day, in the morning
//! where the base is in reach then. A fighter group's escorts are the
//! aircraft it is able to fly less its patrol, and they go with the raid
//! from their base. A hex is 40 nm.
//!
//! The placeholder dive bomber (type 3): a bomb, a strike radius of 250
//! nm. The placeholder fighter (type 4): a reach of 250 nm.
//!
//! Base 2 is 5 hexes from base 1: 200 nm. A raid is a sortie of 400 nm: a
//! group that flies it gains 3 + 8 = 11 in the morning and, not flying in
//! the afternoon, loses 5: 6 in the day. A day on which a group does not
//! fly at all takes 10 off.
//!
//! ## Inputs
//!
//! A placeholder map of open sea. Allied base 1 at (0,0). Japanese base 2
//! at (5,0), on the Allied map, with nothing at it. The game is 12 days.
//!
//! | Group | Type        | Ready aircraft | At the start |
//! | ----- | ----------- | -------------- | ------------ |
//! | 1     | dive bomber | 10             | resting, no held share |
//! | 2     | fighter     | 8              | resting, no setting (a patrol share of 1) |
//! | 3     | dive bomber | 10             | resting, no held share |
//!
//! The Allied side's commands of day 1: intent 1, neutralise base 2; leave
//! to prepare and leave to launch both given. No limit is set. So the risk
//! is balanced: a bomber rests above a fatigue of 45, holds none back, a
//! raid sends at least 6 and goes without an escort if it must, and a group
//! stands down after losing a quarter of what it sent.
//!
//! ## The tree on the first morning
//!
//! Plan 1 is drafted, made ready and released. At the area's level the
//! staff finds one base of the side, base 1, with bombers that reach base
//! 2 on normal reach, 200 nm against 250, and forms the task.
//!
//! | Intent | Kind | Parent | Maker | Addressee | Plan |
//! | ------ | ---- | ------ | ----- | --------- | ---- |
//! | 1      | neutralise base 2 | none | the commander | the area | 1 |
//! | 2      | raid base 2 from base 1 | 1 | the staff | base 1 | 1 |
//!
//! The reason of the forming serves intent 1, was decided at the area's
//! level, and gives the distance and the reach. Every order below serves
//! intent 2, was decided at the base's level, and has a chain of one:
//! intent 1, the commander's.
//!
//! ## Day by day
//!
//! Fatigue is read in the morning. All three groups have the same fatigue
//! on every morning.
//!
//! | Day | Fatigue | The raid | Each bomber group sends | Escorts | The staff's orders |
//! | --- | ------- | -------- | ----------------------- | ------- | ------------------ |
//! | 1   | 0       | goes     | 10                      | 8       | Groups 1 and 3: strike base 2. Group 2: patrol, share 0 |
//! | 2   | 6       | goes     | 9                       | 7       | none |
//! | 3   | 12      | goes     | 8                       | 7       | none |
//! | 4   | 18      | goes     | 8                       | 6       | none |
//! | 5   | 24      | goes     | 7                       | 6       | none |
//! | 6   | 30      | goes     | 7                       | 5       | none |
//! | 7   | 36      | goes     | 6                       | 5       | none |
//! | 8   | 42      | goes     | 5                       | 4       | none |
//! | 9   | 48      | does not |                         |         | Groups 1 and 3: rest |
//! | 10  | 38      | goes     | 6                       | 4       | Groups 1 and 3: strike base 2 |
//! | 11  | 44      | goes     | 5                       | 4       | none |
//! | 12  | 50      | does not |                         |         | Groups 1 and 3: rest |
//!
//! At the end of day 12 every group has fatigue 40.
//!
//! Why, where it is not plain:
//!
//! - **Day 1** has four orders: a mission for each bomber group, and for
//!   group 2 the patrol mission and the share. A mission and a setting are
//!   two orders. No held share is ordered: the groups have none and
//!   balanced asks for none.
//! - **What goes.** 10 x 0.94 = 9.4, cut to 9, and so on down the column.
//!   The fighters: 8 x 0.94 = 7.52, cut to 7; 8 x 0.88 = 7.04; 8 x 0.82 =
//!   6.56; 8 x 0.76 = 6.08; 8 x 0.70 = 5.6; 8 x 0.64 = 5.12; 8 x 0.58 =
//!   4.64; on day 10, 8 x 0.62 = 4.96; on day 11, 8 x 0.56 = 4.48. On
//!   every raid morning the two groups send 10 or more between them,
//!   against the 6 a balanced raid asks.
//! - **Day 9.** Both bomber groups are at 48, above 45: neither goes, the
//!   raid does not go, and both rest. Group 2 is given no order: it keeps
//!   its mission and, with no raid, flies nothing. All three lose 10.
//! - **Day 10.** At 38 the bombers go again.
//! - **Day 12.** At 50 they rest.
//!
//! Ten raids are launched, on days 1 to 8, 10 and 11. The staff gives 4,
//! 0, 0, 0, 0, 0, 0, 0, 2, 2, 0 and 2 orders: 10 in all. No staff order
//! and no staff command is refused, and no notice is handed in.
//!
//! ## The measure
//!
//! At the end of day 12 the measure of intent 1 reads: 12 days in force;
//! 10 raids flown at base 2; no air raid on a base of the side, so it was
//! met on every day. The aircraft claimed on the ground are not asserted.
//!
//! # Properties
//!
//! - **Preserve the force.** With the risk of intent 1 set to preserve on
//!   day 1, day 1 has six orders: the four above and a held share of
//!   0.333333 for groups 1 and 3. On days 1 and 2 each bomber group sends
//!   6 (10 x 0.666667 = 6.66667, and 9 x 0.666667 = 6.000003, each cut to
//!   6): 12 against the 12 preserve asks, and the raid goes. On day 3, at
//!   12, each is able to fly 8 and sends 5: 10 against 12. The raid does
//!   not go, both rest, and the reason gives 10 against 12. On day 4, at
//!   2, each is able to fly 9 and sends 6, and the raid goes.
//! - **Press.** With the risk of intent 1 set to press on day 1, the raid
//!   goes on days 1 to 11, the last at a fatigue of 60, which is not above
//!   the level, with 4 from each bomber group. On day 12, at 66, both
//!   rest. The staff gives 4 orders on day 1 and 2 on day 12.
//! - **No escort.** With no group 2: at balanced the raids go as in the
//!   main case, with no escort. At preserve no raid goes on any day, and
//!   the staff hands in on day 1 that it cannot meet the intent.
//! - **Out of reach.** With base 2 at (7,0), 280 nm: the staff forms no
//!   raid task, hands in on day 1 that it cannot meet intent 1, and its
//!   reason gives 280 nm against a reach of 250.
//! - **What the last raid cost.** In a view built for it, at balanced: a
//!   group that sent 10 on its last raid and lost 3 stands down the next
//!   morning, and its reason gives 3 of 10 against a quarter; one that
//!   lost 2 goes. At press the first goes too.
//! - **A base held and a base to neutralise.** With the commander also
//!   stating, first, intent 1, hold base 1, and neutralise base 2 as
//!   intent 2: the staff forms cover of base 1 as intent 3 and the raid as
//!   intent 4. On day 1 group 2 is given the patrol and a share of 0.5,
//!   serving intent 3, and the raid task gives it no order; groups 1 and 3
//!   are given the strike, serving intent 4. Four orders. The raid of day
//!   1 has 20 bombers and 4 escorts.
//! - **A raid stated alone.** With the commander stating "raid base 2 from
//!   base 1" as intent 1 and nothing else, the orders are the main case's,
//!   each serving intent 1 with an empty chain.
//! - **A held group is left alone:** with group 1 held from day 3, it is
//!   given no order from then, and group 3 goes and rests by the main
//!   case's days: it sends 8 on day 3, against the 6 a raid asks.
//! - **Withdrawing the end** on day 5 withdraws the raid task that day,
//!   and the staff gives no order from then. The groups keep the missions
//!   they had.
//! - **The level is named.** Every reason serving intent 2 says the base
//!   decided, and the reason of the forming says the area decided.
//! - The staff orders no group the raid task may not use, and the same
//!   start gives the same orders.
//! - Two worlds that differ only in what the Allied side cannot see give
//!   the same intents, orders and reasons.
//! - Every staffed scenario built before this one gives the orders it
//!   gave.
