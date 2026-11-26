//! Scenario 137: a plan the commander writes. The commander says "control
//! the island" and writes two of its landings himself: the anchorage
//! first, by the force and the unit he names, and the airfield after it,
//! with what the staff chooses. The staff forms no take beside his, fills
//! what he left blank, and carries both out as it would its own.
//!
//! Proves decision 0232, step 1, against a hand calculation. It stands on
//! scenario 76's for an island controlled and on scenario 81's for a take
//! told to wait. The data is placeholder data: see
//! `data/scenarios/a-plan-the-commander-writes/137_a_plan_the_commander_writes.toml`.
//! It runs on the frozen step 1 figures, with scenario 76's ground figure
//! of its own, `preparation_per_day` 100. No enemy man is ashore anywhere,
//! so no die is rolled in the main game or in any variant but the two of
//! "A force he named that is not a landing's", which are stated of the
//! staff's commands and reasons alone.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! The doctrines that exist are worked from the code as it stands (the
//! take's, the island's, the hold's and the supply's, as scenarios 76 and
//! 81 prove them). What decision 0232 adds is worked from its rules, and
//! "What this assumes of the build", at the end, lists each thing the
//! numbers rest on that is not yet code. Each game below was also played
//! once on a throwaway copy of the code with those rules roughed in, to
//! check the reading of the doctrines that exist. That copy is not the
//! build and nothing of it is kept.
//!
//! ## Figures
//!
//! As scenario 76's. A man weighs 0.25 t to load and to land and eats
//! 0.005 t a day, paid at the start of the night pulse by the side's
//! bases in the hex in base order. Supply landed goes into the side's
//! lowest-numbered base in the hex. Base A's port loads 150 t an hour for
//! each ship: a unit of 600 men is 150 t, an hour. A size 0 port works by
//! lighter at 20 t an hour for each ship: 80 men an hour. Day d begins at
//! hour 24 x (d - 1): a night pulse of 12 hours and two daylight pulses of
//! 6. In a pulse ships move before cargo is worked. A force that has
//! landed its load steams on at the start of the next pulse. A hex is
//! 40 nm; the transports make 10 kn and burn 0.1 t of fuel a mile.
//!
//! The staff's figures: at balanced the margin of the check that a place
//! can be held is 5 days, and the supply that lands with the men is their
//! daily charge for the days until supply follows and the margin; a
//! supply intent with no interval uses 15 days; `preparation_per_day` 100
//! brings a force's loading and a unit's rehearsal to the 100 asked at
//! balanced in one day. A supply intent is first served on the morning
//! after it is formed. A run ordered on a morning loads that day, sails
//! the next and is due that same day; it is ordered when the reading of
//! the day after it is due would be below the floor's stock, which is the
//! reading of the morning less two days of use.
//!
//! ## Inputs
//!
//! The placeholder map of scenario 27: A at (0,0), port size 5; B at
//! (4,0), port size 0, 160 nm from A, 16 hours; C at (0,6), port size 0,
//! 240 nm from A, 24 hours. The game is 16 days.
//!
//! | Base | Side | Hex | Supply |
//! | ---- | ---- | --- | ------ |
//! | 1, "Placeholder base A" | Allied | A | 1,000 t |
//! | 2, "Placeholder airfield B" | Japanese | B | 0 |
//! | 4, "Placeholder anchorage C" | Japanese | C | 0 |
//!
//! Bases 2 and 4 are on the Allied map and have no gun, aircraft or man.
//! No base has a reserve or fuel. The Allied side holds no estimate and
//! no contact.
//!
//! Island 1, "Placeholder island": base 2, then base 4.
//!
//! | Force | Mission | Ships | Where |
//! | ----- | ------- | ----- | ----- |
//! | Task force 1 | transport | ships 1 and 2 | stopped at base 1 |
//! | Task force 3 | transport | ship 4 | stopped at base 1 |
//!
//! Each ship is a placeholder troop transport: 600 men, 3,000 t of cargo,
//! 10 kn, 1,000 t of fuel, nothing aboard.
//!
//! Units 1, 2 and 3, "Placeholder First", "Second" and "Third": Allied
//! fighting units of 600 men, ashore at A on the defence, experience 2,
//! every skill 2.
//!
//! The Allied side's commands of day 1, in this order:
//!
//! 1. intent 1, control island 1. No forces limit, no risk (so balanced),
//!    no date. Plan 1 is drafted.
//! 2. intent 2, take the place at C, with parent 1. Plan 2 is drafted.
//! 3. a forces limit on intent 2: task force 3 and land unit 3. No source.
//! 4. intent 3, take the place at B, with parent 1. Plan 3 is drafted. No
//!    forces limit.
//! 5. the limit "after intent 2" on intent 3.
//! 6. the limit "not before day 4" on intent 3.
//! 7. leave to prepare, given.
//! 8. leave to launch, given.
//!
//! ## The tree on the first morning
//!
//! | Intent | Kind | Under | Made by | Plan |
//! | ------ | ---- | ----- | ------- | ---- |
//! | 1 | Control island 1 | none | the commander | 1 |
//! | 2 | Take the place at C | 1 | the commander | 2 |
//! | 3 | Take the place at B | 1 | the commander | 3 |
//! | 4 | Hold base 2 | 1 | the staff | 1 |
//! | 5 | Hold base 4 | 1 | the staff | 1 |
//! | 6 | Deny the waters within 3 hexes of (4,0) | 1 | the staff | 1 |
//! | 7 | Neutralise base 2 | 1 | the staff | 1 |
//! | 8 | Neutralise base 4 | 1 | the staff | 1 |
//! | 9 | Ready a landing by task force 3 at C | 2 | the staff | 2 |
//! | 10 | Ready a landing by task force 1 at B | 3 | the staff | 3 |
//!
//! The staff's commands of the first morning, in order: plan 1 approved
//! to preparing and then to released; plan 2 approved to preparing; plan
//! 3 approved to preparing (the leaves are worked in priority order, the
//! island first, so the island's plan is released before his takes are
//! reached: rule 10); intent 4 formed and its forces limit set to base 1
//! and task forces 1 and 3; intent 5 formed and its limit set the same;
//! intents 6, 7 and 8 formed; intent 9 formed and its forces limit set;
//! intent 10 formed and its forces limit set. The staff forms no take:
//! the island's reason of the morning has a line for each place, "intent
//! 3, the commander's, plans the place at (4,0): the staff forms no take
//! beside it" and the same for intent 2 at (0,6) (rule 9), in the order
//! of the island's places.
//!
//! The source of supply is base 1, the side's only base off the island,
//! where both transport forces lie: that is the holds' limit. Each hold
//! waits for its base to be the side's. The side has no carrier force,
//! no bomber and no bombardment force: the denial and the two
//! neutralisings form nothing under them, and each hands in the notice
//! that plan 1 cannot meet its intent on every morning it stands, as
//! scenario 76's do: three on each of days 1 to 5, two on days 6 to 9
//! and one on days 10 to 16, 30 in all. The standing interrupt fires on
//! day 1.
//!
//! ## His forces, and his blanks
//!
//! The takes are served by rank, then by number: intent 2, then intent 3.
//!
//! - **Intent 2, the anchorage.** He named the task forces and the land
//!   units and not the source. The forces it may take are task force 3;
//!   the units, unit 3. No estimate is held, so the staff takes as many
//!   as can be carried: 600 berths, unit 3 in ship 4. The source is the
//!   staff's choice: base 1, the only base with the force and the unit.
//!   The staff's limit on intent 9 names what he left blank and nothing
//!   else: base 1 as the source, no task force and no land unit.
//! - **Intent 3, the airfield.** He named nothing, and the island has no
//!   limit: all three kinds are the staff's choice. Task force 3 and unit
//!   3 are intent 2's (rule 11: he named them for it, and its landing has
//!   them), and the reason says so, naming intent 2 for each. Of what is
//!   left, task force 1 has 1,200 berths at base 1 and units 1 and 2 are
//!   ashore beside it: unit 1 in ship 1 and unit 2 in ship 2, no unit left
//!   out. The staff's limit on intent 10 names the whole choice: task
//!   force 1, units 1 and 2, base 1 as the source.
//!
//! The check of decision 0144, made when each plan is released:
//!
//! | | The anchorage | The airfield |
//! | --- | --- | --- |
//! | Men | 600 | 1,200 |
//! | Daily need | 600 x 0.005 = 3 t | 1,200 x 0.005 = 6 t |
//! | Days until supply follows | 1 to land, 1 back, 1 to load, 1 out: 4 | 4 |
//! | Lands with the men | 3 x (4 + 5) = 27 t | 6 x 9 = 54 t |
//! | First run | 3 x 15 = 45 t | 6 x 15 = 90 t |
//! | Base 1 above its reserve | 991 t on the morning of day 2 | 934 t on the morning of day 6 |
//! | Days to embark | 1 for the supply and 1 for the unit: 2 | 1 and 2: 3 |
//! | Passage | 24 hours, 1 day | 16 hours, 1 day |
//!
//! Each place can be held.
//!
//! ## Day by day
//!
//! Orders are numbered as the staff gives them. Hours are of the game.
//!
//! | Day | Hour | The staff's commands and orders | What happens |
//! | --- | --- | --- | --- |
//! | 1 | 0 | The tree, above. Order 1: task force 1, `prepare_landing` at B, roster 1 and 2. Order 2: task force 3, `prepare_landing` at C, roster 3 | Each force's loading 100; each unit's rehearsal 100 |
//! | 2 | 24 | Notices: the landings of plans 2 and 3 are ready. Plan 2 approved to released. Plan 3 is not released: it waits. Order 3: task force 3 loads supply to a weight of 27 t | Ship 4 loads 27 t |
//! | 3 | 48 | Order 4: task force 3 loads unit 3. Order 5: unit 3 set to attack | Ship 4 takes 600 men in the night, after the night's charge |
//! | 4 | 72 | Order 6: task force 3, a route to C and back to A, acting at C. Order 7: its unload order | Sails at hour 72, arrives at hour 96, as the day ends, with no hour left to work cargo |
//! | 5 | 96 | none | In the night ship 4 lands 600 men in 7.5 hours and then 27 t in 1.35 hours, complete at hour 104.85. Base 5, "Landing at (0,6)", is formed. The force steams for A at hour 108. Base 4 changes side at the end of the day |
//! | 6 | 120 | Intent 8 withdrawn. Intent 11, keep base 4 supplied with a floor of 5 days, formed under intent 5; plan 4 drafted. Plan 2 marked complete; notice: intent 2 is met. Plan 3 approved to released. Order 8: task force 1 loads supply to a weight of 54 t. Order 9: unit 3 set to defend | Ship 1 loads 54 t. Task force 3 arrives at A at hour 132 |
//! | 7 | 144 | Plan 4 approved to preparing and to released. Order 10: task force 1 loads unit 1. Order 11: unit 1 set to attack | Ship 1 takes 600 men |
//! | 8 | 168 | Order 12: task force 1 loads unit 2. Order 13: unit 2 set to attack | Ship 2 takes 600 men |
//! | 9 | 192 | Run 1 of 45 t with 1 ship added to plan 4, loading day 9, due day 10; ship 4 committed to it from day 9 to day 12. Order 14: task force 1, a route to B and back to A, acting at B. Order 15: its unload order. Order 16: task force 3 loads supply to a weight of 45 t | Task force 1 sails at hour 192 and arrives at hour 208. Each ship lands 160 men in the morning's last 2 hours and 440 in the afternoon in 5.5 hours; ship 1 then lands 10 t. Base 6, "Landing at (4,0)", is formed. Ship 4 loads 45 t. Base 2 changes side at the end of the day |
//! | 10 | 216 | Intent 7 withdrawn. Intent 12, keep base 2 supplied with a floor of 5 days, formed under intent 4; plan 5 drafted. Plan 3 marked complete; notice: intent 3 is met. Order 17: task force 3, a route to C and back, acting at C. Order 18: its unload order. Orders 19 and 20: units 1 and 2 set to defend | In the night ship 1 lands 44 t in 2.2 hours, complete at hour 218.2. Task force 1 steams for A at hour 228. Task force 3 sails at hour 216 and arrives at C at hour 240 |
//! | 11 | 240 | Plan 5 approved to preparing and to released | Task force 1 arrives at A at hour 244. In the night ship 4 lands 45 t in 2.25 hours. Task force 3 steams for A at hour 252 |
//! | 12 | 264 | none | Task force 3 arrives at A at hour 276 |
//! | 13 | 288 | Run 1 of 90 t with 2 ships added to plan 5, loading day 13, due day 14; ships 1 and 2 committed to it from day 13 to day 16. Order 21: task force 1 loads supply to a weight of 90 t | Ship 1 loads 90 t |
//! | 14 | 312 | Order 22: task force 1, a route to B and back, acting at B. Order 23: its unload order | Sails at hour 312, arrives at hour 328. Ship 1 lands 40 t in the morning and 50 t in the afternoon, complete at hour 332.5 |
//! | 15 | 336 | none | Task force 1 steams for A at hour 336 and arrives at hour 352 |
//! | 16 | 360 | none | |
//!
//! 23 orders in all: 2, 1, 2, 2, 0, 2, 2, 2, 3, 4 on days 1 to 10, 1 on
//! day 13, 2 on day 14 and none on the other days. No staff order or
//! command is refused. No command the staff hands in withdraws, limits
//! or forms again intent 2 or intent 3.
//!
//! ## His order
//!
//! Intent 2 has no wait: it is readied on day 1 and released on day 2.
//!
//! Intent 3 is readied on day 1 with it, and its landing is ready on the
//! morning of day 2. It is not released on days 2 to 5, no leave to launch
//! is asked for it and no notice that it waits for leave is handed in.
//! The reason of each of those mornings under intent 3 names each wait
//! still unmet:
//!
//! | Morning | "Not before day 4" | "After intent 2" |
//! | ------- | ------------------ | ---------------- |
//! | 2 | named: it is not to go before day 4 | named: the place of intent 2 at (0,6) is not yet the side's |
//! | 3 | named | named |
//! | 4 | not named: its day has come | named |
//! | 5 | not named | named |
//!
//! Base 4 is the side's from the end of day 5. On the morning of day 6
//! both waits are met and plan 3 is released: two days after its "not
//! before".
//!
//! ## The supply doctrine's sums
//!
//! - **Base 4** (intent 11, plan 4, first served on day 7). The use is
//!   3 t a day and the floor's stock 15 t. The stock in C's hex is 24 t
//!   on the morning of day 7, 21 t on day 8 and 18 t on day 9; less two
//!   days of use, 18 t, 15 t and 12 t. The first below 15 is on day 9.
//!   The load is 0 plus 15 days of use, 45 t. Base 2 is still the enemy's
//!   that morning, 6 hexes from C, so the run is reckoned as one by
//!   night; at 10 kn it arrives as the night begins and its hours are
//!   those of any run to C. Its ship is ship 4: on the morning of day 9
//!   task force 1 lies at base 1 with the men of the airfield aboard,
//!   readying a landing, and a force readying a landing is not a supply
//!   run's (rule 11). After it lands, 54 t on the morning of day 12, 51 t
//!   on day 13 and so on: no second run is needed.
//! - **Base 2** (intent 12, plan 5, first served on day 11). The use is
//!   6 t a day and the floor's stock 30 t. The stock in B's hex is 48 t
//!   on the morning of day 11, 42 t on day 12 and 36 t on day 13; less
//!   two days of use, 36 t, 30 t and 24 t. The first below 30 is on day
//!   13. The load is 0 plus 15 days of use, 90 t. Its ships are the
//!   pool's free ships in number order: task force 1, ships 1 and 2.
//!
//! ## Supply, fuel and men
//!
//! Supply at the end of each day, in tons:
//!
//! | Day | Base 1 | Base 2 | Base 4 | Base 5 | Base 6 |
//! | --- | ------ | ------ | ------ | ------ | ------ |
//! | 1 | 991 | 0 | 0 | | |
//! | 2 | 955 | 0 | 0 | | |
//! | 3 | 946 | 0 | 0 | | |
//! | 4 | 940 | 0 | 0 | | |
//! | 5 | 934 | 0 | 0 | 27 | |
//! | 6 | 874 | 0 | 0 | 24 | |
//! | 7 | 868 | 0 | 0 | 21 | |
//! | 8 | 865 | 0 | 0 | 18 | |
//! | 9 | 820 | 0 | 0 | 15 | 10 |
//! | 10 | 820 | 44 | 0 | 12 | 4 |
//! | 11 | 820 | 38 | 45 | 9 | 4 |
//! | 12 | 820 | 32 | 42 | 9 | 4 |
//! | 13 | 730 | 26 | 39 | 9 | 4 |
//! | 14 | 730 | 110 | 36 | 9 | 4 |
//! | 15 | 730 | 104 | 33 | 9 | 4 |
//! | 16 | 730 | 98 | 30 | 9 | 4 |
//!
//! - **Base 1** feeds 1,800 men, 9 t, on the nights of days 1 to 3; 1,200
//!   men, 6 t, on the nights of days 4 to 7; 600 men, 3 t, on the night
//!   of day 8; nobody after: 54 t. It loads out 27 t on day 2, 54 t on
//!   day 6, 45 t on day 9 and 90 t on day 13: 216 t. 1,000 - 54 - 216 =
//!   730 t.
//! - **Base 5** is given 27 t in the night of day 5, after that night's
//!   charge, and pays 3 t on the nights of days 6 to 11: 9 t, which it
//!   keeps, base 4 paying first from day 12.
//! - **Base 4** takes 45 t in the night of day 11, after that night's
//!   charge, and pays 3 t on the nights of days 12 to 16: 30 t.
//! - **Base 6** is given 10 t on the afternoon of day 9 and pays 6 t on
//!   the night of day 10, when base 2 holds nothing: 4 t, which it keeps.
//! - **Base 2** takes 44 t in the night of day 10, after that night's
//!   charge; pays 6 t on the nights of days 11 to 14; takes 90 t on day
//!   14; pays 6 t on the nights of days 15 and 16: 98 t.
//! - **Ships 1 and 2** steam to B and back twice, 640 nm, and burn 64 t:
//!   936 t each. **Ship 4** steams to C and back twice, 960 nm, and burns
//!   96 t: 904 t.
//!
//! Men ashore at the end of each day:
//!
//! | Days | At A | At C | At B |
//! | ---- | ---- | ---- | ---- |
//! | 1, 2 | 1,800 | 0 | 0 |
//! | 3, 4 | 1,200 | 0 | 0 |
//! | 5, 6 | 1,200 | 600 | 0 |
//! | 7 | 600 | 600 | 0 |
//! | 8 | 0 | 600 | 0 |
//! | 9 to 16 | 0 | 600 | 1,200 |
//!
//! No man is unfed on any night. The men, the ships and the tons are
//! conserved, as in scenarios 76 and 81.
//!
//! ## The plan orders
//!
//! Read at the staff's hour of each morning, before that morning's
//! decisions, as scenario 84 reads them.
//!
//! **Intent 1.** Its tasks are every intent in force beneath it, in the
//! tree's order, each with its maker (rule 18):
//!
//! | Mornings | Tasks |
//! | -------- | ----- |
//! | 1 | 2 and 3, the commander's |
//! | 2 to 6 | 2, 9, 3, 10, 4, 5, 6, 7, 8 |
//! | 7 to 10 | 2, 9, 3, 10, 4, 5, 11, 6, 7 |
//! | 11 to 16 | 2, 9, 3, 10, 4, 12, 5, 11, 6 |
//!
//! Intents 2 and 3 are the commander's on every morning and every other
//! task is the staff's.
//!
//! **Intent 2**, plan 2:
//!
//! | Morning | Plan | Prepare | Move | Land | Waits for | Expects to be ashore |
//! | --- | --- | --- | --- | --- | --- | --- |
//! | 1 | draft | to come | to come | to come | nothing | none |
//! | 2 | preparing | under way | to come | to come | nothing | day 4 |
//! | 3, 4 | released | done | under way | to come | nothing | day 4 |
//! | 6 | released | done | done | done | nothing | none: it is ashore |
//! | 7 | complete | done | done | done | nothing | none |
//!
//! The day it expects: the landing is ready on day 2, takes 2 days to
//! embark and 1 of passage, less 1: day 4, the day it arrives, as the
//! take's own sums for a date count a passage of 24 hours. Its men are
//! ashore in the night of day 5; morning 5 is not asserted.
//!
//! **Intent 3**, plan 3:
//!
//! | Morning | Plan | Prepare | Move | Land | Waits for | Expects to be ashore |
//! | --- | --- | --- | --- | --- | --- | --- |
//! | 1 | draft | to come | to come | to come | a day: not before day 4, not met, expected day 4; an intent: intent 2, not met, no day | none |
//! | 2, 3 | preparing | under way | to come | to come | the same two | none |
//! | 4, 5 | preparing | under way | to come | to come | the day, met; intent 2, not met, no day | none |
//! | 6 | preparing | under way | to come | to come | both met | day 9 |
//! | 7 to 9 | released | done | under way | to come | nothing | day 9 |
//! | 10 | released | done | done | done | nothing | none: it is ashore |
//! | 11 | complete | done | done | done | nothing | none |
//!
//! The day it expects on morning 6: 6 + 3 to embark + 1 of passage - 1 =
//! day 9. A thing waited for with no day leaves it without one. No count
//! of the enemy is held, so neither order gives men wanted. The one task
//! under intent 2 is intent 9 and under intent 3 intent 10, from morning
//! 2, each in the phase prepare.
//!
//! ## The tree proposed
//!
//! `propose` of intent 1, asked on the morning of day 2: the tree has an
//! entry for intent 2 and one for intent 3, each with its own number,
//! parent 1, `maker` "commander" and `standing` true, before the staff's
//! entries, and its `no_orders` hold rule 9's line for each (rule 18).
//!
//! # The game with nothing written
//!
//! The file with the commands between its two markers taken out: intent
//! 1, and the two leaves. The staff forms both takes itself, and serves
//! the airfield's first.
//!
//! | Intent | Kind | Under | Plan |
//! | ------ | ---- | ----- | ---- |
//! | 1 | Control island 1 | none | 1 |
//! | 2 | Take the place at B | 1 | 2 |
//! | 3 | Hold base 2 | 1 | 1 |
//! | 4 | Take the place at C | 1 | 3 |
//! | 5 | Hold base 4 | 1 | 1 |
//! | 6 | Deny the waters within 3 hexes of (4,0) | 1 | 1 |
//! | 7 | Neutralise base 2 | 1 | 1 |
//! | 8 | Neutralise base 4 | 1 | 1 |
//! | 9 | Ready a landing by task force 1 at B | 2 | 2 |
//! | 10 | Ready a landing by task force 3 at C | 4 | 3 |
//!
//! All are the staff's but intent 1, formed on day 1 in that order (each
//! take is approved to preparing as it is formed, and each hold's limit
//! set as it is formed).
//!
//! - **Intent 2, the airfield,** chooses of everything at the source. No
//!   estimate: the lift that gives the most men, task force 1 with 1,200
//!   berths. The units in their order, all of assault skill 2 and
//!   experience 2, so by number: unit 1 in ship 1, unit 2 in ship 2; unit
//!   3 is left out, no ship being left for it.
//! - **Intent 4, the anchorage,** served the same morning, chooses of
//!   what intent 2's landing has not (rule 11, "What a landing has"):
//!   task force 1 and units 1 and 2 are intent 2's, and the reason names
//!   it. Task force 3, 600 berths; unit 3 in ship 4.
//!
//! No unit and no force is on both landings.
//!
//! | Day | Hour | The staff's commands and orders | What happens |
//! | --- | --- | --- | --- |
//! | 1 | 0 | The tree. Order 1: task force 1, `prepare_landing` at B, roster 1 and 2. Order 2: task force 3, `prepare_landing` at C, roster 3 | Loading and rehearsal 100 |
//! | 2 | 24 | Notices: both landings ready. Plans 2 and 3 approved to released. Order 3: task force 1 loads supply to 54 t. Order 4: task force 3 loads supply to 27 t | |
//! | 3 | 48 | Order 5: task force 1 loads unit 1. Order 6: task force 3 loads unit 3. Orders 7 and 8: units 1 and 3 set to attack | |
//! | 4 | 72 | Order 9: task force 1 loads unit 2. Orders 10 and 11: task force 3's route to C and back, and its unload order. Order 12: unit 2 set to attack | Task force 3 sails at hour 72 and arrives at hour 96 |
//! | 5 | 96 | Orders 13 and 14: task force 1's route to B and back, and its unload order | In the night ship 4 lands 600 men and 27 t at C; base 5, "Landing at (0,6)", is formed. Task force 1 sails at hour 96 and arrives at hour 112; each ship lands 160 men in the morning and 440 in the afternoon, and ship 1 then 10 t; base 6, "Landing at (4,0)", is formed. Task force 3 steams for A at hour 108. Bases 4 and 2 change side at the end of the day |
//! | 6 | 120 | Intents 7 and 8 withdrawn. Intent 11, keep base 2 supplied, formed under intent 3, plan 4 drafted; intent 12, keep base 4 supplied, formed under intent 5, plan 5 drafted; each with a floor of 5 days. Plans 2 and 3 marked complete; notices: intents 2 and 4 are met. Orders 15, 16 and 17: units 1, 2 and 3 set to defend | In the night ship 1 lands 44 t. Task force 3 arrives at A at hour 132. Task force 1 steams for A at hour 132 |
//! | 7 | 144 | Plans 4 and 5 each approved to preparing and to released | Task force 1 arrives at A at hour 148 |
//! | 9 | 192 | Run 1 of 90 t with 2 ships added to plan 4 and ships 1 and 2 committed to it, and run 1 of 45 t with 1 ship added to plan 5 and ship 4 committed to it, each loading day 9, due day 10, committed from day 9 to day 12. Order 18: task force 1 loads supply to 90 t. Order 19: task force 3 loads supply to 45 t | |
//! | 10 | 216 | Orders 20 and 21: task force 1's route to B and back, and its unload order. Orders 22 and 23: task force 3's, to C | Task force 1 arrives at hour 232; ship 1 lands 40 t in the morning and 50 t in the afternoon. Task force 3 arrives at C at hour 240 |
//! | 11 | 240 | none | Task force 1 steams for A at hour 240 and arrives at hour 256. In the night ship 4 lands 45 t; task force 3 steams for A at hour 252 |
//! | 12 | 264 | none | Task force 3 arrives at A at hour 276 |
//!
//! No order on days 8 and 13 to 16. 23 orders in all: 2, 2, 4, 4, 2, 3 on
//! days 1 to 6, 2 on day 9 and 4 on day 10.
//!
//! The supply's sums: at B the stock in the hex is 48 t on the morning of
//! day 7, 42 t on day 8 and 36 t on day 9, and less two days of use,
//! 12 t, the first below 30 t is on day 9. At C they are the main game's:
//! day 9.
//!
//! Supply at the end of each day, in tons:
//!
//! | Day | Base 1 | Base 2 | Base 4 | Base 5 | Base 6 |
//! | --- | ------ | ------ | ------ | ------ | ------ |
//! | 1 | 991 | 0 | 0 | | |
//! | 2 | 901 | 0 | 0 | | |
//! | 3 | 892 | 0 | 0 | | |
//! | 4 | 889 | 0 | 0 | | |
//! | 5 | 889 | 0 | 0 | 27 | 10 |
//! | 6 | 889 | 44 | 0 | 24 | 4 |
//! | 7 | 889 | 38 | 0 | 21 | 4 |
//! | 8 | 889 | 32 | 0 | 18 | 4 |
//! | 9 | 754 | 26 | 0 | 15 | 4 |
//! | 10 | 754 | 110 | 0 | 12 | 4 |
//! | 11 | 754 | 104 | 45 | 9 | 4 |
//! | 16 | 754 | 74 | 30 | 9 | 4 |
//!
//! From day 11 to day 16 base 2 pays 6 t and base 4 3 t each night. Base
//! 1 feeds 9 t on the nights of days 1 to 3 and 3 t on the night of day
//! 4, 30 t, and loads out 216 t: 754 t. Ships 1 and 2 end with 936 t of
//! fuel and ship 4 with 904 t, as in the main game.
//!
//! ## The two set side by side
//!
//! | | His plan | The staff's plan |
//! | --- | --- | --- |
//! | The anchorage, C, is the side's from the end of | day 5 | day 5 |
//! | The airfield, B, is the side's from the end of | day 9 | day 5 |
//! | The anchorage's plan complete on the morning of | day 6 | day 6 |
//! | The airfield's plan complete on the morning of | day 10 | day 6 |
//! | Both places the side's, counted from day 1 | 9 days | 5 days |
//! | Orders | 23 | 23 |
//! | Base 1 at the end | 730 t | 754 t |
//! | Base 2 at the end | 98 t | 74 t |
//!
//! His plan takes the airfield four days later: that is the wait he
//! wrote. Neither is the better by statement: the test asserts both as
//! calculated.
//!
//! # Variants
//!
//! Each is the main game with the change named, and asserts what is said
//! and no more.
//!
//! - **Whichever is served first.** The written part with the numbers the
//!   other way round, and unit 3 at experience 3: intent 2 is the take of
//!   B, naming nothing, with "after intent 3" and "not before day 4";
//!   intent 3 is the take of C with a forces limit of task force 3 and
//!   unit 3. The commands state intent 2, then intent 3 and its limit,
//!   then the two waits on intent 2. Intent 2 is served first. Left to
//!   itself it would take unit 3 first, the higher experience, and unit 1
//!   with it, as the staff's own plan does on this file (in the game with
//!   nothing written and unit 3 at experience 3, the landing at B has
//!   task force 1 with units 3 and 1 and the landing at C task force 3
//!   with unit 2). Here it does not: intent 9, ready a landing by task
//!   force 1 at B, is formed under intent 2 with units 1 and 2, and its
//!   reason names intent 3 as the node that has task force 3 and unit 3;
//!   intent 10, ready a landing by task force 3 at C, is formed under
//!   intent 3 with unit 3. Order 1 is task force 1's, roster 1 and 2;
//!   order 2 task force 3's, roster 3. Day 1 is asserted.
//! - **The day alone holds it.** With "not before day 8" on intent 3: on
//!   mornings 2 to 5 the reason names both waits; on mornings 6 and 7 it
//!   names "not before day 8" alone, base 4 being the side's; on the
//!   morning of day 8 plan 3 is approved to released and task force 1 is
//!   ordered to load supply to 54 t (base 1 holds 922 t that morning).
//!   Unit 1 is loaded on day 9 and unit 2 on day 10; the force sails at
//!   hour 240; base 2 changes side at the end of day 11 and plan 3 is
//!   marked complete on day 12. On day 9 run 1 of plan 4, 45 t, is
//!   ordered with ship 4 as in the main game: task force 1 lies at base 1
//!   that morning with nothing aboard but its 54 t, readying its landing,
//!   and is not the run's (rule 11). Days 6 to 9 are asserted, and the
//!   day base 2 changes side.
//! - **A mediocre plan waits.** With an opening estimate, held by the
//!   Allied side from the start, of 400 enemy men at B, and nobody there
//!   in truth; and a forces limit on intent 3 of land unit 1. At balanced
//!   the staff wants 2 men for each: 800. The units it may take are unit
//!   1, his; the force is its choice, task force 1; the lift gives 600
//!   men: short by 200. No unit or force of the side is coming, and no
//!   day up to day 8, the 7 days it looks ahead, brings them. On day 1
//!   plan 3 stays a draft, nothing is formed under intent 3, and the
//!   notice "cannot bring enough" is handed in for plan 3; the standing
//!   interrupt fires. On days 2 to 8 the same reckoning with the day
//!   moved on by one, no second notice. The staff adds no unit: unit 2
//!   is given no order on any day and is on no limit the staff sets. On
//!   no day of the 16 is a take of B or a landing under intent 3 formed.
//!   Intent 2 goes as in the main game to day 6, and the staff's first
//!   intent of day 6, keep base 4 supplied, is intent 10. Days 1 to 8 are
//!   asserted for intent 3. (From day 9 the supply of base 4 may use task
//!   force 1, which no landing has: those days are not asserted.)
//! - **A mediocre plan at press.** As "A mediocre plan waits", with the
//!   island's risk set to press on day 1, after intent 1 is stated. A
//!   risk on his take must narrow the island's (rule 6), and press is
//!   wider than balanced: "risk press" on intent 3 alone, the island at
//!   balanced, is refused at intake as wider than its parent allows, and
//!   that refusal is asserted. At press the staff wants 1 man for each,
//!   400: unit 1's 600 are enough. On day 1 plan 3 is approved to
//!   preparing and intent 10, ready a landing by task force 1 at B, is
//!   formed under intent 3 with unit 1 and no other; no notice "cannot
//!   bring enough" is handed in. The margin at press is 0: the supply
//!   that lands with the men is 3 x 4 = 12 t. Plan 3 is released on the
//!   morning of day 6 with a load of 12 t; unit 1 is loaded on day 7; the
//!   force sails at hour 168 and arrives at hour 184; ship 1 lands 160
//!   men in the morning and 440 in the afternoon, and then 10 t; base 2
//!   changes side at the end of day 8; the men find nobody and the
//!   estimate is cleared; plan 3 is marked complete on day 9. Days 1 to 9
//!   are asserted for intent 3 and what is under it, and nothing of the
//!   anchorage, whose supply at press is scenario 136's matter.
//! - **A plan that has ended.** With the commander's `cancel_plan` of
//!   plan 3 on day 2: plan 3 goes from preparing to cancelled. On every
//!   morning from day 2 to day 16 the staff forms no take of B; the
//!   reason under intent 3 says its plan is cancelled and nothing is
//!   formed or ordered under it; and the island's reason says that
//!   intent 3, the commander's, plans the place at (4,0), that its plan
//!   is cancelled, that the staff forms no take beside it and the island
//!   cannot be controlled while the place is his to take, and what would
//!   change it. No notice of its own is handed in for it: the notice
//!   "cannot meet" is once for its kind and its plan, and the
//!   neutralisings and the denial have handed it in for plan 1 on day 1
//!   (rule 14). Base 2 is Japanese at the end. Task force 1 is given no
//!   order by the take on any day.
//! - **Given back to the staff.** As "A plan that has ended", with two
//!   commands more. On day 3: `set_intent` 20, take the place at B, with
//!   parent 1. It is refused as "Already planned", naming intent 3, and
//!   changes nothing but the log. On day 4: `withdraw_intent` 3. Intent
//!   10 is withdrawn with it. That morning the staff forms intent 11,
//!   take the place at B, under intent 1, with plan 4, and approves it to
//!   preparing; forms intent 12, ready a landing by task force 1 at B,
//!   under it, with units 1 and 2; finds the force's loading and the
//!   units' rehearsal of day 1 standing, so the landing ready, hands in
//!   that notice and approves plan 4 to released; and orders task force 1
//!   to load supply to 54 t (order 6; base 1 holds 886 t at the end of
//!   the day). It gives no second landing order. Unit 1 is loaded on day
//!   5 and unit 2 on day 6; the force sails at hour 144; base 2 changes
//!   side at the end of day 7; plan 4 is marked complete on day 8. Days 2
//!   to 4 are asserted, and the day base 2 changes side.
//! - **Taken and done.** In the main game, from the morning of day 10 to
//!   the end intent 3 is in force with plan 3 complete, and on no morning
//!   does the staff form a take of B; so intent 2 from day 6, and C.
//! - **The island's date.** With intent 1 asked for by day 12 and no date
//!   on either take: intent 9 is formed as "ready a landing by task force
//!   3 at C by the morning of day 10", 12 less the 2 days to embark and
//!   the day of passage, plus 1; intent 10 as "ready a landing by task
//!   force 1 at B by the morning of day 9", 12 - 3 - 1 + 1: the dates a
//!   staff's take formed with day 12 would have given them. No notice
//!   `late` is handed in on day 1. Only the forming is asserted.
//! - **His own later date.** As "The island's date", with intent 3 asked
//!   for by day 14: intent 10 is formed "by the morning of day 11", 14 -
//!   3 - 1 + 1: his date is kept. On day 1 the notice `late` is handed in
//!   once for plan 1; its reason names intent 3, day 12, day 14 and the 2
//!   days between. None on day 2. Days 1 and 2 are asserted.
//! - **Before the island is released.** With leave to launch withheld on
//!   day 1 and given on day 3. On days 1 and 2 plan 1 is approved to
//!   preparing (day 1) and the notice that it waits for leave to launch
//!   is handed in each morning; plans 2 and 3 stay drafts; nothing is
//!   formed and no order given; the reason under each of intents 2 and 3
//!   says it waits for the plan of intent 1. On day 3 the staff does
//!   what it does on day 1 of the main game, with the same numbers:
//!   plan 1 released, plans 2 and 3 approved to preparing, intents 4 to
//!   10, orders 1 and 2. On day 4 plan 2 is released and task force 3
//!   ordered to load 27 t (base 1 holds 937 t at the end of the day);
//!   intent 3's reason names "after intent 2" alone, its day having come.
//!   Base 4 changes side at the end of day 7 and plan 3 is released on
//!   day 8. Days 1 to 4 are asserted, and those two days.
//! - **Refused at intake.** Each alone, each changing nothing but the
//!   log, with the reason and its words as rule 3 has them:
//!   - a parent not in force: on day 1, intent 2 with parent 7, which is
//!     not set: "No such intent", naming intent 7;
//!   - a parent the staff formed: on day 2, intent 20, take the place at
//!     B, with parent 4, the staff's hold: "Cannot stand under", naming
//!     intent 4;
//!   - a number not above the parent's: with the island stated as intent
//!     5, intent 2 with parent 5: "Intent number not usable";
//!   - a pair not open: on day 1, intent 2, hold base 2, with parent 1;
//!     and, with intent 2 a root hold of base 1, intent 3, take the place
//!     at B, with parent 2: each "Cannot stand under", naming the parent;
//!   - a place that is not a place of the island: intent 2, take the
//!     place at (0,0), with parent 1: "Cannot stand under", naming intent
//!     1;
//!   - a place a take already plans, the staff's: in the game with
//!     nothing written, on day 2, intent 20, take the place at B, with
//!     parent 1: "Already planned", naming intent 2;
//!   - and a root of his own: with intent 2 a root take of the place at B
//!     stated before it, intent 3, take the place at B, with parent 1:
//!     "Already planned", naming intent 2;
//!   - a parent changed on an intent that stands: in the main game on day
//!     2, intent 2 set again with parent 3: "Parent cannot change", and
//!     its words say intent 2 stands under intent 1; and intent 1 set
//!     again with parent 2: its words say intent 1 is a root.
//! - **No wait that waits for itself.** Each alone, refused as "Not an
//!   end to wait for": "after intent 1" on intent 2, its own island;
//!   "after intent 2" on intent 1, a take beneath it; and, with intent 4
//!   a root take of the place at (0,0) and "after intent 2" set on it and
//!   taken in, "after intent 4" on intent 1, which intent 2 would read.
//! - **Set again.** Taken in, each alone: in the main game on day 2,
//!   intent 3 set again as take the place at B by day 12 with no parent
//!   given: it stands under intent 1 still, the commander's, and plan 3,
//!   approved, is amended. And, with intent 2 alone written on day 1
//!   (take the place at C, parent 1, no limit), intent 2 set again the
//!   same day as take the place at B with parent 1: it is the take of B,
//!   and on the first morning the staff forms a take of C and none of B.
//! - **A carrier force he named.** With scenario 73's carrier force as
//!   scenario 76's property has it (task force 2, ship 3 and air groups
//!   1, 2 and 3, stopped at base 1, its home port), on scenario 76's
//!   carrier figures, and intent 2's forces limit naming task forces 2
//!   and 3 and unit 3. On day 1 "operate task force 2 from (0,0)" is
//!   formed under the denial, intent 6, as intent 7; the neutralisings
//!   are intents 8 and 9 and the landings intents 10 and 11. No carrier
//!   task is formed under intent 2 on day 1 or day 2, for cover or to
//!   look; the landing under intent 2 is task force 3's with unit 3; plan
//!   2 is released on day 2 as in the main game; and a reason under
//!   intent 2, on day 2 at the latest, says task force 2 is on the
//!   carrier mission, that under an island the carriers are the island's,
//!   and that it is not this landing's (rule 10). Days 1 and 2 are
//!   asserted, of commands and reasons alone.
//! - **A bombardment force he named.** With scenario 74's battleship as
//!   scenario 76's property has it, here task force 5 with ship 5, on the
//!   bombardment mission, stopped at base 1, its home port; and intent
//!   2's forces limit naming task forces 3 and 5 and unit 3. C holds an
//!   enemy base, so the bombard step has a target under intent 2. On day
//!   1 "bombard base 2 with task force 5" is formed under the
//!   neutralising of base 2, intent 7, as intent 11, after the landings,
//!   intents 9 and 10. No bombard task is formed under intent 2 on any of
//!   days 1 to 3, and its reason of the morning its plan is released,
//!   day 2, says task force 5 is on the bombardment mission, that under
//!   an island the bombardment forces are the island's, and that it is
//!   not this landing's. (The island has the force from day 1, so the
//!   code as it stands would form none under the take either, saying the
//!   force has one bombard task: what this proves is the take's reason.)
//!   Days 1 to 3 are asserted, of commands and reasons alone.
//! - **A unit he named and then held.** With a forces limit on intent 3
//!   of land units 1 and 2 and the commander's hold of land unit 2, both
//!   on day 1: intent 10 is formed with unit 1 alone in task force 1, and
//!   the reason says unit 2 is held by the commander's hold of day 1.
//!   Order 1 is task force 1's landing order with roster 1. Day 1 is
//!   asserted.
//! - **Withdrawn with its island.** With `withdraw_intent` 1 on day 4,
//!   when plan 2 is released and plan 3 preparing: intents 2 to 10 are
//!   withdrawn with it, each with its `IntentWithdrawn`; plan 2 goes from
//!   released to complete, plan 3 from preparing to cancelled, and plan 1
//!   from released to complete. No order is given on day 4 or after: five
//!   orders in the game. Task force 3 lies at base 1 at the end with unit
//!   3's 600 men and 27 t aboard; units 1 and 2 are ashore at A; base 1
//!   holds 868 t (940 t at the end of day 4, less 6 t on each of the 12
//!   nights after).
//! - **The same twice; saved and resumed; what the side cannot see.**
//!   The same start gives the same game, hash for hash. A save on the
//!   morning of day 8, resumed, is the game not interrupted. Two worlds
//!   that differ only in what the Allied side cannot see give the same
//!   commands, orders, reasons and plan orders.
//!
//! # Properties
//!
//! Held in the main game and in every variant:
//!
//! - no command the staff hands in withdraws, limits or names as its own
//!   an intent the commander made;
//! - no take of a place is formed while his take of it is in force and
//!   its plan is not complete;
//! - every unit and force a landing of his uses is one he named, where he
//!   named that kind;
//! - no landing uses a unit or a force that another node of his names;
//! - no unit and no task force is on two landings under the island whose
//!   takes' plans have not ended;
//! - no supply run is added with a ship of a force that is readying a
//!   landing;
//! - the men, the ships and the tons are conserved.
//!
//! # What this assumes of the build
//!
//! Decision 0232's rules, none of them code when this was written:
//!
//! - a scenario's `[[command]]` and the engine's `set_intent` take
//!   `parent`, and a written take drafts its plan and records no
//!   `IntentFormed` (rules 1, 4 and 18);
//! - the island forms no take of a place he wrote, and says the line of
//!   rule 9, one for each place in the island's order, on each morning
//!   his take is in force and its plan is not complete; the place's
//!   check for a base of the side comes first, as today, so the line
//!   ends when the place is the side's;
//! - the leaves pass his take over while the island's plan is a draft or
//!   preparing, with one reason, and the island is worked before his
//!   takes in the same step, as priority order has it (rule 10);
//! - rule 11's three parts: what he named is that node's; what a landing
//!   has (its own force, and the task forces and land units of its
//!   limit) is kept from the takes beside it while its take's plan has
//!   not ended; and a force readying a landing (its order to prepare
//!   the landing standing, and its take's plan not ended) is not a free
//!   unit of a supply pool;
//! - the date a take is held to is the island's where he gave it none,
//!   and `late` on the island's plan where his is later (rules 11 and
//!   14);
//! - no carrier and no bombard task is formed under his take, with the
//!   reason of rule 10;
//! - the plan order's tasks are every intent in force beneath, each with
//!   its maker, and a proposal's tree has the standing entries (rule 18);
//! - the two new refusals and the ring check of rule 7.
//!
//! The words of a reason are the build's to choose where a rule does not
//! give them; the tests read them for what this comment says they name.
//! Intent, plan, order and base numbers are asserted as given.

#![allow(clippy::doc_lazy_continuation, clippy::doc_markdown)]

/// Statement 1: intents 2 and 3 stand under intent 1 with the commander
/// as maker; plans 2 and 3 are drafted on day 1; each is recorded by its
/// `CommandTaken` and a `PlanDrafted`, and no `IntentFormed` names either.
#[test]
#[ignore = "statement written, not built"]
fn taken_in() {
    todo!()
}

/// Statement 2: the tree of the first morning, intents 4 to 10 in order
/// with their plans; on no morning of the 16 is a take of B or of C
/// formed; the island's reason of each morning names intents 2 and 3 as
/// the commander's while each stands unmet.
#[test]
#[ignore = "statement written, not built"]
fn nothing_beside_them() {
    todo!()
}

/// Statement 3: the landing under intent 2 has task force 3 and unit 3
/// and no other, and the staff's limit on intent 9 names base 1 alone;
/// the landing under intent 3 has task force 1 and units 1 and 2, and its
/// reason names intent 2 for task force 3 and unit 3.
#[test]
#[ignore = "statement written, not built"]
fn his_forces_and_his_blanks() {
    todo!()
}

/// Statement 3's variant, "Whichever is served first": day 1 with the
/// numbers the other way round and unit 3 at experience 3, and the game
/// with nothing written on the same file beside it.
#[test]
#[ignore = "statement written, not built"]
fn whichever_is_served_first() {
    todo!()
}

/// Statement 4: plan 2 released on day 2; plan 3 preparing on days 2 to
/// 5 with no leave asked and no waits-for-leave notice, the reason of
/// each morning naming the waits of the table; plan 3 released on day 6.
#[test]
#[ignore = "statement written, not built"]
fn his_order() {
    todo!()
}

/// Statement 4's variant, "The day alone holds it": days 6 to 9 with
/// "not before day 8", and base 2 the side's at the end of day 11.
#[test]
#[ignore = "statement written, not built"]
fn the_day_alone_holds_it() {
    todo!()
}

/// Statement 5: the staff's commands and the 23 orders of the day-by-day
/// table, each on its day with its force or unit, its weight or its
/// roster; the hours each force sails and arrives; the men and tons
/// landed by pulse; the days bases 4 and 2 change side; none refused.
#[test]
#[ignore = "statement written, not built"]
fn the_days_and_the_orders() {
    todo!()
}

/// Statement 5: the supply's sums (runs of 45 t on day 9 with ship 4 and
/// of 90 t on day 13 with ships 1 and 2); the supply of the five bases
/// at the end of every day; the ships' fuel at the end; the men ashore
/// at the end of every day; no man unfed.
#[test]
#[ignore = "statement written, not built"]
fn supply_fuel_and_men() {
    todo!()
}

/// Statement 6: the plan orders of intents 1, 2 and 3 on the mornings of
/// the tables, with every task's maker.
#[test]
#[ignore = "statement written, not built"]
fn the_plan_orders() {
    todo!()
}

/// Rule 18: the tree `propose` gives for intent 1 on the morning of day
/// 2 has the standing entries of intents 2 and 3 and rule 9's lines.
#[test]
#[ignore = "statement written, not built"]
fn the_tree_proposed() {
    todo!()
}

/// Statement 7: the game with nothing written, its tree, its two
/// landings with no unit or force on both, its 23 orders by day, its
/// supply at the end of the days of its table, and the table of the two
/// games side by side.
#[test]
#[ignore = "statement written, not built"]
fn against_the_game_with_nothing_written() {
    todo!()
}

/// Statement 8, "A mediocre plan waits": days 1 to 8 for intent 3, one
/// "cannot bring enough" on day 1, no unit added, and no take or landing
/// formed for B on any day.
#[test]
#[ignore = "statement written, not built"]
fn a_mediocre_plan_waits() {
    todo!()
}

/// Statement 8, "A mediocre plan at press": the refusal of press on
/// intent 3 alone; with the island at press, days 1 to 9 for intent 3:
/// the landing with unit 1 alone, 12 t, base 2 the side's at the end of
/// day 8.
#[test]
#[ignore = "statement written, not built"]
fn a_mediocre_plan_at_press() {
    todo!()
}

/// Statement 9, "A plan that has ended": no take of B on any morning
/// from day 2, the two reasons of each morning, no notice of its own,
/// base 2 Japanese at the end.
#[test]
#[ignore = "statement written, not built"]
fn a_plan_that_has_ended() {
    todo!()
}

/// Statement 9, "Given back to the staff": the second take refused on
/// day 3; days 2 to 4; intent 11 and plan 4 on day 4, its landing ready
/// and released that morning; base 2 the side's at the end of day 7.
#[test]
#[ignore = "statement written, not built"]
fn given_back_to_the_staff() {
    todo!()
}

/// Statement 9, "Taken and done": with each plan complete his take
/// stays in force and the staff forms no take of the place.
#[test]
#[ignore = "statement written, not built"]
fn taken_and_done() {
    todo!()
}

/// Statement 10, "The island's date": the landings formed for the
/// mornings of day 10 and day 9, and no `late`.
#[test]
#[ignore = "statement written, not built"]
fn the_islands_date() {
    todo!()
}

/// Statement 10, "His own later date": the landing at B formed for the
/// morning of day 11, and `late` once for plan 1 on day 1 with its
/// reason.
#[test]
#[ignore = "statement written, not built"]
fn his_own_later_date() {
    todo!()
}

/// Statement 11, "Before the island is released": days 1 to 4, base 4
/// the side's at the end of day 7 and plan 3 released on day 8.
#[test]
#[ignore = "statement written, not built"]
fn before_the_island_is_released() {
    todo!()
}

/// Statement 12, "Refused at intake": each command of the list refused
/// with its reason and words, changing nothing but the log.
#[test]
#[ignore = "statement written, not built"]
fn refused_at_intake() {
    todo!()
}

/// Statement 12, "No wait that waits for itself": the three rings, each
/// refused as "Not an end to wait for".
#[test]
#[ignore = "statement written, not built"]
fn no_wait_that_waits_for_itself() {
    todo!()
}

/// Statement 12, "Set again": his take set again with a date and no
/// parent given; his take set again for the other place.
#[test]
#[ignore = "statement written, not built"]
fn set_again() {
    todo!()
}

/// Statement 12, "A carrier force he named": days 1 and 2, no carrier
/// task under intent 2, the island's under its denial, the take's
/// reason.
#[test]
#[ignore = "statement written, not built"]
fn a_carrier_force_he_named() {
    todo!()
}

/// Statement 12, "A bombardment force he named": days 1 to 3, no
/// bombard task under intent 2, the island's under its neutralising, the
/// take's reason.
#[test]
#[ignore = "statement written, not built"]
fn a_bombardment_force_he_named() {
    todo!()
}

/// Statement 12, "A unit he named and then held": day 1, the landing
/// with unit 1 alone and the reason naming the hold.
#[test]
#[ignore = "statement written, not built"]
fn a_unit_he_named_and_then_held() {
    todo!()
}

/// Statement 13, "Withdrawn with its island": the nine `IntentWithdrawn`,
/// plan 2 complete and plan 3 cancelled, five orders in the game, and
/// the end.
#[test]
#[ignore = "statement written, not built"]
fn withdrawn_with_its_island() {
    todo!()
}

/// Statement 14: the same start gives the same game; a save on day 8
/// and a resume give the same game; two worlds that differ only in what
/// the Allied side cannot see give the same commands, orders, reasons
/// and plan orders.
#[test]
#[ignore = "statement written, not built"]
fn the_same_twice_saved_and_unseen() {
    todo!()
}

/// The properties, over the main game and every variant: what the staff
/// may not do to a node of his, what each landing uses, and what is kept
/// from the takes beside it and from the supply.
#[test]
#[ignore = "statement written, not built"]
fn the_staff_keeps_to_his_plan() {
    todo!()
}

/// The property of conservation: the men, the ships and the tons, in the
/// main game and the game with nothing written.
#[test]
#[ignore = "statement written, not built"]
fn men_ships_and_tons_are_conserved() {
    todo!()
}
