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
//!    hands in nothing for the order.
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
//!     0600 of day 1, not at dusk). It keeps contact 1, (2,0), the afternoon
//!     of day 0 at once, and acts from dusk of day 2 (decision 0219). No
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
//!     is unchanged: not a carrier.
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
//! 42. **A seat below the area.** A seat at base 1 that gives the order
//!     names a contact by the number of base 1's picture; the order follows
//!     the area's contact of the same hex and kind. Where the area's
//!     picture holds no contact of that hex and kind the command is
//!     refused: the contact is not held.
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
//!
//! If a game cannot be set up as written (a carrier force with no home
//! port, a seat at a base giving an order to the area, an order given at
//! the stop at 0600, a trial made in the middle of a game), the build says
//! so and it comes back to the architect; no statement is changed to fit.

#![allow(clippy::doc_lazy_continuation)]

#[test]
#[ignore = "statement written, not built"]
fn a_given_the_order_keeps_its_contact_and_forms_a_denys_tasks() {}

#[test]
#[ignore = "statement written, not built"]
fn a_a_refreshed_contact_is_followed_with_its_number() {}

#[test]
#[ignore = "statement written, not built"]
fn a_the_force_seen_again_as_a_new_contact_is_taken() {}

#[test]
#[ignore = "statement written, not built"]
fn a_a_base_out_of_reach_two_dusks_running_has_its_task_withdrawn_and_formed_again() {}

#[test]
#[ignore = "statement written, not built"]
fn a_lost_then_ended_not_found() {}

#[test]
#[ignore = "statement written, not built"]
fn a_the_notices_the_paper_and_no_strike() {}

#[test]
#[ignore = "statement written, not built"]
fn s_search_only_forms_no_strike_and_searches_in_both_pulses() {}

#[test]
#[ignore = "statement written, not built"]
fn b_the_sort_decides_then_the_nearest_then_the_number() {}

#[test]
#[ignore = "statement written, not built"]
fn b_at_the_distance_and_beyond_it() {}

#[test]
#[ignore = "statement written, not built"]
fn b_two_orders_and_one_new_contact() {}

#[test]
#[ignore = "statement written, not built"]
fn b_given_inside_a_day_lost_and_found_again() {}

#[test]
#[ignore = "statement written, not built"]
fn d_a_contact_dropped_by_a_strike_that_found_nothing() {}

#[test]
#[ignore = "statement written, not built"]
fn e_a_kind_that_changes_is_no_notice_and_no_stop() {}

#[test]
#[ignore = "statement written, not built"]
fn g_the_station_chosen_and_chosen_again() {}

#[test]
#[ignore = "statement written, not built"]
fn g_a_carrier_force_closes_on_cruisers_at_balanced_and_strikes() {}

#[test]
#[ignore = "statement written, not built"]
fn g_a_surface_force_named_is_not_sent() {}

#[test]
#[ignore = "statement written, not built"]
fn p_the_orders_leave_to_pursue_and_what_binds_it() {}

#[test]
#[ignore = "statement written, not built"]
fn command_levels_on_the_order_reads_the_areas_picture() {}

#[test]
#[ignore = "statement written, not built"]
fn refusals_and_a_stale_contact_named() {}

#[test]
#[ignore = "statement written, not built"]
fn the_stops_are_keys_of_the_calls_of_time() {}

#[test]
#[ignore = "statement written, not built"]
fn withdrawn_by_the_commander() {}

#[test]
#[ignore = "statement written, not built"]
fn a_side_sees_its_own_and_nothing_it_cannot_see_changes_it() {}

#[test]
#[ignore = "statement written, not built"]
fn a_dry_run_of_a_side_with_the_order() {}

#[test]
#[ignore = "statement written, not built"]
fn hashed_named_replayed_and_the_same_twice() {}

#[test]
#[ignore = "statement written, not built"]
fn no_game_on_file_moves_and_the_frozen_figures_are_untouched() {}
