//! Scenario 86: fed with the lift. The commander says "control the
//! island". The side's men are at sea in transports whose home is a base
//! with no supply. The staff names the base that has supply as the source
//! on the first morning, sends the transports on to it when the men are
//! ashore, and feeds the place with those same ships.
//!
//! Proves decision 0163, the second part of the first mend of step 3's
//! stage 4, against a hand calculation. It is scenario 77 with task
//! forces 2 and 3 taken away and the lift's home changed. The data is
//! placeholder data: see `data/scenarios/86_fed_with_the_lift.toml`. It
//! runs on the frozen step 1 figures. No enemy man is ashore anywhere, so
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
//! As scenario 77's. A man weighs 0.25 t to land and eats 0.005 t a day,
//! paid at the start of the night pulse by the side's bases in the hex in
//! base order. Supply landed goes into the side's lowest-numbered base in
//! the hex. A size 0 port works by lighter at 20 t an hour for each ship,
//! and on the frozen figures a landing is not slowed by its loading. Day
//! d begins at hour 24 x (d - 1): a night pulse of 12 hours and two
//! daylight pulses of 6. In a pulse ships move before cargo is worked. A
//! force that has landed its load steams on at the start of the next
//! pulse. A hex is 40 nm and the transports make 10 kn and burn 0.1 t of
//! fuel a mile.
//!
//! A working unit ashore at a base with nothing else to do works on its
//! port: unit 2, 400 men, works from day 2, and the rate of unloading
//! there is 1.4 times the lighter's: 28 t an hour for each ship.
//!
//! The staff's figures: at balanced the margin is 5 days; a supply intent
//! with no interval uses 15 days. A run ordered on a morning loads that
//! day, sails the next and arrives that same day; it is ordered when the
//! reading of the day after it would arrive is below the floor's stock.
//! A supply intent is first served on the morning after it is formed. A
//! force found empty at the source on a morning is in the pool from the
//! next morning.
//!
//! ## Inputs
//!
//! The placeholder map of scenario 27: A at (0,0), port size 5; B at
//! (4,0), port size 0; C at (0,6), port size 0. The game is 8 days.
//!
//! | Base | Side | Hex | Supply | Fuel |
//! | ---- | ---- | --- | ------ | ---- |
//! | 1, "Placeholder base A" | Allied | A | 1,000 t | 0 |
//! | 2, "Placeholder airfield B" | Japanese | B | 0 | 0 |
//! | 4, "Placeholder staging C" | Allied | C | 0 | 0 |
//!
//! Base 2 is on the Allied map, with no gun, aircraft or man. No base
//! has a reserve. No estimate and no contact is held. Island 1,
//! "Placeholder island": base 2.
//!
//! Task force 1, Allied, on the transport mission: ships 1 and 2,
//! placeholder troop transports (600 men, 3,000 t of cargo, 1,000 t of
//! fuel, full), at sea at (2,0), stopped. Its home port is base 4. Unit 1
//! is in ship 1 with 30 t of supply; unit 2 is in ship 2. There is no
//! other force.
//!
//! | Unit | Kind | Men | Posture |
//! | ---- | ---- | --- | ------- |
//! | 1, "Placeholder Infantry" | fighting | 600 | defend |
//! | 2, "Placeholder Pioneers" | working | 400 | |
//!
//! The Allied side's commands of day 1: intent 1, control island 1. No
//! forces limit and no risk, so balanced. Leave to prepare and leave to
//! launch are both given.
//!
//! ## Day by day
//!
//! | Day | Hour | The staff's orders | What happens |
//! | --- | --- | --- | --- |
//! | 1 | 0 | Task force 1: `prepare_landing` at B, roster 1; a route to B and on to A, acting at B; an unload order | Sails at hour 0 and arrives at hour 8. In the night's last 4 hours each ship lands 320 men. In the morning ship 1 lands 280 men in 3.5 hours and then 30 t in 1.5 hours; ship 2 lands 80 men in an hour. Base 3, "Landing at (4,0)", is formed. The force steams for A at hour 18 |
//! | 2 | 24 | Unit 1 set to attack | The force arrives at A at hour 34. Base 2 changes side at the end of the day |
//! | 3 | 48 | Unit 1 set to defend | |
//! | 4 | 72 | Task force 1: a load of supply to a weight of 85 t | Ship 1 loads 85 t at A |
//! | 5 | 96 | Task force 1: a route to B and back to A, acting at B, and an unload order | Sails at hour 96 and arrives at hour 112. At 28 t an hour ship 1 lands 56 t in the morning's 2 hours and 29 t in the afternoon. The force steams for A at hour 120 and arrives at hour 136 |
//! | 6 to 8 | | none | |
//!
//! Eight orders in all. No staff order or command is refused.
//!
//! - **Day 1.** The tree is scenario 77's. The take's first lift is task
//!   force 1, a lift as it stands, and its plan is released. The source
//!   of supply is base 1: 1,000 t above its reserve, against nothing at
//!   base 4, and no transport force lies there. The hold's forces limit
//!   is base 1 and no force. The lift is routed to B and on to base 1,
//!   the source of the hold of the place: each ship has 1,000 t of fuel
//!   for a passage of 80 nm and 160 nm, 24 t. The route's reason names
//!   base 1 and says it is the source of supply.
//! - **Day 2.** The judgement is to attack. The hold of base 2 still
//!   waits, and the side has base 3 in the hex with 1,000 men ashore:
//!   "keep base 3 supplied", with a floor of 5 days, is formed under the
//!   hold.
//! - **Day 3.** The take is met and unit 1 is set to defend. Task force 1
//!   lies empty at base 1: the hold's forces limit is set to base 1 and
//!   task force 1. The supply intent is served for the first time, on
//!   the limit as it stood that morning, base 1 and no force. The stock
//!   at B is 25 t, the use is 1,000 x 0.005 = 5 t a day, and the floor's
//!   stock is 25 t. A run ordered today would arrive on day 4, and the
//!   reading of day 5 is 25 - 10 = 15 t, below 25: a run is wanted and
//!   no ship of the limit can carry it. The staff gives no order for it
//!   and gives the reason.
//! - **Day 4.** The limit is base 1 and task force 1. The stock at B is
//!   20 t. A run ordered today arrives on day 5, and the reading of day
//!   6 is 20 - 10 = 10 t, below 25: a run. Its load is the floor's stock
//!   on the day it arrives, 25 t, less that day's reading, 15 t, which
//!   is 10 t; plus 15 days of use, 75 t: 85 t. Ship 1's room covers it.
//! - **Day 5 and after.** No second run is called: with the run landed
//!   the hex holds 95 t at the end of day 5 against a use of 5 t.
//!
//! ## Supply, fuel and men
//!
//! - **Base 3** takes 30 t on the morning of day 1. It pays 5 t for 1,000
//!   men on the nights of days 2, 3, 4 and 5: base 2, Allied from the
//!   end of day 2 and the lower numbered, holds nothing on those nights.
//!   It holds 25, 20, 15 and 10 t after them and ends with 10 t.
//! - **Base 2** takes 85 t on day 5. It pays 5 t on the nights of days
//!   6, 7 and 8: 70 t at the end.
//! - **Base 1** loads out 85 t and feeds nobody: 915 t at the end.
//!   **Base 4** holds nothing throughout, and no force comes to it.
//! - **Fuel.** Each ship steams 80 nm to B and 160 nm to A, then 320 nm
//!   on the run: 560 nm, 56 t. Each ends with 944 t.
//! - **Every man is fed every night.** Unit 1 has 600 men at the end and
//!   unit 2 has 400.
//!
//! As the rule stood before decision 0163 the lift was routed back to
//! base 4, and base 4, the first base at which a force lay empty, was
//! named the source, with no supply: no run was ever sent.
//!
//! # Properties
//!
//! - **A base with nothing cannot be the source.** With base 1's supply
//!   0: no base can be the source, and the reason of day 1 says so. The
//!   hold's forces limit names no source on any day, no "keep supplied"
//!   is formed, and the lift is routed to B and back to base 4, as
//!   before.
//! - **Not the fuel to go on.** With each ship holding 20 t of fuel: the
//!   passage to B burns 8 t and the passage on to A 16 t, 24 t against
//!   20 t. The lift is routed to B and back to base 4, as before, and
//!   the route's reason gives the two figures. Only the orders of day 1
//!   are asserted.
//! - **The commander's source.** With intent 1 given a forces limit that
//!   names base 4 as the source: the source is base 4, as before, and
//!   the lift is routed to B and back to base 4.
//! - **Scenario 77 gives the orders it gave:** its lift's home is the
//!   source, and a transport lies empty there on day 1.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see give the same orders and reasons; a
//!   save on day 4 and a resume give the same game.
