//! Scenario 133: a force followed. The commander orders an enemy force
//! destroyed, or shadowed, by naming a contact; the order follows the force
//! from report to report, says when it is lost, and ends when it is not
//! found (decision 0220 as amended on review, with the architect's rulings
//! in its section "The scenario's statements, as written": R1 to R12).
//!
//! # The architect's statements
//!
//! Written by the architect on Nov 23, 2026, before any code. Nobody else
//! changes a statement, a hex, a day or a number here, and never to make a
//! test pass; a mismatch goes back to the architect. Everything stated is
//! asserted exactly. The days and hexes are worked by hand below.
//!
//! It is several small games, one for each part, so that no part rests on
//! another's. Each is a file of `data/scenarios/a-force-followed/`. The data
//! is placeholder data.
//!
//! ## Figures
//!
//! The frozen step 1 figures, with the reports made exact through
//! `common::altered_reference`: `position_error_chance`,
//! `count_over_chance` and `count_under_chance` 0, and every row of
//! `kind_confusion` its own kind with a chance of 1. So a report names the
//! hex the force was in, the ships it has and its most important kind. No
//! upkeep, wear, fatigue or supply figures.
//!
//! - A hex is 40 nm. A day is the night pulse of 12 hours, then the morning
//!   and the afternoon of 6 each. The staff is asked at dusk, before the
//!   night: "dusk of day N" is the start of day N.
//! - Search runs before movement in a pulse and sees a force where the last
//!   pulse left it. A force that has steamed 40n nm is in the nth hex of its
//!   way (scenario 125).
//! - The placeholder flying boat searches to 600 nm with a sweep of 40 nm.
//!   Group 1 has 100: the chance to sight a force in its arc is 57.295779 x
//!   100 x 40 = 229,183.116 over the distance times the arc's width, which
//!   is at most 600 x 360 = 216,000: so 1 at every distance and width.
//! - The placeholder dive bomber's strike radius is 250 nm, and 312 nm on
//!   extended reach (250 x 1.25 = 312.5, cut). `strike_contact_max_age_pulses`
//!   is 0. The frozen figures give no aircraft for each ship, so a strike
//!   sends everything.
//! - A report refreshes a contact within 2 hexes that is no more than 2
//!   pulses old (the frozen figures have no figure for the same pulse, so
//!   within a pulse too); otherwise it makes a new one. A contact gains a
//!   pulse of age at the end of each pulse it was not reported in and is
//!   dropped at 5.
//! - The placeholder cruiser makes 32 knots at most, the carrier 16 at
//!   cruise and 30 at most, the destroyer 36 at most.
//! - The staff's figures, in the code: `followed_force_knots` 20,
//!   `lost_contact_days` 2, `station_rechosen_hexes` 3. At 20 knots a force
//!   makes one hex in 2 hours.
//!
//! ## The rule in the terms the statements use
//!
//! As the decision record has it, with the architect's rulings R1 to R12
//! (the record's section "The scenario's statements, as written").
//!
//! - **The age of a contact at dusk says when it was last reported.** One
//!   reported in the day just played is 0 (the afternoon), 1 (the morning)
//!   or 2 (the night) pulses old; one last reported the day before is 3 or
//!   4. **Fresh** is no more than `fresh_contact_max_age_pulses`, 2: last
//!   reported in the day just played (R1). The pulse of a contact's last
//!   report is the one its age gives: age + 1 pulses before the dusk. A
//!   contact the side starts with at age 0 is as one reported in the
//!   afternoon of day 0 (R2). Where the followed contact is held its last
//!   report is read from the picture; where it is not, from what the order
//!   kept.
//! - **What the order keeps** (rule 5): the number of the contact it
//!   follows, the hex and the day and pulse of that contact's last report,
//!   and whether it was taken for a carrier when the order was given or last
//!   took a contact. The hex and the report are kept anew at each dusk at
//!   which the followed contact is held and its last report is not the one
//!   kept; the staff hands in the notice **follows** for it, and the engine
//!   writes what is kept when it records the notice (R3). A notice
//!   **follows** says whether the number changed (a taking) and whether the
//!   contact taken is of the other sort.
//! - **Rule 7, the distance** (R4): a contact qualifies where its hex is
//!   within the hexes a force at 20 knots steams in the hours from the start
//!   of the pulse of the followed contact's last report to the end of the
//!   pulse of its own last report, counted from the kept hex. Equal is
//!   within.
//! - **At a dusk the order is worked in this order** (R5): follow a fresh
//!   contact; else take one; else, on the dusk that begins the day
//!   `lost_contact_days` + 1 after the day of the last report kept, end;
//!   else be lost. So a report of day D with nothing after it gives: lost at
//!   dusk of day D + 2, ended at dusk of day D + 3.
//! - **Lost** (rule 9): the staff withdraws the strike tasks and the carrier
//!   tasks it formed under the order and keeps the search tasks (R6). **A
//!   base's task** is withdrawn at the second dusk running at which nothing
//!   it may use there touches the waters, a bomber's reach for a strike
//!   task and a searcher's range for a search task: the waters as they are
//!   at this dusk, and the waters round the hex the order kept before this
//!   dusk (R7). **Ended:** the engine withdraws the order and all beneath
//!   it when it records the notice (R8). **Search only** reads its leave to
//!   pursue as stay (R9). **A station is chosen again** at a dusk at which
//!   it lies more than `station_rechosen_hexes` hexes beyond the farthest a
//!   station may be from the centre (R10).
//!
//! ## Game A: followed, lost and not found
//!
//! `133_a_followed.toml`, 7 days. Allied base 1 at (0,0) with group 1, 100
//! flying boats, and group 2, 10 dive bombers, both resting. Japanese task
//! force 1, two cruisers, at (8,0). The Allied side starts holding contact
//! 1: 2 ships, cruisers, at (8,0), age 0. The Allied commander's commands of
//! day 1: intent 1, destroy the force of contact 1, within 2 hexes; leave to
//! prepare and leave to launch given. No risk is set: balanced.
//!
//! The Japanese force: on day 2, 20 knots and a route to (14,0); on day 4 a
//! route to (8,0); on day 5, full speed and a route to (20,0).
//!
//! **Where the force is, and what is seen.** Every hex is on row 0, due
//! east of base 1 on bearing 090. The waters of the order are always
//! centred on that row and lie wholly east of the base, so the arc the
//! search task gives, which holds every hex of the waters, holds bearing
//! 090: a force on the row within 600 nm is in the arc. The flying boats
//! search once a day, in the morning (a base's search task at balanced).
//!
//! | Morning of day | The force is at | From base 1 | Seen | The report |
//! | --- | --- | --- | --- | --- |
//! | 1 | (8,0) | 320 nm | yes | refreshes contact 1 (0 hexes, 1 pulse old) |
//! | 2 | (14,0): 240 nm in the 12 hours of the night | 560 nm | yes | 6 hexes from contact 1: makes contact 2 |
//! | 3 | (14,0) | 560 nm | yes | refreshes contact 2 (0 hexes, 2 pulses old) |
//! | 4 | (8,0): 240 nm in the night | 320 nm | yes | 6 hexes from contact 2: makes contact 3 |
//! | 5 | (17,0): 32 x 12 = 384 nm, 9 hexes | 680 nm | no: beyond 600 nm | none |
//! | 6, 7 | (20,0), reached at hour 15 of day 5 | 800 nm | no | none |
//!
//! The dive bombers launch at nothing on any day: a contact is 0 pulses old
//! only in the morning it is reported, and then it is 320 nm or 560 nm from
//! base 1 against a reach of 250. No strike, no fight, no draw but the
//! sightings, each at a chance of 1.
//!
//! **The contacts at each dusk** (age in pulses; "-" not held):
//!
//! | Dusk of day | Contact 1 | Contact 2 | Contact 3 |
//! | --- | --- | --- | --- |
//! | 1 | (8,0), 0 | - | - |
//! | 2 | (8,0), 1 | - | - |
//! | 3 | (8,0), 4 | (14,0), 1 | - |
//! | 4 | dropped in the night of day 3, at 5 | (14,0), 1 | - |
//! | 5 | - | (14,0), 4 | (8,0), 1 |
//! | 6 | - | dropped in the night of day 5 | (8,0), 4 |
//! | 7 | - | - | dropped in the night of day 6 |
//!
//! **The waters.** Within 2 hexes of (8,0), 19 hexes, in hex order: (6,0),
//! (6,1), (6,2), (7,-1), (7,0), (7,1), (7,2), (8,-2), (8,-1), (8,0), (8,1),
//! (8,2), (9,-2), (9,-1), (9,0), (9,1), (10,-2), (10,-1), (10,0). The
//! nearest to base 1 is (6,0), 240 nm: a dive bomber's 250 nm touches the
//! waters by that one hex (the next nearest, (6,1) and (7,-1), are 6.557
//! spacings, 262 nm). Within 2 hexes of (14,0) they are the same 19 with 6
//! added to each q; the nearest is (12,0), 480 nm: no bomber touches them,
//! and the flying boats' 600 nm does.
//!
//! **Rule 7 at dusk of day 3.** Contact 1 is 4 pulses old: not fresh.
//! Contact 2 was last reported in the morning of day 2, later than contact
//! 1's last report, the morning of day 1. From the start of the morning of
//! day 1 to the end of the morning of day 2 is 6 + 6 + 12 + 6 = 30 hours: 15
//! hexes at 20 knots. Contact 2 is 6 hexes from (8,0). It is taken. At dusk
//! of day 5 the same sum from the morning of day 3 to the end of the
//! morning of day 4 gives 15 hexes against 6: contact 3 is taken.
//!
//! Statements:
//!
//! 1. **Given.** On day 1 the order is taken in. It keeps contact 1, (8,0),
//!    the afternoon of day 0, not a carrier. Its leave to pursue reads go,
//!    read from the order's kind, with none set. No notice of this rule is
//!    handed in on day 1.
//! 2. **The tree on day 1.** The staff forms intent 2, strike ships from
//!    base 1, and intent 3, search the waters from base 1, both under intent
//!    1, as a deny of those waters forms them. It forms no stand task and no
//!    carrier task.
//! 3. **The orders of day 1.** Group 1: the search mission, search once a
//!    day, and an arc at 600 nm that holds every one of the 19 hexes by the
//!    engine's own test. Group 2: what a deny's strike task gives a bomber
//!    at balanced (the naval strike mission and strike sizing on, the rest
//!    being what the group has), and between them one order more: its waters
//!    to strike first, the 19 hexes. The reason of that order names intent 1
//!    as the order to destroy a force and says a contact in them is struck
//!    before every one outside.
//! 4. **Dusk of day 2: refreshed.** Contact 1 keeps its number. One notice,
//!    follows: contact 1, the number unchanged, (8,0), the morning of day 1.
//!    The order keeps that report. No task is formed or withdrawn and no
//!    order is given to a group.
//! 5. **Dusk of day 3: the force seen again as a new contact.** The side
//!    holds contact 1, 4 pulses old, and contact 2. One notice, follows:
//!    contact 2 is taken to be the force of contact 1, the number changed,
//!    the same sort, at (14,0), 2 ships, cruisers, the morning of day 2. The
//!    order keeps contact 2, (14,0), the morning of day 2, not a carrier.
//!    Its waters are the 19 hexes round (14,0). Group 2 is ordered those
//!    waters to strike first and group 1 an arc that holds every one of
//!    them. Intents 2 and 3 stand: it is the first dusk at which no bomber
//!    of base 1 touches the waters.
//! 6. **Dusk of day 4: the base out of reach.** One notice, follows: contact
//!    2, the number unchanged, the morning of day 3. The staff withdraws
//!    intent 2, the second dusk running at which no bomber touches the
//!    waters; its reason gives 480 nm to the nearest hex of the waters
//!    against the reach of 250 nm. Intent 3 stands. Group 2 is given no
//!    order and keeps what it has.
//! 7. **Dusk of day 5: back in reach.** One notice, follows: contact 3 is
//!    taken to be the force of contact 2, at (8,0), the morning of day 4.
//!    The staff forms intent 4, strike ships from base 1, under intent 1.
//!    Group 2 is ordered the 19 hexes round (8,0) to strike first, and
//!    group 1 an arc that holds them.
//! 8. **Dusk of day 6: lost.** Contact 3 is 4 pulses old and the side holds
//!    no other. One notice, contact lost, naming contact 3 and (8,0), where
//!    it was last reported. The order's centre stays (8,0). The staff
//!    withdraws intent 4; intent 3 stands; group 1 is given no order, its
//!    arc being what it has.
//! 9. **Dusk of day 7: ended.** The last report kept is of day 4, and 4 + 2
//!    + 1 is 7. One notice, ended, the force not found. Intents 1 and 3 are
//!    withdrawn, with the notice's event as the cause. On day 7 the staff
//!    hands in nothing for the order. The side no longer holds contact 3,
//!    and the notice says what the order kept of its last report: 2 ships,
//!    cruisers, at (8,0), the morning of day 4 (added on Nov 24, 2026:
//!    ruling 6 after the build).
//! 10. **What is handed in, in all.** The notices of this rule in the game
//!     are, by the day of the dusk: follows on days 2, 3, 4 and 5, lost on
//!     day 6, ended on day 7, and no other. The lost notice is handed in
//!     once. No notice of a pursuit and none that the order cannot be met is
//!     handed in on any day.
//! 11. **The paper.** `command_state` shows, for intent 1 at each dusk, the
//!     contact it follows by the report (where, how many, taken for what,
//!     how old), each taking in the order it happened (contact 2 for 1,
//!     contact 3 for 2), and whether it is lost.
//! 12. **Nothing is struck.** No `StrikeLaunched` is recorded on any day.
//!
//! ## Game S: search only
//!
//! Game A with `search_only = true` on the order. A search-only order's
//! searches fly in both daylight pulses at any risk, so each sighting of the
//! table above is repeated in the afternoon of its day and refreshes the
//! contact the morning made. At dusk a contact reported that afternoon is 0
//! pulses old and one last reported the afternoon before is 3.
//!
//! 13. The staff forms intent 2, search the waters from base 1, and no
//!     other task on any day: no strike task, though a bomber's reach
//!     touches the waters on day 1. Group 2 is given no order on any day.
//!     No group is ordered waters to strike first. The order's leave to
//!     pursue reads stay.
//! 14. Group 1 is ordered the search mission and an arc, and is not set to
//!     search once a day: a `SearchFlown` of group 1 is recorded in the
//!     morning and in the afternoon of every day from 1 to 6.
//! 15. The notices fall on the days of game A: follows on days 2, 3, 4 and
//!     5, lost on day 6, ended on day 7. At dusk of day 3 contact 1 is 3
//!     pulses old, not fresh, and contact 2 is 0: from the start of the
//!     afternoon of day 1 to the end of the afternoon of day 2 is 6 + 12 + 6
//!     + 6 = 30 hours, 15 hexes against 6. The order keeps contact 2,
//!     (14,0), the afternoon of day 2.
//!
//! ## Game B: which contact is taken
//!
//! `133_b_which_contact.toml`, 3 days. Base 1 with group 1 and no bomber.
//! The Allied side starts holding contact 1: 2 ships, cruisers, at (2,0),
//! age 0, where no ship is. Japanese task force 1, one carrier with no
//! aircraft, at (6,0); Japanese task force 2, two cruisers, at (10,0);
//! neither moves. Intent 1: destroy the force of contact 1, within 1 hex;
//! both leaves; balanced.
//!
//! The waters are the 7 hexes within 1 of (2,0); (2,-1) lies on bearing 060
//! from base 1 and (1,1) on 120, so the arc holds 060 to 120 before it is
//! widened by 30 at each end, and every hex below is on a bearing between
//! 073 and 107. In the morning of day 1 the forces are reported in task
//! force order: contact 2, 1 ship, a carrier, at (6,0), 4 hexes from (2,0);
//! contact 3, 2 ships, cruisers, at (10,0), 8 hexes from (2,0) and 4 from
//! contact 2. Contact 1 is not reported again.
//!
//! At dusk of day 2 contact 1 is 3 pulses old: not fresh. Its last report
//! is the afternoon of day 0. From the start of that pulse to the end of
//! the morning of day 1 is 6 + 12 + 6 = 24 hours: 12 hexes.
//!
//! 16. **The sort decides.** Contacts 2 and 3 both qualify. The followed
//!     force was not taken for a carrier, so contact 3, the cruisers, is
//!     taken before contact 2, the carrier, though it is the farther. One
//!     notice, follows: contact 3 for contact 1, the same sort.
//! 17. **The other sort** (game B with task force 2 taken out). Contact 2
//!     alone qualifies and is taken. The notice says it is of the other
//!     sort: a carrier for a force that was not. The order then keeps
//!     "taken for a carrier".
//! 18. **The nearest** (game B with task force 1 made two cruisers).
//!     Contacts 2 and 3 are of one sort; contact 2, 4 hexes off, is taken.
//! 19. **A tie** (game B with task force 1 made two cruisers at (5,2) and
//!     task force 2 at (7,-2)). Each is 5 hexes from (2,0) and they are 4
//!     apart, on bearings 106 and 074. Contact 2, at (5,2), the lower
//!     number, is taken.
//! 20. **At the distance** (game B with task force 1 taken out and task
//!     force 2 at (14,0)). Contact 2 is 12 hexes from (2,0), which is the
//!     distance: it is taken.
//! 21. **Too far, then seen again** (as 20, with task force 2 at (15,0),
//!     600 nm from base 1, the range). At dusk of day 2 contact 2, 13 hexes
//!     off, does not qualify: the order is lost, with a contact one pulse
//!     old on the chart, and its centre stays (2,0). In the morning of day
//!     2 the search of the lost waters refreshes contact 2. At dusk of day
//!     3, the dusk the order would end at (0 + 2 + 1), rule 7 is worked
//!     first: from the start of the afternoon of day 0 to the end of the
//!     morning of day 2 is 48 hours, 24 hexes against 13. Contact 2 is
//!     taken and the order does not end.
//! 22. **Two orders and one new contact** (game B with task force 1 taken
//!     out and task force 2 at (12,0); a second contact at the start,
//!     contact 2, 2 ships, cruisers, at (7,0), age 0, where no ship is; and
//!     intent 2, destroy the force of contact 2, within 1 hex; the contacts
//!     are numbered in the order the file gives them). In the
//!     morning of day 1 the cruisers are reported 5 hexes from contact 2
//!     and 10 from contact 1: contact 3. At dusk of day 2 it qualifies for
//!     both orders (12 hexes). Intent 1, the lower number, takes it. No
//!     other contact qualifies for intent 2, which is lost. With a priority
//!     limit of rank 1 set on intent 2 it is the other way: intent 2 takes
//!     contact 3 and intent 1 is lost.
//! 23. **Given inside a day** (game B with the order given at the stop at
//!     0600 of day 1, not at dusk). It is taken in at dusk of day 2
//!     (decision 0219) and keeps then contact 1, (2,0), the afternoon of day
//!     0, read from the contact as it is held at that dusk (restated on Nov
//!     24, 2026: the architect's ruling 3 after the build). No
//!     search flew on day 1, so the side holds contact 1 alone, 3 pulses
//!     old: the order is lost at once, and the staff forms the search task.
//!     The forces are reported in the morning of day 2. At dusk of day 3
//!     both qualify (48 hours, 24 hexes) and contact 3, the cruisers, is
//!     taken: lost, then found again under a new number.
//!
//! ## Game D: dropped by a strike that found nothing
//!
//! `133_d_a_strike_that_found_nothing.toml`, 2 days. Base 1 with groups 1
//! and 2. Contact 1 at the start: 2 ships, cruisers, at (5,0), age 0, where
//! no ship is. Japanese task force 1, two cruisers, at (13,0), 520 nm from
//! base 1; it does not move. Intent 1: destroy the force of contact 1,
//! within 2 hexes, at press; both leaves.
//!
//! At press a bomber launches on a contact up to 2 pulses old, on extended
//! reach, 312 nm, and the flying boats search in both daylight pulses. In
//! the morning of day 1 the cruisers are reported, contact 2, 8 hexes from
//! (5,0) and 520 nm from base 1, out of reach. Contact 1 is 1 pulse old,
//! 200 nm away and in the waters to strike first: group 2 sends its 10
//! there, finds nothing, and contact 1 is dropped. In the afternoon contact
//! 2 is reported again.
//!
//! 24. In the morning of day 1 group 2 launches 10 at (5,0),
//!     `StrikeFoundNothing` and `ContactNotThere` are recorded, and the side
//!     no longer holds contact 1. No other strike is launched in the game.
//! 25. At dusk of day 2 the order follows a contact the side does not hold.
//!     Rule 7 is worked from what the order kept: (5,0), the afternoon of
//!     day 0. Contact 2's last report is the afternoon of day 1: 6 + 12 + 6
//!     + 6 = 30 hours, 15 hexes against 8. Contact 2 is taken; the order is
//!     not lost.
//! 26. **With no Japanese force** (game D with task force 1 taken out): at
//!     dusk of day 2 nothing qualifies and the order is lost, its centre
//!     (5,0). The staff withdraws the strike task and keeps the search task.
//!     The side no longer holds contact 1, and the notice says what the
//!     order kept of its last report: 2 ships, cruisers, at (5,0), the
//!     afternoon of day 0 (added on Nov 24, 2026: ruling 6 after the build).
//!
//! ## Game E: a kind that changes
//!
//! `133_e_a_kind_that_changes.toml`, 3 days. Base 1 with group 1. Japanese
//! task force 1, two cruisers, at (8,0), which does not move; task force 2,
//! two destroyers, at (16,0), 640 nm from base 1, which on day 2 steams to
//! (8,0) at 30 knots: 320 nm in 10 hours and 40 minutes, there before the
//! morning. Contact 1 at the start: 2 ships, cruisers, at (8,0). Intent 1:
//! destroy the force of contact 1, within 2 hexes; both leaves.
//!
//! In the morning of day 2 both forces are in (8,0) and are reported in
//! task force order. The cruisers' report refreshes contact 1; the
//! destroyers' then refreshes it again and replaces what it held.
//!
//! 27. At dusk of day 3 contact 1 is 1 pulse old, 2 ships, destroyers. The
//!     order follows it: one notice, follows, the number unchanged, the
//!     morning of day 2. No notice says the kind changed. The paper says
//!     the force is taken for destroyers. What the order keeps of the sort
//!     is unchanged: not a carrier. Of the report it keeps 2 ships,
//!     destroyers (added on Nov 24, 2026: ruling 6 after the build).
//! 28. A call of `run` for 3 days with all three stops of this rule asked
//!     for runs to the end without stopping.
//!
//! ## Game G: a free carrier force
//!
//! `133_g_a_carrier_force.toml`, 2 days. Base 1 with group 1. Allied task
//! force 1, one carrier, stopped at (3,0), with no home port, group 2 (12
//! fighters) and group 3 (10 dive bombers) aboard and no scout. Contact 1
//! at the start: 2 ships, cruisers, at (9,0), where no ship is. Japanese
//! task force 2, two cruisers, at (13,0); it does not move. Intent 1:
//! destroy the force of contact 1, within 2 hexes; both leaves; balanced.
//! No enemy base is on the side's map and no contact is a carrier, so there
//! is no circle of danger. With no home port the fuel is not weighed.
//!
//! **Day 1.** The stations a carrier force may take for a deny of 2 hexes
//! are within 250 - 2 x 40 = 170 nm of the centre: every hex within 4 hexes
//! of it (160 nm along a row and less off it) and none at 5 (the nearest of
//! those are 4.359 spacings, 174 nm). The one nearest the force by sea is
//! (5,0), 2 hexes from (3,0) and 4 from (9,0). The force is 240 nm from
//! contact 1, within its bombers' 250: it does not close, and is routed to
//! (5,0), 80 nm, 5 hours at cruise, so at cruise. In the morning the
//! cruisers are reported at (13,0): contact 2, 4 hexes from (9,0), 520 nm
//! from base 1 and 320 nm from the carrier at (5,0). Nothing is launched.
//!
//! **Dusk of day 2.** Contact 1 is 3 pulses old. Contact 2: 24 hours, 12
//! hexes against 4; taken. The centre is (13,0). A station may be up to 4
//! hexes from the centre (170 nm, cut to whole hexes), and the station,
//! (5,0), is 8 from it: more than 4 + 3, so the station is chosen again
//! (R10). The nearest hex to the force within 4 of (13,0) is (9,0), 4 hexes
//! from (5,0). Contact 2 is 1 pulse old, in the waters the order
//! puts first, and 320 nm from the force: beyond 250 and within a night at
//! full speed and the reach, 360 + 250 = 610. At balanced a carrier force
//! closes on a contact in those waters whatever its kind. The first hex
//! along the row within 250 nm of (13,0) is (7,0), 240 nm from it and 80
//! nm from the force.
//!
//! 29. **The station.** On day 1 the carrier pass forms intent 2, operate
//!     task force 1 from (5,0), under intent 1; then the deny's doctrine
//!     forms intent 3, search the waters from base 1. No strike task is
//!     formed (base 1 has no bomber) and no stand task. The force is routed
//!     to (5,0) at cruise. Group 3 is set to the naval strike mission.
//! 30. **Chosen again.** At dusk of day 2 the notice follows: contact 2 for
//!     contact 1. The staff withdraws intent 2 and forms intent 4, operate
//!     task force 1 from (9,0), under intent 1, in the same hand-in.
//!     Intent 3 stands. Its reason gives the station's 8 hexes from the
//!     centre against the 4 a station may be and the 3 allowed beyond.
//! 31. **It closes on cruisers at balanced.** In that hand-in the force is
//!     given a route to (7,0), not to (9,0), at cruise. The reason says it
//!     closes on contact 2, taken for cruisers, because it lies in the
//!     waters the order to destroy puts first.
//! 32. **It strikes.** In the morning of day 2 the cruisers are reported
//!     again at (13,0), 240 nm from the force at (7,0), and group 3
//!     launches its 10 at (13,0). Nothing of what the strike did is stated.
//! 33. **A surface force named** (game G with Allied task force 3, two
//!     cruisers, stopped at (1,0), and a forces limit on intent 1 that
//!     names task force 3). No stand task is formed for it on any day and
//!     it is given no order; the reason says a surface force is not sent
//!     after a force on the move. Task force 1 is outside the limit and is
//!     given no task.
//! 34. **Not rechosen within the figure** (game G with task force 2 at
//!     (12,0)). In the morning of day 1 the cruisers are reported 3 hexes
//!     from (9,0), a new contact, 280 nm from the carrier. At dusk of day 2
//!     it is taken and the centre is (12,0), 7 hexes from the station
//!     (5,0): not more than 4 + 3, so intent 2 stands with its station.
//!     Contact 2 is 280 nm from the force: it closes, to (6,0), 240 nm
//!     from (12,0).
//!
//! ## Game P: the order's leave to pursue
//!
//! `133_p_leave_to_pursue.toml`, 1 day, run with `pursuit_nights` 2. Base 1
//! with group 1. Allied task force 1 as in game G, at (3,0). Contact 1 at
//! the start: 2 ships, cruisers, at (20,0), 17 hexes and 680 nm from the
//! force. No Japanese unit. Intent 1: destroy the force of contact 1,
//! within 2 hexes; both leaves; balanced; no leave to pursue set.
//!
//! The station is (16,0), the nearest hex to the force within 4 of (20,0).
//! Base 1's flying boats do not touch the waters (the nearest hex, (18,0),
//! is 720 nm). The contact is in the waters the order puts first, fresh,
//! and 680 nm off: more than 610, so too far for the closing rule, and
//! within two nights and the reach, 720 + 250 = 970, so a find. The first
//! hex along the row within 250 nm of (20,0) is (14,0), 440 nm from the
//! force: 27 and a half hours at cruise, so full speed, and 440 / 30 hours.
//!
//! 35. **Go, by the order.** The staff forms intent 2, operate task force 1
//!     from (16,0), and no task at base 1; it hands in no notice that the
//!     order cannot be met. The force is given full speed and a route to
//!     (14,0). One notice, pursues: task force 1, contact 1, cruisers, age
//!     0, reported at (20,0), to (14,0), the hours 440 divided by 30 in the
//!     game's own arithmetic. Its reason says the leave is the order's own:
//!     an order to destroy a force gives leave to pursue.
//! 36. **Ask** (with `set_pursuit` ask on intent 1): the notice is
//!     `asks_leave_to_pursue` and the force is routed to its station,
//!     (16,0), not to (14,0).
//! 37. **Stay** (with `set_pursuit` stay): no notice of a pursuit; the
//!     force is routed to (16,0).
//! 38. **An area limit binds** (with an area limit on intent 1 of the hexes
//!     within 10 of (3,0), which does not hold (20,0)): no find. No notice
//!     of a pursuit and no route to (14,0).
//! 39. **Search only** (with `search_only = true`): no carrier task is
//!     formed, the force is given no order, and the staff hands in that the
//!     order cannot be met, as a deny nothing can serve does.
//!
//! ## Command levels on
//!
//! Game S with `command_levels = true` and the report figures
//! `reports_placeholder_scenario_133.toml`: a report sent up always arrives
//! a pulse late; none arrives sideways or down. The Allied commands are the
//! area command and base 1; each starts with contact 1. Base 1's flying
//! boats report to base 1's picture; the order is the area's and reads the
//! area's picture.
//!
//! The area hears each morning's report in the afternoon and each
//! afternoon's in the night that follows, after the staff was asked. A
//! report heard late makes or refreshes a contact at the age it has.
//!
//! 40. At dusk of day 3 the area's picture holds contact 1, 3 pulses old
//!     (last refreshed in the night of day 2 by the report of the afternoon
//!     of day 1, at 1 pulse old), and contact 2 at (14,0), 1 pulse old, of
//!     the morning of day 2; the afternoon's report has not arrived. The
//!     order takes contact 2 and keeps (14,0), the morning of day 2, where
//!     game S keeps the afternoon of day 2. The distance: from the start of
//!     the afternoon of day 1 to the end of the morning of day 2, 24 hours,
//!     12 hexes against 6.
//! 41. The notices fall on the days of game S.
//! 42. **A seat below the area cannot give the order** (restated on Nov
//!     24, 2026: ruling 1 after the build). A seat at base 1 that calls
//!     `set_intent` with the kind destroy a force is answered as it is for
//!     every kind: stating an order from a seat that is not the area
//!     command's is not built. Nothing is queued and no order is made.
//!
//! # Properties
//!
//! 43. **Refused.** `set_intent` with the kind destroy a force and a number
//!     the commander's picture does not hold is refused with the reason
//!     that the contact is not held, at dusk and inside a day; so is one
//!     with no contact. The Japanese seat cannot name an Allied contact.
//! 44. **A contact held and not fresh may be named.** In game A the
//!     commander withdraws intent 1 at dusk of day 3 and at the same dusk
//!     gives a new order naming contact 1, then 4 pulses old. It is
//!     accepted and keeps contact 1, (8,0), the morning of day 1 (age 4: five
//!     pulses before the dusk). At that dusk it takes contact 2 by rule 7:
//!     30 hours, 15 hexes against 6.
//! 45. **The stops are keys of the calls of time,** off unless asked for.
//!     `run` asked for the stop "lost" in game A stops at dusk of day 7,
//!     the dusk after the day the notice was handed in, and names the
//!     notice; `advance` asked for it stops at 0600 of day 6. Asked for the
//!     stop "ended" they stop at the end of the game and at 0600 of day 7.
//!     `run` asked for the stop "taken, of the other sort" stops after day
//!     2 in the game of statement 17 and does not stop in game A. None of
//!     them stops before a day is played, and none fires an interrupt.
//! 46. **Withdrawn.** In game A the commander withdraws intent 1 at dusk of
//!     day 4: everything beneath it is withdrawn with it, nothing of the
//!     order is kept in the command state, and no notice of this rule is
//!     handed in after.
//! 47. **A side sees its own.** Each notice of this rule is a `StaffNotice`
//!     the Allied side sees whole and the Japanese side does not see. No
//!     event the Japanese side is given, and no answer to its seat, differs
//!     between game A and game A with no order given.
//! 48. **Nothing it cannot see.** Two worlds of game A that differ only in
//!     what the Allied side cannot see (the Japanese force's fuel; a
//!     Japanese force at (20,5), never reported) give the same orders,
//!     commands, notices and kept state on every day.
//! 49. **A dry run.** A trial of the Allied side made at dusk of day 4 of
//!     game A, of one world and one day: in the made world the order is in
//!     force and follows the contact at (14,0), found by its place in the
//!     order of the picture's contacts, whatever number the made world
//!     gives it; what it keeps is what the live order keeps; and the staff
//!     of the made world hands in, on its first dusk, the commands and the
//!     orders the live staff hands in at dusk of day 4.
//! 50. **Hashed.** What the order keeps is in the state hash: two worlds
//!     of game A that differ only in the kept pulse of the last report
//!     hash differently, and each kept field is named in
//!     `hash::name_every_field`. A game saved at dusk of day 5 and replayed
//!     gives the hash of the game played through.
//! 51. **The same start gives the same game.** Game A played twice gives
//!     the same hash on every day.
//! 52. **No game on file moves.** No scenario on file gives the order: the
//!     comparison of every run, live and frozen, with the hashes cut, shows
//!     no difference from the commit this is built on.
//! 53. **The frozen figures are untouched.** No file of
//!     `data/reference/step1` changes and no file of this scenario is
//!     copied to `data/scenarios/step2-frozen`; the three figures of the
//!     staff are constants of the doctrine.
//!     (Read, by the architect's ruling of Nov 24, 2026: the build of this
//!     decision changes none, shown once with its pull request; the test
//!     holds what stays true, that no frozen file names the order or its
//!     figures and none is a copy of this scenario. A frozen file a later
//!     decision adds is no breach of it.)
//!
//! If a game cannot be set up as written (a carrier force with no home
//! port, a seat at a base giving an order to the area, an order given at
//! the stop at 0600, a trial made in the middle of a game), the build says
//! so and it comes back to the architect; no statement is changed to fit.

#![allow(clippy::doc_lazy_continuation)]

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::bearing::in_arc;
use sim_core::command::{
    CommandKind, CommandState, FollowNotice, Followed, IntentKind, NoticeKind, Reason,
    StaffCommand, StaffDecisions, StaffNotice, StaffOrder,
};
use sim_core::events::{Cause, Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::TaskForceId;
use sim_core::orders::{AirGroupOrder, AirMission, OrderKind, SpeedSetting, TaskForceOrder};
use sim_core::refdata::ShipKind;
use sim_core::state::Side;
use sim_core::time::Pulse;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

use crate::common;
use crate::common::replace_once as changed;
use crate::common::values::{command_table, hex, intent, n};

fn path_of(file: &str) -> PathBuf {
    common::data_dir()
        .join("scenarios/a-force-followed")
        .join(file)
}

fn text_of(file: &str) -> String {
    std::fs::read_to_string(path_of(file)).expect("the scenario file reads")
}

/// The frozen figures with the reports made exact: no error of position, no
/// count wrong, every kind reported as itself (the doc comment's "Figures").
/// With `nights` the figure `pursuit_nights` is taken to that many (game P).
fn figures_with(nights: Option<u32>) -> sim_data::LoadOptions {
    common::altered_reference("s133", |dir| {
        common::edit_file(dir, "search.toml", |text| {
            let text = common::replace_once(
                &text,
                "position_error_chance = \"0.3\"",
                "position_error_chance = \"0\"",
            );
            let text = common::replace_once(
                &text,
                "count_over_chance = \"0.2\"",
                "count_over_chance = \"0\"",
            );
            let text = common::replace_once(
                &text,
                "count_under_chance = \"0.2\"",
                "count_under_chance = \"0\"",
            );
            let (head, rows) = text
                .split_once("[kind_confusion.")
                .expect("the table of kinds is in the file");
            let mut out = head.to_string();
            for block in format!("[kind_confusion.{rows}").split("[kind_confusion.") {
                let Some((kind, _)) = block.split_once(']') else {
                    continue;
                };
                out.push_str(&format!("[kind_confusion.{kind}]\n{kind} = \"1\"\n\n"));
            }
            out
        });
        if let Some(nights) = nights {
            common::edit_file(dir, "air_combat.toml", |text| {
                common::replace_once(
                    &text,
                    "pursuit_nights = 1",
                    &format!("pursuit_nights = {nights}"),
                )
            });
        }
    })
}

fn figures() -> sim_data::LoadOptions {
    figures_with(None)
}

/// A game played to its end with the doctrine's staffs: what the Allied staff
/// handed in each day, the truth and the Allied command state at the end of
/// each day (index 0 is the start), and the log.
struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    states: Vec<CommandState>,
    /// The contacts the Allied side holds at the end of each day (index 0 is
    /// the start): number, hex, kind, ships and age in pulses.
    held: Vec<Vec<HeldRow>>,
    events: Vec<Event>,
}

/// What an order keeps, as a row: contact, hex, day, pulse and lost.
type KeptRow = (u32, Hex, u32, Pulse, bool);

/// A contact the Allied side holds: number, hex, kind, ships and age.
type HeldRow = (u32, Hex, ShipKind, u32, u32);

fn held_by(world: &sim_core::world::World) -> Vec<HeldRow> {
    let view = world.side_view(Side::Allied);
    // With command levels on the order reads the area command's picture.
    if world.command_levels() {
        let area = view
            .command(sim_core::pictures::CommandRef::Area)
            .expect("the area command has a picture");
        return area
            .contacts()
            .map(|c| {
                (
                    area.contact_number(c.id),
                    c.estimated_hex,
                    c.estimated_kind,
                    c.estimated_ship_count,
                    c.age_pulses,
                )
            })
            .collect();
    }
    view.contacts()
        .map(|c| {
            (
                view.contact_number(c.id),
                c.estimated_hex,
                c.estimated_kind,
                c.estimated_ship_count,
                c.age_pulses,
            )
        })
        .collect()
}

fn play_file(file: &str, edit: impl FnOnce(String) -> String) -> Played {
    play_with(file, edit, &figures())
}

fn play_with(
    file: &str,
    edit: impl FnOnce(String) -> String,
    figures: &sim_data::LoadOptions,
) -> Played {
    let text = edit(text_of(file));
    let mut scenario: Scenario =
        common::from_text_with(&text, &path_of(file), figures).expect("the scenario loads");
    let held = staffs();
    let mut handed = BTreeMap::new();
    let mut states = vec![scenario.world.command_state(Side::Allied).clone()];
    let mut pictures = vec![held_by(&scenario.world)];
    loop {
        let day = scenario.world.day() + 1;
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
        handed.insert(
            day,
            scenario
                .world
                .staff_handed_in()
                .get(&Side::Allied)
                .cloned()
                .unwrap_or_default(),
        );
        states.push(scenario.world.command_state(Side::Allied).clone());
        pictures.push(held_by(&scenario.world));
    }
    Played {
        handed,
        states,
        held: pictures,
        events: scenario.world.events().iter().cloned().collect(),
    }
}

fn same(text: String) -> String {
    text
}

/// The command that gives the order of game A, B, D, E, G or P as a shadow.
fn shadow(text: String) -> String {
    changed(
        &text,
        "contact = 1\ndistance = 2",
        "contact = 1\ndistance = 2\nsearch_only = true",
    )
}

impl Played {
    fn decisions(&self, day: u32) -> &StaffDecisions {
        &self.handed[&day]
    }

    fn notices(&self, day: u32) -> &[StaffNotice] {
        &self.decisions(day).notices
    }

    /// The notices of this rule handed in on `day`, with what each says.
    fn follows(&self, day: u32) -> Vec<(NoticeKind, FollowNotice)> {
        self.notices(day)
            .iter()
            .filter_map(|notice| notice.follow.map(|f| (notice.kind, f)))
            .collect()
    }

    fn kinds(&self, day: u32) -> Vec<NoticeKind> {
        self.notices(day).iter().map(|n| n.kind).collect()
    }

    /// The notice of this rule handed in on `day`, which must be the only one.
    fn the_follow(&self, day: u32) -> (NoticeKind, FollowNotice) {
        let all = self.follows(day);
        assert_eq!(all.len(), 1, "day {day}: one notice of this rule: {all:?}");
        all[0]
    }

    /// The reason a notice rests on.
    fn notice_reason(&self, day: u32, kind: NoticeKind) -> &Reason {
        let d = self.decisions(day);
        let notice = d
            .notices
            .iter()
            .find(|n| n.kind == kind)
            .unwrap_or_else(|| panic!("day {day} has no notice {kind:?}"));
        match notice.rests_on {
            sim_core::command::NoticeBasis::NoOrder(i) => &d.no_orders[i],
            sim_core::command::NoticeBasis::Command(i) => &d.commands[i].reason,
        }
    }

    fn orders_to_air(&self, day: u32, raw: u32) -> Vec<AirGroupOrder> {
        self.decisions(day)
            .orders
            .iter()
            .filter_map(|o| match &o.kind {
                OrderKind::AirGroup { target, order } if target.raw() == raw => Some(order.clone()),
                _ => None,
            })
            .collect()
    }

    fn air_orders(&self, day: u32) -> Vec<(u32, AirGroupOrder)> {
        self.decisions(day)
            .orders
            .iter()
            .filter_map(|o| match &o.kind {
                OrderKind::AirGroup { target, order } => Some((target.raw(), order.clone())),
                _ => None,
            })
            .collect()
    }

    fn orders_to_force(&self, day: u32, raw: u32) -> Vec<TaskForceOrder> {
        self.decisions(day)
            .orders
            .iter()
            .filter_map(|o| match &o.kind {
                OrderKind::TaskForce { target, order } if target.raw() == raw => {
                    Some(order.clone())
                }
                _ => None,
            })
            .collect()
    }

    fn command_kinds(&self, day: u32) -> Vec<CommandKind> {
        self.decisions(day)
            .commands
            .iter()
            .map(|c| c.kind.clone())
            .collect()
    }

    fn command_of(&self, day: u32, wanted: &CommandKind) -> &StaffCommand {
        self.decisions(day)
            .commands
            .iter()
            .find(|c| &c.kind == wanted)
            .unwrap_or_else(|| panic!("day {day} has no command {wanted:?}"))
    }

    /// What the order keeps at the end of `day`.
    fn kept(&self, day: u32, order: u32) -> Option<Followed> {
        self.states[day as usize]
            .followed
            .get(&intent(order))
            .copied()
    }

    fn in_force(&self, day: u32, raw: u32) -> bool {
        self.states[day as usize]
            .intents
            .get(&intent(raw))
            .is_some_and(|i| i.in_force())
    }

    fn all(&self, pick: impl Fn(&EventKind) -> bool) -> Vec<&Event> {
        self.events.iter().filter(|e| pick(&e.kind)).collect()
    }

    fn launches(&self) -> Vec<&Event> {
        self.all(|k| matches!(k, EventKind::StrikeLaunched { .. }))
    }

    fn searches_of(&self, air_group: u32) -> Vec<(u32, Option<Pulse>)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::SearchFlown { air_group: g, .. } if g.raw() == air_group => {
                    Some((e.day, e.pulse))
                }
                _ => None,
            })
            .collect()
    }
}

fn said(reason: &Reason) -> String {
    let evidence: Vec<&str> = reason.evidence.iter().map(|e| e.what.as_str()).collect();
    format!(
        "{} | {} | {}",
        reason.worked,
        evidence.join(" | "),
        reason.binding.join(" | ")
    )
}

fn form(parent: u32, kind: IntentKind) -> CommandKind {
    CommandKind::FormIntent {
        parent: intent(parent),
        kind,
    }
}

fn strike_from(base: u32) -> IntentKind {
    IntentKind::StrikeShips {
        base: sim_core::ids::BaseId::new(base),
    }
}

fn search_from(base: u32) -> IntentKind {
    IntentKind::SearchWaters {
        base: sim_core::ids::BaseId::new(base),
    }
}

fn withdraw(raw: u32) -> CommandKind {
    CommandKind::WithdrawIntent {
        intent: intent(raw),
    }
}

/// The hexes within `distance` of `(q, r)` on the map of the games, in hex
/// order (every hex of these maps is on the map for the waters used).
fn waters(q: i32, r: i32, distance: i32) -> Vec<Hex> {
    let mut found = Vec::new();
    for dq in -distance..=distance {
        for dr in -distance..=distance {
            let h = hex(q + dq, r + dr);
            if h.distance(hex(q, r)) <= u32::try_from(distance).expect("a distance") {
                found.push(h);
            }
        }
    }
    found.sort();
    found
}

/// The waters to strike first the staff ordered a group on `day`, if any.
fn first_waters(played: &Played, day: u32, group: u32) -> Option<Vec<Hex>> {
    played
        .orders_to_air(day, group)
        .into_iter()
        .find_map(|o| match o {
            AirGroupOrder::SetStrikeFirstWaters { hexes } => Some(hexes),
            _ => None,
        })
}

/// The search arc the staff ordered a group on `day`, if any.
fn arc_of(played: &Played, day: u32, group: u32) -> Option<(u32, u32, Fixed)> {
    played
        .orders_to_air(day, group)
        .into_iter()
        .find_map(|o| match o {
            AirGroupOrder::SetSearchArc {
                from_deg,
                to_deg,
                range_nm,
            } => Some((
                u32::try_from(from_deg).expect("a bearing"),
                u32::try_from(to_deg).expect("a bearing"),
                range_nm,
            )),
            _ => None,
        })
}

/// Whether every hex of `waters` is in the arc by the engine's own test.
fn arc_holds(base: Hex, arc: (u32, u32, Fixed), waters: &[Hex]) -> bool {
    waters.iter().all(|h| in_arc(base, *h, arc.0, arc.1))
}

/// The causes of the events that withdrew an intent, by intent.
fn withdrawal_causes(played: &Played, raw: u32) -> Vec<Vec<Cause>> {
    played
        .events
        .iter()
        .filter(
            |e| matches!(&e.kind, EventKind::IntentWithdrawn { intent: i, .. } if i.raw() == raw),
        )
        .map(|e| e.causes.clone())
        .collect()
}

/// The `StaffNotice` events of this rule in the log: the day and the kind.
fn notice_events(played: &Played) -> Vec<(u32, NoticeKind, &Event)> {
    played
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::StaffNotice {
                kind,
                follow: Some(_),
                ..
            } => Some((e.day, *kind, e)),
            _ => None,
        })
        .collect()
}

// ------------------------------------------------- The games made from a game

/// The text of a scenario with the blocks of the task force `id` and of its
/// ships taken out.
fn without_force(text: &str, id: u32) -> String {
    let mut out = String::new();
    let mut chunk = String::new();
    let flush = |chunk: &mut String, out: &mut String| {
        let own = chunk.starts_with("[[task_force]]") && chunk.contains(&format!("\nid = {id}\n"));
        let ship =
            chunk.starts_with("[[ship]]") && chunk.contains(&format!("\ntask_force = {id}\n"));
        if !own && !ship {
            out.push_str(chunk);
        }
        chunk.clear();
    };
    for line in text.lines() {
        if line.starts_with("[[") {
            flush(&mut chunk, &mut out);
        }
        chunk.push_str(line);
        chunk.push('\n');
    }
    flush(&mut chunk, &mut out);
    out
}

/// Game B with task force `id` moved to `(q, r)`.
fn force_moved(text: &str, name: &str, from: (i32, i32), to: (i32, i32)) -> String {
    changed(
        text,
        &format!(
            "name = \"{name}\"\nside = \"japanese\"\nq = {}\nr = {}\n",
            from.0, from.1
        ),
        &format!(
            "name = \"{name}\"\nside = \"japanese\"\nq = {}\nr = {}\n",
            to.0, to.1
        ),
    )
}

/// Game B with task force 1, the carrier, made two cruisers where it lies.
fn force_1_is_two_cruisers(text: &str) -> String {
    let text = changed(
        text,
        "name = \"Placeholder carrier 1\"\nclass = 4\nside = \"japanese\"\nfuel_t = 3000\ntask_force = 1\n",
        "name = \"Placeholder cruiser 1\"\nclass = 1\nside = \"japanese\"\nfuel_t = 2000\ntask_force = 1\n\n[[ship]]\nid = 4\nname = \"Placeholder cruiser 4\"\nclass = 1\nside = \"japanese\"\nfuel_t = 2000\ntask_force = 1\n",
    );
    changed(
        &text,
        "name = \"Placeholder carrier force 1\"",
        "name = \"Placeholder cruiser force 1\"",
    )
}

const FORCE_1_AT_6: (i32, i32) = (6, 0);
const FORCE_2_AT_10: (i32, i32) = (10, 0);

fn cruiser_force_name() -> &'static str {
    "Placeholder cruiser force 2"
}

// ------------------------------------------------------ A game on a seat

/// A session over a scenario's text as `sim-cli` opens one, the Allied seat
/// played, the staffs asked each day, the scripted commands handed in each
/// dusk, and orders allowed inside a day.
fn session_of(file: &str, text: &str, figures: &sim_data::LoadOptions) -> sim_agent::Session {
    let mut scenario: Scenario =
        common::from_text_with(text, &path_of(file), figures).expect("the scenario loads");
    let commands = scenario.scripted_commands().clone();
    scenario.replace_commands(BTreeMap::new());
    let inside = scenario.orders_inside_a_day();
    let script_inside = scenario.scripted_orders_inside_day().clone();
    // The Japanese seat is the script's: the orders the file gives it.
    let mut japanese: BTreeMap<u32, Vec<sim_core::orders::OrderRequest>> = BTreeMap::new();
    for (day, list) in scenario.scripted_orders() {
        for (side, request) in list {
            if *side == Side::Japanese {
                japanese.entry(*day).or_default().push(request.clone());
            }
        }
    }
    let title = scenario.name.clone();
    sim_agent::Session::new(
        scenario.world,
        &title,
        scenario.days,
        sim_agent::Seat::Player,
        sim_agent::Seat::Script(japanese),
    )
    .with_staffs(staffs())
    .with_commands(commands)
    .with_orders_inside_a_day(inside, script_inside)
}

fn ask(session: &mut sim_agent::Session, tool: &str, args: serde_json::Value) -> serde_json::Value {
    session
        .call(Side::Allied, tool, &args)
        .unwrap_or_else(|e| panic!("{tool} {args}: {}", e.message))
}

/// Whether a task force order is a route to exactly these hexes.
fn is_route_to(order: &TaskForceOrder, wanted: &[Hex]) -> bool {
    matches!(order, TaskForceOrder::SetRoute { waypoints, .. } if waypoints == wanted)
}

/// Whether a task force order sets full speed.
fn is_full_speed(order: &TaskForceOrder) -> bool {
    matches!(order, TaskForceOrder::SetSpeed(SpeedSetting::Full))
}

fn operate(force: u32, at: Hex) -> IntentKind {
    IntentKind::OperateCarriers {
        force: TaskForceId::new(force),
        hex: at,
    }
}

/// What the paper of the order to destroy a force shows at one dusk, as
/// `command_state` gives it: the contact followed, its hex, how many ships
/// and taken for what, how old, the report it is of, each taking in order
/// (the number taken, the number it was taken from and the day), and whether
/// the order is lost.
#[derive(Debug, PartialEq, Eq)]
struct Paper {
    contact: u64,
    hex: (i64, i64),
    ships: u64,
    taken_for: String,
    age: u64,
    reported: (u64, String),
    takings: Vec<(u64, u64, u64)>,
    lost: bool,
}

fn paper_of(session: &mut sim_agent::Session) -> Paper {
    let state = ask(session, "command_state", serde_json::json!({}));
    let order = state["intents"]
        .as_array()
        .expect("the intents")
        .iter()
        .find(|i| i["kind"] == serde_json::json!("destroy_force"))
        .expect("the order is in the command state");
    let f = &order["following"];
    Paper {
        contact: f["contact"].as_u64().expect("contact"),
        hex: (
            f["hex"][0].as_i64().expect("q"),
            f["hex"][1].as_i64().expect("r"),
        ),
        ships: f["ships"].as_u64().expect("ships"),
        taken_for: f["taken_for"].as_str().expect("taken_for").to_string(),
        age: f["age_pulses"].as_u64().expect("age"),
        reported: (
            f["reported"]["day"].as_u64().expect("day"),
            f["reported"]["pulse"].as_str().expect("pulse").to_string(),
        ),
        takings: f["takings"]
            .as_array()
            .expect("takings")
            .iter()
            .map(|t| {
                (
                    t["contact"].as_u64().expect("contact"),
                    t["taken_from"].as_u64().expect("taken_from"),
                    t["day"].as_u64().expect("day"),
                )
            })
            .collect(),
        lost: f["lost"].as_bool().expect("lost"),
    }
}

#[test]
fn a_given_the_order_keeps_its_contact_and_forms_a_denys_tasks() {
    let played = play_file("133_a_followed.toml", same);
    let allied = &played.states[1];

    // 1. Given: it keeps contact 1, (8,0), the afternoon of day 0, not a
    // carrier; its leave to pursue reads go from the order's kind with none
    // set; no notice of this rule on day 1.
    assert_eq!(
        played.kept(1, 1),
        Some(Followed {
            contact: 1,
            hex: hex(8, 0),
            day: 0,
            pulse: Pulse::Afternoon,
            ship_count: 2,
            kind: ShipKind::Cruiser,
            carrier: false,
            lost: false,
        })
    );
    assert!(allied.pursue_leaves.is_empty(), "no leave is set on it");
    assert_eq!(
        allied.pursue_read_up(intent(1)),
        (sim_core::command::PursueLeave::Go, Some(intent(1)))
    );
    assert!(
        played.follows(1).is_empty(),
        "no notice of this rule on day 1"
    );

    // 2. The tree on day 1: intent 2, strike ships from base 1, and intent 3,
    // search the waters from base 1, both under intent 1; no stand task and
    // no carrier task.
    let formed: Vec<CommandKind> = played
        .command_kinds(1)
        .into_iter()
        .filter(|k| matches!(k, CommandKind::FormIntent { .. }))
        .collect();
    assert_eq!(
        formed,
        vec![form(1, strike_from(1)), form(1, search_from(1))]
    );
    let strike = &allied.intents[&intent(2)];
    assert_eq!(strike.kind, strike_from(1));
    assert_eq!(strike.parent, Some(intent(1)));
    assert_eq!(strike.maker, sim_core::command::Maker::Staff);
    let search = &allied.intents[&intent(3)];
    assert_eq!(search.kind, search_from(1));
    assert_eq!(search.parent, Some(intent(1)));
    assert_eq!(allied.intents.len(), 3, "intents 1, 2 and 3 and no other");
    assert!(allied.intents.values().all(|i| !matches!(
        i.kind,
        IntentKind::StandAt { .. } | IntentKind::OperateCarriers { .. }
    )));

    // 3. The orders of day 1. Group 1: the search mission, search once a day
    // and an arc at 600 nm that holds every one of the 19 hexes.
    let nineteen = waters(8, 0, 2);
    assert_eq!(nineteen.len(), 19);
    let group_1 = played.orders_to_air(1, 1);
    assert_eq!(group_1.len(), 3, "{group_1:?}");
    assert_eq!(group_1[0], AirGroupOrder::SetAirMission(AirMission::Search));
    assert_eq!(group_1[1], AirGroupOrder::SetSearchOnce { on: true });
    let arc = arc_of(&played, 1, 1).expect("group 1 is given an arc");
    assert_eq!(arc.2, n(600));
    assert!(arc_holds(hex(0, 0), arc, &nineteen));
    // Group 2: the naval strike mission, the waters to strike first (the one
    // order more) and strike sizing on, and nothing else.
    assert_eq!(
        played.orders_to_air(1, 2),
        vec![
            AirGroupOrder::SetAirMission(AirMission::NavalStrike),
            AirGroupOrder::SetStrikeFirstWaters {
                hexes: nineteen.clone()
            },
            AirGroupOrder::SetStrikeSizing { on: true },
        ]
    );
    // The reason of that order names intent 1 as the order to destroy a
    // force and says a contact in them is struck before every one outside.
    let waters_order: &StaffOrder = played
        .decisions(1)
        .orders
        .iter()
        .find(|o| {
            matches!(
                &o.kind,
                OrderKind::AirGroup {
                    order: AirGroupOrder::SetStrikeFirstWaters { .. },
                    ..
                }
            )
        })
        .expect("the order that sets the waters");
    assert_eq!(waters_order.reason.serves.intent, Some(intent(1)));
    let text = said(&waters_order.reason);
    assert!(text.contains("intent:1"), "{text}");
    assert!(text.contains("the order to destroy a force"), "{text}");
    assert!(text.contains("struck before every one outside"), "{text}");
}

#[test]
fn a_a_refreshed_contact_is_followed_with_its_number() {
    let played = play_file("133_a_followed.toml", same);

    // 4. Dusk of day 2: refreshed. One notice, follows: contact 1, the number
    // unchanged, (8,0), the morning of day 1.
    let (kind, follow) = played.the_follow(2);
    assert_eq!(kind, NoticeKind::Follows);
    assert_eq!(follow.intent, intent(1));
    assert_eq!(follow.contact, 1);
    assert_eq!(follow.taken_from, None);
    assert!(!follow.other_sort);
    assert_eq!(follow.hex, hex(8, 0));
    assert_eq!((follow.day, follow.pulse), (1, Pulse::Morning));
    // The order keeps that report.
    assert_eq!(
        played.kept(2, 1),
        Some(Followed {
            contact: 1,
            hex: hex(8, 0),
            day: 1,
            pulse: Pulse::Morning,
            ship_count: 2,
            kind: ShipKind::Cruiser,
            carrier: false,
            lost: false,
        })
    );
    // No task is formed or withdrawn and no order is given to a group.
    assert!(played.decisions(2).commands.is_empty());
    assert!(played.decisions(2).orders.is_empty());
}

#[test]
fn a_the_force_seen_again_as_a_new_contact_is_taken() {
    let played = play_file("133_a_followed.toml", same);

    // 5. Dusk of day 3: the side holds contact 1, 4 pulses old, and contact 2.
    // One notice, follows: contact 2 for contact 1, the same sort, at (14,0),
    // 2 ships, cruisers, the morning of day 2.
    let (kind, follow) = played.the_follow(3);
    assert_eq!(kind, NoticeKind::Follows);
    assert_eq!(follow.contact, 2);
    assert_eq!(follow.taken_from, Some(1));
    assert!(!follow.other_sort);
    assert_eq!(follow.hex, hex(14, 0));
    assert_eq!((follow.day, follow.pulse), (2, Pulse::Morning));
    assert_eq!(follow.ship_count, 2);
    assert_eq!(follow.kind, ShipKind::Cruiser);
    assert!(!follow.carrier);
    assert_eq!(
        played.kept(3, 1),
        Some(Followed {
            contact: 2,
            hex: hex(14, 0),
            day: 2,
            pulse: Pulse::Morning,
            ship_count: 2,
            kind: ShipKind::Cruiser,
            carrier: false,
            lost: false,
        })
    );
    // Its waters are the 19 hexes round (14,0): group 2 is ordered those to
    // strike first and group 1 an arc that holds every one of them.
    let round = waters(14, 0, 2);
    assert_eq!(round.len(), 19);
    assert_eq!(first_waters(&played, 3, 2), Some(round.clone()));
    let arc = arc_of(&played, 3, 1).expect("group 1 is given an arc");
    assert!(arc_holds(hex(0, 0), arc, &round));
    // Intents 2 and 3 stand: the first dusk at which no bomber of base 1
    // touches the waters.
    assert!(played.decisions(3).commands.is_empty());
    assert!(played.in_force(3, 2) && played.in_force(3, 3));
}

#[test]
fn a_a_base_out_of_reach_two_dusks_running_has_its_task_withdrawn_and_formed_again() {
    let played = play_file("133_a_followed.toml", same);

    // 6. Dusk of day 4: one notice, follows: contact 2, the number unchanged,
    // the morning of day 3.
    let (kind, follow) = played.the_follow(4);
    assert_eq!(kind, NoticeKind::Follows);
    assert_eq!(follow.contact, 2);
    assert_eq!(follow.taken_from, None);
    assert_eq!((follow.day, follow.pulse), (3, Pulse::Morning));
    // The staff withdraws intent 2, the second dusk running at which no
    // bomber touches the waters; its reason gives 480 nm to the nearest hex
    // of the waters against the reach of 250 nm. Intent 3 stands.
    assert_eq!(played.command_kinds(4), vec![withdraw(2)]);
    let reason = said(&played.command_of(4, &withdraw(2)).reason);
    assert!(reason.contains("480"), "{reason}");
    assert!(reason.contains("250"), "{reason}");
    assert!(!played.in_force(4, 2));
    assert!(played.in_force(4, 3));
    // Group 2 is given no order and keeps what it has.
    assert!(played.orders_to_air(4, 2).is_empty());

    // 7. Dusk of day 5: back in reach. One notice, follows: contact 3 is taken
    // to be the force of contact 2, at (8,0), the morning of day 4.
    let (kind, follow) = played.the_follow(5);
    assert_eq!(kind, NoticeKind::Follows);
    assert_eq!(follow.contact, 3);
    assert_eq!(follow.taken_from, Some(2));
    assert_eq!(follow.hex, hex(8, 0));
    assert_eq!((follow.day, follow.pulse), (4, Pulse::Morning));
    // The staff forms intent 4, strike ships from base 1, under intent 1.
    assert_eq!(
        played
            .command_kinds(5)
            .into_iter()
            .filter(|k| matches!(k, CommandKind::FormIntent { .. }))
            .collect::<Vec<_>>(),
        vec![form(1, strike_from(1))]
    );
    let four = &played.states[5].intents[&intent(4)];
    assert_eq!(four.kind, strike_from(1));
    assert_eq!(four.parent, Some(intent(1)));
    // Group 2 is ordered the 19 hexes round (8,0) to strike first and group 1
    // an arc that holds them.
    let round = waters(8, 0, 2);
    assert_eq!(first_waters(&played, 5, 2), Some(round.clone()));
    let arc = arc_of(&played, 5, 1).expect("group 1 is given an arc");
    assert!(arc_holds(hex(0, 0), arc, &round));
}

#[test]
fn a_lost_then_ended_not_found() {
    let played = play_file("133_a_followed.toml", same);

    // 8. Dusk of day 6: lost. Contact 3 is 4 pulses old and the side holds no
    // other. One notice, contact lost, naming contact 3 and (8,0), where it
    // was last reported.
    let (kind, follow) = played.the_follow(6);
    assert_eq!(kind, NoticeKind::ContactLost);
    assert_eq!(follow.contact, 3);
    assert_eq!(follow.hex, hex(8, 0));
    assert_eq!((follow.day, follow.pulse), (4, Pulse::Morning));
    // The order's centre stays (8,0): kept, and lost.
    let kept = played.kept(6, 1).expect("the order keeps its report");
    assert_eq!((kept.contact, kept.hex, kept.lost), (3, hex(8, 0), true));
    // The staff withdraws intent 4; intent 3 stands; group 1 is given no
    // order, its arc being what it has.
    assert_eq!(played.command_kinds(6), vec![withdraw(4)]);
    assert!(!played.in_force(6, 4));
    assert!(played.in_force(6, 3));
    assert!(played.orders_to_air(6, 1).is_empty());

    // 9. Dusk of day 7: ended. The last report kept is of day 4, and 4 + 2 + 1
    // is 7. One notice, ended, the force not found.
    let (kind, follow) = played.the_follow(7);
    assert_eq!(kind, NoticeKind::ForceNotFound);
    assert_eq!(follow.contact, 3);
    assert_eq!((follow.day, follow.pulse), (4, Pulse::Morning));
    // The side no longer holds contact 3: the notice says what the order
    // kept of its last report (ruling 6 after the build).
    assert_eq!(follow.hex, hex(8, 0));
    assert_eq!((follow.ship_count, follow.kind), (2, ShipKind::Cruiser));
    // Intents 1 and 3 are withdrawn, with the notice's event as the cause.
    let notice_event = notice_events(&played)
        .into_iter()
        .find(|(_, kind, _)| *kind == NoticeKind::ForceNotFound)
        .map(|(_, _, e)| e.id)
        .expect("the notice is in the log");
    for raw in [1, 3] {
        assert!(!played.in_force(7, raw), "intent {raw} is withdrawn");
    }
    assert_eq!(
        withdrawal_causes(&played, 3),
        vec![vec![Cause::Event(notice_event)]],
        "intent 3"
    );
    // Intent 1 itself is withdrawn the same way: its plan completes, the
    // notice's event the cause.
    let plan_ended: Vec<&Event> = played
        .events
        .iter()
        .filter(|e| e.day == 7 && matches!(e.kind, EventKind::PlanStateChanged { .. }))
        .collect();
    assert_eq!(plan_ended.len(), 1);
    assert_eq!(plan_ended[0].causes, vec![Cause::Event(notice_event)]);
    assert_eq!(
        played.states[7].followed.len(),
        0,
        "nothing of the order is kept"
    );
    // On day 7 the staff hands in nothing for the order.
    assert!(played.decisions(7).orders.is_empty());
    assert!(played.decisions(7).commands.is_empty());
}

#[test]
fn a_the_notices_the_paper_and_no_strike() {
    let played = play_file("133_a_followed.toml", same);

    // 10. What is handed in, in all: follows on days 2, 3, 4 and 5, lost on
    // day 6, ended on day 7, and no other; the lost notice once; no notice of
    // a pursuit and none that the order cannot be met on any day.
    let expected: BTreeMap<u32, Vec<NoticeKind>> = BTreeMap::from([
        (1, vec![]),
        (2, vec![NoticeKind::Follows]),
        (3, vec![NoticeKind::Follows]),
        (4, vec![NoticeKind::Follows]),
        (5, vec![NoticeKind::Follows]),
        (6, vec![NoticeKind::ContactLost]),
        (7, vec![NoticeKind::ForceNotFound]),
    ]);
    let handed: BTreeMap<u32, Vec<NoticeKind>> = (1..=7).map(|d| (d, played.kinds(d))).collect();
    assert_eq!(handed, expected);
    for day in 1..=7 {
        assert!(played
            .notices(day)
            .iter()
            .all(|n| n.pursuit.is_none() && n.kind != NoticeKind::CannotMeet));
    }

    // 11. The paper: what the order keeps at each dusk, by the report, each
    // taking in the order it happened (contact 2 for 1, contact 3 for 2), and
    // whether it is lost.
    let kept: Vec<(u32, Option<KeptRow>)> = (1..=7)
        .map(|day| {
            (
                day,
                played
                    .kept(day, 1)
                    .map(|k| (k.contact, k.hex, k.day, k.pulse, k.lost)),
            )
        })
        .collect();
    assert_eq!(
        kept,
        vec![
            (1, Some((1, hex(8, 0), 0, Pulse::Afternoon, false))),
            (2, Some((1, hex(8, 0), 1, Pulse::Morning, false))),
            (3, Some((2, hex(14, 0), 2, Pulse::Morning, false))),
            (4, Some((2, hex(14, 0), 3, Pulse::Morning, false))),
            (5, Some((3, hex(8, 0), 4, Pulse::Morning, false))),
            (6, Some((3, hex(8, 0), 4, Pulse::Morning, true))),
            (7, None),
        ]
    );
    let takings: Vec<(u32, u32, Option<u32>)> = (1..=7)
        .flat_map(|day| {
            played
                .follows(day)
                .into_iter()
                .filter(|(kind, f)| *kind == NoticeKind::Follows && f.taken_from.is_some())
                .map(move |(_, f)| (day, f.contact, f.taken_from))
        })
        .collect();
    assert_eq!(takings, vec![(3, 2, Some(1)), (5, 3, Some(2))]);

    // 11, as the tool shows it: command_state at each dusk, before the staff
    // is asked on it, over the same game played on a seat.
    let file = "133_a_followed.toml";
    let mut session = session_of(file, &text_of(file), &figures());
    let mut papers = Vec::new();
    for _ in 1..=7 {
        papers.push(paper_of(&mut session));
        ask(&mut session, "end_day", serde_json::json!({}));
    }
    let cruisers = |contact: u64,
                    hex: (i64, i64),
                    age: u64,
                    reported: (u64, &str),
                    takings: &[(u64, u64, u64)],
                    lost: bool| Paper {
        contact,
        hex,
        ships: 2,
        taken_for: "cruiser".to_string(),
        age,
        reported: (reported.0, reported.1.to_string()),
        takings: takings.to_vec(),
        lost,
    };
    let first = [(2, 1, 3)];
    let both = [(2, 1, 3), (3, 2, 5)];
    assert_eq!(
        papers,
        vec![
            cruisers(1, (8, 0), 0, (0, "afternoon"), &[], false),
            cruisers(1, (8, 0), 1, (1, "morning"), &[], false),
            cruisers(1, (8, 0), 4, (1, "morning"), &[], false),
            cruisers(2, (14, 0), 1, (3, "morning"), &first, false),
            cruisers(2, (14, 0), 4, (3, "morning"), &first, false),
            cruisers(3, (8, 0), 4, (4, "morning"), &both, false),
            cruisers(3, (8, 0), 7, (4, "morning"), &both, true),
        ]
    );

    // 12. Nothing is struck: no StrikeLaunched on any day.
    assert!(played.launches().is_empty());
}

#[test]
fn s_search_only_forms_no_strike_and_searches_in_both_pulses() {
    let played = play_file("133_a_followed.toml", shadow);
    let allied = &played.states[1];

    // 13. The staff forms intent 2, search the waters from base 1, and no
    // other task on any day: no strike task, though a bomber's reach touches
    // the waters on day 1. Group 2 is given no order on any day. No group is
    // ordered waters to strike first. The order's leave to pursue reads stay.
    for day in 1..=7 {
        let formed: Vec<CommandKind> = played
            .command_kinds(day)
            .into_iter()
            .filter(|k| matches!(k, CommandKind::FormIntent { .. }))
            .collect();
        if day == 1 {
            assert_eq!(formed, vec![form(1, search_from(1))]);
        } else {
            assert!(formed.is_empty(), "day {day}: {formed:?}");
        }
        assert!(
            played.orders_to_air(day, 2).is_empty(),
            "day {day}: group 2 is given no order"
        );
        for (group, order) in played.air_orders(day) {
            assert!(
                !matches!(order, AirGroupOrder::SetStrikeFirstWaters { .. }),
                "day {day}: group {group} is ordered waters to strike first"
            );
        }
    }
    assert_eq!(allied.intents[&intent(2)].kind, search_from(1));
    assert_eq!(allied.intents[&intent(2)].parent, Some(intent(1)));
    assert_eq!(allied.intents.len(), 2);
    assert_eq!(
        allied.pursue_read_up(intent(1)),
        (sim_core::command::PursueLeave::Stay, Some(intent(1)))
    );

    // 14. Group 1 is ordered the search mission and an arc, and is not set to
    // search once a day: a SearchFlown of group 1 is recorded in the morning
    // and in the afternoon of every day from 1 to 6.
    let first = played.orders_to_air(1, 1);
    assert_eq!(first.len(), 2, "{first:?}");
    assert_eq!(first[0], AirGroupOrder::SetAirMission(AirMission::Search));
    assert!(arc_of(&played, 1, 1).is_some());
    for day in 1..=7 {
        assert!(played
            .orders_to_air(day, 1)
            .iter()
            .all(|o| !matches!(o, AirGroupOrder::SetSearchOnce { .. })));
    }
    let flown = played.searches_of(1);
    for day in 1..=6 {
        assert!(
            flown.contains(&(day, Some(Pulse::Morning))),
            "day {day} morning"
        );
        assert!(
            flown.contains(&(day, Some(Pulse::Afternoon))),
            "day {day} afternoon"
        );
    }

    // 15. The notices fall on the days of game A: follows on days 2, 3, 4 and
    // 5, lost on day 6, ended on day 7. At dusk of day 3 contact 1 is 3
    // pulses old, not fresh, and contact 2 is 0 (the afternoon of day 2):
    // from the start of the afternoon of day 1 to the end of the afternoon of
    // day 2 is 30 hours, 15 hexes against 6. The order keeps contact 2,
    // (14,0), the afternoon of day 2.
    let handed: Vec<(u32, Vec<NoticeKind>)> = (1..=7).map(|d| (d, played.kinds(d))).collect();
    assert_eq!(
        handed,
        vec![
            (1, vec![]),
            (2, vec![NoticeKind::Follows]),
            (3, vec![NoticeKind::Follows]),
            (4, vec![NoticeKind::Follows]),
            (5, vec![NoticeKind::Follows]),
            (6, vec![NoticeKind::ContactLost]),
            (7, vec![NoticeKind::ForceNotFound]),
        ]
    );
    let (_, follow) = played.the_follow(3);
    assert_eq!((follow.contact, follow.taken_from), (2, Some(1)));
    assert_eq!(
        sim_core::command::report_distance_hexes(1, Pulse::Afternoon, 2, Pulse::Afternoon, 20),
        15
    );
    assert_eq!(follow.hex.distance(hex(8, 0)), 6);
    assert_eq!(
        played.kept(3, 1),
        Some(Followed {
            contact: 2,
            hex: hex(14, 0),
            day: 2,
            pulse: Pulse::Afternoon,
            ship_count: 2,
            kind: ShipKind::Cruiser,
            carrier: false,
            lost: false,
        })
    );
}

#[test]
fn b_the_sort_decides_then_the_nearest_then_the_number() {
    let b = "133_b_which_contact.toml";

    // 16. The sort decides. Contacts 2 (the carrier, 4 hexes off) and 3 (the
    // cruisers, 8 hexes off) both qualify. The followed force was not taken
    // for a carrier, so contact 3, the cruisers, is taken before contact 2,
    // though it is the farther. One notice, follows: contact 3 for contact 1,
    // the same sort.
    let played = play_file(b, same);
    let (kind, follow) = played.the_follow(2);
    assert_eq!(kind, NoticeKind::Follows);
    assert_eq!((follow.contact, follow.taken_from), (3, Some(1)));
    assert!(!follow.other_sort);
    assert_eq!(follow.hex, hex(10, 0));
    assert_eq!((follow.day, follow.pulse), (1, Pulse::Morning));
    assert_eq!((follow.ship_count, follow.kind), (2, ShipKind::Cruiser));
    assert!(!played.kept(2, 1).expect("kept").carrier);
    // The distances the statement gives: contact 2, 4 hexes from (2,0), is
    // the carrier; contact 3, 8 hexes from (2,0).
    assert_eq!(hex(6, 0).distance(hex(2, 0)), 4);
    assert_eq!(hex(10, 0).distance(hex(2, 0)), 8);

    // 17. The other sort (game B with task force 2 taken out). Contact 2
    // alone qualifies and is taken. The notice says it is of the other sort:
    // a carrier for a force that was not. The order then keeps "taken for a
    // carrier".
    let played = play_file(b, |t| without_force(&t, 2));
    let (kind, follow) = played.the_follow(2);
    assert_eq!(kind, NoticeKind::Follows);
    assert_eq!((follow.contact, follow.taken_from), (2, Some(1)));
    assert!(follow.other_sort);
    assert!(follow.carrier);
    assert_eq!(follow.kind, ShipKind::Carrier);
    assert_eq!(follow.hex, hex(6, 0));
    assert!(played.kept(2, 1).expect("kept").carrier);

    // 18. The nearest (game B with task force 1 made two cruisers). Contacts
    // 2 and 3 are of one sort; contact 2, 4 hexes off, is taken.
    let played = play_file(b, |t| force_1_is_two_cruisers(&t));
    let (kind, follow) = played.the_follow(2);
    assert_eq!(kind, NoticeKind::Follows);
    assert_eq!((follow.contact, follow.taken_from), (2, Some(1)));
    assert!(!follow.other_sort);
    assert_eq!(follow.hex, hex(6, 0));
    assert_eq!(follow.kind, ShipKind::Cruiser);

    // 19. A tie (game B with task force 1 made two cruisers at (5,2) and task
    // force 2 at (7,-2)). Each is 5 hexes from (2,0) and they are 4 apart.
    // Contact 2, at (5,2), the lower number, is taken.
    let played = play_file(b, |t| {
        let t = force_1_is_two_cruisers(&t);
        let t = force_moved(&t, "Placeholder cruiser force 1", FORCE_1_AT_6, (5, 2));
        force_moved(&t, cruiser_force_name(), FORCE_2_AT_10, (7, -2))
    });
    assert_eq!(hex(5, 2).distance(hex(2, 0)), 5);
    assert_eq!(hex(7, -2).distance(hex(2, 0)), 5);
    assert_eq!(hex(5, 2).distance(hex(7, -2)), 4);
    let (kind, follow) = played.the_follow(2);
    assert_eq!(kind, NoticeKind::Follows);
    assert_eq!((follow.contact, follow.taken_from), (2, Some(1)));
    assert_eq!(follow.hex, hex(5, 2));
}

#[test]
fn b_at_the_distance_and_beyond_it() {
    let b = "133_b_which_contact.toml";

    // 20. At the distance (game B with task force 1 taken out and task force
    // 2 at (14,0)). Contact 2 is 12 hexes from (2,0), which is the distance:
    // it is taken.
    let alone_at = |to: (i32, i32)| {
        move |t: String| {
            let t = without_force(&t, 1);
            force_moved(&t, cruiser_force_name(), FORCE_2_AT_10, to)
        }
    };
    let played = play_file(b, alone_at((14, 0)));
    assert_eq!(hex(14, 0).distance(hex(2, 0)), 12);
    let (kind, follow) = played.the_follow(2);
    assert_eq!(kind, NoticeKind::Follows);
    assert_eq!((follow.contact, follow.taken_from), (2, Some(1)));
    assert_eq!(follow.hex, hex(14, 0));

    // 21. Too far, then seen again (as 20, with task force 2 at (15,0), 600 nm
    // from base 1, the range). At dusk of day 2 contact 2, 13 hexes off, does
    // not qualify: the order is lost, with a contact one pulse old on the
    // chart, and its centre stays (2,0).
    let played = play_file(b, alone_at((15, 0)));
    assert_eq!(hex(15, 0).distance(hex(2, 0)), 13);
    let (kind, follow) = played.the_follow(2);
    assert_eq!(kind, NoticeKind::ContactLost);
    assert_eq!(follow.hex, hex(2, 0));
    let kept = played.kept(2, 1).expect("the order keeps its report");
    assert_eq!((kept.contact, kept.hex, kept.lost), (1, hex(2, 0), true));
    // The contact on the chart, one pulse old: its report is the morning of
    // day 1, 6 + 12 + 6 = 24 hours after the start of the afternoon of day 0
    // to the end of that morning: 12 hexes against 13.
    assert_eq!(
        sim_core::command::report_distance_hexes(0, Pulse::Afternoon, 1, Pulse::Morning, 20),
        12
    );
    // In the morning of day 2 the search of the lost waters refreshes contact
    // 2. At dusk of day 3, the dusk the order would end at (0 + 2 + 1), rule
    // 7 is worked first: from the start of the afternoon of day 0 to the end
    // of the morning of day 2 is 48 hours, 24 hexes against 13. Contact 2 is
    // taken and the order does not end.
    assert_eq!(
        sim_core::command::report_distance_hexes(0, Pulse::Afternoon, 2, Pulse::Morning, 20),
        24
    );
    let (kind, follow) = played.the_follow(3);
    assert_eq!(kind, NoticeKind::Follows);
    assert_eq!((follow.contact, follow.taken_from), (2, Some(1)));
    assert_eq!((follow.day, follow.pulse), (2, Pulse::Morning));
    assert!(played
        .follows(3)
        .iter()
        .all(|(kind, _)| *kind != NoticeKind::ForceNotFound));
    assert!(played.in_force(3, 1), "the order does not end");
    let kept = played.kept(3, 1).expect("the order keeps its report");
    assert_eq!((kept.contact, kept.lost), (2, false));
}

#[test]
fn b_two_orders_and_one_new_contact() {
    let b = "133_b_which_contact.toml";

    // 22. Two orders and one new contact (game B with task force 1 taken out
    // and task force 2 at (12,0); a second contact at the start, contact 2, 2
    // ships, cruisers, at (7,0), age 0, where no ship is; and intent 2,
    // destroy the force of contact 2, within 1 hex). In the morning of day 1
    // the cruisers are reported 5 hexes from contact 2 and 10 from contact 1:
    // contact 3. At dusk of day 2 it qualifies for both orders (12 hexes).
    let two_orders = |extra: &'static str| {
        move |t: String| {
            let t = without_force(&t, 1);
            let t = force_moved(&t, cruiser_force_name(), FORCE_2_AT_10, (12, 0));
            format!(
                "{t}\n[[contact]]\nside = \"allied\"\nq = 7\nr = 0\nship_count = 2\nkind = \"cruiser\"\nage_pulses = 0\n{}{extra}",
                command_table(
                    1,
                    "kind = \"set_intent\"\nintent = 2\nintent_kind = \"destroy_force\"\ncontact = 2\ndistance = 1"
                )
            )
        }
    };
    assert_eq!(hex(12, 0).distance(hex(7, 0)), 5);
    assert_eq!(hex(12, 0).distance(hex(2, 0)), 10);
    let played = play_file(b, two_orders(""));
    // Intent 1, the lower number, takes it. No other contact qualifies for
    // intent 2, which is lost.
    let follows = played.follows(2);
    assert_eq!(follows.len(), 2, "{follows:?}");
    let by_order = |raw: u32| {
        follows
            .iter()
            .find(|(_, f)| f.intent == intent(raw))
            .copied()
            .unwrap_or_else(|| panic!("no notice for intent {raw}"))
    };
    let (kind, taken) = by_order(1);
    assert_eq!(kind, NoticeKind::Follows);
    assert_eq!((taken.contact, taken.taken_from), (3, Some(1)));
    assert_eq!(taken.hex, hex(12, 0));
    let (kind, lost) = by_order(2);
    assert_eq!(kind, NoticeKind::ContactLost);
    assert_eq!(lost.contact, 2);
    assert_eq!(played.kept(2, 1).map(|k| k.contact), Some(3));
    assert_eq!(
        played.kept(2, 2).map(|k| (k.contact, k.lost)),
        Some((2, true))
    );

    // With a priority limit of rank 1 set on intent 2 it is the other way:
    // intent 2 takes contact 3 and intent 1 is lost.
    let ranked = command_table(
        1,
        "kind = \"set_limit\"\nlimit = \"priority\"\nintent = 2\nrank = 1",
    );
    let played = play_file(b, move |t| {
        let leaf = two_orders("")(t);
        format!("{leaf}{ranked}")
    });
    let follows = played.follows(2);
    assert_eq!(follows.len(), 2, "{follows:?}");
    let by_order = |raw: u32| {
        follows
            .iter()
            .find(|(_, f)| f.intent == intent(raw))
            .copied()
            .unwrap_or_else(|| panic!("no notice for intent {raw}"))
    };
    let (kind, taken) = by_order(2);
    assert_eq!(kind, NoticeKind::Follows);
    assert_eq!((taken.contact, taken.taken_from), (3, Some(2)));
    let (kind, lost) = by_order(1);
    assert_eq!(kind, NoticeKind::ContactLost);
    assert_eq!(lost.contact, 1);
    assert_eq!(played.kept(2, 2).map(|k| k.contact), Some(3));
    assert_eq!(
        played.kept(2, 1).map(|k| (k.contact, k.lost)),
        Some((1, true))
    );
}

#[test]
fn b_given_inside_a_day_lost_and_found_again() {
    use serde_json::json;
    let file = "133_b_which_contact.toml";
    // Game B with the order given at the stop at 0600 of day 1, not at dusk.
    let text = changed(
        &text_of(file),
        "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_intent\"\nintent = 1\nintent_kind = \"destroy_force\"\ncontact = 1\ndistance = 1\n",
        "",
    );
    let text = changed(
        &text,
        "days = 3\n",
        "days = 3\norders_inside_a_day = true\n",
    );
    let mut session = session_of(file, &text, &figures());
    ask(&mut session, "advance", json!({"pulses": 1}));
    let given = ask(
        &mut session,
        "set_intent",
        json!({"intent_kind": "destroy_force", "contact": 1, "distance": 1}),
    );
    assert_eq!(given["status"], json!("queued"), "{given}");
    for _ in 0..3 {
        if session.is_over() {
            break;
        }
        ask(&mut session, "end_day", json!({}));
    }
    let handed: BTreeMap<u32, StaffDecisions> = session
        .staff_handed_in()
        .iter()
        .filter_map(|(d, m)| m.get(&Side::Allied).map(|x| (*d, x.clone())))
        .collect();
    let world = session.into_world();
    let events: Vec<Event> = world.events().iter().cloned().collect();

    // 23. It keeps contact 1, (2,0), the afternoon of day 0 at once, and acts
    // from dusk of day 2 (decision 0219): nothing is handed in on day 1. No
    // search flew on day 1, so the side holds contact 1 alone, 3 pulses old:
    // the order is lost at once, and the staff forms the search task.
    assert!(handed.get(&1).is_none_or(|d| d.notices.is_empty()));
    assert!(!events
        .iter()
        .any(|e| e.day == 1 && matches!(e.kind, EventKind::SearchFlown { .. })));
    let day_2 = &handed[&2];
    let kinds: Vec<NoticeKind> = day_2.notices.iter().map(|n| n.kind).collect();
    assert_eq!(kinds, vec![NoticeKind::ContactLost]);
    let lost = day_2.notices[0]
        .follow
        .expect("the notice says what it keeps");
    assert_eq!(lost.contact, 1);
    assert_eq!(lost.hex, hex(2, 0));
    assert_eq!((lost.day, lost.pulse), (0, Pulse::Afternoon));
    assert!(!lost.carrier);
    let formed: Vec<CommandKind> = day_2
        .commands
        .iter()
        .map(|c| c.kind.clone())
        .filter(|k| matches!(k, CommandKind::FormIntent { .. }))
        .collect();
    assert_eq!(formed, vec![form(1, search_from(1))]);
    // The forces are reported in the morning of day 2. At dusk of day 3 both
    // qualify (48 hours, 24 hexes) and contact 3, the cruisers, is taken:
    // lost, then found again under a new number.
    assert_eq!(
        sim_core::command::report_distance_hexes(0, Pulse::Afternoon, 2, Pulse::Morning, 20),
        24
    );
    let day_3 = &handed[&3];
    assert_eq!(day_3.notices.len(), 1);
    assert_eq!(day_3.notices[0].kind, NoticeKind::Follows);
    let taken = day_3.notices[0]
        .follow
        .expect("the notice says what it keeps");
    assert_eq!((taken.contact, taken.taken_from), (3, Some(1)));
    assert!(!taken.other_sort);
    assert_eq!(taken.hex, hex(10, 0));
    let kept = world.command_state(Side::Allied).followed[&intent(1)];
    assert_eq!((kept.contact, kept.lost), (3, false));
}

#[test]
fn d_a_contact_dropped_by_a_strike_that_found_nothing() {
    let d = "133_d_a_strike_that_found_nothing.toml";
    let played = play_file(d, same);

    // 24. In the morning of day 1 group 2 launches 10 at (5,0),
    // StrikeFoundNothing and ContactNotThere are recorded, and the side no
    // longer holds contact 1. No other strike is launched in the game.
    let launches = played.launches();
    assert_eq!(launches.len(), 1, "{launches:?}");
    match &launches[0].kind {
        EventKind::StrikeLaunched {
            air_group,
            hex: at,
            aircraft,
            ..
        } => {
            assert_eq!(air_group.raw(), 2);
            assert_eq!(*at, hex(5, 0));
            assert_eq!(*aircraft, 10);
        }
        other => panic!("{other:?}"),
    }
    assert_eq!(
        (launches[0].day, launches[0].pulse),
        (1, Some(Pulse::Morning))
    );
    assert_eq!(
        played
            .all(|k| matches!(k, EventKind::StrikeFoundNothing { .. }))
            .len(),
        1
    );
    let not_there = played.all(|k| matches!(k, EventKind::ContactNotThere { .. }));
    assert_eq!(not_there.len(), 1);
    assert_eq!(not_there[0].day, 1);
    assert!(
        played.held[1].iter().all(|(number, ..)| *number != 1),
        "the side no longer holds contact 1: {:?}",
        played.held[1]
    );

    // 25. At dusk of day 2 the order follows a contact the side does not
    // hold. Rule 7 is worked from what the order kept: (5,0), the afternoon
    // of day 0. Contact 2's last report is the afternoon of day 1: 6 + 12 + 6
    // + 6 = 30 hours, 15 hexes against 8. Contact 2 is taken; the order is
    // not lost.
    assert_eq!(
        sim_core::command::report_distance_hexes(0, Pulse::Afternoon, 1, Pulse::Afternoon, 20),
        15
    );
    assert_eq!(hex(13, 0).distance(hex(5, 0)), 8);
    let (kind, follow) = played.the_follow(2);
    assert_eq!(kind, NoticeKind::Follows);
    assert_eq!((follow.contact, follow.taken_from), (2, Some(1)));
    assert_eq!(follow.hex, hex(13, 0));
    assert_eq!((follow.day, follow.pulse), (1, Pulse::Afternoon));
    let kept = played.kept(2, 1).expect("kept");
    assert_eq!((kept.contact, kept.lost), (2, false));

    // 26. With no Japanese force (game D with task force 1 taken out): at
    // dusk of day 2 nothing qualifies and the order is lost, its centre
    // (5,0). The staff withdraws the strike task and keeps the search task.
    let played = play_file(d, |t| without_force(&t, 1));
    let (kind, follow) = played.the_follow(2);
    assert_eq!(kind, NoticeKind::ContactLost);
    assert_eq!(follow.hex, hex(5, 0));
    // The side no longer holds contact 1: the notice says what the order
    // kept of its last report, the afternoon of day 0 (ruling 6).
    assert_eq!((follow.day, follow.pulse), (0, Pulse::Afternoon));
    assert_eq!((follow.ship_count, follow.kind), (2, ShipKind::Cruiser));
    let kept = played.kept(2, 1).expect("kept");
    assert_eq!((kept.hex, kept.lost), (hex(5, 0), true));
    assert_eq!(played.command_kinds(2), vec![withdraw(2)]);
    assert!(!played.in_force(2, 2), "the strike task is withdrawn");
    assert!(played.in_force(2, 3), "the search task stands");
    assert_eq!(played.states[2].intents[&intent(3)].kind, search_from(1));
}

#[test]
fn e_a_kind_that_changes_is_no_notice_and_no_stop() {
    use serde_json::json;
    let e = "133_e_a_kind_that_changes.toml";
    let played = play_file(e, same);

    // 27. At dusk of day 3 contact 1 is 1 pulse old, 2 ships, destroyers. The
    // order follows it: one notice, follows, the number unchanged, the
    // morning of day 2. No notice says the kind changed. What the order keeps
    // of the sort is unchanged: not a carrier.
    let held_at_dusk_3 = &played.held[2];
    assert_eq!(
        held_at_dusk_3,
        &vec![(1, hex(8, 0), ShipKind::Destroyer, 2, 1)],
        "contact 1 is 1 pulse old, 2 ships, destroyers"
    );
    let (kind, follow) = played.the_follow(3);
    assert_eq!(kind, NoticeKind::Follows);
    assert_eq!(follow.contact, 1);
    assert_eq!(follow.taken_from, None);
    assert!(!follow.other_sort);
    assert_eq!((follow.day, follow.pulse), (2, Pulse::Morning));
    assert_eq!((follow.ship_count, follow.kind), (2, ShipKind::Destroyer));
    assert_eq!(played.kinds(3), vec![NoticeKind::Follows]);
    let kept = played.kept(3, 1).expect("kept");
    assert!(!kept.carrier);
    assert_eq!(kept.contact, 1);
    assert_eq!((kept.ship_count, kept.kind), (2, ShipKind::Destroyer));

    // The paper says the force is taken for destroyers, and not a carrier.
    let mut session = session_of(e, &text_of(e), &figures());
    for _ in 0..2 {
        ask(&mut session, "end_day", json!({}));
    }
    let paper = paper_of(&mut session);
    assert_eq!(paper.contact, 1);
    assert_eq!((paper.ships, paper.taken_for.as_str()), (2, "destroyer"));
    assert_eq!(paper.age, 1);
    assert!(paper.takings.is_empty());

    // 28. A call of run for 3 days with all three stops of this rule asked
    // for runs to the end without stopping.
    let text = text_of(e);
    let mut session = session_of(e, &text, &figures());
    let run = ask(
        &mut session,
        "run",
        json!({
            "days": 3,
            "stop_on": ["follows_other_sort", "contact_lost", "force_not_found"]
        }),
    );
    assert_eq!(run["days_resolved"]["count"], json!(3), "{run}");
    assert_ne!(run["stopped"], json!("told"), "{run}");
    assert!(
        run.get("told")
            .is_none_or(|t| t.as_array().is_some_and(Vec::is_empty)),
        "{run}"
    );
    assert!(session.is_over());
}

#[test]
fn g_the_station_chosen_and_chosen_again() {
    let g = "133_g_a_carrier_force.toml";
    let played = play_file(g, same);

    // 29. The station. On day 1 the carrier pass forms intent 2, operate task
    // force 1 from (5,0), under intent 1; then the deny's doctrine forms
    // intent 3, search the waters from base 1. No strike task is formed (base
    // 1 has no bomber) and no stand task. The force is routed to (5,0) at
    // cruise. Group 3 is set to the naval strike mission.
    let formed: Vec<CommandKind> = played
        .command_kinds(1)
        .into_iter()
        .filter(|k| matches!(k, CommandKind::FormIntent { .. }))
        .collect();
    assert_eq!(
        formed,
        vec![form(1, operate(1, hex(5, 0))), form(1, search_from(1))]
    );
    let day_1 = &played.states[1];
    assert_eq!(day_1.intents[&intent(2)].kind, operate(1, hex(5, 0)));
    assert_eq!(day_1.intents[&intent(2)].parent, Some(intent(1)));
    assert_eq!(day_1.intents[&intent(3)].kind, search_from(1));
    assert!(day_1.intents.values().all(|i| !matches!(
        i.kind,
        IntentKind::StrikeShips { .. } | IntentKind::StandAt { .. }
    )));
    let route = played.orders_to_force(1, 1);
    assert!(
        route.iter().any(|o| is_route_to(o, &[hex(5, 0)])),
        "{route:?}"
    );
    assert!(!route.iter().any(is_full_speed), "at cruise: {route:?}");
    assert!(played
        .orders_to_air(1, 3)
        .contains(&AirGroupOrder::SetAirMission(AirMission::NavalStrike)));

    // 30. Chosen again. At dusk of day 2 the notice follows: contact 2 for
    // contact 1. The staff withdraws intent 2 and forms intent 4, operate task
    // force 1 from (9,0), under intent 1, in the same hand-in. Intent 3
    // stands. Its reason gives the station's 8 hexes from the centre against
    // the 4 a station may be and the 3 allowed beyond.
    let (kind, follow) = played.the_follow(2);
    assert_eq!(kind, NoticeKind::Follows);
    assert_eq!((follow.contact, follow.taken_from), (2, Some(1)));
    assert_eq!(follow.hex, hex(13, 0));
    let commands = played.command_kinds(2);
    assert_eq!(commands, vec![withdraw(2), form(1, operate(1, hex(9, 0)))]);
    assert!(played.in_force(2, 3), "intent 3 stands");
    assert_eq!(
        played.states[2].intents[&intent(4)].kind,
        operate(1, hex(9, 0))
    );
    assert_eq!(played.states[2].intents[&intent(4)].parent, Some(intent(1)));
    let why = said(&played.command_of(2, &withdraw(2)).reason);
    assert!(why.contains("8 hexes from the centre"), "{why}");
    assert!(why.contains("the 4 hexes a station may be"), "{why}");
    assert!(why.contains("the 3 allowed beyond"), "{why}");

    // 34. Not rechosen within the figure (game G with task force 2 at (12,0)).
    // In the morning of day 1 the cruisers are reported 3 hexes from (9,0), a
    // new contact, 280 nm from the carrier. At dusk of day 2 it is taken and
    // the centre is (12,0), 7 hexes from the station (5,0): not more than 4 +
    // 3, so intent 2 stands with its station. Contact 2 is 280 nm from the
    // force: it closes, to (6,0), 240 nm from (12,0).
    let played = play_file(g, |t| {
        force_moved(&t, cruiser_force_name(), (13, 0), (12, 0))
    });
    assert_eq!(hex(12, 0).distance(hex(9, 0)), 3);
    assert_eq!(hex(12, 0).distance(hex(5, 0)), 7);
    let (kind, follow) = played.the_follow(2);
    assert_eq!(kind, NoticeKind::Follows);
    assert_eq!((follow.contact, follow.taken_from), (2, Some(1)));
    assert_eq!(follow.hex, hex(12, 0));
    assert!(played.in_force(2, 2), "intent 2 stands with its station");
    assert!(played.command_kinds(2).iter().all(|k| !matches!(
        k,
        CommandKind::FormIntent {
            kind: IntentKind::OperateCarriers { .. },
            ..
        } | CommandKind::WithdrawIntent { .. }
    )));
    let close = played.orders_to_force(2, 1);
    assert!(
        close.iter().any(|o| is_route_to(o, &[hex(6, 0)])),
        "{close:?}"
    );
    assert_eq!(hex(6, 0).air_distance_nm(hex(12, 0)), 240);
    assert_eq!(hex(5, 0).air_distance_nm(hex(12, 0)), 280);
}

#[test]
fn g_a_carrier_force_closes_on_cruisers_at_balanced_and_strikes() {
    let g = "133_g_a_carrier_force.toml";
    let played = play_file(g, same);

    // 31. It closes on cruisers at balanced. In that hand-in (dusk of day 2)
    // the force is given a route to (7,0), not to (9,0), at cruise. The
    // reason says it closes on contact 2, taken for cruisers, because it lies
    // in the waters the order to destroy puts first.
    let orders = played.orders_to_force(2, 1);
    assert!(
        orders.iter().any(|o| is_route_to(o, &[hex(7, 0)])),
        "{orders:?}"
    );
    assert!(
        !orders.iter().any(|o| is_route_to(o, &[hex(9, 0)])),
        "{orders:?}"
    );
    assert!(!orders.iter().any(is_full_speed), "at cruise: {orders:?}");
    let route = played
        .decisions(2)
        .orders
        .iter()
        .find(|o| {
            matches!(&o.kind, OrderKind::TaskForce { order, .. } if is_route_to(order, &[hex(7, 0)]))
        })
        .expect("the route");
    let why = said(&route.reason);
    assert!(why.contains("contact 2, a cruiser"), "{why}");
    assert!(why.contains("closes on"), "{why}");
    assert!(
        why.contains("in the waters the order to destroy a force, intent:1, puts first"),
        "{why}"
    );

    // 32. It strikes. In the morning of day 2 the cruisers are reported again
    // at (13,0), 240 nm from the force at (7,0), and group 3 launches its 10
    // at (13,0). Nothing of what the strike did is stated.
    assert!(
        played.launches().iter().all(|e| e.day >= 2),
        "nothing is launched on day 1"
    );
    let first = played
        .launches()
        .into_iter()
        .find(|e| e.day == 2 && e.pulse == Some(Pulse::Morning))
        .expect("a launch in the morning of day 2");
    match &first.kind {
        EventKind::StrikeLaunched {
            air_group,
            hex: at,
            aircraft,
            distance_nm,
            ..
        } => {
            assert_eq!(air_group.raw(), 3);
            assert_eq!(*at, hex(13, 0));
            assert_eq!(*aircraft, 10);
            assert_eq!(*distance_nm, n(240));
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn g_a_surface_force_named_is_not_sent() {
    let g = "133_g_a_carrier_force.toml";

    // 33. A surface force named (game G with Allied task force 3, two
    // cruisers, stopped at (1,0), and a forces limit on intent 1 that names
    // task force 3). No stand task is formed for it on any day and it is
    // given no order; the reason says a surface force is not sent after a
    // force on the move. Task force 1 is outside the limit and is given no
    // task.
    let with_force_3 = |t: String| {
        let t = format!(
            "{t}\n[[task_force]]\nid = 3\nname = \"Placeholder cruiser force 3\"\nside = \"allied\"\nq = 1\nr = 0\nmission = \"surface_combat\"\n\n[[ship]]\nid = 4\nname = \"Placeholder cruiser 4\"\nclass = 1\nside = \"allied\"\nfuel_t = 2000\ntask_force = 3\n\n[[ship]]\nid = 5\nname = \"Placeholder cruiser 5\"\nclass = 1\nside = \"allied\"\nfuel_t = 2000\ntask_force = 3\n"
        );
        format!(
            "{t}{}",
            command_table(
                1,
                "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\ntask_forces = [3]"
            )
        )
    };
    let played = play_file(g, with_force_3);
    for day in 1..=2 {
        let formed: Vec<CommandKind> = played
            .command_kinds(day)
            .into_iter()
            .filter(|k| matches!(k, CommandKind::FormIntent { .. }))
            .collect();
        assert!(
            formed.iter().all(|k| !matches!(
                k,
                CommandKind::FormIntent {
                    kind: IntentKind::StandAt { .. } | IntentKind::OperateCarriers { .. },
                    ..
                }
            )),
            "day {day}: {formed:?}"
        );
        assert!(
            played.orders_to_force(day, 3).is_empty(),
            "day {day}: task force 3 is given no order"
        );
        assert!(
            played.orders_to_force(day, 1).is_empty(),
            "day {day}: task force 1 is given no task and no order"
        );
        assert!(played.states[day as usize]
            .intents
            .values()
            .all(|i| !matches!(
                i.kind,
                IntentKind::StandAt { .. } | IntentKind::OperateCarriers { .. }
            )));
    }
    let said_so = played.decisions(1).no_orders.iter().any(|r| {
        r.worked
            .contains("a surface force is not sent after a force on the move")
    });
    assert!(said_so, "the reason says a surface force is not sent");
}

#[test]
fn p_the_orders_leave_to_pursue_and_what_binds_it() {
    let p = "133_p_leave_to_pursue.toml";
    let nights = figures_with(Some(2));
    let leave = |how: &'static str| {
        move |t: String| {
            format!(
                "{t}{}",
                command_table(
                    1,
                    &format!("kind = \"set_pursuit\"\nintent = 1\nleave = \"{how}\"")
                )
            )
        }
    };
    let pursues = |played: &Played| {
        played
            .notices(1)
            .iter()
            .filter(|n| n.pursuit.is_some())
            .map(|n| n.kind)
            .collect::<Vec<_>>()
    };

    // 35. Go, by the order. The staff forms intent 2, operate task force 1
    // from (16,0), and no task at base 1; it hands in no notice that the
    // order cannot be met. The force is given full speed and a route to
    // (14,0). One notice, pursues: task force 1, contact 1, cruisers, age 0,
    // reported at (20,0), to (14,0), the hours 440 divided by 30 in the
    // game's own arithmetic. Its reason says the leave is the order's own: an
    // order to destroy a force gives leave to pursue.
    let played = play_with(p, same, &nights);
    let formed: Vec<CommandKind> = played
        .command_kinds(1)
        .into_iter()
        .filter(|k| matches!(k, CommandKind::FormIntent { .. }))
        .collect();
    assert_eq!(formed, vec![form(1, operate(1, hex(16, 0)))]);
    assert!(played
        .notices(1)
        .iter()
        .all(|n| n.kind != NoticeKind::CannotMeet));
    let orders = played.orders_to_force(1, 1);
    assert!(orders.iter().any(is_full_speed), "{orders:?}");
    assert!(
        orders.iter().any(|o| is_route_to(o, &[hex(14, 0)])),
        "{orders:?}"
    );
    let notices: Vec<&StaffNotice> = played
        .notices(1)
        .iter()
        .filter(|n| n.pursuit.is_some())
        .collect();
    assert_eq!(notices.len(), 1);
    assert_eq!(played.kinds(1), vec![NoticeKind::Pursues]);
    let pursuit = notices[0].pursuit.expect("a pursuit notice");
    assert_eq!(pursuit.force, TaskForceId::new(1));
    assert_eq!(pursuit.intent, intent(2));
    let find = pursuit.find.expect("a find");
    assert_eq!(find.contact, 1);
    assert_eq!(find.kind, ShipKind::Cruiser);
    assert_eq!(find.age, 0);
    assert_eq!(find.reported_at, hex(20, 0));
    assert_eq!(find.to, hex(14, 0));
    assert_eq!(find.hours, Some(n(440) / n(30)));
    let why = said(played.notice_reason(1, NoticeKind::Pursues));
    assert!(why.contains("the leave is the order's own"), "{why}");
    assert!(why.contains("an order to destroy a force"), "{why}");

    // 36. Ask (with set_pursuit ask on intent 1): the notice is
    // asks_leave_to_pursue and the force is routed to its station, (16,0),
    // not to (14,0).
    let played = play_with(p, leave("ask"), &nights);
    assert_eq!(played.kinds(1), vec![NoticeKind::AsksLeaveToPursue]);
    let orders = played.orders_to_force(1, 1);
    assert!(
        orders.iter().any(|o| is_route_to(o, &[hex(16, 0)])),
        "{orders:?}"
    );
    assert!(
        !orders.iter().any(|o| is_route_to(o, &[hex(14, 0)])),
        "{orders:?}"
    );

    // 37. Stay (with set_pursuit stay): no notice of a pursuit; the force is
    // routed to (16,0).
    let played = play_with(p, leave("stay"), &nights);
    assert!(pursues(&played).is_empty());
    let orders = played.orders_to_force(1, 1);
    assert!(
        orders.iter().any(|o| is_route_to(o, &[hex(16, 0)])),
        "{orders:?}"
    );

    // 38. An area limit binds (with an area limit on intent 1 of the hexes
    // within 10 of (3,0), which does not hold (20,0)): no find. No notice of
    // a pursuit and no route to (14,0).
    // (The map of the games is deep sea from q -4 to 24 and r -8 to 8: the
    // area names the hexes within 10 of (3,0) that are on it.)
    let mut within_10: Vec<String> = Vec::new();
    for q in -4..=24 {
        for r in -8..=8 {
            if hex(q, r).distance(hex(3, 0)) <= 10 {
                within_10.push(format!("[{q}, {r}]"));
            }
        }
    }
    assert!(hex(20, 0).distance(hex(3, 0)) > 10);
    let area = format!(
        "kind = \"set_limit\"\nlimit = \"area\"\nintent = 1\nhexes = [{}]",
        within_10.join(", ")
    );
    let played = play_with(
        p,
        move |t| format!("{t}{}", command_table(1, &area)),
        &nights,
    );
    assert!(pursues(&played).is_empty());
    let orders = played.orders_to_force(1, 1);
    assert!(
        !orders.iter().any(|o| is_route_to(o, &[hex(14, 0)])),
        "{orders:?}"
    );

    // 39. Search only (with search_only = true): no carrier task is formed,
    // the force is given no order, and the staff hands in that the order
    // cannot be met, as a deny nothing can serve does.
    let played = play_with(p, shadow, &nights);
    assert!(played.states[1]
        .intents
        .values()
        .all(|i| !matches!(i.kind, IntentKind::OperateCarriers { .. })));
    assert!(played.orders_to_force(1, 1).is_empty());
    assert!(played.kinds(1).contains(&NoticeKind::CannotMeet));
}

/// Game S with command levels on and the report figures of this scenario.
fn levels_on(text: String) -> String {
    let text = shadow(text);
    let text = changed(&text, "days = 7\n", "days = 7\ncommand_levels = true\n");
    changed(
        &text,
        "air_combat = \"../../reference/air_combat.toml\"\n",
        "air_combat = \"../../reference/air_combat.toml\"\nreports = \"../../reference/reports_placeholder_scenario_133.toml\"\n",
    )
}

#[test]
fn command_levels_on_the_order_reads_the_areas_picture() {
    let played = play_file("133_a_followed.toml", levels_on);

    // 40. At dusk of day 3 the area's picture holds contact 1, 3 pulses old,
    // and contact 2 at (14,0), 1 pulse old, of the morning of day 2; the
    // afternoon's report has not arrived.
    assert_eq!(
        played.held[2],
        vec![
            (1, hex(8, 0), ShipKind::Cruiser, 2, 3),
            (2, hex(14, 0), ShipKind::Cruiser, 2, 1),
        ]
    );
    // 41. The notices fall on the days of game S.
    let handed: Vec<(u32, Vec<NoticeKind>)> = (1..=7).map(|d| (d, played.kinds(d))).collect();
    assert_eq!(
        handed,
        vec![
            (1, vec![]),
            (2, vec![NoticeKind::Follows]),
            (3, vec![NoticeKind::Follows]),
            (4, vec![NoticeKind::Follows]),
            (5, vec![NoticeKind::Follows]),
            (6, vec![NoticeKind::ContactLost]),
            (7, vec![NoticeKind::ForceNotFound]),
        ]
    );
}

#[test]
fn command_levels_on_the_order_keeps_the_report_the_area_heard() {
    let played = play_file("133_a_followed.toml", levels_on);
    // The order takes contact 2 and keeps (14,0), the morning of day 2, where
    // game S keeps the afternoon of day 2.
    let (kind, follow) = played.the_follow(3);
    assert_eq!(kind, NoticeKind::Follows);
    assert_eq!((follow.contact, follow.taken_from), (2, Some(1)));
    assert_eq!(follow.hex, hex(14, 0));
    assert_eq!((follow.day, follow.pulse), (2, Pulse::Morning));
    assert_eq!(
        played.kept(3, 1),
        Some(Followed {
            contact: 2,
            hex: hex(14, 0),
            day: 2,
            pulse: Pulse::Morning,
            ship_count: 2,
            kind: ShipKind::Cruiser,
            carrier: false,
            lost: false,
        })
    );
    let game_s = play_file("133_a_followed.toml", shadow);
    assert_eq!(
        game_s.kept(3, 1).map(|k| (k.day, k.pulse)),
        Some((2, Pulse::Afternoon))
    );
    // The distance: from the start of the afternoon of day 1 to the end of
    // the morning of day 2, 24 hours, 12 hexes against 6.
    assert_eq!(
        sim_core::command::report_distance_hexes(1, Pulse::Afternoon, 2, Pulse::Morning, 20),
        12
    );
    assert_eq!(hex(14, 0).distance(hex(8, 0)), 6);
}

#[test]
fn a_seat_below_the_area_cannot_give_the_order() {
    use serde_json::json;
    // 42. A seat at base 1 that calls set_intent with the kind destroy a
    // force is answered as it is for every kind: stating an order from a seat
    // that is not the area command's is not built. Nothing is queued and no
    // order is made.
    let file = "133_a_followed.toml";
    let text = changed(
        &inside_a_day(without_order(text_of(file))),
        "days = 7\n",
        "days = 7\ncommand_levels = true\n",
    );
    let mut session = session_of(file, &text, &figures())
        .with_post(
            Side::Allied,
            sim_agent::Post::At(sim_core::pictures::CommandRef::Base(
                sim_core::ids::BaseId::new(1),
            )),
        )
        .expect("a seat at base 1");
    let args = json!({"intent_kind": "destroy_force", "contact": 1, "distance": 2});
    let answer = session
        .call(Side::Allied, "set_intent", &args)
        .expect_err("a seat at a base is refused");
    assert!(
        answer.message.contains("is not built"),
        "{}",
        answer.message
    );
    // The same answer as for another kind of intent.
    let other = json!({"intent_kind": "deny", "centre": [8, 0], "distance": 2});
    let other = session
        .call(Side::Allied, "set_intent", &other)
        .expect_err("refused for every kind");
    assert_eq!(answer.message, other.message);
    assert!(
        session
            .into_world()
            .command_state(Side::Allied)
            .intents
            .is_empty(),
        "no order is made"
    );
}

// ------------------------------------------------ Properties: sessions

/// The order's command of game A, as the file gives it on day 1.
const ORDER_COMMAND: &str = "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_intent\"\nintent = 1\nintent_kind = \"destroy_force\"\ncontact = 1\ndistance = 2\n\n";

/// Game A with the order's command taken out: the leaves stay.
fn without_order(text: String) -> String {
    changed(&text, ORDER_COMMAND, "")
}

/// A scenario's text that lets a seat stop the clock inside a day.
fn inside_a_day(text: String) -> String {
    changed(
        &text,
        "days = 7\n",
        "days = 7\norders_inside_a_day = true\n",
    )
}

/// A session over game A on which both seats are played: the Allied seat
/// and the Japanese seat, the doctrine's staffs asked for both, and no order
/// given.
fn both_seats(text: &str) -> sim_agent::Session {
    let file = "133_a_followed.toml";
    let scenario: Scenario =
        common::from_text_with(text, &path_of(file), &figures()).expect("the scenario loads");
    let title = scenario.name.clone();
    sim_agent::Session::new(
        scenario.world,
        &title,
        scenario.days,
        sim_agent::Seat::Player,
        sim_agent::Seat::Player,
    )
    .with_staffs(staffs())
}

fn says_not_held(answer: &serde_json::Value) {
    assert_eq!(answer["status"], serde_json::json!("refused"), "{answer}");
    let reason = answer["reason"].as_str().expect("a reason");
    assert!(reason.contains("not held"), "{reason}");
    assert_eq!(answer["queued"], serde_json::json!(0), "{answer}");
}

#[test]
fn refusals_and_a_stale_contact_named() {
    use serde_json::json;
    let file = "133_a_followed.toml";
    let none_given = |text: String| inside_a_day(without_order(text));

    // 43. Refused: set_intent with the kind destroy a force and a number the
    // commander's picture does not hold is refused with the reason that the
    // contact is not held, at dusk and inside a day; so is one with no
    // contact.
    let mut session = session_of(file, &none_given(text_of(file)), &figures());
    let at_dusk = [
        json!({"intent_kind": "destroy_force", "contact": 9, "distance": 2}),
        json!({"intent_kind": "destroy_force", "distance": 2}),
    ];
    for args in &at_dusk {
        says_not_held(&ask(&mut session, "set_intent", args.clone()));
    }
    // Inside a day: at 0600 of day 1.
    ask(&mut session, "advance", json!({"pulses": 1}));
    for args in &at_dusk {
        says_not_held(&ask(&mut session, "set_intent", args.clone()));
    }
    // A contact held is accepted where the same call names it (the control).
    let held = ask(
        &mut session,
        "set_intent",
        json!({"intent_kind": "destroy_force", "contact": 1, "distance": 2}),
    );
    assert_eq!(held["status"], json!("queued"), "{held}");

    // The Japanese seat cannot name an Allied contact: it holds none.
    let mut both = both_seats(&none_given(text_of(file)));
    let refused = both
        .call(
            Side::Japanese,
            "set_intent",
            &json!({"intent_kind": "destroy_force", "contact": 1, "distance": 2}),
        )
        .expect("an answer");
    says_not_held(&refused);

    // 44. A contact held and not fresh may be named. In game A the commander
    // withdraws intent 1 at dusk of day 3 and at the same dusk gives a new
    // order naming contact 1, then 4 pulses old. It is accepted and keeps
    // contact 1, (8,0), the morning of day 1 (age 4: five pulses before the
    // dusk). At that dusk it takes contact 2 by rule 7: 30 hours, 15 hexes
    // against 6.
    let mut session = session_of(file, &text_of(file), &figures());
    for _ in 0..2 {
        ask(&mut session, "end_day", json!({}));
    }
    let before = paper_of(&mut session);
    assert_eq!((before.contact, before.age), (1, 4));
    ask(&mut session, "withdraw_intent", json!({"intent": 1}));
    let given = ask(
        &mut session,
        "set_intent",
        json!({"intent_kind": "destroy_force", "contact": 1, "distance": 2}),
    );
    assert_eq!(given["status"], json!("queued"), "{given}");
    let number = given["command"]["intent"].as_u64().expect("its number");
    assert!(number > 3, "the next number of the commander's: {given}");
    let state = ask(&mut session, "command_state", json!({}));
    let new_order = state["intents"]
        .as_array()
        .expect("the intents")
        .iter()
        .find(|i| i["number"] == json!(number))
        .unwrap_or_else(|| panic!("order {number} in {state}"));
    let paper = &new_order["following"];
    assert_eq!(paper["contact"], json!(1), "{paper}");
    assert_eq!(paper["hex"], json!([8, 0]), "{paper}");
    assert_eq!(paper["age_pulses"], json!(4), "{paper}");
    assert_eq!(
        paper["reported"],
        json!({"day": 1, "pulse": "morning"}),
        "{paper}"
    );
    ask(&mut session, "end_day", json!({}));
    let handed = session.staff_handed_in()[&3][&Side::Allied].clone();
    let follows: Vec<FollowNotice> = handed.notices.iter().filter_map(|n| n.follow).collect();
    assert_eq!(follows.len(), 1, "{handed:?}");
    assert_eq!(follows[0].intent, intent(u32::try_from(number).unwrap()));
    assert_eq!((follows[0].contact, follows[0].taken_from), (2, Some(1)));
    assert_eq!(follows[0].hex, hex(14, 0));
    assert_eq!(
        sim_core::command::report_distance_hexes(1, Pulse::Morning, 2, Pulse::Morning, 20),
        15
    );
    assert_eq!(hex(14, 0).distance(hex(8, 0)), 6);
    let world = session.into_world();
    let kept = world.command_state(Side::Allied).followed[&intent(u32::try_from(number).unwrap())];
    assert_eq!((kept.contact, kept.hex), (2, hex(14, 0)));
    assert_eq!((kept.day, kept.pulse), (2, Pulse::Morning));
}

/// What a call of `run` or `advance` said of where it stopped.
fn stopped_on(answer: &serde_json::Value) -> (String, Option<String>) {
    (
        answer["stopped"].as_str().expect("stopped").to_string(),
        answer["told"][0]["thing"].as_str().map(str::to_string),
    )
}

#[test]
fn the_stops_are_keys_of_the_calls_of_time() {
    use serde_json::json;
    let a = "133_a_followed.toml";
    let b = "133_b_which_contact.toml";

    // 45. The stops are keys of the calls of time, off unless asked for.
    // `run` asked for the stop "lost" in game A stops at dusk of day 7, the
    // dusk after the day the notice was handed in (day 6), and names the
    // notice.
    let mut session = session_of(a, &text_of(a), &figures());
    let run = ask(
        &mut session,
        "run",
        json!({"days": 7, "stop_on": ["contact_lost"]}),
    );
    assert_eq!(
        stopped_on(&run),
        ("told".to_string(), Some("contact_lost".to_string())),
        "{run}"
    );
    assert_eq!(run["days_resolved"]["last"], json!(6), "{run}");
    assert_eq!(run["fired"], json!([]), "no interrupt fires: {run}");
    assert!(
        run["told"][0]["words"][0]
            .as_str()
            .expect("the notice's words")
            .contains("lost"),
        "{run}"
    );
    assert!(!session.is_over());

    // `advance` asked for it stops at 0600 of day 6.
    let mut session = session_of(a, &inside_a_day(text_of(a)), &figures());
    let adv = ask(
        &mut session,
        "advance",
        json!({"pulses": 60, "stop_on": ["contact_lost"]}),
    );
    assert_eq!(
        stopped_on(&adv),
        ("told".to_string(), Some("contact_lost".to_string())),
        "{adv}"
    );
    assert_eq!(
        (adv["clock"]["day"].as_u64(), adv["clock"]["at"].as_str()),
        (Some(6), Some("0600")),
        "{adv}"
    );
    assert_eq!(adv["fired"], json!([]), "{adv}");

    // Asked for the stop "ended" they stop at the end of the game and at
    // 0600 of day 7.
    let mut session = session_of(a, &text_of(a), &figures());
    let run = ask(
        &mut session,
        "run",
        json!({"days": 7, "stop_on": ["force_not_found"]}),
    );
    assert_eq!(
        stopped_on(&run),
        ("game_over".to_string(), Some("force_not_found".to_string())),
        "{run}"
    );
    assert_eq!(run["days_resolved"]["last"], json!(7), "{run}");
    assert!(session.is_over(), "the end of the game");
    assert_eq!(run["fired"], json!([]), "{run}");
    let mut session = session_of(a, &inside_a_day(text_of(a)), &figures());
    let adv = ask(
        &mut session,
        "advance",
        json!({"pulses": 60, "stop_on": ["force_not_found"]}),
    );
    assert_eq!(
        stopped_on(&adv),
        ("told".to_string(), Some("force_not_found".to_string())),
        "{adv}"
    );
    assert_eq!(
        (adv["clock"]["day"].as_u64(), adv["clock"]["at"].as_str()),
        (Some(7), Some("0600")),
        "{adv}"
    );

    // `run` asked for the stop "taken, of the other sort" stops after day 2
    // in the game of statement 17 and does not stop in game A.
    let other = |t: String| without_force(&t, 2);
    let mut session = session_of(b, &other(text_of(b)), &figures());
    let run = ask(
        &mut session,
        "run",
        json!({"days": 3, "stop_on": ["follows_other_sort"]}),
    );
    assert_eq!(
        stopped_on(&run),
        ("told".to_string(), Some("follows_other_sort".to_string())),
        "{run}"
    );
    assert_eq!(run["days_resolved"]["last"], json!(2), "{run}");
    assert_eq!(run["fired"], json!([]), "{run}");
    let mut session = session_of(a, &text_of(a), &figures());
    let run = ask(
        &mut session,
        "run",
        json!({"days": 7, "stop_on": ["follows_other_sort"]}),
    );
    assert_ne!(run["stopped"], json!("told"), "{run}");
    assert_eq!(run["days_resolved"]["last"], json!(7), "{run}");
    assert!(session.is_over());

    // Off unless asked for: with no key all seven days are played through,
    // and none stops before a day is played.
    let mut session = session_of(a, &text_of(a), &figures());
    let run = ask(&mut session, "run", json!({"days": 7}));
    assert_ne!(run["stopped"], json!("told"), "{run}");
    assert_eq!(run["days_resolved"]["last"], json!(7), "{run}");
    assert_eq!(run["fired"], json!([]), "{run}");
}

#[test]
fn withdrawn_by_the_commander() {
    use serde_json::json;
    let file = "133_a_followed.toml";

    // 46. Withdrawn. In game A the commander withdraws intent 1 at dusk of
    // day 4: everything beneath it is withdrawn with it, nothing of the order
    // is kept in the command state, and no notice of this rule is handed in
    // after.
    let mut session = session_of(file, &text_of(file), &figures());
    for _ in 0..3 {
        ask(&mut session, "end_day", json!({}));
    }
    ask(&mut session, "withdraw_intent", json!({"intent": 1}));
    for _ in 4..=7 {
        ask(&mut session, "end_day", json!({}));
    }
    assert!(session.is_over());
    let handed = session.staff_handed_in().clone();
    for day in 4..=7 {
        let notices = handed
            .get(&day)
            .and_then(|by_side| by_side.get(&Side::Allied))
            .map(|d| d.notices.clone())
            .unwrap_or_default();
        assert!(
            notices.iter().all(|n| n.follow.is_none()),
            "day {day}: {notices:?}"
        );
    }
    let world = session.into_world();
    let state = world.command_state(Side::Allied);
    assert!(state.followed.is_empty(), "nothing of the order is kept");
    assert!(
        state.intents.values().all(|i| !i.in_force()),
        "everything beneath it is withdrawn with it: {:?}",
        state.intents
    );
}

/// A game played to its end with the doctrine's staffs: the world it leaves
/// and the hash at the end of each day.
fn finished(file: &str, edit: impl FnOnce(String) -> String) -> (sim_core::world::World, Vec<u64>) {
    let text = edit(text_of(file));
    let mut scenario: Scenario =
        common::from_text_with(&text, &path_of(file), &figures()).expect("the scenario loads");
    let held = staffs();
    let mut hashes = Vec::new();
    while let Some(summary) = scenario
        .next_day_watched(as_staffs(&held), &mut |_, _| {})
        .expect("a day resolves")
    {
        hashes.push(summary.state_hash);
    }
    (scenario.world, hashes)
}

/// What a side was given of the log, without the umpire's numbers: its own
/// count of each event, the day, the pulse and what it says.
fn seen_by(world: &sim_core::world::World, side: Side) -> Vec<String> {
    world
        .side_view(side)
        .events()
        .iter()
        .map(|e| format!("{} {} {:?} {:?}", e.number, e.day, e.pulse, e.kind))
        .collect()
}

#[test]
fn a_side_sees_its_own_and_nothing_it_cannot_see_changes_it() {
    use serde_json::json;
    let file = "133_a_followed.toml";

    // 47. A side sees its own. Each notice of this rule is a `StaffNotice`
    // the Allied side sees whole and the Japanese side does not see.
    let (world, _) = finished(file, same);
    let whole = |side: Side| -> Vec<NoticeKind> {
        world
            .side_view(side)
            .events()
            .iter()
            .filter_map(|e| match &e.kind {
                sim_core::side_events::SideEventKind::Whole(EventKind::StaffNotice {
                    kind,
                    follow: Some(_),
                    ..
                }) => Some(*kind),
                _ => None,
            })
            .collect()
    };
    assert_eq!(
        whole(Side::Allied),
        vec![
            NoticeKind::Follows,
            NoticeKind::Follows,
            NoticeKind::Follows,
            NoticeKind::Follows,
            NoticeKind::ContactLost,
            NoticeKind::ForceNotFound,
        ]
    );
    assert!(world
        .side_view(Side::Japanese)
        .events()
        .iter()
        .all(|e| !matches!(
            &e.kind,
            sim_core::side_events::SideEventKind::Whole(EventKind::StaffNotice { .. })
        )));
    // No event the Japanese side is given differs between game A and game A
    // with no order given.
    let (none_given, _) = finished(file, without_order);
    assert_eq!(
        seen_by(&world, Side::Japanese),
        seen_by(&none_given, Side::Japanese)
    );
    // No answer to its seat differs, at any dusk, from the same game with no
    // order given: the tools that only read, asked of the Japanese seat.
    let reads = [
        "get_briefing",
        "list_forces",
        "list_contacts",
        "list_bases",
        "events",
        "intelligence",
        "command_state",
        "plans",
        "staff_orders",
        "report",
        "queued",
    ];
    let answers = |text: String| -> Vec<String> {
        let mut session = session_of(file, &text, &figures());
        let mut all = Vec::new();
        for _ in 1..=7 {
            for tool in reads {
                let answer = match session.call(Side::Japanese, tool, &json!({})) {
                    Ok(answer) => answer.to_string(),
                    Err(e) => e.message,
                };
                all.push(format!("{tool}: {answer}"));
            }
            ask(&mut session, "end_day", json!({}));
        }
        all
    };
    let with_order = answers(text_of(file));
    let without = answers(without_order(text_of(file)));
    assert_eq!(with_order.len(), without.len());
    for (a, b) in with_order.iter().zip(&without) {
        assert_eq!(a, b);
    }
}

/// Game A with a Japanese task force 2 of two cruisers at `(q, r)`, which no
/// report of the Allied side names.
fn with_unseen_force(text: String, q: i32, r: i32) -> String {
    format!(
        "{text}\n[[task_force]]\nid = 2\nname = \"Placeholder cruiser force 2\"\nside = \"japanese\"\nq = {q}\nr = {r}\nmission = \"surface_combat\"\n\n[[ship]]\nid = 3\nname = \"Placeholder cruiser 3\"\nclass = 1\nside = \"japanese\"\nfuel_t = 2000\ntask_force = 2\n\n[[ship]]\nid = 4\nname = \"Placeholder cruiser 4\"\nclass = 1\nside = \"japanese\"\nfuel_t = 2000\ntask_force = 2\n"
    )
}

#[test]
fn nothing_the_side_cannot_see_changes_what_it_hands_in() {
    let file = "133_a_followed.toml";
    // 48. Nothing it cannot see. Two worlds of game A that differ only in
    // what the Allied side cannot see (the Japanese force's fuel; a Japanese
    // force at (20,5), never reported) give the same orders, commands,
    // notices and kept state on every day.
    let of = |played: &Played| -> Vec<String> {
        (1..=7)
            .map(|day| {
                format!(
                    "day {day}: {:?} | kept {:?}",
                    played.decisions(day),
                    played.states[day as usize].followed
                )
            })
            .collect()
    };
    let plain = of(&play_file(file, same));
    let fuel = of(&play_file(file, |t| {
        t.replace(
            "fuel_t = 2000\ntask_force = 1",
            "fuel_t = 1500\ntask_force = 1",
        )
    }));
    assert_ne!(
        text_of(file).replace(
            "fuel_t = 2000\ntask_force = 1",
            "fuel_t = 1500\ntask_force = 1"
        ),
        text_of(file),
        "the fuel is changed"
    );
    let unseen = of(&play_file(file, |t| with_unseen_force(t, 20, 5)));
    assert_eq!(plain, fuel, "the Japanese force's fuel");
    assert_eq!(plain, unseen, "a force at (20,5), never reported");
}

#[test]
fn a_dry_run_of_a_side_with_the_order() {
    let file = "133_a_followed.toml";
    // The trial's figures are named by the file as scenario 94's are.
    let text = changed(
        &text_of(file),
        "air_combat = \"../../reference/air_combat.toml\"\n",
        "air_combat = \"../../reference/air_combat.toml\"\ntrial = \"../../reference/trial_placeholder_scenario_94.toml\"\n",
    );
    let mut scenario: Scenario =
        common::from_text_with(&text, &path_of(file), &figures()).expect("the scenario loads");
    let held = staffs();
    for _ in 1..=3 {
        scenario
            .next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day resolves")
            .expect("the day exists");
    }
    // At dusk of day 4: the live order keeps contact 2, (14,0), the morning
    // of day 3, as the live game's second dusk of taking has it.
    let live_kept = scenario.world.command_state(Side::Allied).followed[&intent(1)];
    assert_eq!((live_kept.contact, live_kept.hex), (2, hex(14, 0)));
    let figures_of_trial = scenario
        .trial_figures()
        .expect("the trial's figures")
        .clone();
    let ref_data = scenario.world.ref_data().clone();
    let map = scenario.world.map().clone();
    let asked = sim_trial::Asked {
        trial: 1,
        worlds: 1,
        days: 1,
        course: Vec::new(),
    };
    let trial = sim_trial::Trial {
        seed: figures_of_trial.seed,
        trial: 1,
        worlds: 1,
    };
    let (made, played) = {
        let view = scenario.world.side_view(Side::Allied);
        let made = sim_trial::pictures(
            &view,
            &sim_trial::Believed::default(),
            &figures_of_trial,
            trial,
        )
        .expect("the pictures are made");
        let played = sim_trial::play(
            &view,
            &sim_trial::Believed::default(),
            &figures_of_trial,
            &ref_data,
            &map,
            asked,
        )
        .expect("the trial is played");
        (made, played)
    };
    assert_eq!(made.len(), 1, "one world");
    assert_eq!(played.worlds.len(), 1);

    // In the made world the order is in force and follows the contact at
    // (14,0), found by its place in the order of the picture's contacts,
    // whatever number the made world gives it; what it keeps is what the live
    // order keeps.
    let start =
        sim_trial::build_world(&made[0], ref_data.clone(), map.clone()).expect("the world builds");
    assert!(start.command_state(Side::Allied).intents[&intent(1)].in_force());
    let kept = start.command_state(Side::Allied).followed[&intent(1)];
    let there = start.side_view(Side::Allied);
    let at_14: Vec<u32> = there
        .contacts()
        .filter(|c| c.estimated_hex == hex(14, 0))
        .map(|c| there.contact_number(c.id))
        .collect();
    assert_eq!(at_14, vec![kept.contact], "found by its place in the order");
    assert_eq!(
        Followed {
            contact: live_kept.contact,
            ..kept
        },
        live_kept
    );

    // The staff of the made world hands in, on its first dusk, the commands
    // and the orders the live staff hands in at dusk of day 4.
    scenario
        .next_day_watched(as_staffs(&held), &mut |_, _| {})
        .expect("day 4 resolves")
        .expect("the day exists");
    let live = scenario.world.staff_handed_in()[&Side::Allied].clone();
    let day = &played.worlds[0].days[0];
    let in_made = &day.staff[&Side::Allied];
    assert_eq!(
        format!(
            "{:?}",
            in_made.commands.iter().map(|c| &c.kind).collect::<Vec<_>>()
        ),
        format!(
            "{:?}",
            live.commands.iter().map(|c| &c.kind).collect::<Vec<_>>()
        )
    );
    assert_eq!(
        format!(
            "{:?}",
            in_made.orders.iter().map(|o| &o.kind).collect::<Vec<_>>()
        ),
        format!(
            "{:?}",
            live.orders.iter().map(|o| &o.kind).collect::<Vec<_>>()
        )
    );
    assert_eq!(
        in_made.notices.iter().map(|n| n.kind).collect::<Vec<_>>(),
        live.notices.iter().map(|n| n.kind).collect::<Vec<_>>()
    );
    assert_eq!(live.notices.len(), 1, "a refreshing: follows");
    // What the made world's order keeps after that dusk is what the live
    // order keeps, the number apart.
    let end = played.worlds[0].world.as_ref().expect("the world as left");
    let made_end = end.command_state(Side::Allied).followed[&intent(1)];
    let live_end = scenario.world.command_state(Side::Allied).followed[&intent(1)];
    assert_eq!(
        Followed {
            contact: live_end.contact,
            ..made_end
        },
        live_end
    );
}

#[test]
fn hashed_named_replayed_and_the_same_twice() {
    let file = "133_a_followed.toml";

    // 50. Hashed. What the order keeps is in the state hash: two worlds of
    // game A that differ only in the kept pulse of the last report hash
    // differently.
    let text = changed(
        &text_of(file),
        "air_combat = \"../../reference/air_combat.toml\"\n",
        "air_combat = \"../../reference/air_combat.toml\"\ntrial = \"../../reference/trial_placeholder_scenario_94.toml\"\n",
    );
    let mut scenario: Scenario =
        common::from_text_with(&text, &path_of(file), &figures()).expect("the scenario loads");
    let held = staffs();
    for _ in 1..=3 {
        scenario
            .next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day resolves")
            .expect("the day exists");
    }
    let figures_of_trial = scenario
        .trial_figures()
        .expect("the trial's figures")
        .clone();
    let ref_data = scenario.world.ref_data().clone();
    let map = scenario.world.map().clone();
    let made = sim_trial::pictures(
        &scenario.world.side_view(Side::Allied),
        &sim_trial::Believed::default(),
        &figures_of_trial,
        sim_trial::Trial {
            seed: figures_of_trial.seed,
            trial: 1,
            worlds: 1,
        },
    )
    .expect("the pictures are made");
    let hash_with = |pulse: Option<Pulse>| {
        let mut picture = made[0].clone();
        if let Some(pulse) = pulse {
            picture
                .picture
                .command
                .get_mut(&Side::Allied)
                .expect("the side's command state")
                .followed
                .get_mut(&intent(1))
                .expect("the order keeps a report")
                .pulse = pulse;
        }
        sim_trial::build_world(&picture, ref_data.clone(), map.clone())
            .expect("the world builds")
            .state_hash()
    };
    let kept = made[0].picture.command[&Side::Allied].followed[&intent(1)];
    assert_eq!(kept.pulse, Pulse::Morning);
    let as_made = hash_with(None);
    assert_eq!(as_made, hash_with(Some(Pulse::Morning)), "the same twice");
    assert_ne!(as_made, hash_with(Some(Pulse::Afternoon)));
    assert_ne!(
        hash_with(Some(Pulse::Night)),
        hash_with(Some(Pulse::Afternoon))
    );

    // Each kept field is named in `hash::name_every_field`.
    let source = std::fs::read_to_string(common::data_dir().join("../crates/sim-core/src/hash.rs"))
        .expect("the hash's source reads");
    let (_, block) = source
        .split_once("crate::command::Followed {")
        .expect("the kept state is named in name_every_field");
    let block = block.split_once('}').expect("the pattern ends").0;
    for field in [
        "contact",
        "hex",
        "day",
        "pulse",
        "ship_count",
        "kind",
        "carrier",
        "lost",
    ] {
        assert!(block.contains(&format!("{field}: _")), "{field} is named");
    }
    assert_eq!(block.matches(": _").count(), 8, "and no field more");

    // A game saved at dusk of day 5 and replayed gives the hash of the game
    // played through.
    let saved_path =
        std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("scenario_133_a.save.json");
    let mut live: Scenario = common::from_text_with(&text_of(file), &path_of(file), &figures())
        .expect("the scenario loads");
    let remembered = sim_cli::save::remember(&live).expect("the scenario is remembered");
    let commands = live.scripted_commands().clone();
    let script = live.scripted_orders().clone();
    let inside = live.scripted_orders_inside_day().clone();
    let mut handed: BTreeMap<u32, BTreeMap<Side, StaffDecisions>> = BTreeMap::new();
    let mut hashes: Vec<(u32, u64)> = Vec::new();
    for day in 1..=4 {
        let summary = live
            .next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day resolves")
            .expect("the day exists");
        handed.insert(day, live.world.staff_handed_in().clone());
        hashes.push((day, summary.state_hash));
    }
    sim_cli::save::write(
        saved_path.to_str().expect("a path"),
        path_of(file).to_str().expect("a path"),
        1942,
        &remembered,
        &script,
        &inside,
        &commands,
        &handed,
        &sim_cli::save::script_seats(),
        &hashes,
        None,
    )
    .expect("the game is saved");
    let saved = sim_cli::save::read(saved_path.to_str().expect("a path")).expect("it reads");
    let mut replay: Scenario = common::from_text_with(&text_of(file), &path_of(file), &figures())
        .expect("the scenario loads");
    replay.replace_script(saved.script.clone());
    replay.replace_script_inside_day(saved.script_inside_day.clone());
    replay.replace_commands(saved.commands.clone());
    assert_eq!(saved.hashes.len(), 4);
    let mut last = 0;
    for (day, hash) in &saved.hashes {
        for (side, decisions) in saved.staff.get(day).into_iter().flatten() {
            replay.world.submit_staff(*side, decisions.clone());
        }
        let summary = replay
            .next_day_watched(sim_core::Staffs::none(), &mut |_, _| {})
            .expect("the day resolves")
            .expect("the day exists");
        assert_eq!(hash, &format!("{:016x}", summary.state_hash), "day {day}");
        last = summary.state_hash;
    }
    // The game played through to dusk of day 5 gives that hash.
    let (_, through) = finished(file, same);
    assert_eq!(last, through[3]);
    assert_eq!(hashes.last().map(|(_, h)| *h), Some(through[3]));

    // 51. The same start gives the same game: game A played twice gives the
    // same hash on every day.
    let (_, again) = finished(file, same);
    assert_eq!(through, again);
    assert_eq!(through.len(), 7);
}

#[test]
fn no_game_on_file_moves_and_the_frozen_figures_are_untouched() {
    use std::fs;
    let root = common::data_dir().join("..");

    // 52. No game on file moves. No scenario on file gives the order: the
    // comparison of every run, live and frozen, with the hashes cut, is made
    // by `scripts/compare-scenarios.sh --against e9944c16`, the commit this is
    // built on, and reported with the pull request; here, that no file the
    // loops walk (or a recorded game) names it, which is what the comparison
    // rests on.
    let names = |dir: &std::path::Path, found: &mut Vec<String>| {
        fn walk(dir: &std::path::Path, found: &mut Vec<String>) {
            for entry in fs::read_dir(dir).expect("a folder reads") {
                let path = entry.expect("an entry").path();
                if path.is_dir() {
                    walk(&path, found);
                } else if let Ok(text) = fs::read_to_string(&path) {
                    if text.contains("destroy_force") {
                        found.push(path.display().to_string());
                    }
                }
            }
        }
        walk(dir, found);
    };
    let mut found = Vec::new();
    names(&root.join("data/scenarios"), &mut found);
    names(
        &root.join("crates/sim-cli/tests/recorded_games"),
        &mut found,
    );
    names(&root.join("data/reference"), &mut found);
    let own = "data/scenarios/a-force-followed/";
    let elsewhere: Vec<&String> = found.iter().filter(|f| !f.contains(own)).collect();
    assert!(
        elsewhere.is_empty(),
        "a game on file gives the order: {elsewhere:?}"
    );
    assert!(!found.is_empty(), "the games of this scenario give it");

    // 53. The frozen figures are untouched. No file of `data/reference/step1`
    // changes and no file of this scenario is copied to
    // `data/scenarios/step2-frozen`; the three figures of the staff are
    // constants of the doctrine.
    for entry in fs::read_dir(root.join("data/scenarios/step2-frozen")).expect("it reads") {
        let name = entry.expect("an entry").file_name();
        assert!(
            !name.to_string_lossy().contains("133"),
            "{name:?} is a copy of this scenario"
        );
    }
    let figures_named = [
        "followed_force_knots",
        "lost_contact_days",
        "station_rechosen_hexes",
    ];
    for dir in ["data/reference", "data/reference/step1", "data/scenarios"] {
        for entry in fs::read_dir(root.join(dir)).expect("it reads") {
            let path = entry.expect("an entry").path();
            // Figures and scenario files only: no test reads a Markdown file.
            if path.extension().is_none_or(|x| x != "toml") {
                continue;
            }
            if let Ok(text) = fs::read_to_string(&path) {
                for figure in figures_named {
                    assert!(!text.contains(figure), "{} names {figure}", path.display());
                }
            }
        }
    }
    // That no frozen file was changed by the build of decision 0220 was shown
    // once, with its pull request, against the commit it was built on. It is
    // not asked again here: a comparison with a past commit fails on every
    // frozen file a later decision rightly adds, reads the history, which a
    // shallow clone has not, and read a Markdown file (the architect's ruling
    // of Nov 24, 2026, in decision 0220). What stands is above: no frozen
    // copy of this scenario, and no figures file that names the order or its
    // three figures.
}
