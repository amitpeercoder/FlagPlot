//! Scenario 53: the commander states one end, "deny the waters at (0,5)",
//! at press. The staff at the area's level gives base 1 the tasks of
//! striking ships and of searching those waters; at the base's level it
//! sets the bombers' standing orders; and each strike is sized at the
//! group's level.
//!
//! Proves decisions 0123 and 0118 together against a hand calculation,
//! with scenario 49 for decision 0118 alone. The data is placeholder data:
//! see `data/scenarios/53_deny_an_area.toml`. It runs on the frozen step 1
//! figures with the table of aircraft for each ship added, as below. The
//! upkeep, wear, fatigue and supply figures are not loaded. There is no
//! enemy unit: every strike finds nothing, the search sights nothing, and
//! no die is rolled.
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
//! As scenarios 46 and 49: `strike_contact_max_age_pulses` 0; a contact
//! gains a pulse of age at the end of each pulse and is dropped at 5; a
//! hex is 40 nm; the placeholder dive bomber (type 3) has a strike radius
//! of 250 nm; strikes are decided in a daylight pulse in group number
//! order; a group takes the kind first on its target list, then the
//! nearest. The placeholder flying boat (type 1) searches to 600 nm. From
//! (0,0) the hex (0,5) lies on 150, as scenario 47 has it.
//!
//! Aircraft for each ship reported, in this scenario's own figures:
//! cruiser 12, destroyer 6, transport 6, auxiliary 4, and no figure for a
//! carrier or a battleship, which need everything.
//!
//! ## Inputs
//!
//! A placeholder map of open sea. Allied base 1 at (0,0). The game is 2
//! days. There is no Japanese unit and no Japanese base.
//!
//! | Group | Type        | Ready aircraft | At the start |
//! | ----- | ----------- | -------------- | ------------ |
//! | 1     | dive bomber | 10             | resting, no setting |
//! | 2     | dive bomber | 10             | resting, no setting |
//! | 3     | dive bomber | 10             | resting, no setting |
//! | 4     | flying boat | 6              | resting, no arc |
//! | 5     | fighter     | 8              | resting, no setting |
//!
//! The Allied side starts holding three contacts, numbered from the oldest
//! down:
//!
//! | Contact | Hex    | Ships | Kind      | Age at the start | From base 1 | Need |
//! | ------- | ------ | ----- | --------- | ---------------- | ----------- | ---- |
//! | 1       | (0,4)  | 1     | carrier   | 2                | 160 nm      | everything |
//! | 2       | (3,0)  | 2     | cruiser   | 1                | 120 nm      | 24 |
//! | 3       | (-3,0) | 3     | transport | 0                | 120 nm      | 18 |
//!
//! The Allied side's commands of day 1: intent 1, deny the waters within 0
//! hexes of (0,5); its risk, press; leave to prepare and leave to launch
//! both given. No forces limit.
//!
//! ## The tree on the first morning
//!
//! The waters are the one hex (0,5), 200 nm from base 1. The bombers reach
//! it, 200 against 250, and the flying boats, 200 against 600. Base 1 has
//! no cover. So the staff forms two tasks:
//!
//! | Intent | Kind | Parent | Maker | Addressee |
//! | ------ | ---- | ------ | ----- | --------- |
//! | 1      | deny the waters at (0,5) | none | the commander | the area |
//! | 2      | strike ships from base 1 | 1 | the staff | base 1 |
//! | 3      | search the waters from base 1 | 1 | the staff | base 1 |
//!
//! ## The orders of day 1
//!
//! At press, for each of groups 1, 2 and 3, four orders serving intent 2:
//! the naval strike mission; the oldest contact, 2 (the figure, 0, and two
//! more); extended reach; and strike sizing on. The target list is every
//! kind and the held share none, which is what the groups have: no order.
//! The reason of each sizing order says the group decided, from its task;
//! the others say the base decided.
//!
//! For group 4, two orders serving intent 3: the search mission, and the
//! arc. The one threat is (0,5), on 150: the arcs of ten degrees that hold
//! it are 140 to 150 and 150 to 160, and the lower start is taken. Widened
//! by 30 at each end it is 110 to 180, at 600 nm.
//!
//! For group 5, two orders serving intent 2: combat air patrol, and a
//! patrol share of 0.5.
//!
//! Sixteen orders on day 1 and none on day 2.
//!
//! ## The morning of day 1
//!
//! After the night the contacts are 3, 2 and 1 pulses old. The groups
//! launch on a contact no older than 2: contact 1, the carrier, at 3, does
//! not qualify. Contacts 2 and 3 do, and each group's order among them is
//! contact 2, the cruisers, first.
//!
//! | Group | Can send | Contact | Its need | Sent there before | It sends |
//! | ----- | -------- | ------- | -------- | ----------------- | -------- |
//! | 1     | 10       | 2       | 24       | 0                 | 10       |
//! | 2     | 10       | 2       | 24       | 10                | 10       |
//! | 3     | 10       | 2       | 24       | 20                | 4        |
//!
//! One raid of 24 flies to (3,0), finds nothing, and contact 2 is dropped,
//! by group 1's strike. Nothing is sent at contact 3 in the morning: each
//! group launches once in a pulse.
//!
//! ## The afternoon of day 1
//!
//! Contact 1 is 4 pulses old and contact 3 is 2. Only contact 3 qualifies.
//!
//! | Group | Can send | Contact | Its need | Sent there before | It sends |
//! | ----- | -------- | ------- | -------- | ----------------- | -------- |
//! | 1     | 10       | 3       | 18       | 0                 | 10       |
//! | 2     | 10       | 3       | 18       | 10                | 8        |
//! | 3     | 10       | none    |          |                   | 0        |
//!
//! One raid of 18 flies to (-3,0), finds nothing, and contact 3 is
//! dropped. At the end of the afternoon contact 1 reaches 5 pulses and is
//! dropped.
//!
//! ## Day 2
//!
//! The Allied side holds no contact. Nothing is launched. Group 4 searches
//! in both daylight pulses of both days, with all 6, and sights nothing.
//! Group 5 keeps 4 overhead in every daylight pulse.
//!
//! ## At the end
//!
//! Five strikes were launched: 10, 10 and 4, then 10 and 8: 42 aircraft.
//! Every group has the aircraft it began with. The measure of intent 1
//! reads: 2 days in force; five strikes launched; no contact held in the
//! waters at the end of either day, so it was met on both. The contacts
//! struck lay outside the waters: a base given the task strikes what its
//! groups can reach.
//!
//! # Properties
//!
//! - **Balanced.** With no risk set: each bomber group is given two
//!   orders, the mission and sizing on. Day 1 has six orders for the
//!   bombers, two for group 4 and two for group 5: ten. No strike is
//!   launched on either day: every contact is a pulse old or more in the
//!   morning, against the figure of 0.
//! - **Preserve the force.** Each bomber group is given four orders: the
//!   mission, sizing on, a held share of 0.333333 and the target list
//!   carrier, battleship, cruiser. No strike is launched.
//! - **The carrier needs everything.** At press, with contact 1 one pulse
//!   younger at the start, so that it qualifies in the morning: groups 1,
//!   2 and 3 each send their 10 at it, 30 in one raid, and nothing is sent
//!   at contact 2 in the morning.
//! - **A base with a cover has one arc.** With the commander also stating,
//!   first, intent 1, hold base 1, and the deny end as intent 2: the cover
//!   is intent 3 and has groups 4 and 5; the strike task is intent 4; no
//!   search task is formed; and group 4's arc, with (0,5) the only threat,
//!   is 110 to 180.
//! - **Out of reach.** With the waters at (0,7), 280 nm: no strike task is
//!   formed; the search task is, 280 against 600. With the waters 16 hexes
//!   away, 640 nm: neither is formed, and the staff hands in on day 1 that
//!   it cannot meet intent 1.
//! - **A commander's order overrides the size.** With the commander
//!   ordering sizing off on group 3 on day 1: the group is held, the staff
//!   gives it no order, and it flies nothing, having no mission.
//! - **Withdrawing the end** on day 2 withdraws both tasks that day.
//! - The staff orders no group a task may not use; the same start gives
//!   the same orders; two worlds that differ only in what the Allied side
//!   cannot see give the same intents, orders and reasons.
//! - Every staffed scenario built before this one gives the orders it
//!   gave.
