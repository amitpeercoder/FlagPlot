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

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::choices::Moment;
use sim_core::command::{
    CommandKind, CommandRefusal, CommandState, ForcesLimit, IntentKind, Limit, Maker, NoticeKind,
    PlanOrder, PlanOutcome, PlanState, Reason, Staff, StaffDecisions, StaffOrder, Staffs,
};
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, IntentId, LandUnitId, ShipId, TaskForceId};
use sim_core::orders::{Cargo, LandUnitOrder, OrderKind, Posture, TaskForceOrder};
use sim_core::state::{Side, TruthState};
use sim_core::time::Pulse;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs, SupplyStaff};

mod common;

const BEGIN: &str = "# MARKER: the written part begins";
const END: &str = "# MARKER: the written part ends";
const A: Hex = Hex { q: 0, r: 0 };
const B: Hex = Hex { q: 4, r: 0 };
const C: Hex = Hex { q: 0, r: 6 };

fn path() -> PathBuf {
    common::data_dir()
        .join("scenarios/a-plan-the-commander-writes/137_a_plan_the_commander_writes.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

fn intent(raw: u32) -> IntentId {
    IntentId::new(raw)
}

fn changed(text: &str, original: &str, replacement: &str) -> String {
    common::replace_once(text, original, replacement)
}

/// The frozen step 1 figures; the scenario names its own ground figures.
fn load(text: &str) -> Scenario {
    common::from_text_with(text, &path(), &common::frozen()).expect("the scenario loads")
}

/// The frozen figures with scenario 53's table of aircraft for each ship,
/// as scenario 76's carrier property has it.
fn carrier_figures() -> sim_data::LoadOptions {
    common::altered_reference("s137", |dir| {
        common::edit_file(dir, "air_combat.toml", |text| {
            format!("{text}\ncruiser = 12\ndestroyer = 6\ntransport = 6\nauxiliary = 4\n")
        })
    })
}

fn load_carrier(text: &str) -> Scenario {
    common::from_text_with(text, &path(), &carrier_figures()).expect("the scenario loads")
}

/// The file with the written part taken out: the game with nothing written.
fn nothing_written() -> String {
    let t = text();
    let begin = t.find(BEGIN).expect("the begin marker");
    let end = t.find(END).expect("the end marker");
    format!("{}{}", &t[..begin], &t[end + END.len()..])
}

/// A command table of the Allied side, added to the end of the file.
fn command_table(day: u32, body: &str) -> String {
    format!("\n[[command]]\nday = {day}\nside = \"allied\"\n{body}\n")
}

fn with(extra: &str) -> String {
    format!("{}{}", text(), extra)
}

/// `text` with `extra` added before the first command.
fn before_commands(text: &str, extra: &str) -> String {
    changed(
        text,
        "# The Allied commander's commands",
        &format!("{extra}\n# The Allied commander's commands"),
    )
}

struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    truth: Vec<TruthState>,
    events: Vec<Event>,
    hashes: Vec<u64>,
    state: CommandState,
    /// The Allied plan orders of each morning, read at the staff's moment
    /// (the commander's commands taken in, the staff not yet asked), as
    /// scenario 84 reads them; and each intent's maker as the command state
    /// of that moment gives it.
    plan_orders: BTreeMap<u32, Vec<PlanOrder>>,
    makers: BTreeMap<u32, BTreeMap<u32, Maker>>,
    /// What `propose` of intent 1 gave on the mornings it was asked.
    proposed: BTreeMap<u32, sim_core::command::Proposal>,
    /// The order book as the engine numbered it: (number, day, order) of
    /// each Allied order, in number order.
    book: Vec<(u32, u32, Given)>,
}

fn play(text: &str) -> Played {
    play_from(load(text), BTreeMap::new(), &[])
}

fn play_carrier(text: &str) -> Played {
    play_from(load_carrier(text), BTreeMap::new(), &[])
}

/// Plays the game; the proposal of intent 1 is asked on the mornings in `ask`.
fn play_asking(text: &str, ask: &[u32]) -> Played {
    play_from(load(text), BTreeMap::new(), ask)
}

fn play_from(
    mut scenario: Scenario,
    mut handed: BTreeMap<u32, StaffDecisions>,
    ask: &[u32],
) -> Played {
    let held = staffs();
    let mut truth = vec![scenario.world.umpire().clone()];
    let mut plan_orders = BTreeMap::new();
    let mut makers = BTreeMap::new();
    let mut proposed = BTreeMap::new();
    loop {
        let day = scenario.world.day() + 1;
        let mut morning: Option<(Vec<PlanOrder>, BTreeMap<u32, Maker>)> = None;
        let mut asked = None;
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |world, moment| {
                if let Moment::Orders { .. } = moment {
                    let view = world.side_view(Side::Allied);
                    let orders = SupplyStaff.plan_orders(view);
                    let who = view
                        .command_state()
                        .intents_in_force()
                        .map(|i| (i.id.raw(), i.maker))
                        .collect();
                    morning = Some((orders, who));
                    if ask.contains(&day) {
                        asked = SupplyStaff.propose(view, intent(1));
                    }
                }
            })
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
        if let Some((orders, who)) = morning {
            plan_orders.insert(day, orders);
            makers.insert(day, who);
        }
        if let Some(found) = asked {
            proposed.insert(day, found);
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
        truth.push(scenario.world.umpire().clone());
    }
    Played {
        handed,
        truth,
        events: scenario.world.events().iter().cloned().collect(),
        hashes: scenario.world.day_hashes().to_vec(),
        state: scenario.world.command_state(Side::Allied).clone(),
        plan_orders,
        makers,
        proposed,
        book: scenario
            .world
            .orders()
            .filter(|o| o.side == Side::Allied)
            .map(|o| {
                (
                    o.id.umpire_raw(common::umpire_key()),
                    o.day_issued,
                    given_kind(&o.kind),
                )
            })
            .collect(),
    }
}

/// An order of the staff, plainly, with the task force or unit it is for.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Given {
    Landing(u32, Hex, Vec<u32>),
    LoadTo(u32, Fixed),
    Attack(u32),
    Defend(u32),
    LoadUnit(u32, u32),
    Route(u32, Vec<Hex>, Vec<Hex>),
    Unload(u32),
    Other(String),
}

fn given(o: &StaffOrder) -> Given {
    given_kind(&o.kind)
}

fn given_kind(kind: &OrderKind) -> Given {
    match kind {
        OrderKind::TaskForce { target, order } => {
            let tf = target.raw();
            match order {
                TaskForceOrder::PrepareLanding { hex, roster } => {
                    Given::Landing(tf, *hex, roster.iter().map(|u| u.raw()).collect())
                }
                TaskForceOrder::LoadTo {
                    cargo: Cargo::Supply,
                    weight_t,
                    ..
                } => Given::LoadTo(tf, *weight_t),
                TaskForceOrder::LoadUnit { unit, cargo: None } => Given::LoadUnit(tf, unit.raw()),
                TaskForceOrder::SetRoute {
                    waypoints,
                    act_at,
                    stand_at,
                } if stand_at.is_empty() => {
                    Given::Route(tf, waypoints.clone(), act_at.iter().copied().collect())
                }
                TaskForceOrder::Unload => Given::Unload(tf),
                other => Given::Other(format!("task force {tf}: {other:?}")),
            }
        }
        OrderKind::LandUnit {
            target,
            order:
                LandUnitOrder::SetPosture {
                    posture,
                    when_spent: None,
                },
        } => match posture {
            Posture::Attack => Given::Attack(target.raw()),
            Posture::Defend => Given::Defend(target.raw()),
        },
        other => Given::Other(format!("{other:?}")),
    }
}

/// A staff command, plainly, by kind.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Cmd {
    Approve(u32),
    Form(u32, IntentKind),
    /// A forces limit: intent, task forces, land units, source.
    Limit(u32, Vec<u32>, Vec<u32>, Option<u32>),
    Complete(u32),
    Withdraw(u32),
    Other(String),
}

fn cmd(c: &CommandKind) -> Cmd {
    match c {
        CommandKind::Approve { plan } => Cmd::Approve(plan.raw()),
        CommandKind::FormIntent { parent, kind } => Cmd::Form(parent.raw(), *kind),
        CommandKind::SetLimit(Limit::Forces { intent, limit }) => {
            let ForcesLimit {
                ships,
                task_forces,
                air_groups,
                source,
                land_units,
                ..
            } = limit;
            assert!(ships.is_empty() && air_groups.is_empty(), "{limit:?}");
            Cmd::Limit(
                intent.raw(),
                task_forces.iter().map(|t| t.raw()).collect(),
                land_units.iter().map(|u| u.raw()).collect(),
                source.map(|b| b.raw()),
            )
        }
        CommandKind::MarkPlan {
            plan,
            outcome: PlanOutcome::Complete,
        } => Cmd::Complete(plan.raw()),
        CommandKind::WithdrawIntent { intent } => Cmd::Withdraw(intent.raw()),
        other => Cmd::Other(format!("{other:?}")),
    }
}

fn take_kind(hex: Hex, by_day: Option<u32>) -> IntentKind {
    IntentKind::TakePlace { hex, by_day }
}

fn hold_kind(base: u32) -> IntentKind {
    IntentKind::Hold {
        base: BaseId::new(base),
    }
}

fn deny_kind() -> IntentKind {
    IntentKind::Deny {
        hex: B,
        distance: 3,
    }
}

fn neutralise_kind(base: u32) -> IntentKind {
    IntentKind::Neutralise {
        target: BaseId::new(base),
    }
}

fn keep_kind(base: u32) -> IntentKind {
    IntentKind::KeepSupplied {
        base: BaseId::new(base),
        floor_days: 5,
        interval_days: None,
    }
}

fn landing_kind(force: u32, hex: Hex, by_day: Option<u32>) -> IntentKind {
    IntentKind::ReadyLanding {
        force: TaskForceId::new(force),
        hex,
        by_day,
    }
}

fn text_of(reason: &Reason) -> String {
    format!(
        "{} | {} | {} | {}",
        reason.worked,
        reason.binding.join("; "),
        reason
            .evidence
            .iter()
            .map(|e| e.what.clone())
            .collect::<Vec<_>>()
            .join("; "),
        reason.would_change.join("; ")
    )
}

fn assert_says(all: &str, parts: &[&str]) {
    for part in parts {
        assert!(all.contains(part), "the reason says {part:?}: {all}");
    }
}

impl Played {
    fn days(&self) -> u32 {
        self.handed.len() as u32
    }

    fn orders(&self, day: u32) -> Vec<Given> {
        self.handed[&day].orders.iter().map(given).collect()
    }

    fn all_orders(&self) -> Vec<(u32, Given)> {
        (1..=self.days())
            .flat_map(|d| self.orders(d).into_iter().map(move |g| (d, g)))
            .collect()
    }

    fn commands(&self, day: u32) -> Vec<Cmd> {
        self.handed[&day]
            .commands
            .iter()
            .map(|c| cmd(&c.kind))
            .collect()
    }

    fn all_commands(&self) -> Vec<(u32, Cmd)> {
        (1..=self.days())
            .flat_map(|d| self.commands(d).into_iter().map(move |c| (d, c)))
            .collect()
    }

    /// The forces limit the staff set for `of` on `day`: task forces, land
    /// units and source.
    fn staff_limit(&self, day: u32, of: u32) -> Option<(Vec<u32>, Vec<u32>, Option<u32>)> {
        self.commands(day).into_iter().find_map(|c| match c {
            Cmd::Limit(i, tfs, units, source) if i == of => Some((tfs, units, source)),
            _ => None,
        })
    }

    /// Every intent the staff formed: (day, parent, kind), in order.
    fn forms(&self) -> Vec<(u32, u32, IntentKind)> {
        self.all_commands()
            .into_iter()
            .filter_map(|(d, c)| match c {
                Cmd::Form(p, k) => Some((d, p, k)),
                _ => None,
            })
            .collect()
    }

    /// Every "take a place" the staff formed: its day and place.
    fn staff_takes(&self) -> Vec<(u32, Hex)> {
        self.forms()
            .into_iter()
            .filter_map(|(d, _, k)| match k {
                IntentKind::TakePlace { hex, .. } => Some((d, hex)),
                _ => None,
            })
            .collect()
    }

    fn notices(&self) -> Vec<(u32, NoticeKind, u32)> {
        (1..=self.days())
            .flat_map(|d| {
                self.handed[&d]
                    .notices
                    .iter()
                    .map(move |nt| (d, nt.kind, nt.plan.raw()))
            })
            .collect()
    }

    /// The reasons for giving no order of `day` that serve `of`.
    fn no_orders(&self, day: u32, of: u32) -> Vec<String> {
        self.handed[&day]
            .no_orders
            .iter()
            .filter(|r| r.serves.intent == Some(intent(of)))
            .map(text_of)
            .collect()
    }

    /// Every reason of `day` (commands, orders and decisions to give no
    /// order) that serves intent `of`, as text.
    fn reasons_serving(&self, day: u32, of: u32) -> String {
        let d = &self.handed[&day];
        let mut all: Vec<&Reason> = d.commands.iter().map(|c| &c.reason).collect();
        all.extend(d.orders.iter().map(|o| &o.reason));
        all.extend(d.no_orders.iter());
        all.into_iter()
            .filter(|r| r.serves.intent == Some(intent(of)))
            .map(text_of)
            .collect::<Vec<_>>()
            .join(" || ")
    }

    fn unit_state(&self, day: u32, u: u32) -> &sim_core::state::LandUnit {
        &self.truth[day as usize].land_units[&LandUnitId::new(u)]
    }

    fn base(&self, day: u32, b: u32) -> Option<&sim_core::state::Base> {
        self.truth[day as usize].bases.get(&BaseId::new(b))
    }

    /// The supply of base `b` at the end of `day`.
    fn supply(&self, day: u32, b: u32) -> Fixed {
        self.base(day, b).expect("the base exists").supply_t
    }

    fn ship(&self, day: u32, s: u32) -> &sim_core::state::Ship {
        &self.truth[day as usize].ships[&ShipId::new(s)]
    }

    fn plan_day(&self, plan: u32, to: PlanState) -> Option<u32> {
        self.events.iter().find_map(|e| match &e.kind {
            EventKind::PlanStateChanged {
                side,
                plan: p,
                to: t,
                ..
            } if *side == Side::Allied && p.raw() == plan && *t == to => Some(e.day),
            _ => None,
        })
    }

    /// (day, from, to) of every change of the state of plan `plan`.
    fn plan_changes(&self, plan: u32) -> Vec<(u32, PlanState, PlanState)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::PlanStateChanged {
                    side: Side::Allied,
                    plan: p,
                    from,
                    to,
                } if p.raw() == plan => Some((e.day, *from, *to)),
                _ => None,
            })
            .collect()
    }

    fn changed_side_day(&self, b: u32) -> Option<u32> {
        self.events.iter().find_map(|e| match &e.kind {
            EventKind::BaseChangedSide { base, .. } if base.raw() == b => Some(e.day),
            _ => None,
        })
    }

    /// (day, intent, parent) of every intent the staff formed.
    fn formed_intents(&self) -> Vec<(u32, u32, u32)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::IntentFormed {
                    side: Side::Allied,
                    intent: i,
                    parent: p,
                } => Some((e.day, i.raw(), p.raw())),
                _ => None,
            })
            .collect()
    }

    /// (day, intent, parent) of every intent withdrawn with the one above.
    fn withdrawn_with(&self) -> Vec<(u32, u32, u32)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::IntentWithdrawn {
                    side: Side::Allied,
                    intent: i,
                    parent: p,
                } => Some((e.day, i.raw(), p.raw())),
                _ => None,
            })
            .collect()
    }

    /// (day, plan, intent) of every plan drafted.
    fn plans_drafted(&self) -> Vec<(u32, u32, u32)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::PlanDrafted {
                    side: Side::Allied,
                    plan,
                    intent: i,
                } => Some((e.day, plan.raw(), i.raw())),
                _ => None,
            })
            .collect()
    }

    /// The commands the commander gave that were refused, by day.
    fn commander_refused(&self) -> Vec<(u32, CommandKind, CommandRefusal)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::CommandRefused {
                    side: Side::Allied,
                    issuer: sim_core::orders::Issuer::Player,
                    command,
                    reason,
                } => Some((e.day, command.clone(), *reason)),
                _ => None,
            })
            .collect()
    }

    fn intent_state(&self, raw: u32) -> &sim_core::command::Intent {
        &self.state.intents[&intent(raw)]
    }

    /// (hour, from) of every steaming of task force `tf`: the game hour of
    /// the pulse it begins in.
    fn steaming(&self, tf: u32) -> Vec<(Fixed, Hex)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::TaskForceSteamed {
                    task_force, from, ..
                } if task_force.raw() == tf => {
                    Some((sim_core::time::pulse_start(e.day, e.pulse?), *from))
                }
                _ => None,
            })
            .collect()
    }

    /// (hex, hour) of every arrival of task force `tf`.
    fn arrivals(&self, tf: u32) -> Vec<(Hex, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::TaskForceArrived {
                    task_force,
                    hex,
                    elapsed_hours,
                }
                | EventKind::TaskForceHalted {
                    task_force,
                    hex,
                    elapsed_hours,
                    ..
                } if task_force.raw() == tf => Some((*hex, *elapsed_hours)),
                _ => None,
            })
            .collect()
    }

    /// (base, hours) of every complete unloading of task force `tf`.
    fn unloading_complete(&self, tf: u32) -> Vec<(u32, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::UnloadingComplete {
                    task_force,
                    base,
                    elapsed_hours,
                } if task_force.raw() == tf => Some((base.raw(), *elapsed_hours)),
                _ => None,
            })
            .collect()
    }

    /// (day, pulse, unit, ship, men) of every landing of men.
    fn landed(&self) -> Vec<(u32, Pulse, u32, u32, u32)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::MenLanded {
                    unit, ship, men, ..
                } => Some((e.day, e.pulse?, unit.raw(), ship.raw(), *men)),
                _ => None,
            })
            .collect()
    }

    /// (day, pulse, ship, tons) of every landing of supply.
    fn supply_landed(&self) -> Vec<(u32, Pulse, u32, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::CargoUnloaded {
                    ship,
                    cargo: Cargo::Supply,
                    unloaded_t,
                    ..
                } => Some((e.day, e.pulse?, ship.raw(), *unloaded_t)),
                _ => None,
            })
            .collect()
    }

    fn refused(&self) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|e| {
                matches!(
                    e.kind,
                    EventKind::OrderRejected { .. } | EventKind::CommandRefused { .. }
                )
            })
            .collect()
    }

    fn nothing_refused(&self) {
        assert!(self.refused().is_empty(), "{:?}", self.refused());
    }

    fn order_of(&self, day: u32, intent: u32) -> &PlanOrder {
        self.plan_orders[&day]
            .iter()
            .find(|o| o.intent.raw() == intent)
            .unwrap_or_else(|| panic!("a plan order of intent {intent} on morning {day}"))
    }
}

fn main_game() -> &'static Played {
    static GAME: std::sync::OnceLock<Played> = std::sync::OnceLock::new();
    GAME.get_or_init(|| play(&text()))
}

fn empty_game() -> &'static Played {
    static GAME: std::sync::OnceLock<Played> = std::sync::OnceLock::new();
    GAME.get_or_init(|| play(&nothing_written()))
}

// ---------------------------------------------------------------- The variants

/// The written part, replaced by `written` (command tables).
fn with_written(text: &str, written: &str) -> String {
    let begin = text.find(BEGIN).expect("the begin marker");
    let end = text.find(END).expect("the end marker");
    format!("{}{}{}", &text[..begin], written, &text[end + END.len()..])
}

fn set_intent(intent: u32, parent: Option<u32>, hex: &str) -> String {
    let parent = parent.map_or_else(String::new, |p| format!("parent = {p}\n"));
    command_table(
        1,
        &format!(
            "kind = \"set_intent\"\nintent = {intent}\n{parent}intent_kind = \"take_place\"\nhex = {hex}"
        ),
    )
}

fn forces(day: u32, intent: u32, body: &str) -> String {
    command_table(
        day,
        &format!("kind = \"set_limit\"\nlimit = \"forces\"\nintent = {intent}\n{body}"),
    )
}

fn limit(day: u32, intent: u32, body: &str) -> String {
    command_table(
        day,
        &format!("kind = \"set_limit\"\nintent = {intent}\n{body}"),
    )
}

/// The written part with the numbers the other way round (the variant
/// "Whichever is served first"): intent 2 is the take of B, naming nothing,
/// with "after intent 3" and "not before day 4"; intent 3 is the take of C
/// with task force 3 and unit 3. Unit 3 is given experience 3.
fn swapped() -> String {
    let written = format!(
        "{}{}{}{}{}",
        set_intent(2, Some(1), "[4, 0]"),
        set_intent(3, Some(1), "[0, 6]"),
        forces(1, 3, "task_forces = [3]\nland_units = [3]"),
        limit(1, 2, "limit = \"after\"\nother_intent = 3"),
        limit(1, 2, "limit = \"not_before\"\nfrom_day = 4"),
    );
    with_written(&text(), &written)
}

fn experienced(text: String) -> String {
    changed(
        &text,
        "name = \"Placeholder Third\"\nside = \"allied\"\nmen = 600\n",
        "name = \"Placeholder Third\"\nside = \"allied\"\nmen = 600\nexperience = 3\n",
    )
}

/// The opening estimate of 400 enemy men at B and a forces limit of land
/// unit 1 on intent 3 (the variant "A mediocre plan waits").
fn mediocre() -> String {
    let estimate = "# The Allied side's opening estimate of the enemy's men at B.\n[[estimate]]\nside = \"allied\"\nq = 4\nr = 0\nmen = 400\n";
    let text = before_commands(&text(), estimate);
    format!("{text}{}", forces(1, 3, "land_units = [1]"))
}

/// "A mediocre plan at press": the island's risk set to press on day 1,
/// after intent 1 is stated.
fn mediocre_at_press() -> String {
    format!(
        "{}{}",
        mediocre(),
        limit(1, 1, "limit = \"risk\"\nrisk = \"press\"")
    )
}

fn cancelled() -> String {
    with(&command_table(2, "kind = \"cancel_plan\"\nplan = 3"))
}

fn given_back() -> String {
    format!(
        "{}{}{}",
        cancelled(),
        set_intent_on(3, 20, Some(1), "[4, 0]"),
        command_table(4, "kind = \"withdraw_intent\"\nintent = 3")
    )
}

fn set_intent_on(day: u32, intent: u32, parent: Option<u32>, hex: &str) -> String {
    let parent = parent.map_or_else(String::new, |p| format!("parent = {p}\n"));
    command_table(
        day,
        &format!(
            "kind = \"set_intent\"\nintent = {intent}\n{parent}intent_kind = \"take_place\"\nhex = {hex}"
        ),
    )
}

const ISLAND: &str = "intent_kind = \"control_island\"\nisland = 1\n";

fn island_dated(day: u32) -> String {
    changed(&text(), ISLAND, &format!("{ISLAND}by_day = {day}\n"))
}

fn later_date_text() -> String {
    changed(
        &island_dated(12),
        "intent = 3\nparent = 1\nintent_kind = \"take_place\"\nhex = [4, 0]\n",
        "intent = 3\nparent = 1\nintent_kind = \"take_place\"\nhex = [4, 0]\nby_day = 14\n",
    )
}

fn launch_withheld() -> String {
    let t = changed(
        &text(),
        "leave = \"launch\"\ngiven = true",
        "leave = \"launch\"\ngiven = false",
    );
    format!(
        "{t}{}",
        command_table(3, "kind = \"set_leave\"\nleave = \"launch\"\ngiven = true")
    )
}

fn with_a_carrier(text: &str) -> String {
    let carrier = "# Task force 2: one placeholder carrier, stopped in port at base 1, its home\n# port, as scenario 73 has it.\n[[task_force]]\nid = 2\nname = \"Placeholder carrier force 2\"\nside = \"allied\"\nat_base = 1\nmission = \"carrier\"\nhome_port = 1\n\n[[ship]]\nid = 3\nname = \"Placeholder carrier 3\"\nclass = 4\nside = \"allied\"\nfuel_t = 3000\ntask_force = 2\n\n[[air_group]]\nid = 1\nname = \"Placeholder fighters 1\"\nside = \"allied\"\nship = 3\naircraft_type = 4\nready = 12\n\n[[air_group]]\nid = 2\nname = \"Placeholder carrier scouts 2\"\nside = \"allied\"\nship = 3\naircraft_type = 2\nready = 6\n\n[[air_group]]\nid = 3\nname = \"Placeholder dive bombers 3\"\nside = \"allied\"\nship = 3\naircraft_type = 3\nready = 10\n\n";
    before_commands(text, carrier)
}

/// A carrier force named in intent 2's forces limit.
fn carrier_named() -> String {
    with_a_carrier(&changed(
        &text(),
        "task_forces = [3]\nland_units = [3]",
        "task_forces = [2, 3]\nland_units = [3]",
    ))
}

/// A battleship force, task force 5 with ship 5, on the bombardment mission,
/// named in intent 2's forces limit.
fn bombardment_named() -> String {
    let force = "# Task force 5: scenario 74's placeholder battleship, stopped in port at\n# base 1, its home port, on the bombardment mission.\n[[task_force]]\nid = 5\nname = \"Placeholder bombardment force 5\"\nside = \"allied\"\nat_base = 1\nmission = \"bombardment\"\nstance = \"avoid\"\nhome_port = 1\n\n[[ship]]\nid = 5\nname = \"Placeholder battleship 5\"\nclass = 9\nside = \"allied\"\nfuel_t = 2000\ntask_force = 5\n\n";
    let t = changed(
        &text(),
        "task_forces = [3]\nland_units = [3]",
        "task_forces = [3, 5]\nland_units = [3]",
    );
    changed(
        &before_commands(&t, force),
        "    \"../../reference/ship_classes_placeholder_transports.csv\",\n]",
        "    \"../../reference/ship_classes_placeholder_transports.csv\",\n    \"../../reference/ship_classes_placeholder_battleship.csv\",\n]",
    )
}

/// A forces limit of land units 1 and 2 on intent 3 and the commander's hold
/// of land unit 2, both on day 1.
fn unit_named_and_held() -> String {
    format!(
        "{}{}{}",
        text(),
        forces(1, 3, "land_units = [1, 2]"),
        command_table(1, "kind = \"hold\"\nland_unit = 2")
    )
}

fn island_withdrawn_text() -> String {
    with(&command_table(4, "kind = \"withdraw_intent\"\nintent = 1"))
}

/// Every game of the scenario the properties hold in: the main game and the
/// variants, with whether each is played on the carrier figures.
fn every_game() -> Vec<(&'static str, String, bool)> {
    vec![
        ("main", text(), false),
        ("nothing written", nothing_written(), false),
        ("swapped", experienced(swapped()), false),
        (
            "the day alone holds it",
            changed(&text(), "from_day = 4", "from_day = 8"),
            false,
        ),
        ("mediocre", mediocre(), false),
        ("mediocre at press", mediocre_at_press(), false),
        ("cancelled", cancelled(), false),
        ("given back", given_back(), false),
        ("the island's date", island_dated(12), false),
        ("his own later date", later_date_text(), false),
        ("launch withheld", launch_withheld(), false),
        ("a carrier named", carrier_named(), true),
        ("a bombardment force named", bombardment_named(), false),
        ("a unit named and held", unit_named_and_held(), false),
        ("withdrawn with its island", island_withdrawn_text(), false),
    ]
}

fn play_every(name: &str) -> &'static Played {
    static GAMES: std::sync::OnceLock<BTreeMap<&'static str, Played>> = std::sync::OnceLock::new();
    &GAMES.get_or_init(|| {
        every_game()
            .into_iter()
            .map(|(name, text, carrier)| {
                (
                    name,
                    if carrier {
                        play_carrier(&text)
                    } else {
                        play(&text)
                    },
                )
            })
            .collect()
    })[name]
}

// ---------------------------------------------------------------- The statements

/// Statement 1: intents 2 and 3 stand under intent 1 with the commander
/// as maker; plans 2 and 3 are drafted on day 1; each is recorded by its
/// `CommandTaken` and a `PlanDrafted`, and no `IntentFormed` names either.
#[test]
fn taken_in() {
    let g = main_game();
    for (raw, plan, hex) in [(2u32, 2u32, C), (3, 3, B)] {
        let i = g.intent_state(raw);
        assert_eq!(i.parent, Some(intent(1)), "intent {raw}");
        assert_eq!(i.maker, Maker::Commander, "intent {raw}");
        assert_eq!(i.plan.raw(), plan, "intent {raw}");
        assert_eq!(i.set_day, 1);
        assert_eq!(i.kind, take_kind(hex, None), "intent {raw}");
    }
    // Plans 2 and 3 are drafted on day 1, after plan 1.
    let drafted = g.plans_drafted();
    assert_eq!(drafted[..3], [(1, 1, 1), (1, 2, 2), (1, 3, 3)]);
    // Each is recorded by its CommandTaken, which holds its parent, and each
    // PlanDrafted cites that CommandTaken.
    for (raw, plan, hex) in [(2u32, 2u32, C), (3, 3, B)] {
        let taken = g
            .events
            .iter()
            .find(|e| {
                matches!(&e.kind, EventKind::CommandTaken {
                    side: Side::Allied,
                    command: CommandKind::SetIntent { intent: i, parent, .. },
                    ..
                } if i.raw() == raw && *parent == Some(intent(1)))
            })
            .unwrap_or_else(|| panic!("a CommandTaken for intent {raw}"));
        assert_eq!(taken.day, 1);
        assert!(matches!(
            &taken.kind,
            EventKind::CommandTaken { command: CommandKind::SetIntent { kind, .. }, .. }
                if *kind == take_kind(hex, None)
        ));
        let drafted = g
            .events
            .iter()
            .find(|e| {
                matches!(&e.kind, EventKind::PlanDrafted { plan: p, intent: i, .. }
                    if p.raw() == plan && i.raw() == raw)
            })
            .unwrap_or_else(|| panic!("a PlanDrafted for plan {plan}"));
        assert!(
            drafted
                .causes
                .contains(&sim_core::events::Cause::Event(taken.id)),
            "plan {plan} cites its command"
        );
    }
    // No IntentFormed names either: that event is a form_intent's.
    for (_, formed, _) in g.formed_intents() {
        assert!(formed != 2 && formed != 3, "intent {formed} was formed");
    }
    // Nothing is refused.
    g.nothing_refused();
}

/// Statement 2: the tree of the first morning, intents 4 to 10 in order
/// with their plans; on no morning of the 16 is a take of B or of C
/// formed; the island's reason of each morning names intents 2 and 3 as
/// the commander's while each stands unmet.
#[test]
fn nothing_beside_them() {
    let g = main_game();
    // The tree of the first morning: intents 4 to 10, in order, each formed
    // on day 1.
    assert_eq!(
        g.formed_intents()
            .into_iter()
            .filter(|(d, ..)| *d == 1)
            .collect::<Vec<_>>(),
        vec![
            (1, 4, 1),
            (1, 5, 1),
            (1, 6, 1),
            (1, 7, 1),
            (1, 8, 1),
            (1, 9, 2),
            (1, 10, 3)
        ]
    );
    let kinds = [
        (4, hold_kind(2), 1),
        (5, hold_kind(4), 1),
        (6, deny_kind(), 1),
        (7, neutralise_kind(2), 1),
        (8, neutralise_kind(4), 1),
        (9, landing_kind(3, C, None), 2),
        (10, landing_kind(1, B, None), 3),
    ];
    for (raw, kind, plan) in kinds {
        let i = g.intent_state(raw);
        assert_eq!(i.kind, kind, "intent {raw}");
        assert_eq!(i.maker, Maker::Staff, "intent {raw}");
        assert_eq!(i.plan.raw(), plan, "intent {raw}");
    }
    // The staff forms no take of B or of C on any morning of the 16.
    assert!(g.staff_takes().is_empty(), "{:?}", g.staff_takes());
    // It forms the holds of both places, the denial and the neutralisings as
    // the game with nothing written does.
    let not_taking = |g: &Played| -> Vec<(u32, u32, IntentKind)> {
        g.forms()
            .into_iter()
            .filter(|(d, _, k)| {
                *d == 1
                    && !matches!(
                        k,
                        IntentKind::TakePlace { .. } | IntentKind::ReadyLanding { .. }
                    )
            })
            .collect()
    };
    let mut ours = not_taking(g);
    let mut theirs = not_taking(empty_game());
    ours.sort_by_key(|(d, _, k)| (*d, format!("{k:?}")));
    theirs.sort_by_key(|(d, _, k)| (*d, format!("{k:?}")));
    assert_eq!(ours, theirs);
    // The island's reason of each morning names intents 2 and 3 as the
    // commander's while each stands unmet: intent 3 while B is not the
    // side's (to day 9), intent 2 while C is not (to day 5), in the order of
    // the island's places.
    let line = |raw: u32, hex: &str| {
        format!("intent:{raw}, the commander's, plans the place at {hex}: the staff forms no take beside it")
    };
    for day in 1..=16 {
        let all = g.no_orders(day, 1).join(" || ");
        assert_eq!(
            all.contains(&line(3, "(4,0)")),
            day <= 9,
            "day {day}: {all}"
        );
        assert_eq!(
            all.contains(&line(2, "(0,6)")),
            day <= 5,
            "day {day}: {all}"
        );
        if day <= 5 {
            assert!(
                all.find(&line(3, "(4,0)")) < all.find(&line(2, "(0,6)")),
                "day {day}: the places are in the island's order"
            );
        }
    }
}

/// Statement 3: the landing under intent 2 has task force 3 and unit 3
/// and no other, and the staff's limit on intent 9 names base 1 alone;
/// the landing under intent 3 has task force 1 and units 1 and 2, and its
/// reason names intent 2 for task force 3 and unit 3.
#[test]
fn his_forces_and_his_blanks() {
    let g = main_game();
    // The landing under intent 2 has task force 3 and unit 3 and no other;
    // the staff's limit on intent 9 names what he left blank and nothing
    // else: base 1 as the source.
    assert_eq!(g.intent_state(9).kind, landing_kind(3, C, None));
    assert_eq!(g.staff_limit(1, 9), Some((vec![], vec![], Some(1))));
    // The landing under intent 3 has what the staff chooses of what no node
    // of his names: task force 1 and units 1 and 2, with base 1.
    assert_eq!(g.intent_state(10).kind, landing_kind(1, B, None));
    assert_eq!(g.staff_limit(1, 10), Some((vec![1], vec![1, 2], Some(1))));
    // Orders 1 and 2 are the two landings.
    assert_eq!(
        g.book[..2],
        [
            (1, 1, Given::Landing(1, B, vec![1, 2])),
            (2, 1, Given::Landing(3, C, vec![3]))
        ]
    );
    // Intent 3's reason names intent 2 as the node that has task force 3 and
    // unit 3.
    let reason = g.reasons_serving(1, 3);
    assert_says(
        &reason,
        &[
            "task_force:3 is kept for intent:2",
            "land_unit:3 (Placeholder Third) is kept for intent:2",
        ],
    );
}

/// Statement 3's variant, "Whichever is served first": day 1 with the
/// numbers the other way round and unit 3 at experience 3, and the game
/// with nothing written on the same file beside it.
#[test]
fn whichever_is_served_first() {
    let g = play(&experienced(swapped()));
    // Intent 2 is the take of B and is served first.
    assert_eq!(g.intent_state(2).kind, take_kind(B, None));
    assert_eq!(g.intent_state(3).kind, take_kind(C, None));
    // Intent 9, ready a landing by task force 1 at B, is formed under intent
    // 2 with units 1 and 2; intent 10, ready a landing by task force 3 at C,
    // under intent 3 with unit 3.
    assert_eq!(
        g.formed_intents()
            .into_iter()
            .filter(|(d, i, _)| *d == 1 && *i >= 9)
            .collect::<Vec<_>>(),
        vec![(1, 9, 2), (1, 10, 3)]
    );
    assert_eq!(g.intent_state(9).kind, landing_kind(1, B, None));
    assert_eq!(g.intent_state(10).kind, landing_kind(3, C, None));
    assert_eq!(g.staff_limit(1, 9), Some((vec![1], vec![1, 2], Some(1))));
    assert_eq!(g.staff_limit(1, 10), Some((vec![], vec![], Some(1))));
    // Order 1 is task force 1's, roster 1 and 2; order 2 task force 3's,
    // roster 3. Day 1 is asserted.
    assert_eq!(
        g.book[..2],
        [
            (1, 1, Given::Landing(1, B, vec![1, 2])),
            (2, 1, Given::Landing(3, C, vec![3]))
        ]
    );
    // Its reason names intent 3 as the node that has task force 3 and unit 3.
    let reason = g.reasons_serving(1, 2);
    assert_says(
        &reason,
        &[
            "task_force:3 is kept for intent:3",
            "land_unit:3 (Placeholder Third) is kept for intent:3",
        ],
    );
    // Left to itself it would take unit 3 first, the higher experience, and
    // unit 1 with it, as the staff's own plan does on this file (in the game
    // with nothing written and unit 3 at experience 3, the landing at B has
    // task force 1 with units 3 and 1 and the landing at C task force 3 with
    // unit 2).
    let own = play(&experienced(nothing_written()));
    assert_eq!(own.intent_state(9).kind, landing_kind(1, B, None));
    assert_eq!(own.staff_limit(1, 9), Some((vec![1], vec![1, 3], Some(1))));
    assert_eq!(own.intent_state(10).kind, landing_kind(3, C, None));
    assert_eq!(own.staff_limit(1, 10), Some((vec![3], vec![2], Some(1))));
}

/// Statement 4: plan 2 released on day 2; plan 3 preparing on days 2 to
/// 5 with no leave asked and no waits-for-leave notice, the reason of
/// each morning naming the waits of the table; plan 3 released on day 6.
#[test]
fn his_order() {
    let g = main_game();
    // Plan 2 is released on day 2.
    assert_eq!(g.plan_day(2, PlanState::Preparing), Some(1));
    assert_eq!(g.plan_day(2, PlanState::Released), Some(2));
    // Plan 3 is readied on day 1 and released on day 6, two days after its
    // "not before".
    assert_eq!(g.plan_day(3, PlanState::Preparing), Some(1));
    assert_eq!(g.plan_day(3, PlanState::Released), Some(6));
    // No leave is asked for it and no notice that it waits for leave is
    // handed in.
    for (day, kind, plan) in g.notices() {
        assert!(
            !matches!(kind, NoticeKind::WaitsForLeave(_)),
            "day {day}: plan {plan}"
        );
    }
    // The reason of each morning names each wait still unmet.
    let not_before = "it is not to go before day 4";
    let after = "the place of intent:2 at (0,6) is not yet the side's";
    for day in 2..=5 {
        let reason = g.reasons_serving(day, 3);
        assert_eq!(reason.contains(not_before), day <= 3, "day {day}: {reason}");
        assert!(reason.contains(after), "day {day}: {reason}");
        assert!(
            reason.contains("limit after intent:2"),
            "day {day}: {reason}"
        );
    }
    // On the morning of day 6 both waits are met and the plan is released.
    let day6 = g.reasons_serving(6, 3);
    assert!(
        !day6.contains(not_before) && !day6.contains(after),
        "{day6}"
    );
    // Intent 2 has no wait: it is readied on day 1 and released on day 2.
    for day in 1..=2 {
        let reason = g.reasons_serving(day, 2);
        assert!(
            !reason.contains("told the staff to wait"),
            "day {day}: {reason}"
        );
    }
    // Base 4 is the side's from the end of day 5.
    assert_eq!(g.base(4, 4).map(|b| b.side), Some(Side::Japanese));
    assert_eq!(g.changed_side_day(4), Some(5));
}

/// Statement 4's variant, "The day alone holds it": days 6 to 9 with
/// "not before day 8", and base 2 the side's at the end of day 11.
#[test]
fn the_day_alone_holds_it() {
    let g = play(&changed(&text(), "from_day = 4", "from_day = 8"));
    // On mornings 2 to 5 the reason names both waits.
    for day in 2..=5 {
        let reason = g.reasons_serving(day, 3);
        assert_says(
            &reason,
            &[
                "it is not to go before day 8",
                "the place of intent:2 at (0,6) is not yet the side's",
            ],
        );
    }
    // On mornings 6 and 7 it names "not before day 8" alone, base 4 being
    // the side's.
    for day in 6..=7 {
        let reason = g.reasons_serving(day, 3);
        assert_says(&reason, &["it is not to go before day 8"]);
        assert!(
            !reason.contains("is not yet the side's"),
            "day {day}: {reason}"
        );
    }
    // On the morning of day 8 plan 3 is approved to released and task force
    // 1 is ordered to load supply to 54 t (base 1 holds 922 t that morning).
    assert_eq!(g.plan_day(3, PlanState::Released), Some(8));
    assert!(
        g.orders(8).contains(&Given::LoadTo(1, n(54))),
        "{:?}",
        g.orders(8)
    );
    assert_eq!(g.supply(7, 1), n(922));
    // Unit 1 is loaded on day 9 and unit 2 on day 10.
    assert!(
        g.orders(9).contains(&Given::LoadUnit(1, 1)),
        "{:?}",
        g.orders(9)
    );
    assert!(
        g.orders(10).contains(&Given::LoadUnit(1, 2)),
        "{:?}",
        g.orders(10)
    );
    // The force sails at hour 240; base 2 changes side at the end of day 11
    // and plan 3 is marked complete on day 12.
    assert_eq!(g.steaming(1)[0], (n(240), A));
    assert_eq!(g.changed_side_day(2), Some(11));
    assert_eq!(g.plan_day(3, PlanState::Complete), Some(12));
    // On day 9 run 1 of plan 4, 45 t, is ordered with ship 4 as in the main
    // game: task force 1 lies at base 1 that morning with nothing aboard but
    // its 54 t, readying its landing, and is not the run's.
    let added: Vec<String> = g
        .commands(9)
        .into_iter()
        .filter_map(|c| match c {
            Cmd::Other(text) if text.starts_with("AddRun") => Some(text),
            _ => None,
        })
        .collect();
    assert_eq!(added.len(), 1, "{added:?}");
    assert!(
        added[0].contains("plan: PlanId(4)")
            && added[0].contains("ships: {ShipId(4)}")
            && added[0].contains("load_t: 45"),
        "{}",
        added[0]
    );
}

/// Statement 5: the staff's commands and the 23 orders of the day-by-day
/// table, each on its day with its force or unit, its weight or its
/// roster; the hours each force sails and arrives; the men and tons
/// landed by pulse; the days bases 4 and 2 change side; none refused.
#[test]
fn the_days_and_the_orders() {
    let g = main_game();
    g.nothing_refused();
    use Given::{Attack, Defend, LoadTo, LoadUnit, Unload};
    let route = |tf: u32, to: Hex| Given::Route(tf, vec![to, A], vec![to]);
    // The 23 orders as the engine numbers them, each on its day with its
    // force or unit, its weight or its roster.
    let book: Vec<(u32, u32, Given)> = vec![
        (1, 1, Given::Landing(1, B, vec![1, 2])),
        (2, 1, Given::Landing(3, C, vec![3])),
        (3, 2, LoadTo(3, n(27))),
        (4, 3, LoadUnit(3, 3)),
        (5, 3, Attack(3)),
        (6, 4, route(3, C)),
        (7, 4, Unload(3)),
        (8, 6, LoadTo(1, n(54))),
        (9, 6, Defend(3)),
        (10, 7, LoadUnit(1, 1)),
        (11, 7, Attack(1)),
        (12, 8, LoadUnit(1, 2)),
        (13, 8, Attack(2)),
        (14, 9, route(1, B)),
        (15, 9, Unload(1)),
        (16, 9, LoadTo(3, n(45))),
        (17, 10, route(3, C)),
        (18, 10, Unload(3)),
        (19, 10, Defend(1)),
        (20, 10, Defend(2)),
        (21, 13, LoadTo(1, n(90))),
        (22, 14, route(1, B)),
        (23, 14, Unload(1)),
    ];
    assert_eq!(g.book, book);
    let counts: Vec<usize> = (1..=16).map(|d| g.orders(d).len()).collect();
    assert_eq!(counts, vec![2, 1, 2, 2, 0, 2, 2, 2, 3, 4, 0, 0, 1, 2, 0, 0]);

    // The staff's commands of each day.
    let approve = Cmd::Approve;
    let form = |parent: u32, kind: IntentKind| Cmd::Form(parent, kind);
    assert_eq!(
        g.commands(1),
        vec![
            approve(1),
            approve(1),
            approve(2),
            approve(3),
            form(1, hold_kind(2)),
            Cmd::Limit(4, vec![1, 3], vec![], Some(1)),
            form(1, hold_kind(4)),
            Cmd::Limit(5, vec![1, 3], vec![], Some(1)),
            form(1, deny_kind()),
            form(1, neutralise_kind(2)),
            form(1, neutralise_kind(4)),
            form(2, landing_kind(3, C, None)),
            Cmd::Limit(9, vec![], vec![], Some(1)),
            form(3, landing_kind(1, B, None)),
            Cmd::Limit(10, vec![1], vec![1, 2], Some(1)),
        ]
    );
    assert_eq!(g.commands(2), vec![approve(2)]);
    assert!(g.commands(3).is_empty() && g.commands(4).is_empty() && g.commands(5).is_empty());
    assert_eq!(
        g.commands(6),
        vec![
            Cmd::Withdraw(8),
            form(5, keep_kind(4)),
            Cmd::Complete(2),
            approve(3)
        ]
    );
    assert_eq!(g.commands(7), vec![approve(4), approve(4)]);
    assert_eq!(
        g.commands(10),
        vec![Cmd::Withdraw(7), form(4, keep_kind(2)), Cmd::Complete(3)]
    );
    assert_eq!(g.commands(11), vec![approve(5), approve(5)]);
    // The runs: 45 t with ship 4 on day 9 (plan 4), 90 t with ships 1 and 2
    // on day 13 (plan 5), each committed from its day to three days on.
    let runs = |day: u32| -> Vec<String> {
        g.commands(day)
            .into_iter()
            .filter_map(|c| match c {
                Cmd::Other(text) => Some(text),
                _ => None,
            })
            .collect()
    };
    let r9 = runs(9);
    assert_eq!(r9.len(), 2, "{r9:?}");
    assert!(
        r9[0].starts_with("AddRun { plan: PlanId(4)")
            && r9[0].contains("ships: {ShipId(4)}")
            && r9[0].contains("load_t: 45")
            && r9[0].contains("load: 9, sail: 10, arrive: 10"),
        "{}",
        r9[0]
    );
    assert!(
        r9[1].contains("CommitShips { plan: PlanId(4), run: Some(1), ships: {ShipId(4)}, from_day: 9, to_day: 12 }"),
        "{}",
        r9[1]
    );
    let r13 = runs(13);
    assert_eq!(r13.len(), 2, "{r13:?}");
    assert!(
        r13[0].starts_with("AddRun { plan: PlanId(5)")
            && r13[0].contains("ships: {ShipId(1), ShipId(2)}")
            && r13[0].contains("load_t: 90")
            && r13[0].contains("load: 13, sail: 14, arrive: 14"),
        "{}",
        r13[0]
    );
    assert!(r13[1].contains("from_day: 13, to_day: 16"), "{}", r13[1]);
    // Plans 2 and 3: marked complete on days 6 and 10, with the notices that
    // intents 2 and 3 are met.
    assert_eq!(g.plan_day(2, PlanState::Complete), Some(6));
    assert_eq!(g.plan_day(3, PlanState::Complete), Some(10));
    let met: Vec<(u32, u32)> = g
        .notices()
        .into_iter()
        .filter(|(_, k, _)| *k == NoticeKind::Met)
        .map(|(d, _, p)| (d, p))
        .collect();
    assert_eq!(met, vec![(6, 2), (10, 3)]);

    // The hours each force sails and arrives.
    let sails = |tf: u32, from: Hex, after: i64| {
        g.steaming(tf)
            .into_iter()
            .find(|(h, f)| *f == from && *h >= n(after))
            .map(|(h, _)| h)
    };
    assert_eq!(sails(3, A, 0), Some(n(72)));
    assert_eq!(sails(3, C, 96), Some(n(108)));
    assert_eq!(sails(3, A, 200), Some(n(216)));
    assert_eq!(sails(3, C, 240), Some(n(252)));
    assert_eq!(sails(1, A, 0), Some(n(192)));
    assert_eq!(sails(1, B, 200), Some(n(228)));
    assert_eq!(sails(1, A, 300), Some(n(312)));
    assert_eq!(sails(1, B, 330), Some(n(336)));
    assert_eq!(
        g.arrivals(3),
        vec![(C, n(96)), (A, n(132)), (C, n(240)), (A, n(276))]
    );
    assert_eq!(
        g.arrivals(1),
        vec![(B, n(208)), (A, n(244)), (B, n(328)), (A, n(352))]
    );
    // The men and the tons landed by pulse.
    assert_eq!(
        g.landed(),
        vec![
            (5, Pulse::Night, 3, 4, 600),
            (9, Pulse::Morning, 1, 1, 160),
            (9, Pulse::Morning, 2, 2, 160),
            (9, Pulse::Afternoon, 1, 1, 440),
            (9, Pulse::Afternoon, 2, 2, 440),
        ]
    );
    assert_eq!(
        g.supply_landed(),
        vec![
            (5, Pulse::Night, 4, n(27)),
            (9, Pulse::Afternoon, 1, n(10)),
            (10, Pulse::Night, 1, n(44)),
            (11, Pulse::Night, 4, n(45)),
            (14, Pulse::Morning, 1, n(40)),
            (14, Pulse::Afternoon, 1, n(50)),
        ]
    );
    // Unloading complete at hours 104.85, 218.2, 242.25 and 332.5.
    let done: Vec<Fixed> = [3, 1, 3, 1]
        .iter()
        .zip([0usize, 0, 1, 1])
        .map(|(tf, i)| g.unloading_complete(*tf)[i].1)
        .collect();
    assert_eq!(
        done,
        vec![fx("104.85"), fx("218.2"), fx("242.25"), fx("332.5")]
    );
    // Base 5, "Landing at (0,6)", is formed on day 5; base 6, "Landing at
    // (4,0)", on day 9. Base 4 changes side at the end of day 5, base 2 at
    // the end of day 9.
    assert_eq!(g.base(5, 5).expect("base 5").name, "Landing at (0,6)");
    assert!(g.base(4, 5).is_none());
    assert_eq!(g.base(9, 6).expect("base 6").name, "Landing at (4,0)");
    assert!(g.base(8, 6).is_none());
    assert_eq!(g.changed_side_day(4), Some(5));
    assert_eq!(g.changed_side_day(2), Some(9));
}

/// Statement 5: the supply's sums (runs of 45 t on day 9 with ship 4 and
/// of 90 t on day 13 with ships 1 and 2); the supply of the five bases
/// at the end of every day; the ships' fuel at the end; the men ashore
/// at the end of every day; no man unfed.
#[test]
fn supply_fuel_and_men() {
    let g = main_game();
    // The supply of the five bases at the end of every day (0 where the base
    // is not yet formed is left out: the blank cells of the table).
    let table: [[Option<i64>; 5]; 16] = [
        [Some(991), Some(0), Some(0), None, None],
        [Some(955), Some(0), Some(0), None, None],
        [Some(946), Some(0), Some(0), None, None],
        [Some(940), Some(0), Some(0), None, None],
        [Some(934), Some(0), Some(0), Some(27), None],
        [Some(874), Some(0), Some(0), Some(24), None],
        [Some(868), Some(0), Some(0), Some(21), None],
        [Some(865), Some(0), Some(0), Some(18), None],
        [Some(820), Some(0), Some(0), Some(15), Some(10)],
        [Some(820), Some(44), Some(0), Some(12), Some(4)],
        [Some(820), Some(38), Some(45), Some(9), Some(4)],
        [Some(820), Some(32), Some(42), Some(9), Some(4)],
        [Some(730), Some(26), Some(39), Some(9), Some(4)],
        [Some(730), Some(110), Some(36), Some(9), Some(4)],
        [Some(730), Some(104), Some(33), Some(9), Some(4)],
        [Some(730), Some(98), Some(30), Some(9), Some(4)],
    ];
    for (i, row) in table.iter().enumerate() {
        let day = (i + 1) as u32;
        for (b, want) in [1u32, 2, 4, 5, 6].iter().zip(row) {
            let held = g.base(day, *b).map(|base| base.supply_t);
            assert_eq!(held, want.map(n), "day {day}, base {b}");
        }
    }
    // The runs: 45 t on day 9 with ship 4 and 90 t on day 13 with ships 1
    // and 2 (the supply doctrine's sums: base 4's first stock below 15 t is
    // on day 9, base 2's below 30 t on day 13), and no second run.
    let adds: Vec<(u32, String)> = (1..=16)
        .flat_map(|d| {
            g.commands(d).into_iter().filter_map(move |c| match c {
                Cmd::Other(text) if text.starts_with("AddRun") => Some((d, text)),
                _ => None,
            })
        })
        .collect();
    assert_eq!(adds.len(), 2, "{adds:?}");
    assert_eq!(adds[0].0, 9);
    assert!(adds[0].1.contains("ships: {ShipId(4)}") && adds[0].1.contains("load_t: 45"));
    assert_eq!(adds[1].0, 13);
    assert!(
        adds[1].1.contains("ships: {ShipId(1), ShipId(2)}") && adds[1].1.contains("load_t: 90")
    );
    // The ships steam to B and back twice, and to C and back twice, and end
    // with 936 t and 904 t of fuel.
    assert_eq!(g.ship(16, 1).fuel_t, n(936));
    assert_eq!(g.ship(16, 2).fuel_t, n(936));
    assert_eq!(g.ship(16, 4).fuel_t, n(904));
    // Men ashore at the end of each day.
    let at = |day: usize, hex: Hex| -> u32 {
        g.truth[day]
            .land_units
            .values()
            .filter(|u| u.hex == Some(hex))
            .map(|u| u.men_ashore)
            .sum()
    };
    for day in 1..=16usize {
        let (a, c, b) = match day {
            1 | 2 => (1800, 0, 0),
            3 | 4 => (1200, 0, 0),
            5 | 6 => (1200, 600, 0),
            7 => (600, 600, 0),
            8 => (0, 600, 0),
            _ => (0, 600, 1200),
        };
        assert_eq!((at(day, A), at(day, C), at(day, B)), (a, c, b), "day {day}");
    }
    // No man is unfed on any night.
    let fed: Vec<_> = g
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::MenFed {
                side: Side::Allied,
                unfed,
                ..
            } => Some(*unfed),
            _ => None,
        })
        .collect();
    assert!(!fed.is_empty());
    assert!(fed.iter().all(|u| *u == 0), "{fed:?}");
    assert!(!g
        .events
        .iter()
        .any(|e| matches!(e.kind, EventKind::MenLostUnfed { .. })));
}

/// Statement 6: the plan orders of intents 1, 2 and 3 on the mornings of
/// the tables, with every task's maker.
#[test]
fn the_plan_orders() {
    use sim_core::command::{OrderTask, Phase, PhaseEnd, PhaseName, PhaseStage, Waited, WaitedFor};
    use PhaseStage::{Done, ToCome, UnderWay};
    let g = main_game();
    let phases = |at: Hex, prepare, moving, land| {
        vec![
            Phase {
                name: PhaseName::Prepare,
                ends: PhaseEnd::Released,
                stage: prepare,
            },
            Phase {
                name: PhaseName::Move,
                ends: PhaseEnd::FirstManAshore(at),
                stage: moving,
            },
            Phase {
                name: PhaseName::Land,
                ends: PhaseEnd::PlaceIsTheSides(at),
                stage: land,
            },
        ]
    };
    let not_met = |thing, expected_day| Waited {
        thing,
        met: false,
        expected_day,
    };
    let met = |thing| Waited {
        thing,
        met: true,
        expected_day: None,
    };

    // Intent 1: every intent in force beneath it, in the tree's order.
    for day in 1..=16u32 {
        let tasks: Vec<u32> = g
            .order_of(day, 1)
            .tasks
            .iter()
            .map(|t| t.intent.raw())
            .collect();
        let want: Vec<u32> = match day {
            1 => vec![2, 3],
            2..=6 => vec![2, 9, 3, 10, 4, 5, 6, 7, 8],
            7..=10 => vec![2, 9, 3, 10, 4, 5, 11, 6, 7],
            _ => vec![2, 9, 3, 10, 4, 12, 5, 11, 6],
        };
        assert_eq!(tasks, want, "morning {day}");
        // Intents 2 and 3 are the commander's on every morning and every
        // other task is the staff's.
        for raw in &tasks {
            let want = if *raw <= 3 {
                Maker::Commander
            } else {
                Maker::Staff
            };
            assert_eq!(g.makers[&day][raw], want, "morning {day}, intent {raw}");
        }
    }

    // Intent 2, plan 2.
    let two = |day| g.order_of(day, 2);
    for day in 1..=7 {
        assert_eq!(two(day).plan.raw(), 2);
        assert_eq!(two(day).mission, "take the place at (0,6)");
        assert_eq!(two(day).men, None, "no count is held");
        assert!(two(day).waits_for.is_empty(), "morning {day}");
    }
    assert_eq!(two(1).plan_state, PlanState::Draft);
    assert_eq!(two(1).phases, phases(C, ToCome, ToCome, ToCome));
    assert_eq!(two(1).expected_ashore, None);
    assert_eq!(two(2).plan_state, PlanState::Preparing);
    assert_eq!(two(2).phases, phases(C, UnderWay, ToCome, ToCome));
    assert_eq!(two(2).expected_ashore, Some(4));
    for day in 3..=4 {
        assert_eq!(two(day).plan_state, PlanState::Released);
        assert_eq!(two(day).phases, phases(C, Done, UnderWay, ToCome));
        assert_eq!(two(day).expected_ashore, Some(4));
    }
    assert_eq!(two(6).plan_state, PlanState::Released);
    assert_eq!(two(6).phases, phases(C, Done, Done, Done));
    assert_eq!(two(6).expected_ashore, None);
    assert_eq!(two(7).plan_state, PlanState::Complete);
    assert_eq!(two(7).phases, phases(C, Done, Done, Done));
    assert_eq!(two(7).expected_ashore, None);

    // Intent 3, plan 3.
    let three = |day| g.order_of(day, 3);
    for day in 1..=11 {
        assert_eq!(three(day).plan.raw(), 3);
        assert_eq!(three(day).mission, "take the place at (4,0)");
        assert_eq!(three(day).men, None, "no count is held");
    }
    let both_unmet = vec![
        not_met(WaitedFor::Day(4), Some(4)),
        not_met(WaitedFor::Intent(intent(2)), None),
    ];
    assert_eq!(three(1).plan_state, PlanState::Draft);
    assert_eq!(three(1).phases, phases(B, ToCome, ToCome, ToCome));
    assert_eq!(three(1).waits_for, both_unmet);
    assert_eq!(three(1).expected_ashore, None);
    for day in 2..=3 {
        assert_eq!(three(day).plan_state, PlanState::Preparing);
        assert_eq!(three(day).phases, phases(B, UnderWay, ToCome, ToCome));
        assert_eq!(three(day).waits_for, both_unmet, "morning {day}");
        assert_eq!(three(day).expected_ashore, None);
    }
    for day in 4..=5 {
        assert_eq!(three(day).plan_state, PlanState::Preparing);
        assert_eq!(three(day).phases, phases(B, UnderWay, ToCome, ToCome));
        assert_eq!(
            three(day).waits_for,
            vec![
                met(WaitedFor::Day(4)),
                not_met(WaitedFor::Intent(intent(2)), None)
            ],
            "morning {day}"
        );
        assert_eq!(three(day).expected_ashore, None);
    }
    assert_eq!(three(6).plan_state, PlanState::Preparing);
    assert_eq!(three(6).phases, phases(B, UnderWay, ToCome, ToCome));
    assert_eq!(
        three(6).waits_for,
        vec![met(WaitedFor::Day(4)), met(WaitedFor::Intent(intent(2)))]
    );
    assert_eq!(three(6).expected_ashore, Some(9));
    for day in 7..=9 {
        assert_eq!(three(day).plan_state, PlanState::Released);
        assert_eq!(three(day).phases, phases(B, Done, UnderWay, ToCome));
        assert!(three(day).waits_for.is_empty(), "morning {day}");
        assert_eq!(three(day).expected_ashore, Some(9), "morning {day}");
    }
    assert_eq!(three(10).plan_state, PlanState::Released);
    assert_eq!(three(10).phases, phases(B, Done, Done, Done));
    assert_eq!(three(10).expected_ashore, None);
    assert_eq!(three(11).plan_state, PlanState::Complete);
    assert_eq!(three(11).phases, phases(B, Done, Done, Done));
    assert_eq!(three(11).expected_ashore, None);

    // The one task under intent 2 is intent 9 and under intent 3 intent 10,
    // from morning 2, each in the phase prepare.
    for day in 1..=16u32 {
        let (under_2, under_3) = if day == 1 {
            (vec![], vec![])
        } else {
            (
                vec![OrderTask {
                    intent: intent(9),
                    kind: landing_kind(3, C, None),
                    phase: Some(PhaseName::Prepare),
                }],
                vec![OrderTask {
                    intent: intent(10),
                    kind: landing_kind(1, B, None),
                    phase: Some(PhaseName::Prepare),
                }],
            )
        };
        assert_eq!(g.order_of(day, 2).tasks, under_2, "morning {day}");
        assert_eq!(g.order_of(day, 3).tasks, under_3, "morning {day}");
    }
}

/// Rule 18: the tree `propose` gives for intent 1 on the morning of day
/// 2 has the standing entries of intents 2 and 3 and rule 9's lines.
#[test]
fn the_tree_proposed() {
    let g = play_asking(&text(), &[2]);
    let proposal = g
        .proposed
        .get(&2)
        .expect("a proposal of intent 1 on morning 2");
    // The staff's side of the tree: on the morning of day 2 everything the
    // staff forms has been formed, and rule 9's line for each of his takes is
    // among its decisions to form nothing, in the order of the island's
    // places.
    assert_eq!(proposal.intent, intent(1));
    let lines: Vec<String> = proposal
        .decisions
        .no_orders
        .iter()
        .filter(|r| r.worked.contains("the commander's, plans the place at"))
        .map(|r| r.worked.clone())
        .collect();
    assert_eq!(
        lines,
        vec![
            "intent:3, the commander's, plans the place at (4,0): the staff forms no take beside it.",
            "intent:2, the commander's, plans the place at (0,6): the staff forms no take beside it.",
        ]
    );
    // The staff forms no take of B or of C in it.
    assert!(
        proposal
            .decisions
            .commands
            .iter()
            .all(|c| !matches!(cmd(&c.kind), Cmd::Form(_, IntentKind::TakePlace { .. }))),
        "{:?}",
        proposal
            .decisions
            .commands
            .iter()
            .map(|c| cmd(&c.kind))
            .collect::<Vec<_>>()
    );
}

/// Rule 18, as the agent's tool gives it (`sim_agent::propose`): the tree
/// has the standing entries of intents 2 and 3 before the staff's. That is
/// `tree_json` of `crates/sim-agent`, the other developer's step; the
/// staff's part of the same rule is `the_tree_proposed`.
#[test]
#[ignore = "waits for sim-agent's tree_json (decision 0232, rule 18)"]
fn the_tree_proposed_as_the_tool_gives_it() {
    let held = staffs();
    let mut scenario = load(&text());
    let mut tree = None;
    for _ in 0..2 {
        let day = scenario.world.day() + 1;
        scenario
            .next_day_watched(as_staffs(&held), &mut |world, moment| {
                if matches!(moment, Moment::Orders { .. }) && day == 2 {
                    let view = world.side_view(Side::Allied);
                    tree = Some(
                        sim_agent::propose(&view, held[&Side::Allied].as_ref(), 1)
                            .expect("a proposal"),
                    );
                }
            })
            .expect("a day resolves");
    }
    let tree = tree.expect("asked on morning 2");
    let sub = tree["tree"]["sub_intents"].as_array().expect("sub_intents");
    let first_two: Vec<(u64, u64, &str, bool)> = sub
        .iter()
        .take(2)
        .map(|e| {
            (
                e["number"].as_u64().unwrap_or(0),
                e["parent"].as_u64().unwrap_or(0),
                e["maker"].as_str().unwrap_or(""),
                e["standing"].as_bool().unwrap_or(false),
            )
        })
        .collect();
    assert_eq!(
        first_two,
        vec![(2, 1, "commander", true), (3, 1, "commander", true)],
        "{sub:?}"
    );
    let no_orders = tree["tree"]["no_orders"].as_array().expect("no_orders");
    for line in [
        "intent:3, the commander's, plans the place at (4,0): the staff forms no take beside it.",
        "intent:2, the commander's, plans the place at (0,6): the staff forms no take beside it.",
    ] {
        assert!(
            no_orders
                .iter()
                .any(|r| r.as_str().is_some_and(|t| t.contains(line))),
            "{line}: {no_orders:?}"
        );
    }
}

/// Statement 7: the game with nothing written, its tree, its two
/// landings with no unit or force on both, its 23 orders by day, its
/// supply at the end of the days of its table, and the table of the two
/// games side by side.
#[test]
fn against_the_game_with_nothing_written() {
    let g = empty_game();
    let m = main_game();
    g.nothing_refused();
    // The tree: intents 2 to 10 in the table's order, each the staff's.
    assert_eq!(
        g.formed_intents()
            .into_iter()
            .filter(|(d, ..)| *d == 1)
            .collect::<Vec<_>>(),
        vec![
            (1, 2, 1),
            (1, 3, 1),
            (1, 4, 1),
            (1, 5, 1),
            (1, 6, 1),
            (1, 7, 1),
            (1, 8, 1),
            (1, 9, 2),
            (1, 10, 4)
        ]
    );
    let kinds = [
        (2, take_kind(B, None), 2),
        (3, hold_kind(2), 1),
        (4, take_kind(C, None), 3),
        (5, hold_kind(4), 1),
        (6, deny_kind(), 1),
        (7, neutralise_kind(2), 1),
        (8, neutralise_kind(4), 1),
        (9, landing_kind(1, B, None), 2),
        (10, landing_kind(3, C, None), 3),
    ];
    for (raw, kind, plan) in kinds {
        let i = g.intent_state(raw);
        assert_eq!(i.kind, kind, "intent {raw}");
        assert_eq!(i.maker, Maker::Staff, "intent {raw}");
        assert_eq!(i.plan.raw(), plan, "intent {raw}");
    }
    // Intent 2, the airfield, chooses of everything at the source: task force
    // 1 with units 1 and 2; unit 3 is left out. Intent 4, the anchorage,
    // chooses of what intent 2's landing has not: task force 3 and unit 3.
    assert_eq!(g.staff_limit(1, 9), Some((vec![1], vec![1, 2], Some(1))));
    assert_eq!(g.staff_limit(1, 10), Some((vec![3], vec![3], Some(1))));
    // Intent 4's reason names intent 2 as the take that has task force 1 and
    // units 1 and 2.
    let reason = g.reasons_serving(1, 4);
    assert_says(
        &reason,
        &[
            "task_force:1 is kept for intent:2",
            "land_unit:1 (Placeholder First) is kept for intent:2",
            "land_unit:2 (Placeholder Second) is kept for intent:2",
        ],
    );
    // No unit and no force is on both landings.
    assert_eq!(
        g.book[..2],
        [
            (1, 1, Given::Landing(1, B, vec![1, 2])),
            (2, 1, Given::Landing(3, C, vec![3]))
        ]
    );

    // The 23 orders by day.
    use Given::{Attack, Defend, LoadTo, LoadUnit, Unload};
    let route = |tf: u32, to: Hex| Given::Route(tf, vec![to, A], vec![to]);
    let book: Vec<(u32, u32, Given)> = vec![
        (1, 1, Given::Landing(1, B, vec![1, 2])),
        (2, 1, Given::Landing(3, C, vec![3])),
        (3, 2, LoadTo(1, n(54))),
        (4, 2, LoadTo(3, n(27))),
        (5, 3, LoadUnit(1, 1)),
        (6, 3, LoadUnit(3, 3)),
        (7, 3, Attack(1)),
        (8, 3, Attack(3)),
        (9, 4, LoadUnit(1, 2)),
        (10, 4, route(3, C)),
        (11, 4, Unload(3)),
        (12, 4, Attack(2)),
        (13, 5, route(1, B)),
        (14, 5, Unload(1)),
        (15, 6, Defend(1)),
        (16, 6, Defend(2)),
        (17, 6, Defend(3)),
        (18, 9, LoadTo(1, n(90))),
        (19, 9, LoadTo(3, n(45))),
        (20, 10, route(1, B)),
        (21, 10, Unload(1)),
        (22, 10, route(3, C)),
        (23, 10, Unload(3)),
    ];
    assert_eq!(g.book, book);
    let counts: Vec<usize> = (1..=16).map(|d| g.orders(d).len()).collect();
    assert_eq!(counts, vec![2, 2, 4, 4, 2, 3, 0, 0, 2, 4, 0, 0, 0, 0, 0, 0]);
    // The supply's sums: the first run to each place is ordered on day 9.
    let adds: Vec<(u32, String)> = (1..=16)
        .flat_map(|d| {
            g.commands(d).into_iter().filter_map(move |c| match c {
                Cmd::Other(text) if text.starts_with("AddRun") => Some((d, text)),
                _ => None,
            })
        })
        .collect();
    assert_eq!(adds.len(), 2, "{adds:?}");
    assert!(adds.iter().all(|(d, _)| *d == 9));
    assert!(
        adds[0].1.contains("plan: PlanId(4)")
            && adds[0].1.contains("ships: {ShipId(1), ShipId(2)}")
            && adds[0].1.contains("load_t: 90")
    );
    assert!(
        adds[1].1.contains("plan: PlanId(5)")
            && adds[1].1.contains("ships: {ShipId(4)}")
            && adds[1].1.contains("load_t: 45")
    );
    // Supply at the end of each day.
    let table: [(usize, [Option<i64>; 5]); 12] = [
        (1, [Some(991), Some(0), Some(0), None, None]),
        (2, [Some(901), Some(0), Some(0), None, None]),
        (3, [Some(892), Some(0), Some(0), None, None]),
        (4, [Some(889), Some(0), Some(0), None, None]),
        (5, [Some(889), Some(0), Some(0), Some(27), Some(10)]),
        (6, [Some(889), Some(44), Some(0), Some(24), Some(4)]),
        (7, [Some(889), Some(38), Some(0), Some(21), Some(4)]),
        (8, [Some(889), Some(32), Some(0), Some(18), Some(4)]),
        (9, [Some(754), Some(26), Some(0), Some(15), Some(4)]),
        (10, [Some(754), Some(110), Some(0), Some(12), Some(4)]),
        (11, [Some(754), Some(104), Some(45), Some(9), Some(4)]),
        (16, [Some(754), Some(74), Some(30), Some(9), Some(4)]),
    ];
    for (day, row) in table {
        for (b, want) in [1u32, 2, 4, 5, 6].iter().zip(row) {
            let held = g.base(day as u32, *b).map(|base| base.supply_t);
            assert_eq!(held, want.map(n), "day {day}, base {b}");
        }
    }
    // From day 11 to day 16 base 2 pays 6 t and base 4 3 t each night.
    for day in 12..=16u32 {
        let k = i64::from(day - 11);
        assert_eq!(g.supply(day, 2), n(104 - 6 * k), "day {day}");
        assert_eq!(g.supply(day, 4), n(45 - 3 * k), "day {day}");
    }
    assert_eq!(g.ship(16, 1).fuel_t, n(936));
    assert_eq!(g.ship(16, 2).fuel_t, n(936));
    assert_eq!(g.ship(16, 4).fuel_t, n(904));

    // The two set side by side.
    // The anchorage, C, is the side's from the end of day 5 in both.
    assert_eq!(m.changed_side_day(4), Some(5));
    assert_eq!(g.changed_side_day(4), Some(5));
    // The airfield, B: day 9 against day 5.
    assert_eq!(m.changed_side_day(2), Some(9));
    assert_eq!(g.changed_side_day(2), Some(5));
    // The anchorage's plan is complete on the morning of day 6 in both, the
    // airfield's on day 10 against day 6.
    assert_eq!(m.plan_day(2, PlanState::Complete), Some(6));
    assert_eq!(g.plan_day(3, PlanState::Complete), Some(6));
    assert_eq!(m.plan_day(3, PlanState::Complete), Some(10));
    assert_eq!(g.plan_day(2, PlanState::Complete), Some(6));
    // Both places the side's, counted from day 1: 9 days against 5.
    let both = |g: &Played| {
        g.changed_side_day(2)
            .max(g.changed_side_day(4))
            .expect("both change side")
    };
    assert_eq!(both(m), 9);
    assert_eq!(both(g), 5);
    // Orders: 23 in both. Base 1 at the end: 730 t against 754 t. Base 2:
    // 98 t against 74 t.
    assert_eq!(m.book.len(), 23);
    assert_eq!(g.book.len(), 23);
    assert_eq!(m.supply(16, 1), n(730));
    assert_eq!(g.supply(16, 1), n(754));
    assert_eq!(m.supply(16, 2), n(98));
    assert_eq!(g.supply(16, 2), n(74));
}

/// Statement 8, "A mediocre plan waits": days 1 to 8 for intent 3, one
/// "cannot bring enough" on day 1, no unit added, and no take or landing
/// formed for B on any day.
#[test]
fn a_mediocre_plan_waits() {
    let g = play(&mediocre());
    g.nothing_refused();
    // On day 1 plan 3 stays a draft, nothing is formed under intent 3, and
    // the notice "cannot bring enough" is handed in for plan 3, once.
    let cannot: Vec<(u32, NoticeKind, u32)> = g
        .notices()
        .into_iter()
        .filter(|(_, k, p)| *k == NoticeKind::CannotBringEnough && *p == 3)
        .collect();
    assert_eq!(cannot, vec![(1, NoticeKind::CannotBringEnough, 3)]);
    // The standing interrupt fires on day 1.
    let fired: Vec<u32> = g
        .events
        .iter()
        .filter(|e| {
            matches!(
                &e.kind,
                EventKind::InterruptFired {
                    side: Side::Allied,
                    interrupt: sim_core::command::InterruptRef::Standing {
                        kind: sim_core::command::StandingKind::CannotBringEnough,
                        subject: sim_core::command::StandingSubject::Plan(plan),
                    },
                    ..
                } if plan.raw() == 3
            )
        })
        .map(|e| e.day)
        .collect();
    assert_eq!(fired, vec![1], "the interrupt fires once, on day 1");
    for day in 1..=8 {
        // Plan 3 stays a draft: nothing is formed or ordered under it.
        assert!(
            g.plan_changes(3).iter().all(|(d, ..)| *d > day),
            "day {day}"
        );
        for (d, parent, kind) in g.forms() {
            assert!(
                d != day || parent != 3,
                "day {day}: {kind:?} formed under intent 3"
            );
        }
        // The reckoning is the same every morning: the men wanted, 800, and
        // the lift's 600.
        let reason = g.no_orders(day, 3).join(" || ");
        assert_says(&reason, &["800", "600"]);
        // The staff adds no unit: unit 2 is given no order on any day.
        assert!(
            !g.orders(day).iter().any(|o| matches!(
                o,
                Given::LoadUnit(_, 2) | Given::Attack(2) | Given::Defend(2)
            ) || matches!(o, Given::Landing(_, _, roster) if roster.contains(&2))),
            "day {day}: {:?}",
            g.orders(day)
        );
    }
    // No unit 2 on any limit the staff sets.
    for (day, cmd) in g.all_commands() {
        if let Cmd::Limit(_, _, units, _) = cmd {
            assert!(!units.contains(&2), "day {day}");
        }
    }
    // On no day of the 16 is a take of B or a landing under intent 3 formed.
    assert!(g.staff_takes().is_empty());
    assert!(g.forms().iter().all(|(_, parent, _)| *parent != 3));
    assert!(g.plan_changes(3).is_empty(), "plan 3 is a draft to the end");
    // Intent 2 goes as in the main game to day 6, and the staff's first
    // intent of day 6, keep base 4 supplied, is intent 10.
    let m = main_game();
    let anchorage = |g: &Played, day: u32| -> Vec<Given> {
        g.orders(day)
            .into_iter()
            .filter(|o| match o {
                Given::Landing(tf, ..) | Given::LoadTo(tf, _) => *tf == 3,
                Given::LoadUnit(tf, unit) => *tf == 3 && *unit == 3,
                Given::Route(tf, ..) | Given::Unload(tf) => *tf == 3,
                Given::Attack(u) | Given::Defend(u) => *u == 3,
                Given::Other(_) => false,
            })
            .collect()
    };
    for day in 1..=6 {
        assert_eq!(anchorage(&g, day), anchorage(m, day), "day {day}");
    }
    assert_eq!(g.plan_day(2, PlanState::Released), Some(2));
    assert_eq!(g.plan_day(2, PlanState::Complete), Some(6));
    assert_eq!(g.changed_side_day(4), Some(5));
    assert!(
        g.formed_intents().contains(&(6, 10, 5)),
        "{:?}",
        g.formed_intents()
    );
}

/// Statement 8, "A mediocre plan at press": the refusal of press on
/// intent 3 alone; with the island at press, days 1 to 9 for intent 3:
/// the landing with unit 1 alone, 12 t, base 2 the side's at the end of
/// day 8.
#[test]
fn a_mediocre_plan_at_press() {
    // "Risk press" on intent 3 alone, the island at balanced, is refused at
    // intake as wider than its parent allows.
    let alone = format!(
        "{}{}",
        mediocre(),
        limit(1, 3, "limit = \"risk\"\nrisk = \"press\"")
    );
    let g = play(&alone);
    let refused = g.commander_refused();
    assert_eq!(refused.len(), 1, "{refused:?}");
    assert_eq!(refused[0].0, 1);
    assert_eq!(refused[0].2, CommandRefusal::WiderThanParent(intent(1)));

    // With the island at press: the lift's 600 men are enough for the 400.
    let g = play(&mediocre_at_press());
    g.nothing_refused();
    // On day 1 plan 3 is approved to preparing and intent 10, ready a
    // landing by task force 1 at B, is formed under intent 3 with unit 1 and
    // no other.
    assert_eq!(g.plan_day(3, PlanState::Preparing), Some(1));
    assert_eq!(g.intent_state(10).kind, landing_kind(1, B, None));
    assert_eq!(g.intent_state(10).parent, Some(intent(3)));
    assert!(g.formed_intents().contains(&(1, 10, 3)));
    // The staff's limit names what he left blank: the force and the source;
    // the unit is his.
    assert_eq!(g.staff_limit(1, 10), Some((vec![1], vec![], Some(1))));
    assert_eq!(
        g.state.land_units_on_chain(intent(10)),
        Some([LandUnitId::new(1)].into_iter().collect())
    );
    // No notice "cannot bring enough" is handed in.
    assert!(g
        .notices()
        .iter()
        .all(|(_, k, _)| *k != NoticeKind::CannotBringEnough));
    // The margin at press is 0: the supply that lands with the men is 3 x 4
    // = 12 t. Plan 3 is released on the morning of day 6 with a load of 12 t;
    // unit 1 is loaded on day 7; the force sails at hour 168 and arrives at
    // hour 184; ship 1 lands 160 men in the morning and 440 in the afternoon,
    // and then 10 t; base 2 changes side at the end of day 8; the men find
    // nobody and the estimate is cleared; plan 3 is marked complete on day 9.
    assert_eq!(g.plan_day(3, PlanState::Released), Some(6));
    assert!(
        g.orders(6).contains(&Given::LoadTo(1, n(12))),
        "{:?}",
        g.orders(6)
    );
    assert!(
        g.orders(7).contains(&Given::LoadUnit(1, 1)),
        "{:?}",
        g.orders(7)
    );
    assert_eq!(
        g.steaming(1).iter().find(|(_, f)| *f == A).map(|(h, _)| *h),
        Some(n(168))
    );
    assert!(g.arrivals(1).contains(&(B, n(184))), "{:?}", g.arrivals(1));
    let landed: Vec<_> = g
        .landed()
        .into_iter()
        .filter(|(_, _, _, s, _)| *s == 1)
        .collect();
    assert_eq!(
        landed,
        vec![
            (8, Pulse::Morning, 1, 1, 160),
            (8, Pulse::Afternoon, 1, 1, 440)
        ]
    );
    assert!(g
        .supply_landed()
        .iter()
        .any(|(d, p, s, t)| (*d, *p, *s, *t) == (8, Pulse::Afternoon, 1, n(10))));
    assert_eq!(g.changed_side_day(2), Some(8));
    assert!(g.events.iter().any(|e| matches!(
        &e.kind,
        EventKind::GroundFoundEmpty { side: Side::Allied, hex, .. } if *hex == B
    )));
    assert_eq!(g.plan_day(3, PlanState::Complete), Some(9));
}

/// Statement 9, "A plan that has ended": no take of B on any morning
/// from day 2, the two reasons of each morning, no notice of its own,
/// base 2 Japanese at the end.
#[test]
fn a_plan_that_has_ended() {
    let g = play(&cancelled());
    g.nothing_refused();
    // Plan 3 goes from preparing to cancelled on day 2.
    assert!(g
        .plan_changes(3)
        .contains(&(2, PlanState::Preparing, PlanState::Cancelled)));
    // On every morning from day 2 to day 16 the staff forms no take of B.
    assert!(g.staff_takes().is_empty(), "{:?}", g.staff_takes());
    for day in 2..=16 {
        // The reason under intent 3 says its plan is cancelled and nothing
        // is formed or ordered under it.
        let under = g.no_orders(day, 3).join(" || ");
        assert_says(
            &under,
            &[
                "plan:3",
                "cancelled",
                "forms nothing and gives no order under intent:3",
            ],
        );
        assert!(
            g.forms()
                .iter()
                .all(|(d, parent, _)| !(*d == day && *parent == 3)),
            "day {day}"
        );
        assert!(
            g.handed[&day]
                .orders
                .iter()
                .all(|o| o.intent != intent(3) && o.intent != intent(10)),
            "day {day}"
        );
        // The island's reason says that intent 3, the commander's, plans the
        // place, that its plan is cancelled, that the staff forms no take
        // beside it and the island cannot be controlled while the place is
        // his to take, and what would change it.
        let island = g.no_orders(day, 1).join(" || ");
        assert_says(
            &island,
            &[
                "intent:3, the commander's, plans the place at (4,0)",
                "its plan is cancelled",
                "the staff forms no take beside it",
                "intent:1 cannot be controlled while the place is his to take",
                "intent:3 set again, which drafts it a new plan",
                "intent:3 withdrawn",
            ],
        );
        // No notice of its own is handed in for it: the neutralisings and
        // the denial have handed in "cannot meet" for plan 1 (rule 14).
        assert!(
            !island.contains("hands in the notice that it cannot meet the intent"),
            "day {day}"
        );
    }
    // On days 2 to 5 the three notices are the main game's, the neutralisings'
    // and the denial's, and no more.
    for day in 2..=5 {
        let cannot = g.handed[&day]
            .notices
            .iter()
            .filter(|nt| nt.kind == NoticeKind::CannotMeet)
            .count();
        assert_eq!(cannot, 3, "day {day}");
    }
    // Base 2 is Japanese at the end. Task force 1 is given no order by the
    // take from day 2.
    assert_eq!(
        g.truth.last().expect("a last day").bases[&BaseId::new(2)].side,
        Side::Japanese
    );
    assert_eq!(g.changed_side_day(2), None);
}

/// Statement 9, "Given back to the staff": the second take refused on
/// day 3; days 2 to 4; intent 11 and plan 4 on day 4, its landing ready
/// and released that morning; base 2 the side's at the end of day 7.
#[test]
fn given_back_to_the_staff() {
    let g = play(&given_back());
    // On day 3 the second take of B, intent 20, is refused as "Already
    // planned", naming intent 3, and changes nothing but the log.
    let refused = g.commander_refused();
    assert_eq!(refused.len(), 1, "{refused:?}");
    assert_eq!(refused[0].0, 3);
    assert_eq!(refused[0].2, CommandRefusal::AlreadyPlanned(intent(3), B));
    assert!(refused[0]
        .2
        .to_string()
        .contains("intent:3 already plans the place at (4,0)"));
    // On day 4 intent 3 is withdrawn, and intent 10 with it.
    assert!(g.withdrawn_with().contains(&(4, 10, 3)));
    assert_eq!(g.intent_state(3).withdrawn_day, Some(4));
    // That morning the staff forms intent 11, take the place at B, under
    // intent 1, with plan 4, and approves it to preparing; forms intent 12,
    // ready a landing by task force 1 at B, under it, with units 1 and 2;
    // hands in the notice that the landing is ready and approves plan 4 to
    // released; and orders task force 1 to load supply to 54 t.
    assert!(g.formed_intents().contains(&(4, 11, 1)));
    assert!(g.formed_intents().contains(&(4, 12, 11)));
    assert_eq!(g.intent_state(11).kind, take_kind(B, None));
    assert_eq!(g.intent_state(11).maker, Maker::Staff);
    assert_eq!(g.intent_state(11).plan.raw(), 4);
    assert_eq!(g.intent_state(12).kind, landing_kind(1, B, None));
    assert_eq!(g.staff_limit(4, 12), Some((vec![1], vec![1, 2], Some(1))));
    assert_eq!(g.plan_day(4, PlanState::Preparing), Some(4));
    assert_eq!(g.plan_day(4, PlanState::Released), Some(4));
    assert!(g.notices().contains(&(4, NoticeKind::Ready, 4)));
    // It gives no second landing order: the force's loading and the units'
    // rehearsal of day 1 stand.
    assert!(
        g.orders(4).contains(&Given::LoadTo(1, n(54))),
        "{:?}",
        g.orders(4)
    );
    assert!(
        g.all_orders()
            .iter()
            .filter(|(d, _)| *d >= 4)
            .all(|(_, o)| !matches!(o, Given::Landing(1, _, _))),
        "{:?}",
        g.all_orders()
    );
    // The order numbers: the 54 t load is order 6.
    let load = g
        .book
        .iter()
        .find(|(_, d, o)| *d == 4 && *o == Given::LoadTo(1, n(54)))
        .expect("the 54 t load");
    assert_eq!(load.0, 6, "{:?}", g.book);
    assert_eq!(g.supply(4, 1), n(886));
    // Unit 1 is loaded on day 5 and unit 2 on day 6; the force sails at hour
    // 144; base 2 changes side at the end of day 7; plan 4 is marked complete
    // on day 8.
    assert!(
        g.orders(5).contains(&Given::LoadUnit(1, 1)),
        "{:?}",
        g.orders(5)
    );
    assert!(
        g.orders(6).contains(&Given::LoadUnit(1, 2)),
        "{:?}",
        g.orders(6)
    );
    assert_eq!(
        g.steaming(1).iter().find(|(_, f)| *f == A).map(|(h, _)| *h),
        Some(n(144))
    );
    assert_eq!(g.changed_side_day(2), Some(7));
    assert_eq!(g.plan_day(4, PlanState::Complete), Some(8));
}

/// Statement 9, "Taken and done": with each plan complete his take
/// stays in force and the staff forms no take of the place.
#[test]
fn taken_and_done() {
    let g = main_game();
    // From the morning of day 10 to the end intent 3 is in force with plan 3
    // complete, and so is intent 2 from day 6.
    for (raw, plan, done) in [(2u32, 2u32, 6u32), (3, 3, 10)] {
        let i = g.intent_state(raw);
        assert!(i.in_force(), "intent {raw} stays in force");
        assert_eq!(g.plan_day(plan, PlanState::Complete), Some(done));
        assert_eq!(g.state.plans[&i.plan].state, PlanState::Complete);
    }
    // On no morning does the staff form a take of B or of C.
    assert!(g.staff_takes().is_empty());
    // The reason of each morning gives the plans as complete and says nothing
    // of a take being formed beside them.
    for day in 11..=16 {
        for raw in [2, 3] {
            let reason = g.no_orders(day, raw).join(" || ");
            assert_says(&reason, &["is complete", "forms nothing"]);
        }
        let island = g.no_orders(day, 1).join(" || ");
        assert!(
            !island.contains("the staff forms no take beside it"),
            "day {day}"
        );
    }
}

/// Statement 10, "The island's date": the landings formed for the
/// mornings of day 10 and day 9, and no `late`.
#[test]
fn the_islands_date() {
    let g = play(&island_dated(12));
    g.nothing_refused();
    // Intent 9 is formed as "ready a landing by task force 3 at C by the
    // morning of day 10", 12 less the 2 days to embark and the day of passage,
    // plus 1; intent 10 as "ready a landing by task force 1 at B by the
    // morning of day 9", 12 - 3 - 1 + 1.
    assert_eq!(g.intent_state(9).kind, landing_kind(3, C, Some(10)));
    assert_eq!(g.intent_state(10).kind, landing_kind(1, B, Some(9)));
    assert!(g.formed_intents().contains(&(1, 9, 2)));
    assert!(g.formed_intents().contains(&(1, 10, 3)));
    // His takes keep no date of their own.
    assert_eq!(g.intent_state(2).kind, take_kind(C, None));
    assert_eq!(g.intent_state(3).kind, take_kind(B, None));
    // No notice `late` is handed in on day 1.
    assert!(
        g.notices()
            .iter()
            .all(|(d, k, _)| !(*d == 1 && *k == NoticeKind::Late)),
        "{:?}",
        g.notices()
    );
}

/// Statement 10, "His own later date": the landing at B formed for the
/// morning of day 11, and `late` once for plan 1 on day 1 with its
/// reason.
#[test]
fn his_own_later_date() {
    let g = play(&later_date_text());
    g.nothing_refused();
    // His date is kept: intent 10 is formed "by the morning of day 11", 14 -
    // 3 - 1 + 1; intent 9 by the island's 12.
    assert_eq!(g.intent_state(9).kind, landing_kind(3, C, Some(10)));
    assert_eq!(g.intent_state(10).kind, landing_kind(1, B, Some(11)));
    // On day 1 the notice `late` is handed in once for plan 1; none on day 2.
    let late: Vec<(u32, NoticeKind, u32)> = g
        .notices()
        .into_iter()
        .filter(|(_, k, p)| *k == NoticeKind::Late && *p == 1)
        .collect();
    assert_eq!(late, vec![(1, NoticeKind::Late, 1)]);
    // Its reason names intent 3, day 12, day 14 and the 2 days between.
    let reason = g.no_orders(1, 1).join(" || ");
    assert_says(&reason, &["intent:3", "day 12", "day 14", "2 days"]);
    // The reason is given on day 2 as well, with no second notice.
    let day2 = g.no_orders(2, 1).join(" || ");
    assert_says(&day2, &["intent:3", "day 12", "day 14", "2 days"]);
}

/// Statement 11, "Before the island is released": days 1 to 4, base 4
/// the side's at the end of day 7 and plan 3 released on day 8.
#[test]
fn before_the_island_is_released() {
    let g = play(&launch_withheld());
    let m = main_game();
    g.nothing_refused();
    // On days 1 and 2 plan 1 is approved to preparing (day 1) and the notice
    // that it waits for leave to launch is handed in each morning; plans 2
    // and 3 stay drafts; nothing is formed and no order given.
    assert_eq!(g.plan_day(1, PlanState::Preparing), Some(1));
    for day in 1..=2 {
        let waits: Vec<_> = g.handed[&day]
            .notices
            .iter()
            .filter(|nt| {
                nt.kind == NoticeKind::WaitsForLeave(sim_core::command::Leave::Launch)
                    && nt.plan.raw() == 1
            })
            .collect();
        assert_eq!(waits.len(), 1, "day {day}");
        assert!(g.orders(day).is_empty(), "day {day}");
        assert!(g.forms().iter().all(|(d, ..)| *d != day), "day {day}");
        for raw in [2, 3] {
            let reason = g.no_orders(day, raw).join(" || ");
            assert_says(&reason, &["waits for the plan of intent:1"]);
        }
    }
    assert_eq!(g.commands(1), vec![Cmd::Approve(1)]);
    assert!(g.commands(2).is_empty());
    assert!(g.plan_changes(2).iter().all(|(d, ..)| *d >= 3));
    assert!(g.plan_changes(3).iter().all(|(d, ..)| *d >= 3));
    // On day 3 the staff does what it does on day 1 of the main game, with
    // the same numbers: plan 1 released, plans 2 and 3 approved to preparing,
    // intents 4 to 10, orders 1 and 2.
    let mut same = m.commands(1);
    same.remove(0);
    assert_eq!(g.commands(3), same);
    assert_eq!(g.plan_day(1, PlanState::Released), Some(3));
    assert_eq!(g.plan_day(2, PlanState::Preparing), Some(3));
    assert_eq!(g.plan_day(3, PlanState::Preparing), Some(3));
    assert_eq!(g.orders(3), m.orders(1));
    assert_eq!(
        g.book[..2],
        [
            (1, 3, Given::Landing(1, B, vec![1, 2])),
            (2, 3, Given::Landing(3, C, vec![3]))
        ]
    );
    // On day 4 plan 2 is released and task force 3 ordered to load 27 t (base
    // 1 holds 937 t at the end of the day); intent 3's reason names "after
    // intent 2" alone, its day having come.
    assert_eq!(g.plan_day(2, PlanState::Released), Some(4));
    assert_eq!(g.orders(4), vec![Given::LoadTo(3, n(27))]);
    assert_eq!(g.supply(4, 1), n(937));
    let reason = g.reasons_serving(4, 3);
    assert_says(
        &reason,
        &["the place of intent:2 at (0,6) is not yet the side's"],
    );
    assert!(!reason.contains("it is not to go before day 4"), "{reason}");
    // Base 4 changes side at the end of day 7 and plan 3 is released on day 8.
    assert_eq!(g.changed_side_day(4), Some(7));
    assert_eq!(g.plan_day(3, PlanState::Released), Some(8));
}

/// Statement 12, "Refused at intake": each command of the list refused
/// with its reason and words, changing nothing but the log.
#[test]
fn refused_at_intake() {
    // Each alone: refused with the reason, the game otherwise exactly the
    // game without the command (only the log differs).
    let check =
        |with_it: String, without: String, day: u32, reason: CommandRefusal, words: &str| {
            let a = play(&with_it);
            let b = play(&without);
            let refused = a.commander_refused();
            assert_eq!(refused.len(), 1, "{refused:?}");
            assert_eq!(refused[0].0, day, "{refused:?}");
            assert_eq!(refused[0].2, reason, "{refused:?}");
            assert!(refused[0].2.to_string().contains(words), "{}", refused[0].2);
            assert_eq!(format!("{:?}", a.truth), format!("{:?}", b.truth));
            assert_eq!(a.handed, b.handed);
            assert_eq!(a.book, b.book);
            assert!(b.commander_refused().is_empty());
        };
    let none = nothing_written();
    // A parent not in force: intent 7 is not set.
    check(
        format!("{none}{}", set_intent(2, Some(7), "[4, 0]")),
        none.clone(),
        1,
        CommandRefusal::NoSuchIntent(intent(7)),
        "intent:7 is not in force",
    );
    // A parent the staff formed: intent 4 is the staff's hold of base 2.
    check(
        format!("{}{}", text(), set_intent_on(2, 20, Some(4), "[4, 0]")),
        text(),
        2,
        CommandRefusal::CannotStandUnder(intent(4)),
        "intent:4",
    );
    // A number not above the parent's: the island is stated as intent 5.
    let five = changed(
        &none,
        "intent = 1\nintent_kind = \"control_island\"",
        "intent = 5\nintent_kind = \"control_island\"",
    );
    check(
        format!("{five}{}", set_intent(2, Some(5), "[4, 0]")),
        five.clone(),
        1,
        CommandRefusal::IntentNumberNotUsable(intent(2)),
        "intent:2",
    );
    // A pair not open: a hold under the island.
    check(
        format!(
            "{none}{}",
            command_table(
                1,
                "kind = \"set_intent\"\nintent = 2\nparent = 1\nintent_kind = \"hold\"\nbase = 2"
            )
        ),
        none.clone(),
        1,
        CommandRefusal::CannotStandUnder(intent(1)),
        "intent:1",
    );
    // A pair not open: a take under a hold.
    let root_hold = format!(
        "{none}{}",
        command_table(
            1,
            "kind = \"set_intent\"\nintent = 2\nintent_kind = \"hold\"\nbase = 1"
        )
    );
    check(
        format!("{root_hold}{}", set_intent(3, Some(2), "[4, 0]")),
        root_hold.clone(),
        1,
        CommandRefusal::CannotStandUnder(intent(2)),
        "intent:2",
    );
    // A place that is not a place of the island.
    check(
        format!("{none}{}", set_intent(2, Some(1), "[0, 0]")),
        none.clone(),
        1,
        CommandRefusal::CannotStandUnder(intent(1)),
        "intent:1",
    );
    // A place a take already plans, the staff's: intent 2 is its take of B.
    check(
        format!("{none}{}", set_intent_on(2, 20, Some(1), "[4, 0]")),
        none.clone(),
        2,
        CommandRefusal::AlreadyPlanned(intent(2), B),
        "intent:2 already plans the place at (4,0)",
    );
    // And a root of his own, stated before it.
    let root_take = format!("{none}{}", set_intent(2, None, "[4, 0]"));
    check(
        format!("{root_take}{}", set_intent(3, Some(1), "[4, 0]")),
        root_take.clone(),
        1,
        CommandRefusal::AlreadyPlanned(intent(2), B),
        "intent:2 already plans the place at (4,0)",
    );
    // A parent changed on an intent that stands: intent 2 stands under intent
    // 1, and intent 1 is a root.
    check(
        format!("{}{}", text(), set_intent_on(2, 2, Some(3), "[0, 6]")),
        text(),
        2,
        CommandRefusal::ParentCannotChange(intent(2), Some(intent(1))),
        "intent:2 stands under intent:1 and cannot be moved",
    );
    check(
        format!(
            "{}{}",
            text(),
            command_table(
                2,
                "kind = \"set_intent\"\nintent = 1\nparent = 2\nintent_kind = \"control_island\"\nisland = 1"
            )
        ),
        text(),
        2,
        CommandRefusal::ParentCannotChange(intent(1), None),
        "intent:1 is a root and cannot be moved under another",
    );
}

/// Statement 12, "No wait that waits for itself": the three rings, each
/// refused as "Not an end to wait for".
#[test]
fn no_wait_that_waits_for_itself() {
    let check = |with_it: String, without: String, other: u32| {
        let a = play(&with_it);
        let b = play(&without);
        let refused = a.commander_refused();
        assert_eq!(refused.len(), 1, "{refused:?}");
        assert_eq!(refused[0].0, 1);
        assert_eq!(
            refused[0].2,
            CommandRefusal::NotAnEndToWaitFor(intent(other))
        );
        assert_eq!(format!("{:?}", a.truth), format!("{:?}", b.truth));
        assert_eq!(a.handed, b.handed);
        assert_eq!(a.book, b.book);
    };
    // "After intent 1" on intent 2, its own island.
    check(
        with(&limit(1, 2, "limit = \"after\"\nother_intent = 1")),
        text(),
        1,
    );
    // "After intent 2" on intent 1, a take beneath it.
    check(
        with(&limit(1, 1, "limit = \"after\"\nother_intent = 2")),
        text(),
        2,
    );
    // The ring through a root take: intent 4 a root take of the place at
    // (0,0), waiting for intent 2; then "after intent 4" on intent 1, which
    // intent 2 would read.
    let root = format!(
        "{}{}{}",
        text(),
        set_intent(4, None, "[0, 0]"),
        limit(1, 4, "limit = \"after\"\nother_intent = 2")
    );
    check(
        format!(
            "{root}{}",
            limit(1, 1, "limit = \"after\"\nother_intent = 4")
        ),
        root,
        4,
    );
}

/// Statement 12, "Set again": his take set again with a date and no
/// parent given; his take set again for the other place.
#[test]
fn set_again() {
    // His take set again with a date and no parent given: it stands under
    // intent 1 still, the commander's, and plan 3, approved, is amended.
    let g = play(&with(
        &set_intent_on(2, 3, None, "[4, 0]").replace("hex = [4, 0]", "hex = [4, 0]\nby_day = 12"),
    ));
    assert!(
        g.commander_refused().is_empty(),
        "{:?}",
        g.commander_refused()
    );
    let i = g.intent_state(3);
    assert_eq!(i.parent, Some(intent(1)));
    assert_eq!(i.maker, Maker::Commander);
    assert_eq!(i.kind, take_kind(B, Some(12)));
    assert_eq!(i.plan.raw(), 3);
    let amended = g.events.iter().any(|e| {
        e.day == 2
            && matches!(&e.kind, EventKind::PlanAmended { side: Side::Allied, plan, .. } if plan.raw() == 3)
    });
    assert!(amended, "plan 3 is amended on day 2");

    // With intent 2 alone written on day 1 (take the place at C, parent 1, no
    // limit), intent 2 set again the same day as take the place at B with
    // parent 1: it is the take of B, and on the first morning the staff forms
    // a take of C and none of B.
    let written = format!(
        "{}{}",
        set_intent(2, Some(1), "[0, 6]"),
        set_intent(2, Some(1), "[4, 0]")
    );
    let g = play(&with_written(&text(), &written));
    assert!(
        g.commander_refused().is_empty(),
        "{:?}",
        g.commander_refused()
    );
    assert_eq!(g.intent_state(2).kind, take_kind(B, None));
    assert_eq!(g.intent_state(2).parent, Some(intent(1)));
    assert_eq!(g.intent_state(2).maker, Maker::Commander);
    assert_eq!(g.staff_takes(), vec![(1, C)]);
}

/// Statement 12, "A carrier force he named": days 1 and 2, no carrier
/// task under intent 2, the island's under its denial, the take's
/// reason.
#[test]
fn a_carrier_force_he_named() {
    let g = play_carrier(&carrier_named());
    g.nothing_refused();
    // On day 1 "operate task force 2 from (0,0)" is formed under the denial,
    // intent 6, as intent 7; the neutralisings are intents 8 and 9 and the
    // landings intents 10 and 11.
    assert_eq!(
        g.formed_intents()
            .into_iter()
            .filter(|(d, ..)| *d == 1)
            .collect::<Vec<_>>(),
        vec![
            (1, 4, 1),
            (1, 5, 1),
            (1, 6, 1),
            (1, 7, 6),
            (1, 8, 1),
            (1, 9, 1),
            (1, 10, 2),
            (1, 11, 3)
        ]
    );
    let carrier = IntentKind::OperateCarriers {
        force: TaskForceId::new(2),
        hex: A,
    };
    assert_eq!(g.intent_state(7).kind, carrier);
    assert_eq!(g.intent_state(7).parent, Some(intent(6)));
    // No carrier task is formed under intent 2 on day 1 or day 2, for cover
    // or to look.
    assert!(
        g.forms().iter().all(
            |(_, parent, k)| !(*parent == 2 && matches!(k, IntentKind::OperateCarriers { .. }))
        ),
        "{:?}",
        g.forms()
    );
    // The landing under intent 2 is task force 3's with unit 3.
    assert_eq!(g.intent_state(10).kind, landing_kind(3, C, None));
    assert_eq!(g.staff_limit(1, 10), Some((vec![], vec![], Some(1))));
    assert!(
        g.orders(1).contains(&Given::Landing(3, C, vec![3])),
        "{:?}",
        g.orders(1)
    );
    // Plan 2 is released on day 2 as in the main game.
    assert_eq!(g.plan_day(2, PlanState::Released), Some(2));
    // A reason under intent 2, on day 2 at the latest, says task force 2 is on
    // the carrier mission, that under an island the carriers are the island's,
    // and that it is not this landing's.
    let reasons = format!("{} || {}", g.reasons_serving(1, 2), g.reasons_serving(2, 2));
    assert_says(
        &reasons,
        &[
            "task_force:2 is on the carrier mission",
            "under an island the carriers are the island's",
            "it is not this landing's",
        ],
    );
}

/// Statement 12, "A bombardment force he named": days 1 to 3, no
/// bombard task under intent 2, the island's under its neutralising, the
/// take's reason.
#[test]
fn a_bombardment_force_he_named() {
    let g = play(&bombardment_named());
    g.nothing_refused();
    // On day 1 "bombard base 2 with task force 5" is formed under the
    // neutralising of base 2, intent 7, as intent 11, after the landings,
    // intents 9 and 10.
    assert!(g.formed_intents().contains(&(1, 9, 2)));
    assert!(g.formed_intents().contains(&(1, 10, 3)));
    assert!(
        g.formed_intents().contains(&(1, 11, 7)),
        "{:?}",
        g.formed_intents()
    );
    assert_eq!(
        g.intent_state(11).kind,
        IntentKind::Bombard {
            force: TaskForceId::new(5),
            target: BaseId::new(2)
        }
    );
    // No bombard task is formed under intent 2 on any of days 1 to 3.
    assert!(
        g.forms()
            .iter()
            .all(|(_, parent, k)| !(*parent == 2 && matches!(k, IntentKind::Bombard { .. }))),
        "{:?}",
        g.forms()
    );
    // Its reason of the morning its plan is released, day 2, says task force 5
    // is on the bombardment mission, that under an island the bombardment
    // forces are the island's, and that it is not this landing's.
    assert_eq!(g.plan_day(2, PlanState::Released), Some(2));
    let reason = g.reasons_serving(2, 2);
    assert_says(
        &reason,
        &[
            "task_force:5 is on the bombardment mission",
            "under an island the bombardment forces are the island's",
            "it is not this landing's",
        ],
    );
}

/// Statement 12, "A unit he named and then held": day 1, the landing
/// with unit 1 alone and the reason naming the hold.
#[test]
fn a_unit_he_named_and_then_held() {
    let g = play(&unit_named_and_held());
    g.nothing_refused();
    // Intent 10 is formed with unit 1 alone in task force 1, and the reason
    // says unit 2 is held by the commander's hold of day 1.
    assert_eq!(g.intent_state(10).kind, landing_kind(1, B, None));
    assert_eq!(g.staff_limit(1, 10), Some((vec![1], vec![1], Some(1))));
    let reason = g.reasons_serving(1, 3);
    assert_says(&reason, &["land_unit:2", "held"]);
    // Order 1 is task force 1's landing order with roster 1.
    assert_eq!(g.book[0], (1, 1, Given::Landing(1, B, vec![1])));
}

/// Statement 13, "Withdrawn with its island": the nine `IntentWithdrawn`,
/// plan 2 complete and plan 3 cancelled, five orders in the game, and
/// the end.
#[test]
fn withdrawn_with_its_island() {
    let g = play(&island_withdrawn_text());
    // Intents 2 to 10 are withdrawn with it, each with its IntentWithdrawn.
    let mut with_it: Vec<(u32, u32)> = g
        .withdrawn_with()
        .into_iter()
        .map(|(d, i, _)| (d, i))
        .collect();
    with_it.sort();
    assert_eq!(with_it, (2..=10).map(|i| (4, i)).collect::<Vec<_>>());
    for raw in 1..=10 {
        assert_eq!(g.intent_state(raw).withdrawn_day, Some(4), "intent {raw}");
    }
    // Plan 2 goes from released to complete, plan 3 from preparing to
    // cancelled, and plan 1 from released to complete.
    assert!(g
        .plan_changes(2)
        .contains(&(4, PlanState::Released, PlanState::Complete)));
    assert!(g
        .plan_changes(3)
        .contains(&(4, PlanState::Preparing, PlanState::Cancelled)));
    assert!(g
        .plan_changes(1)
        .contains(&(4, PlanState::Released, PlanState::Complete)));
    // No order is given on day 4 or after: five orders in the game.
    assert_eq!(g.book.len(), 5, "{:?}", g.book);
    for day in 4..=16 {
        assert!(g.orders(day).is_empty(), "day {day}");
    }
    // Task force 3 lies at base 1 at the end with unit 3's 600 men and 27 t
    // aboard; units 1 and 2 are ashore at A; base 1 holds 868 t (940 t at the
    // end of day 4, less 6 t on each of the 12 nights after).
    let end = g.truth.last().expect("a last day");
    assert_eq!(end.men_aboard(LandUnitId::new(3)), 600);
    assert_eq!(end.ships[&ShipId::new(4)].supply_t, n(27));
    assert_eq!(end.task_forces[&TaskForceId::new(3)].from_hex, A);
    for u in [1, 2] {
        assert_eq!(g.unit_state(16, u).men_ashore, 600);
        assert_eq!(g.unit_state(16, u).hex, Some(A));
    }
    assert_eq!(g.supply(4, 1), n(940));
    assert_eq!(g.supply(16, 1), n(868));
}

/// Statement 14: the same start gives the same game; a save on day 8
/// and a resume give the same game; two worlds that differ only in what
/// the Allied side cannot see give the same commands, orders, reasons
/// and plan orders.
#[test]
fn the_same_twice_saved_and_unseen() {
    // The same start gives the same game, hash for hash.
    let a = play(&text());
    assert_eq!(a.handed, main_game().handed);
    assert_eq!(a.hashes, main_game().hashes);
    assert_eq!(a.plan_orders, main_game().plan_orders);

    // A save on the morning of day 8, resumed, is the game not interrupted.
    let through = main_game();
    let rebuilt = {
        let mut scenario = load(&text());
        for day in 1..=7 {
            scenario
                .world
                .submit_staff(Side::Allied, through.handed[&day].clone());
            scenario
                .next_day_watched(Staffs::none(), &mut |_, _| {})
                .expect("a day resolves")
                .expect("a day to resolve");
        }
        scenario
    };
    assert_eq!(rebuilt.world.day_hashes(), &through.hashes[..7]);
    let first: BTreeMap<u32, StaffDecisions> =
        (1..=7).map(|d| (d, through.handed[&d].clone())).collect();
    let resumed = play_from(rebuilt, first, &[]);
    assert_eq!(resumed.hashes, through.hashes);
    assert_eq!(resumed.handed, through.handed);

    // Two worlds that differ only in what the Allied side cannot see give the
    // same commands, orders, reasons and plan orders.
    let hidden = |supply: u32| {
        format!(
            "[[base]]\nid = 9\nname = \"Placeholder base 9\"\nside = \"japanese\"\nq = 3\nr = 4\nsupply_t = {supply}\non_map = false\n\n"
        )
    };
    let x = play(&before_commands(&text(), &hidden(50)));
    let y = play(&before_commands(&text(), &hidden(400)));
    assert_ne!(
        x.truth.last(),
        y.truth.last(),
        "the two worlds do not differ"
    );
    assert_eq!(x.handed, y.handed);
    assert_eq!(x.plan_orders, y.plan_orders);
}

/// The properties, over the main game and every variant: what the staff
/// may not do to a node of his, what each landing uses, and what is kept
/// from the takes beside it and from the supply.
#[test]
fn the_staff_keeps_to_his_plan() {
    // Each property holds in the main game and in every variant.
    for (name, _, _) in every_game() {
        let g = play_every(name);
        let state = &g.state;
        let by_commander = |raw: IntentId| {
            state
                .intents
                .get(&raw)
                .is_some_and(|i| i.maker == Maker::Commander)
        };
        // The commander's takes under the island.
        let takes: Vec<&sim_core::command::Intent> = state
            .intents
            .values()
            .filter(|i| {
                i.maker == Maker::Commander
                    && i.parent.is_some()
                    && matches!(i.kind, IntentKind::TakePlace { .. })
            })
            .collect();
        // The plan state of plan `p` at the start of `day`.
        let plan_before = |p: sim_core::command::PlanId, day: u32| {
            g.plan_changes(p.raw())
                .into_iter()
                .rfind(|(d, ..)| *d < day)
                .map_or(PlanState::Draft, |(_, _, to)| to)
        };
        // 1. No command the staff hands in withdraws, limits or cancels an
        // intent the commander made, or marks its plan failed.
        for (day, handed) in &g.handed {
            for c in &handed.commands {
                match &c.kind {
                    CommandKind::WithdrawIntent { intent } => {
                        assert!(
                            !by_commander(*intent),
                            "{name}, day {day}: withdraws {intent}"
                        )
                    }
                    CommandKind::SetLimit(l) => {
                        let on = match l {
                            Limit::Forces { intent, .. } => Some(*intent),
                            _ => None,
                        };
                        if let Some(on) = on {
                            assert!(!by_commander(on), "{name}, day {day}: limits {on}");
                        }
                    }
                    CommandKind::CancelPlan { plan } => {
                        let owner = state.plans[plan].intent;
                        assert!(!by_commander(owner), "{name}, day {day}: cancels {plan}");
                    }
                    CommandKind::MarkPlan { plan, outcome } => {
                        let owner = state.plans[plan].intent;
                        assert!(
                            !by_commander(owner) || *outcome == PlanOutcome::Complete,
                            "{name}, day {day}: marks {plan}"
                        );
                    }
                    _ => {}
                }
            }
        }
        // 2. No take of a place is formed while his take of it is in force and
        // its plan is not complete.
        for (day, _, kind) in g.forms() {
            let IntentKind::TakePlace { hex, .. } = kind else {
                continue;
            };
            for t in &takes {
                let IntentKind::TakePlace { hex: h, .. } = t.kind else {
                    continue;
                };
                let in_force = t.set_day <= day && t.withdrawn_day.is_none_or(|w| w > day);
                if h == hex && in_force {
                    let state = plan_before(t.plan, day);
                    assert!(
                        state == PlanState::Complete,
                        "{name}, day {day}: a take of {hex} formed beside {} ({state:?})",
                        t.id
                    );
                }
            }
        }
        // 3 and 4. Every unit and force a landing of his uses is one he named,
        // where he named that kind, and none that another node of his names.
        let named = |t: &sim_core::command::Intent| -> (Vec<TaskForceId>, Vec<LandUnitId>) {
            t.limits.forces.as_ref().map_or((vec![], vec![]), |f| {
                (
                    f.task_forces.iter().copied().collect(),
                    f.land_units.iter().copied().collect(),
                )
            })
        };
        for t in &takes {
            let (tfs, units) = named(t);
            for landing in state.intents.values().filter(|i| {
                i.parent == Some(t.id) && matches!(i.kind, IntentKind::ReadyLanding { .. })
            }) {
                let IntentKind::ReadyLanding { force, .. } = landing.kind else {
                    continue;
                };
                if !tfs.is_empty() {
                    assert!(tfs.contains(&force), "{name}: {} uses {force}", landing.id);
                }
                let used: Vec<LandUnitId> = state
                    .land_units_on_chain(landing.id)
                    .map(|u| u.into_iter().collect())
                    .unwrap_or_default();
                if !units.is_empty() {
                    assert!(
                        used.iter().all(|u| units.contains(u)),
                        "{name}: {} uses {used:?}",
                        landing.id
                    );
                }
                for other in takes.iter().filter(|o| o.id != t.id) {
                    let (otfs, ounits) = named(other);
                    // A thing both name is the first served's: not a breach of
                    // the landing that has it.
                    if !tfs.contains(&force) {
                        assert!(
                            !otfs.contains(&force),
                            "{name}: {} uses {force}, named by {}",
                            landing.id,
                            other.id
                        );
                    }
                    for u in &used {
                        if !units.contains(u) {
                            assert!(
                                !ounits.contains(u),
                                "{name}: {} uses {u}, named by {}",
                                landing.id,
                                other.id
                            );
                        }
                    }
                }
            }
        }
        // 5. No unit and no task force is on two landings under the island
        // whose takes' plans have not ended at the same time.
        let live = |landing: &sim_core::command::Intent| -> (u32, u32) {
            let take = landing.parent.expect("a landing has a take");
            let plan = state.intents[&take].plan;
            let ended = g
                .plan_changes(plan.raw())
                .into_iter()
                .find(|(_, _, to)| {
                    matches!(
                        to,
                        PlanState::Complete | PlanState::Cancelled | PlanState::Failed
                    )
                })
                .map_or(u32::MAX, |(d, ..)| d);
            (landing.set_day, ended)
        };
        let landings: Vec<&sim_core::command::Intent> = state
            .intents
            .values()
            .filter(|i| matches!(i.kind, IntentKind::ReadyLanding { .. }))
            .filter(|i| i.parent.and_then(|p| state.island_on_chain(p)).is_some())
            .collect();
        for (i, a) in landings.iter().enumerate() {
            for b in &landings[i + 1..] {
                let (a0, a1) = live(a);
                let (b0, b1) = live(b);
                if a0.max(b0) >= a1.min(b1) {
                    continue;
                }
                let (
                    IntentKind::ReadyLanding { force: fa, .. },
                    IntentKind::ReadyLanding { force: fb, .. },
                ) = (a.kind, b.kind)
                else {
                    continue;
                };
                assert_ne!(fa, fb, "{name}: {} and {} are both {fa}'s", a.id, b.id);
                let ua = state.land_units_on_chain(a.id).unwrap_or_default();
                let ub = state.land_units_on_chain(b.id).unwrap_or_default();
                assert!(
                    ua.is_disjoint(&ub),
                    "{name}: {} and {} share units",
                    a.id,
                    b.id
                );
            }
        }
        // 6. No supply run is added with a ship of a force that is readying a
        // landing.
        for (day, handed) in &g.handed {
            for c in &handed.commands {
                let CommandKind::AddRun { run, .. } = &c.kind else {
                    continue;
                };
                let before = &g.truth[*day as usize - 1];
                for ship in &run.ships {
                    let sim_core::state::ShipLocation::TaskForce(force) =
                        before.ships[ship].location
                    else {
                        continue;
                    };
                    let readying = before.task_forces[&force].preparation.is_some()
                        && landings.iter().any(|l| {
                            matches!(l.kind, IntentKind::ReadyLanding { force: f, .. } if f == force)
                                && live(l).0 <= *day
                                && live(l).1 > *day
                        });
                    assert!(
                        !readying,
                        "{name}, day {day}: a run takes {ship} of {force}, readying a landing"
                    );
                }
            }
        }
    }
}

/// The property of conservation: the men, the ships and the tons, in the
/// main game and the game with nothing written.
#[test]
fn men_ships_and_tons_are_conserved() {
    for g in [main_game(), empty_game()] {
        // The men: 1,800, ashore or aboard, at the end of every day.
        for (day, truth) in g.truth.iter().enumerate().skip(1) {
            let men: u32 = truth
                .land_units
                .values()
                .map(|u| u.men_ashore + truth.men_aboard(u.id))
                .sum();
            assert_eq!(men, 1800, "day {day}");
            // The ships: three, all afloat.
            assert_eq!(truth.ships.len(), 3);
            assert!(truth
                .ships
                .values()
                .all(|s| s.location != sim_core::state::ShipLocation::Sunk));
        }
        // The tons: what the bases and the ships hold at the end of each day
        // is the 1,000 t of the start less what the men ate.
        let mut eaten = Fixed::ZERO;
        let mut by_day: BTreeMap<u32, Fixed> = BTreeMap::new();
        for e in &g.events {
            if let EventKind::MenFed {
                side: Side::Allied,
                fed,
                ..
            } = &e.kind
            {
                let tons = Fixed::from_int(i64::from(*fed)) * fx("0.005");
                *by_day.entry(e.day).or_insert(Fixed::ZERO) += tons;
            }
        }
        for (day, truth) in g.truth.iter().enumerate().skip(1) {
            eaten += by_day.get(&(day as u32)).copied().unwrap_or(Fixed::ZERO);
            let held = truth
                .bases
                .values()
                .filter(|b| b.side == Side::Allied)
                .fold(Fixed::ZERO, |sum, b| sum + b.supply_t)
                + truth
                    .ships
                    .values()
                    .fold(Fixed::ZERO, |sum, s| sum + s.supply_t);
            assert_eq!(held + eaten, n(1000), "day {day}");
        }
    }
}
