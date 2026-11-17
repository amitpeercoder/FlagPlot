//! Scenario 77: forces with men aboard. The commander says "control the
//! island". Its one place is an empty enemy airfield. The side's men are
//! at sea in their transports, as on 7 August 1942. The staff lands
//! them, takes the airfield, sends the builders who become usable on day
//! 4, and feeds the place with the ships that lie empty at the source.
//!
//! Proves decision 0152 against a hand calculation. It stands on
//! scenarios 67, 69 and 76. The data is placeholder data: see
//! `data/scenarios/77_forces_with_men_aboard.toml`. It runs on the frozen
//! step 1 figures. No enemy man is ashore anywhere, so no die is rolled.
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
//! A man weighs 0.25 t to land and eats 0.005 t a day, paid at the start
//! of the night pulse by the side's bases in the hex in base order.
//! Supply landed goes into the side's lowest-numbered base in the hex. A
//! size 0 port works by lighter at 20 t an hour for each ship, and on the
//! frozen figures a landing is not slowed by its loading. Day d begins at
//! hour 24 x (d - 1): a night pulse of 12 hours and two daylight pulses
//! of 6. In a pulse ships move before cargo is worked. A force that has
//! landed its load steams on at the start of the next pulse. A hex is
//! 40 nm and the transports make 10 kn.
//!
//! ## Inputs
//!
//! The placeholder map of scenario 27: A at (0,0), port size 5, and B at
//! (4,0), port size 0. The game is 8 days.
//!
//! Base 1, Allied, at A, with 1,000 t of supply. Base 2, Japanese, at B,
//! on the Allied map, with no gun, aircraft, supply or man. No estimate
//! and no contact is held. Island 1, "Placeholder island": base 2.
//!
//! | Task force | Ships | Where | Home port | Aboard |
//! | --- | --- | --- | --- | --- |
//! | 1 | 1 and 2, placeholder troop transports | at sea at (2,0), stopped | base 1 | unit 1 in ship 1 with 30 t of supply; unit 2 in ship 2 |
//! | 2, usable from day 4 | 3, a placeholder troop transport | in port at base 1 | base 1 | unit 3, with 20 t of supply |
//! | 3 | 4, a placeholder troop transport | stopped at base 1 | base 1 | nothing |
//!
//! All are Allied and on the transport mission.
//!
//! | Unit | Kind | Men | Posture |
//! | ---- | ---- | --- | ------- |
//! | 1, "Placeholder Infantry" | fighting | 600 | defend |
//! | 2, "Placeholder Pioneers" | working | 400 | |
//! | 3, "Placeholder Builders" | working | 300 | |
//!
//! The Allied side's commands of day 1: intent 1, control island 1. No
//! forces limit and no risk, so balanced. Leave to prepare and leave to
//! launch are both given.
//!
//! ## Day by day
//!
//! | Day | Hour | The staff's orders | What happens |
//! | --- | --- | --- | --- |
//! | 1 | 0 | Task force 1: `prepare_landing` at B, roster 1; a route to B and back to A, acting at B; an unload order | Sails at hour 0 and arrives at hour 8. In the night's last 4 hours each ship lands 320 men. In the morning ship 1 lands 280 men in 3.5 hours and then 30 t in 1.5 hours; ship 2 lands 80 men in an hour. Base 3, "Landing at (4,0)", is formed. The force steams for A at hour 18 |
//! | 2 | 24 | Unit 1 set to attack | The force arrives at A at hour 34. Base 2 changes side at the end of the day |
//! | 3 | 48 | Unit 1 set to defend | |
//! | 4 | 72 | Task force 1: a load of supply to a weight of 85 t. Task force 2: a route to B and back to A, acting at B, and an unload order | Task force 2 sails at hour 72 and arrives at hour 88. It lands 160 men in the morning, and in the afternoon 140 men in 1.75 hours and then 20 t in an hour. It steams for A at hour 96 |
//! | 5 | 96 | Task force 1: a route to B and back to A, acting at B, and an unload order | Sails at hour 96, arrives at hour 112. Ship 1 lands 40 t in the morning and 45 t in the afternoon. Task force 2 arrives at A at hour 112 |
//! | 6 to 8 | | none | Task force 1 steams for A at hour 120 and arrives at hour 136 |
//!
//! Ten orders in all. No staff order or command is refused.
//!
//! - **Day 1.** The tree: "take the place at base 2", "hold base 2",
//!   "deny the waters within 3 hexes of (4,0)" and "neutralise base 2"
//!   under intent 1, in that order. The neutralising finds no bomber and
//!   no bombardment force, and the staff withdraws it on the morning of
//!   day 3, base 2 the side's. The take's first lift is task force 1, a lift as it stands:
//!   its fighting unit is unit 1, and unit 2 lands with it. The force
//!   does not lie at a base: its landing is ready as it stands, and it
//!   goes with the 30 t aboard. The plan of the take is released that
//!   morning and the force routed. The source of supply is base 1, where
//!   task force 3 lies empty: the hold's forces limit is base 1 and task
//!   force 3.
//! - **Day 2.** No estimate is held: the judgement is to attack, and unit
//!   1 is ashore and not on attack. At the end of the day the Japanese
//!   side has a base and no man at B, and base 2 is Allied.
//! - **Day 3.** The take is met: its plan is marked complete. The hold of
//!   base 2 is no longer waiting: unit 1 is set to defend, and "keep base
//!   2 supplied", with a floor of 5 days, is formed under it. Task force
//!   1 lies empty at base 1 beside task force 3: the hold's forces limit
//!   is set to base 1 and task forces 1 and 3.
//! - **Day 4.** Task force 2 may be used. It has men aboard, all of a
//!   working unit, and no take is in force: the hold of base 2 sends it.
//!   The supply intent is served for the first time: the stock at B is
//!   20 t, the use is 1,000 x 0.005 = 5 t a day, and the floor's stock is
//!   25 t. A run ordered today arrives on day 5, and the reading of day 6
//!   is 20 - 10 = 10 t, below 25: a run. Its load is the floor's stock on
//!   the day it arrives, 25 t, less that day's reading, 15 t, plus 15
//!   days of use, 75 t: 85 t. The free ships in number order begin with
//!   ship 1, whose room covers it: task force 1 takes the run.
//!
//! ## Supply
//!
//! - **Base 3** takes 30 t on the morning of day 1. It pays 5 t for 1,000
//!   men on the nights of days 2, 3 and 4: base 2, Allied from the end
//!   of day 2 and the lower numbered, holds nothing on those nights. It
//!   ends with 15 t.
//! - **Base 2** takes 20 t on the afternoon of day 4. On the night of day
//!   5 it pays 6.5 t for 1,300 men: 13.5 t. It takes 85 t on day 5:
//!   98.5 t. It pays 6.5 t on the nights of days 6, 7 and 8: 79 t at the
//!   end.
//! - **Base 1** loads out 85 t and feeds nobody: 915 t at the end.
//!
//! ## The measure
//!
//! Base 2 is the side's at the end of each day from day 2. The floor is
//! 5 days. At the end of day 2 the hex holds 25 t at 5 t a day, 5 days:
//! it counts. At the end of day 3 it holds 20 t, 4 days: it does not. At
//! the end of day 4 it holds 35 t at 6.5 t a day, and from day 5 more
//! than 90 t: it counts. The island is controlled on days 2 and 4 to 8:
//! on 6 of 8 days in force.
//!
//! Restated by the architect on Nov 17, 2026, with decision 0151's
//! amendment: the first writing counted any supply at all and gave 7 of
//! 8, and had no neutralising of base 2.
//!
//! # Properties
//!
//! - **A later lift of fighting men.** With unit 3 a fighting unit of
//!   300 men, and unit 10, Japanese, 2,000 men, ashore at B: the take is
//!   not met on day 4, and no estimate or any estimate leaves the men
//!   wanted above the men ashore. On the morning of day 4 the take sends
//!   task force 2, a later lift as it stands, with a route to B and back
//!   and an unload order, unit 3 is added to the forces limit of the
//!   task "ready a landing", and unit 3 is given no posture order before
//!   it sails. Only the orders of days 1 and 4 are asserted: the fights
//!   are not this scenario's.
//! - **Two islands.** With a second island, "Placeholder island 2", of a
//!   Japanese base 4 at C, (0,6), on the Allied map with nothing at it,
//!   intent 2, control island 2, stated on day 1 after intent 1, and
//!   unit 3 a fighting unit of 300 men usable from day 1: on the morning
//!   of day 1 the take of base 2 takes task force 1 as its first lift
//!   and sends no other, its first lift being on its way; the take of
//!   base 4 takes task force 2 as its first lift and routes it to C. No
//!   force but task force 1 is routed to B on day 1. Only the orders of
//!   day 1 are asserted.
//! - **Nothing aboard that can fight.** With unit 1 a working unit: no
//!   lift as it stands has a fighting unit, and no base has a transport
//!   force beside a fighting unit ashore. The take forms nothing, hands
//!   in the notice that it cannot meet its intent on day 1 and gives the
//!   reason each morning. No force is sent: the hold's base is not the
//!   side's.
//! - **A force that waits for its day.** Task force 2 is given no order
//!   before day 4 in any case, and each morning's reason names its day.
//! - **Held.** With task force 2 held from day 1: it is given no order,
//!   and the reason of day 4 names the hold.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see give the same orders and reasons; a
//!   save on day 3 and a resume give the same game; scenarios 67 to 76
//!   give the orders they gave.
