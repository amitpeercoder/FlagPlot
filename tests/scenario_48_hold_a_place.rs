//! Scenario 48: the commander states one end, "hold base 1", and the staff
//! forms cover from the air under it and gives the orders of scenario 47.
//!
//! Proves decision 0117, the intent tree in the command state and the end
//! "hold a place", against a hand calculation. It stands on scenario 47's
//! hand calculation for the cover doctrine (decision 0115) and repeats none
//! of its arithmetic. The data is placeholder data: see
//! `data/scenarios/48_hold_a_place.toml`. It runs on the frozen step 1
//! figures, with the fatigue figures loaded and the wear, upkeep and supply
//! figures not. No die is rolled.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Inputs
//!
//! Scenario 47's, unchanged: the map, Allied base 1 at (0,0), Japanese base
//! 2 at (5,0) with nothing at it, the four groups at base 1 (fighters 1 and
//! 2, flying boats 3, dive bombers 4), the one contact at (0,5), 12 days.
//!
//! The Allied side's commands of day 1 are different: intent 1, hold base
//! 1; leave to prepare and leave to launch both given. No forces limit and
//! no risk are set, on anything.
//!
//! ## The tree on the first morning
//!
//! Plan 1 is drafted for intent 1 when it is set. The staff makes it ready
//! and releases it, by the two leaves. Intent 1 has no forces limit and
//! nothing above it, so it may use what its addressee commands: the air
//! groups at base 1, which are groups 1, 2, 3 and 4. Groups 1 and 2 are
//! fighters and group 3 can search, and no cover intent for base 1 is in
//! force, so the staff forms one.
//!
//! | Intent | Kind | Parent | Maker | Addressee | Plan | Set |
//! | ------ | ---- | ------ | ----- | --------- | ---- | --- |
//! | 1      | hold base 1 | none | the commander | base 1 | 1 | day 1 |
//! | 2      | cover base 1 from the air | 1 | the staff | base 1 | 1 | day 1 |
//!
//! Intent 2 takes the number 2: one above the highest the side has used.
//! There is one plan. The reason handed in with the forming serves intent
//! 1, and its chain is empty: intent 1 is the commander's own.
//!
//! Intent 2 has no limit of its own. Read up the tree, its forces are the
//! four groups and its risk is balanced: the share is 0.5 and a group rests
//! when its fatigue in the morning is above 45.
//!
//! ## Day by day
//!
//! The cover doctrine runs for intent 2 on the morning it is formed. From
//! there every order, every group's fatigue on every morning and at the end
//! of day 12, and the search arc of every day are those of scenario 47's
//! table. By day the staff gives 6, 1, 2, 1, 0, 0, 0, 0, 1, 3, 2 and 1
//! orders: 17 in all.
//!
//! Every one of the 17, and the reason of every morning without an order,
//! serves intent 2 and plan 1, and its chain is one intent long: intent 1,
//! made by the commander, with its statement in words. No staff order and
//! no staff command is refused.
//!
//! ## The measure
//!
//! At the end of day 12 the measure of intent 1 reads: 12 days in force; 12
//! days on which base 1 was Allied and its airfield open; no air raid on
//! it and none fought. It was met on every day. Its days of supply are not
//! asserted: the supply figures are not loaded.
//!
//! # Properties
//!
//! - **A forces limit is read up the tree.** With a forces limit on intent
//!   1, set on day 1, that names groups 1 and 3: group 2 is given no order
//!   on any day and ends at fatigue 0. Group 1 is the only fighter the
//!   staff may order: it is given the patrol and the share on day 1 and no
//!   order after, it is kept on patrol on day 9 at 48 and on every later
//!   day, and it ends day 12 at 72. Group 3's days are unchanged. The staff
//!   gives 4, 1, 2, 1, then none until 1 on day 10 and 1 on day 11: 10
//!   orders.
//! - **A risk level is read up the tree.** With the risk of intent 1 set to
//!   preserve the force on day 1, the share ordered on day 1 is 0.75, and
//!   both fighters first pass the level of 30 on the morning of day 7, at
//!   36, when group 1 rests and group 2 is kept.
//! - **A limit on a sub-intent may narrow.** In the main case, with the
//!   commander setting the risk of intent 2 to preserve the force on day
//!   2: it is taken. That morning the staff gives three orders: a share of
//!   0.75 to group 1 and to group 2, and rest to group 3, which is at 54.
//!   Group 1 rests on day 7, as above.
//! - **And may not widen.** In the main case, with the commander setting
//!   the risk of intent 2 to press on day 2: it is refused, as bolder than
//!   its parent's, and the orders are those of the main case. A forces
//!   limit on intent 2 that names a group intent 1 may not use is refused
//!   the same way: in the first property's variant, one naming group 2.
//! - **A staff-made intent is the staff's.** The commander's `set_intent`
//!   and `withdraw_intent` on intent 2 are each refused and change nothing.
//! - **Withdrawing a tree.** With intent 1 withdrawn on day 5: intent 2 is
//!   withdrawn on day 5 by an event caused by intent 1's, plan 1 ends
//!   complete, and the staff gives no order and no reason from day 5. The
//!   groups keep the missions they had: groups 1 and 2 the patrol, group 3
//!   rest.
//! - **Nothing before release.** With leave to launch not given: on day 1
//!   the staff makes plan 1 ready, hands in that it waits for leave to
//!   launch, forms no intent and gives no order, and none on day 2. With
//!   the commander approving plan 1 on day 3, the staff forms intent 2 that
//!   morning and gives six orders: the patrol and a share of 0.5 to groups
//!   1 and 2, and to group 3 the search mission and the arc 050 to 120 at
//!   600 nm, the contact being gone.
//! - **A cover intent that stands.** With the commander also stating
//!   intent 2, cover base 1, on day 1: the staff forms nothing under intent
//!   1, and its reason names intent 2. The orders are scenario 47's, each
//!   serving intent 2 with an empty chain.
//! - **Nothing to cover with.** With a forces limit on intent 1 that names
//!   group 4 alone: the staff forms nothing, on every day, gives no order,
//!   and its reason says there is nothing at the base to cover it with.
//! - **The top of every chain is the commander's.** In every variant,
//!   every reason's chain, followed to its end, reaches an intent the
//!   commander made, and `why` on any staff order prints it.
//! - **Stage 1 is unchanged.** Every staffed scenario of stage 1 gives the
//!   orders it gave. Its intents are roots, made by the commander and
//!   addressed to the area.
//! - Two worlds that differ only in what the Allied side cannot see give
//!   the same intents, orders and reasons, and the same start gives the
//!   same.
