//! Scenario 68: ashore. The staff lands two units on an airfield that its
//! estimate says 500 men hold. There are 1,200. The first day's fight
//! shows it, the staff stops the attack, rests the men, brings the unit it
//! left behind, and attacks again when the men are enough. The airfield
//! falls.
//!
//! Proves decision 0143, step 4 of stage 3, first part, against a hand
//! calculation. The data is placeholder data: see
//! `data/scenarios/68_ashore.toml`. It runs on the frozen step 1 figures,
//! with ground figures of its own that differ from the frozen ones in
//! eight: `preparation_per_day` 100, `hit_chance` 1, `defender_hit_chance`
//! 1, `men_per_hit` 50, `fatigue_per_attack_day` 50, `rest_per_day` 25,
//! `report_error_low` 1 and `report_error_high` 1. Every shot is a whole
//! shot and hits, and a report gives the true count, so no die changes
//! anything.
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
//! A unit fires one shot for each 100 firing men, and its firing men are
//! its men ashore times its fed share times 100 less its fatigue over
//! 100. Every shot hits and a hit takes 50 men, shared between the units
//! hit in proportion to their men. A unit on attack that fired gains 50
//! of fatigue; any other unit loses 25. A unit on attack whose fatigue is
//! above 60 goes back to the defence. A side is beaten where the other
//! attacked and it has no man left, or fewer than 0.2 of the other's. A
//! base that changes side keeps half its supply. A man weighs 0.25 t to
//! land and eats 0.005 t a day, paid at the start of the night pulse by
//! the side's bases in the hex in base order. A force with its roster in
//! port gains 100 of loading in a day and a unit ready on it 100 of
//! rehearsal. A size 5 port works at 150 t an hour; a size 0 port by
//! lighter at 20 t an hour for each ship. Day d begins at hour
//! 24 x (d - 1): a night pulse of 12 hours and two daylight pulses of 6.
//! In a pulse ships move before cargo is worked. The fight and the
//! reports are at the end of the day.
//!
//! The staff's figures: at balanced it wants 2 men for each man of its
//! estimate, and a unit can attack at a fatigue of 40 or less (two thirds
//! of 60) or if it is on attack already. A unit's fit men are its men
//! ashore times 100 less its fatigue over 100.
//!
//! ## Inputs
//!
//! The placeholder map of scenario 27: A at (0,0), a coast hex with port
//! size 5, and B at (4,0), a coast hex with port size 0, 160 nm apart: 16
//! hours at 10 kn. No land link. The game is 14 days.
//!
//! | Base | Side | Hex | Supply | Fuel |
//! | ---- | ---- | --- | ------ | ---- |
//! | 1, "Placeholder base A" | Allied | A | 1,000 t | 0 |
//! | 2, "Placeholder airfield B" | Japanese | B | 200 t | 0 |
//!
//! Task force 1, Allied, on the transport mission: ships 1 and 2,
//! placeholder troop transports (600 men, 3,000 t of cargo, 10 kn, 0.1 t
//! of fuel a mile, 1,000 t of fuel), stopped at base 1, full of fuel,
//! nothing aboard.
//!
//! | Unit | Side | Where | Men | Experience | Assault skill | Posture |
//! | ---- | ---- | ----- | --- | ---------- | ------------- | ------- |
//! | 1, "Placeholder First" | Allied | A | 600 | 2 | 2 | defend |
//! | 2, "Placeholder Second" | Allied | A | 600 | 2 | 2 | defend |
//! | 3, "Placeholder Third" | Allied | A | 600 | 2 | 2 | defend |
//! | 10, "Placeholder Garrison" | Japanese | B | 1,200 | 2 | | defend |
//!
//! All are fighting units, fed in full, with no fatigue. The Allied side
//! has an opening estimate of 500 enemy men at B. The Japanese side has
//! no staff intent and gives no order.
//!
//! The Allied side's commands of day 1: intent 1, take the place at base
//! 2, with a forces limit of task force 1, units 1, 2 and 3 and base 1 as
//! the source; no risk set, so balanced; no date. Leave to prepare and
//! leave to launch are both given.
//!
//! ## The choice
//!
//! The estimate is 500: the men wanted are 1,000. The units in their
//! order are 1, 2, 3. Unit 1 takes ship 1: 600 men, fewer than 1,000.
//! Unit 2 takes ship 2: 1,200, enough. Unit 3 is left behind. The supply
//! is 1,200 x 0.005 x 15 = 90 t.
//!
//! ## Day by day
//!
//! | Day | Hour | The staff's commands and orders | What happens |
//! | --- | --- | --- | --- |
//! | 1 | 0 | Approves plan 1 to preparing. Forms intent 2, ready a landing by task force 1 at B, and sets its forces limit to units 1 and 2. Orders `prepare_landing` at B, roster 1 and 2 | Loading 100; each unit's rehearsal 100 |
//! | 2 | 24 | Notice: intent 2 is ready. Approves plan 1 to released. Orders a load of supply to a weight of 90 t | Ship 1 loads 90 t |
//! | 3 | 48 | Unit 1 set to attack. Load unit 1 | Ship 1 takes 600 men |
//! | 4 | 72 | Unit 2 set to attack. Load unit 2 | Ship 2 takes 600 men |
//! | 5 | 96 | A route to B and back to A, acting at B, and an unload order | Sails at hour 96, arrives at hour 112. Each ship lands 160 men in the morning and 440 in the afternoon; ship 1 then lands 10 t. Base 3, "Landing at (4,0)", is formed. The first fight |
//! | 6 | 120 | Units 1 and 2 set to defend | Ship 1 lands 80 t in 4 hours. The force steams for A at hour 132. No fight |
//! | 7 | 144 | none | The force arrives at A at hour 148 |
//! | 8 | 168 | Sets the forces limit of intent 2 to units 1, 2 and 3. Orders a load of supply to a weight of 45 t | Ship 1 loads 45 t |
//! | 9 | 192 | Load unit 3 | Ship 1 takes 600 men |
//! | 10 | 216 | A route to B and back to A, acting at B, and an unload order | Sails at hour 216, arrives at hour 232. Ship 1 lands 160 men in the morning, 440 in the afternoon and then 10 t |
//! | 11 | 240 | Units 1, 2 and 3 set to attack | Ship 1 lands 35 t in 1.75 hours. The force steams for A at hour 252. The second fight. Base 2 changes side |
//! | 12 | 264 | Marks plan 1 complete. Notice: intent 1 is met | The force arrives at A at hour 268 |
//! | 13, 14 | | none | |
//!
//! Seventeen orders in all and six staff commands. No staff order is
//! refused.
//!
//! ## The first fight, day 5
//!
//! | | Allied | Japanese |
//! | --- | --- | --- |
//! | Men ashore | 600 and 600 | 1,200 |
//! | Firing men | 600 and 600 | 1,200 |
//! | Shots, all hits | 6 and 6 | 12 |
//! | Men lost | 12 x 50 = 600: 300 each | 12 x 50 = 600 |
//! | Men left | 300 and 300 | 600 |
//! | Fatigue after | 50 and 50 | 0 |
//!
//! The Japanese side has 600 men against 0.2 x 600 = 120: it is not
//! beaten. The Allied report of that evening gives 600 enemy men at B.
//!
//! ## The judgement, morning by morning
//!
//! | Morning | The estimate | Men wanted | Units that can attack, with their fit men | The staff |
//! | --- | --- | --- | --- | --- |
//! | 6 | 600, of day 5 | 1,200 | 1 and 2, on attack, at fatigue 50: 150 each, 300 | Short by 900: both set to defend |
//! | 7 | 600, of day 6 | 1,200 | 1 and 2 at fatigue 25: 225 each, 450 | Short by 750: waits |
//! | 8 | 600 | 1,200 | 1 and 2 at fatigue 0: 300 each, 600 | Short by 600: waits. The force lies at A: unit 3 is to be sent |
//! | 9, 10 | 600 | 1,200 | 600 | Short by 600: waits |
//! | 11 | 600, of day 10 | 1,200 | 1, 2 and 3 at fatigue 0: 300, 300 and 600: 1,200 | Enough: all three set to attack |
//!
//! On the morning of day 8 the men ashore who can attack are 600 and unit
//! 3 has 600: together 1,200, the men wanted. Its supply is
//! 600 x 0.005 x 15 = 45 t, and it takes ship 1.
//!
//! ## The second fight, day 11
//!
//! | | Allied | Japanese |
//! | --- | --- | --- |
//! | Men ashore | 300, 300 and 600 | 600 |
//! | Shots, all hits | 3, 3 and 6 | 6 |
//! | Men lost | 6 x 50 = 300: 75, 75 and 150 | 12 x 50 = 600: all |
//! | Men left | 225, 225 and 450 | 0 |
//! | Fatigue after | 50, 50 and 50 | |
//!
//! Unit 10 is destroyed. The Japanese side has no man at B and is beaten:
//! base 2 is Allied. The Allied men find the hex empty and the estimate
//! is cleared. On the morning of day 12 the side holds a base at B and
//! the enemy none: the intent is met.
//!
//! ## Supply and fuel
//!
//! - **Base 1** feeds the men ashore at A at the start of each night,
//!   before that pulse's loading: 1,800 men, 9 t, on days 1 to 3; 1,200
//!   men, 6 t, on day 4; 600 men, 3 t, on days 5 to 9; none after. That
//!   is 27 + 6 + 15 = 48 t. It loads out 90 t and 45 t. It ends with
//!   1,000 - 48 - 135 = 817 t.
//! - **Base 2,** while Japanese, feeds 1,200 men, 6 t, on days 1 to 5 and
//!   600 men, 3 t, on days 6 to 11: 200 - 30 - 18 = 152 t. It changes
//!   side with half, 76 t. It then feeds the 900 Allied men, 4.5 t a
//!   night, being the lower numbered: 71.5 t, 67 t and 62.5 t after the
//!   nights of days 12, 13 and 14.
//! - **Base 3** is given 10 t on the afternoon of day 5. On the night of
//!   day 6 it pays 3 t for 600 men and then takes 80 t: 87 t. It pays 3 t
//!   on each of days 7 to 10: 75 t; takes 10 t on the afternoon of day
//!   10: 85 t; pays 6 t for 1,200 men on the night of day 11 and takes
//!   35 t: 114 t. Base 2 pays from day 12, so it ends with 114 t.
//! - **Each ship** steams 320 nm twice and burns 64 t: 936 t at the end.
//!
//! # Properties
//!
//! - **No estimate at the start.** With no opening estimate the staff
//!   takes every unit the ships can carry, units 1 and 2, and says it goes
//!   with no count of the enemy. Every order and every number after day 1
//!   is as in the main case: the report of day 5 gives the estimate.
//! - **A garrison the estimate was right about.** With unit 10 at 400
//!   men: on day 5 the twelve hits take all 400, the four shots of the
//!   garrison take 200, and units 1 and 2 are left with 500 each. Base 2
//!   changes side on day 5 with (200 - 2 x 5) / 2 = 95 t. The plan is
//!   marked complete and the notice handed in on the morning of day 6.
//!   Eight orders and five staff commands; unit 3 is never sent.
//! - **Nothing left to send.** With unit 3 held from day 1: the first six
//!   days are as in the main case. From the morning of day 6 the staff
//!   waits, says each morning that it is short and has no unit to bring,
//!   and gives no order after day 6: ten orders in all. The place is not
//!   taken by the end. The force lies at A from hour 148.
//! - **A hold is honoured.** With unit 2 held from day 7: unit 3 is sent
//!   as in the main case. On the morning of day 11 the units that can
//!   attack are 1 and 3, 900 fit men against 1,200 wanted: the staff
//!   waits, short by 300, and names the hold. Fourteen orders in all, and
//!   the place is not taken.
//! - **Scenario 67 is not changed:** it gives the orders and the numbers
//!   it gave.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see (base 2 with 300 t of supply in one)
//!   give the same orders and reasons; a save on day 8 and a resume give
//!   the same game; every scenario built before this one gives the events
//!   it gave.
