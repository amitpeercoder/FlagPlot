//! Scenario 67: taking a place. The commander names a place, a transport
//! force, three units and the base they sail from. The staff picks the two
//! units the ships can carry, by what each is good at, makes the landing
//! ready, asks leave to launch, and on leave loads supply, embarks the
//! men, sails, lands them on attack and takes an airfield nobody defends.
//!
//! Proves decision 0142, step 3 of stage 3, against a hand calculation.
//! The data is placeholder data: see
//! `data/scenarios/67_taking_a_place.toml`. It runs on the frozen step 1
//! figures, with ground figures of its own that differ from the frozen
//! ones in five: `preparation_per_day` 25, `training_days_per_level` 3,
//! `assault_skill_strength` 0.25, `unprepared_landing_share` 0.5 and
//! `unprepared_landing_fatigue` 40. No enemy man is ashore anywhere, so no
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
//! A force in port with its roster gains 25 of loading a day, to 100. A
//! unit that stands ready on that roster gains 25 of rehearsal a day times
//! its assault factor, to 100: the factor is 1 at assault skill 2 and
//! 1 + 0.25 x 2 = 1.5 at skill 4, which is 37.5 a day. A man weighs 0.25 t
//! to load or land and eats 0.005 t a day. A size 5 port has four berths
//! at 150 t an hour; a size 0 port works by lighter at 20 t an hour. A
//! force lands at the port's rate times 0.5 + 0.5 x its loading over 100.
//! A unit's first men ashore gain 40 x (100 - its rehearsal) / 100 of
//! fatigue. A day is 24 hours: a night pulse of 12 and two daylight pulses
//! of 6. Day d begins at hour 24 x (d - 1). In a pulse ships move before
//! cargo is worked. The men's supply is paid at the start of the night
//! pulse. Loading, rehearsal and a base that changes side are settled at
//! the end of the day.
//!
//! ## Inputs
//!
//! The placeholder map of scenario 27: A at (0,0), a coast hex with port
//! size 5, and B at (4,0), a coast hex with port size 0. A to B is 4
//! hexes: 160 nm, 16 hours at 10 kn. The game is 14 days.
//!
//! | Base | Side | Hex | Supply | Fuel |
//! | ---- | ---- | --- | ------ | ---- |
//! | 1, "Placeholder base A" | Allied | A | 1,000 t | 0 |
//! | 2, "Placeholder airfield B" | Japanese | B | 0 | 0 |
//!
//! Base 2 has no gun, no aircraft and no man. The Allied side holds no
//! estimate of the enemy's men at B.
//!
//! Task force 1, Allied, on the transport mission: ships 1 and 2,
//! placeholder troop transports (600 men, 3,000 t of cargo, 10 kn at
//! cruise, 0.1 t of fuel a mile, 1,000 t of fuel), stopped at base 1,
//! full of fuel, with nothing aboard.
//!
//! Ashore at A, all fighting units, every man fed in full:
//!
//! | Unit | Men | Experience | Assault skill |
//! | ---- | --- | ---------- | ------------- |
//! | 1, "Placeholder Raw" | 600 | 1 | 2 |
//! | 2, "Placeholder Sound" | 600 | 2 | 2 |
//! | 3, "Placeholder Assault" | 600 | 2 | 4 |
//!
//! The Allied side's commands of day 1: intent 1, take the place at B,
//! with a forces limit of task force 1, units 1, 2 and 3 and base 1 as
//! the source; no risk set, so balanced; no date. Leave to prepare is
//! given and leave to launch is withheld. On day 7 the commander gives
//! leave to launch.
//!
//! ## The choice
//!
//! The units in their order: unit 3 (assault 4), unit 2 (assault 2,
//! experience 2), unit 1 (assault 2, experience 1). Two ships of 600
//! berths: unit 3 takes one and unit 2 the other. No ship is left for
//! unit 1: it is left out. No estimate is held, so the staff takes every
//! unit the ships can carry and says that it goes with no count of the
//! enemy.
//!
//! The supply is 1,200 men x 0.005 x 15 days = 90 t. The days to embark
//! are 3: one for the supply and one for each unit. The passage is 16
//! hours: one day.
//!
//! ## Day by day
//!
//! | Day | Begins at hour | The staff's commands and orders | What happens |
//! | --- | --- | --- | --- |
//! | 1 | 0 | Approves plan 1 to preparing. Forms intent 2, ready a landing by task force 1 at B, and sets its forces limit to units 2 and 3. Orders `prepare_landing` at B, roster 2 and 3 | Loading 25. Unit 2's rehearsal 25, unit 3's 37.5 |
//! | 2 | 24 | none | 50; 50; 75 |
//! | 3 | 48 | none | 75; 75; 100 |
//! | 4 | 72 | none | 100; 100; 100 |
//! | 5 | 96 | none. Notices: intent 2 is ready; plan 1 waits for leave to launch | |
//! | 6 | 120 | none. Notice: plan 1 waits for leave to launch | |
//! | 7 | 144 | Approves plan 1 to released. Orders a load of supply to a weight of 90 t | Ship 1 loads 90 t in 90 / 150 = 0.6 hours |
//! | 8 | 168 | Unit 2 set to attack. Load unit 2, no cargo | Ship 1 takes 600 men: 150 t in 1 hour. Complete at hour 169 |
//! | 9 | 192 | Unit 3 set to attack. Load unit 3, no cargo | Ship 2 takes 600 men. Complete at hour 193 |
//! | 10 | 216 | A route to B and back to A, acting at B, and an unload order | Sails at hour 216, arrives at hour 232. The landing, below. Base 2 changes side |
//! | 11 | 240 | Marks plan 1 complete. Notice: intent 1 is met | The last supply is landed at hour 244. The force steams for A at hour 252 |
//! | 12 | 264 | none | The force arrives at A at hour 268 |
//! | 13, 14 | | none | |
//!
//! Eight orders in all and five staff commands. Unit 1 is given no order
//! on any day. No staff order is refused.
//!
//! ## The landing
//!
//! The loading is 100 and for B: the factor is 0.5 + 0.5 x 1 = 1, and
//! each ship works by lighter at 20 t an hour.
//!
//! | Pulse | Hours | Ship 1 | Ship 2 |
//! | ----- | ----- | ------ | ------ |
//! | Day 10, morning | 2 | 40 t: 160 men of unit 2 | 40 t: 160 men of unit 3 |
//! | Day 10, afternoon | 6 | 440 men take 110 t; then 10 t of supply | 440 men take 110 t: done 5.5 hours in, at hour 239.5 |
//! | Day 11, night | 12 | 80 t of supply in 4 hours: complete at hour 244 | |
//!
//! - The first man forms base 3, "Landing at (4,0)", Allied.
//! - Each unit's first men land with a rehearsal of 100 for B: it gains
//!   40 x 0 = 0 fatigue. Its rehearsal is then 0 and it leaves the roster.
//! - At the end of day 10 units 2 and 3 are on attack at B, where the
//!   Japanese side has a base and no man: the Japanese side is beaten
//!   there without a fight and base 2 is Allied.
//! - When the unloading is complete the force's loading is 0 and its
//!   landing order is done.
//! - Each ship burns 16 t each way and has 968 t of fuel at the end. Base
//!   1 has no fuel to give.
//!
//! ## Supply
//!
//! Base 1 feeds the men ashore at A at the start of each night, before
//! that pulse's loading: 1,800 men, 9 t, on days 1 to 8; 1,200 men, 6 t,
//! on day 9; 600 men, 3 t, on days 10 to 14. That is 72 + 6 + 15 = 93 t.
//! With the 90 t loaded on day 7 it holds 847 t after day 7 and 817 t at
//! the end.
//!
//! At B the 10 t landed on the afternoon of day 10 go into base 3, the
//! side's only base there then. On the night of day 11 the 1,200 men are
//! charged 6 t: base 2, now Allied and the lower numbered, holds nothing,
//! and base 3 pays 6 t and holds 4 t. The 80 t landed in that pulse go
//! into base 2, the side's lowest-numbered base in the hex. Base 2 pays
//! 6 t on the nights of days 12, 13 and 14 and holds 62 t at the end.
//! Base 3 holds 4 t.
//!
//! ## At the end
//!
//! Units 2 and 3 are at B with 600 men each, on attack, fed in full, with
//! fatigue 0. Unit 1 is at A with 600 men, experience 1, never ordered.
//! Task force 1 is stopped at A, empty. Plan 1 is complete. Base 2 is
//! Allied.
//!
//! ## Reasons
//!
//! - The reason for the forces limit of day 1 names the units in their
//!   order with each one's assault skill, experience and men; the 1,200
//!   berths in two ships; unit 1 as left out because no ship is left for
//!   it; that no estimate of the enemy is held; the supply, 90 t; and the
//!   days: 3 to embark and 1 of passage.
//! - The reason of days 1 to 4 for giving intent 1 no order gives the
//!   loading, each unit's rehearsal and the morning it expects the
//!   landing to be ready, day 5.
//! - The reason of days 5 and 6 says that the landing is ready and waits
//!   for leave to launch, and that with leave on a morning it sails three
//!   days after and lands that day.
//!
//! # Properties
//!
//! - **Leave to launch given from day 1.** The plan is released on the
//!   morning of day 5 and no notice says it waits. The supply is loaded
//!   on day 5, unit 2 goes aboard on day 6 and unit 3 on day 7, the force
//!   sails on day 8 and arrives at hour 184, base 2 changes side on day 8,
//!   and the plan is marked complete on the morning of day 9.
//! - **Press.** With intent 1 at press and leave to launch given: the
//!   loading is 50, unit 2's rehearsal 50 and unit 3's 75 after day 2, so
//!   the landing is ready and the plan released on the morning of day 3.
//!   The supply is loaded on day 3, unit 2 goes aboard on day 4 and unit
//!   3 on day 5, and the force sails on day 6. The force and both units
//!   went on gaining in port: all three are at 100 by then, the landing
//!   is at 20 t an hour and neither unit gains fatigue. Base 2 changes
//!   side on day 6.
//! - **A third transport.** With ship 3, a third placeholder troop
//!   transport, in task force 1, and leave to launch given: all three
//!   units are chosen. Unit 1 trains on days 1 to 3, is at experience 2,
//!   is set to stand ready on the morning of day 4 and has a rehearsal of
//!   100 after day 7. The plan is released on the morning of day 8. The
//!   supply is 1,800 x 0.005 x 15 = 135 t, loaded on day 8; units 1, 2
//!   and 3 go aboard on days 9, 10 and 11; the force sails on day 12 and
//!   arrives at hour 280; base 2 changes side on day 12 and the plan is
//!   marked complete on the morning of day 13.
//! - **An estimate of the enemy.** With an opening estimate of 400
//!   Japanese men at B, and no man there:
//!   - at balanced the staff wants 800 men: unit 3's 600 are too few and
//!     with unit 2 it has 1,200: both are chosen, as in the main case;
//!   - at press it wants 400: unit 3 alone is chosen. The supply is
//!     600 x 0.005 x 15 = 45 t. With leave to launch given the plan is
//!     released on the morning of day 3, the supply is loaded that day,
//!     unit 3 goes aboard on day 4, the force sails on day 5 and base 2
//!     changes side on day 5.
//!
//!   With an opening estimate of 700 and preserve the force: it wants
//!   2,100 and the ships can carry 1,200. It forms no task, gives no
//!   order, asks no leave, and hands in the notice that it cannot meet
//!   intent 1 on day 1, with a reason that gives the 1,200 against the
//!   2,100.
//! - **A date it can make.** With a date of day 10: the task's date is
//!   the morning of day 10 - 3 - 1 + 1 = 7, the landing is ready on the
//!   morning of day 5, and no notice says it will be late.
//! - **A date it cannot make.** With a date of day 7: the task's date is
//!   the morning of day 4. On the morning of day 1 the staff hands in the
//!   notice that the landing cannot be ready by its date: 1 day late, and
//!   the reason names the loading.
//! - **A place that is the side's already.** With base 2 Allied at the
//!   start: no task is formed, no order is given, and the notice that
//!   intent 1 is met is handed in on day 1.
//! - **A bare shore.** With no base at B: the first man forms base 2,
//!   "Landing at (4,0)", on day 10, the plan is marked complete on the
//!   morning of day 11, and all 90 t go into that base: it holds 10 t
//!   after day 10, 84 t after day 11 and 66 t at the end.
//! - **Aboard already, and at sea.** With task force 1 stopped at sea at
//!   (2,0), units 2 and 3 aboard ships 1 and 2, 90 t of supply aboard
//!   ship 1, unit 1 ashore at A and leave to launch given: on day 1 the
//!   staff chooses units 2 and 3, who keep their ships, and leaves unit 1
//!   out. The landing is ready as it stands, with a loading of 0 and no
//!   rehearsal, and the reason says so. No supply is loaded and no unit
//!   is ordered aboard. The force is routed on day 1 and arrives at hour
//!   8. It lands at 20 x 0.5 = 10 t an hour: 160 men from each ship in
//!   the 4 hours of the night, 240 in the morning, and the last 200 in
//!   the afternoon, when ship 1 also lands 10 t. Each unit gains 40
//!   fatigue as its first men land. On the morning of day 2 both units
//!   are set to attack; base 2 changes side on day 2; the plan is marked
//!   complete on the morning of day 3. The last 80 t are landed in 8
//!   hours of the night of day 2, complete at hour 32.
//! - **An escort.** With task force 2, one placeholder fast transport on
//!   the escort mission, stopped at base 1 and named in the forces limit,
//!   preserve the force, and leave to launch given: on the morning of day
//!   8 the order that gives task force 2 task force 1 as its charge is
//!   handed in before the route order. At balanced, with no contact held,
//!   no escort order is given and the reason says the force sails alone.
//! - **Without leave to prepare** no command and no order is given, and
//!   the notice says the plan waits for leave to prepare.
//! - **A hold.** With task force 1 held from day 1: no task is formed, no
//!   order is given, and the reason names the hold.
//! - **Saved and resumed.** The game saved after day 6 and resumed, and
//!   saved again after day 9 and resumed, ends with the hash of the game
//!   played through.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see give the same orders and reasons;
//!   every scenario built before this one gives the events it gave.
